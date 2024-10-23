use std::env;

#[derive(Clone)]
pub struct Config {
    pub secret: Vec<u8>,
    pub db_url: String,
    pub db_name: String,
}

impl Config {
    pub fn new() -> Self {
        let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let db_url = env::var("DB_URL").expect("DB_URL must be set");
        let db_name = env::var("DB_NAME").expect("DB_NAME must be set");

        Config {
            secret: secret.into_bytes(),
            db_url,
            db_name,
        }
    }
}
