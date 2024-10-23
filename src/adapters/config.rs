use std::env;

#[derive(Clone)]
pub struct Config {
    pub secret: Vec<u8>,
    pub db_url: String,
    pub db_name: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_auth_url: String,
    pub google_token_url: String,
    pub google_redirect_url: String,
}

impl Config {
    pub fn new() -> Self {
        let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let db_url = env::var("DB_URL").expect("DB_URL must be set");
        let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
        let google_client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
        let google_client_secret =
            env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");
        let google_auth_url = env::var("GOOGLE_AUTH_URL").expect("GOOGLE_AUTH_URL must be set");
        let google_token_url = env::var("GOOGLE_TOKEN_URL").expect("GOOGLE_TOKEN_URL must be set");
        let google_redirect_url =
            env::var("GOOGLE_REDIRECT_URL").expect("GOOGLE_REDIRECT_URL must be set");

        Config {
            secret: secret.into_bytes(),
            db_url,
            db_name,
            google_client_id,
            google_client_secret,
            google_auth_url,
            google_token_url,
            google_redirect_url,
        }
    }
}
