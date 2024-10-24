use crate::{
    application::ports::google_drive_service::{self, GoogleDriveServiceTrait},
    domain::value_objects::file_info::FileInfo,
};
use google_drive3::{hyper_rustls, yup_oauth2::AccessTokenAuthenticator, DriveHub};
use oauth2::{
    basic::{BasicClient, BasicErrorResponseType},
    reqwest::async_http_client,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    StandardErrorResponse, TokenResponse, TokenUrl,
};
use reqwest::{multipart, Client};
use serde::Deserialize;
use serde_json::json;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[derive(Deserialize)]
struct UserInfo {
    email: String,
}

#[derive(Clone)]
pub struct GoogleDriveService {
    client_id: ClientId,
    client_secret: ClientSecret,
    auth_url: AuthUrl,
    token_url: TokenUrl,
    redirect_url: RedirectUrl,
}

impl GoogleDriveService {
    pub async fn new(
        client_id: String,
        client_secret: String,
        auth_url: String,
        token_url: String,
        redirect_url: String,
    ) -> Self {
        Self {
            client_id: ClientId::new(client_id),
            client_secret: ClientSecret::new(client_secret),
            auth_url: AuthUrl::new(auth_url).unwrap(),
            token_url: TokenUrl::new(token_url).unwrap(),
            redirect_url: RedirectUrl::new(redirect_url).unwrap(),
        }
    }

    fn create_oauth_client(
        &self,
    ) -> oauth2::Client<
        StandardErrorResponse<BasicErrorResponseType>,
        oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
        oauth2::basic::BasicTokenType,
        oauth2::StandardTokenIntrospectionResponse<
            oauth2::EmptyExtraTokenFields,
            oauth2::basic::BasicTokenType,
        >,
        oauth2::StandardRevocableToken,
        StandardErrorResponse<oauth2::RevocationErrorResponseType>,
    > {
        BasicClient::new(
            self.client_id.clone(),
            Some(self.client_secret.clone()),
            self.auth_url.clone(),
            Some(self.token_url.clone()),
        )
        .set_redirect_uri(self.redirect_url.clone())
    }
}

impl GoogleDriveServiceTrait for GoogleDriveService {
    async fn get_google_auth_url(&self) -> Result<(String, String), google_drive_service::Error> {
        let client = self.create_oauth_client();

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/drive".to_string(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
            )) // Request Google Drive scope
            // .set_pkce_challenge(PkceCodeChallenge::new_random_sha256().0)
            .url();

        Ok((auth_url.to_string(), csrf_token.secret().clone()))
    }

    async fn handle_google_callback(
        &self,
        code: String,
    ) -> Result<String, google_drive_service::Error> {
        let client = self.create_oauth_client();

        let code = AuthorizationCode::new(code);

        let token_result = client
            .exchange_code(code)
            // .set_pkce_verifier(PkceCodeChallenge::new_random_sha256())
            .request_async(async_http_client)
            .await;

        let token = match token_result {
            Ok(token) => token,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                return Err(google_drive_service::Error::Unknown(err.to_string()));
            }
        };

        Ok(token.access_token().secret().clone())
    }

    async fn get_google_email(
        &self,
        access_token: String,
    ) -> Result<String, google_drive_service::Error> {
        let client = Client::new();
        let userinfo_url = "https://www.googleapis.com/oauth2/v3/userinfo";
        let response_redsult = client
            .get(userinfo_url)
            .bearer_auth(access_token)
            .send()
            .await;

        let response = match response_redsult {
            Ok(response) => response,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                return Err(google_drive_service::Error::Unknown(err.to_string()));
            }
        };

        let user_info_result: Result<_, reqwest::Error> = response.json().await;
        let user_info: UserInfo = match user_info_result {
            Ok(user_info) => user_info,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                return Err(google_drive_service::Error::Unknown(err.to_string()));
            }
        };

        Ok(user_info.email)
    }

    async fn download_p_d_f(
        &self,
        access_token: String,
        file_id: &str,
    ) -> Result<String, google_drive_service::Error> {
        let hub = create_hub(access_token.clone()).await?;

        let file_metadata = hub
            .files()
            .get(file_id)
            .param("fields", "name,mimeType")
            .doit()
            .await
            .map_err(|x| google_drive_service::Error::Unknown(x.to_string()))?;

        if file_metadata.1.mime_type != Some("application/pdf".to_string()) {
            return Err(google_drive_service::Error::Unknown(
                "File is not a PDF".to_string(),
            ));
        }

        let file_path = format!("downloaded_file_{}.pdf", file_id);

        let mut output_file = File::create(&file_path).await.map_err(|e| {
            google_drive_service::Error::Unknown(format!("Error creating file: {}", e))
        })?;

        let client = Client::new();
        let req = client
            .get(&format!(
                "https://www.googleapis.com/drive/v3/files/{}",
                file_id
            ))
            .bearer_auth(&access_token)
            .query(&[("alt", "media")]);

        let mut response = req.send().await.map_err(|e| {
            google_drive_service::Error::Unknown(format!("Error sending request: {}", e))
        })?;

        while let Some(chunk) = response.chunk().await.map_err(|e| {
            google_drive_service::Error::Unknown(format!("Error reading chunk: {}", e))
        })? {
            output_file.write_all(&chunk).await.map_err(|e| {
                google_drive_service::Error::Unknown(format!("Error writing to file: {}", e))
            })?;
        }

        Ok(file_path)
    }

    async fn list_files(
        &self,
        access_token: String,
        folder_id: &str,
    ) -> Result<Vec<FileInfo>, google_drive_service::Error> {
        let hub = create_hub(access_token).await?;
        let result = hub
            .files()
            .list()
            .q(&format!("'{}' in parents", folder_id))
            .page_size(100)
            .add_scope("https://www.googleapis.com/auth/drive.metadata.readonly")
            .param("fields", "files(id, name, mimeType, createdTime)")
            .doit()
            .await;

        let files = match result {
            Ok((_resp, result)) => result,
            Err(google_drive3::Error::BadRequest(json_value)) => {
                if json_value.to_string().contains("UNAUTHENTICATED") {
                    return Err(google_drive_service::Error::GoogleUnauthenticated);
                } else {
                    return Err(google_drive_service::Error::Unknown(json_value.to_string()));
                }
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                return Err(google_drive_service::Error::Unknown(err.to_string()));
            }
        };

        let file_ids = files
            .files
            .unwrap_or_default()
            .into_iter()
            // .filter(|file| file.id.is_some())
            .map(|file| FileInfo {
                id: file.id.unwrap(),
                name: file.name.unwrap(),
                file_type: file.mime_type.unwrap_or(Default::default()),
                created_at: file.created_time,
            })
            .collect();

        Ok(file_ids)
    }

    async fn create_p_d_f(
        &self,
        access_token: String,
        file_name: &str,
        file_path: String,
    ) -> Result<String, google_drive_service::Error> {
        let mut file = File::open(file_path).await.map_err(|e| {
            google_drive_service::Error::Unknown(format!("Error opening file: {}", e))
        })?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await.map_err(|e| {
            google_drive_service::Error::Unknown(format!("Error reading file: {}", e))
        })?;

        let client = Client::new();

        let metadata = json!({
            "name": file_name,
            "mimeType": "application/pdf"
        });

        let boundary = "foo_bar_baz";
        let mut body = Vec::new();

        // Add metadata part
        body.extend(format!("--{}\r\n", boundary).as_bytes());
        body.extend(b"Content-Type: application/json; charset=UTF-8\r\n\r\n");
        body.extend(metadata.to_string().as_bytes());
        body.extend(b"\r\n");

        // Add file part
        body.extend(format!("--{}\r\n", boundary).as_bytes());
        body.extend(b"Content-Type: application/pdf\r\n\r\n");
        body.extend(&buffer);
        body.extend(b"\r\n");

        // End boundary
        body.extend(format!("--{}--\r\n", boundary).as_bytes());

        let req = client
            .post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart")
            .bearer_auth(&access_token)
            .header(
                "Content-Type",
                format!("multipart/related; boundary={}", boundary),
            )
            .header("Content-Length", body.len())
            .body(body);

        let response = req.send().await.map_err(|e| {
            google_drive_service::Error::Unknown(format!("Error sending request: {}", e))
        })?;

        let msg = response.text().await.map_err(|e| {
            google_drive_service::Error::Unknown(format!("Error reading response: {}", e))
        })?;

        Ok(msg)
    }
}

async fn create_hub(
    access_token: String,
) -> Result<
    DriveHub<hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>>,
    google_drive_service::Error,
> {
    let auth = AccessTokenAuthenticator::builder(access_token)
        .build()
        .await
        .map_err(|e| google_drive_service::Error::Unknown(e.to_string()))?;

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .build(),
        );

    Ok(DriveHub::new(client, auth))
}

#[cfg(test)]
mod tests {
    use crate::application::ports::google_drive_service::GoogleDriveServiceTrait;

    #[tokio::test]
    async fn test_get_google_auth_url() {
        let config = crate::adapters::config::Config::new();

        let google_drive_service =
            crate::adapters::driven::google_drive_service::GoogleDriveService::new(
                config.google_client_id.clone(),
                config.google_client_secret.clone(),
                config.google_auth_url.clone(),
                config.google_token_url.clone(),
                config.google_redirect_url.clone(),
            )
            .await;

        let (auth_url, csrf_token) = google_drive_service.get_google_auth_url().await.unwrap();
        assert!(!auth_url.is_empty());
        assert!(!csrf_token.is_empty());

        println!("auth_url: {}", auth_url);
        println!("csrf_token: {}", csrf_token);
    }

    #[tokio::test]
    async fn test_handle_google_callback() {
        let config = crate::adapters::config::Config::new();

        let google_drive_service =
            crate::adapters::driven::google_drive_service::GoogleDriveService::new(
                config.google_client_id.clone(),
                config.google_client_secret.clone(),
                config.google_auth_url.clone(),
                config.google_token_url.clone(),
                config.google_redirect_url.clone(),
            )
            .await;

        let code = "".to_string();
        let access_token = match google_drive_service.handle_google_callback(code).await {
            Ok(access_token) => access_token,
            Err(err) => {
                println!("Error: {}", err);
                panic!();
            }
        };
        assert!(!access_token.is_empty());
    }

    #[tokio::test]
    async fn test_get_google_email() {
        let config = crate::adapters::config::Config::new();

        let google_drive_service =
            crate::adapters::driven::google_drive_service::GoogleDriveService::new(
                config.google_client_id.clone(),
                config.google_client_secret.clone(),
                config.google_auth_url.clone(),
                config.google_token_url.clone(),
                config.google_redirect_url.clone(),
            )
            .await;

        let access_token = "".to_string();

        let email = match google_drive_service.get_google_email(access_token).await {
            Ok(email) => email,
            Err(err) => {
                println!("Error: {}", err);
                panic!();
            }
        };
        assert!(!email.is_empty());

        println!("email: {}", email);
    }
}
