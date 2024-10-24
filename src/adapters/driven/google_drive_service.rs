use crate::application::ports::google_drive_service::GoogleDriveServiceTrait;
use oauth2::{
    basic::{BasicClient, BasicErrorResponseType},
    reqwest::async_http_client,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, StandardErrorResponse, TokenResponse, TokenUrl,
};
use reqwest::Client;
use serde::Deserialize;

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
    async fn get_google_auth_url(&self) -> Result<(String, String), String> {
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

    async fn handle_google_callback(&self, code: String) -> Result<String, String> {
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
                return Err(err.to_string());
            }
        };

        Ok(token.access_token().secret().clone())
    }

    async fn get_google_email(&self, access_token: String) -> Result<String, String> {
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
                return Err(err.to_string());
            }
        };

        let user_info_result: Result<_, reqwest::Error> = response.json().await;
        let user_info: UserInfo = match user_info_result {
            Ok(user_info) => user_info,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                return Err(err.to_string());
            }
        };

        Ok(user_info.email)
    }

    async fn get_file(&self, access_token: String, file_id: &str) -> Result<Vec<u8>, String> {
        todo!()
    }

    async fn list_files(&self, access_token: String, path: &str) -> Result<Vec<String>, String> {
        let client = Client::new();
        let response = client
            .get("https://www.googleapis.com/drive/v3/files")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|x| x.to_string())?;

        let body = response.text().await.map_err(|x| x.to_string())?;
        let files: Vec<String> = serde_json::from_str(&body).map_err(|x| x.to_string())?;

        Ok(files)
    }

    async fn create_file(
        &self,
        access_token: String,
        file_name: &str,
        file_content: &[u8],
    ) -> Result<String, String> {
        todo!()
    }

    async fn delete_file(&self, access_token: String, file_id: &str) -> Result<String, String> {
        todo!()
    }
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
