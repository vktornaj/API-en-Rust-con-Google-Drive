use chrono::{Duration, TimeZone, Utc};
use jsonwebtoken as jwt;
use jwt::{Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::value_objects::id::Id;

#[derive(Debug)]
pub enum TokenDataError {
    InvalidData,
    ExpiredToken,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenData {
    /// timestamp
    pub exp: i64,
    /// user id
    pub user_id: Uuid,
}

impl TokenData {
    pub fn new(id: &Id) -> Self {
        TokenData {
            exp: (Utc::now() + Duration::hours(24)).timestamp(),
            user_id: id.to_owned().into(),
        }
    }

    pub fn token(&self, secret: &[u8]) -> String {
        let encoding_key = EncodingKey::from_base64_secret(std::str::from_utf8(secret).unwrap());
        jwt::encode(&jwt::Header::default(), self, &encoding_key.unwrap()).expect("jwt")
    }

    // TODO: Determinate if token is valid by date
    pub fn from_token(token: &String, secret: &[u8]) -> Result<Self, TokenDataError> {
        if let Some(auth) = decode_token(token, secret) {
            if Utc::now() <= Utc.timestamp_opt(auth.exp, 0).unwrap() {
                Ok(auth)
            } else {
                println!("token error: Expired token");
                return Err(TokenDataError::ExpiredToken);
            }
        } else {
            println!("token error: Invalid token");
            return Err(TokenDataError::InvalidData);
        }
    }
}

fn decode_token(token: &str, secret: &[u8]) -> Option<TokenData> {
    let token = token.strip_prefix("Bearer ").unwrap_or(token);

    let decoding_key = DecodingKey::from_base64_secret(std::str::from_utf8(secret).unwrap());

    jwt::decode(
        token,
        &decoding_key.unwrap(),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|err| {
        eprintln!("Auth decode error: {:?}", err);
    })
    .ok()
    .map(|token_data| token_data.claims)
}
