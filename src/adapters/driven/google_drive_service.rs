use crate::application::ports::google_drive_service::GoogleDriveServiceTrait;
use oauth2::{
    basic::{BasicClient, BasicErrorResponseType},
    reqwest::async_http_client,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, StandardErrorResponse, TokenResponse, TokenUrl,
};

#[derive(Clone)]
pub struct GoogleDriveService {
    client: oauth2::Client<
        StandardErrorResponse<BasicErrorResponseType>,
        oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
        oauth2::basic::BasicTokenType,
        oauth2::StandardTokenIntrospectionResponse<
            oauth2::EmptyExtraTokenFields,
            oauth2::basic::BasicTokenType,
        >,
        oauth2::StandardRevocableToken,
        StandardErrorResponse<oauth2::RevocationErrorResponseType>,
    >,
}

impl GoogleDriveService {
    pub async fn new(
        client_id: String,
        client_secret: String,
        auth_url: String,
        token_url: String,
        redirect_url: String,
    ) -> Self {
        let client_id = ClientId::new(client_id);
        let client_secret = ClientSecret::new(client_secret);
        let auth_url = AuthUrl::new(auth_url).unwrap();
        let token_url = TokenUrl::new(token_url).unwrap();
        let redirect_url = RedirectUrl::new(redirect_url).unwrap();

        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(redirect_url);

        Self { client }
    }
}

impl GoogleDriveServiceTrait for GoogleDriveService {
    async fn get_google_auth_url(&self) -> Result<(String, String), String> {
        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/drive".to_string(),
            )) // Request Google Drive scope
            .set_pkce_challenge(PkceCodeChallenge::new_random_sha256().0)
            .url();

        Ok((auth_url.to_string(), csrf_token.secret().clone()))
    }

    async fn handle_google_callback(&self, code: String) -> Result<String, String> {
        let token_result = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|x| x.to_string())?;

        Ok(token_result.access_token().secret().clone())
    }

    async fn get_file(&self, access_token: String, file_id: &str) -> Result<Vec<u8>, String> {
        todo!()
    }

    async fn get_files(&self, access_token: String, path: &str) -> Result<Vec<String>, String> {
        todo!()
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
}
