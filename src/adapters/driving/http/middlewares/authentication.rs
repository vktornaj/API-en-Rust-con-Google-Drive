use axum::{
    body::Body,
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};

use crate::{adapters::config::Config, domain::entities::token_data::TokenData};

pub async fn auth_middleware(
    Extension(config): Extension<Config>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let path = req.uri().path();

    if !path.starts_with("/api/protected/") {
        return Ok(next.run(req).await);
    }

    // Extract the authorization header
    let auth_header = if let Some(auth_header) = req.headers().get("Authorization") {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    let token = if let Ok(token) = from_token(auth_header) {
        token
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    let token_data = if let Ok(token_data) = TokenData::from_token(&token, &config.secret) {
        token_data
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // Insert the user_id into the request extensions
    req.extensions_mut().insert(token_data.user_id);

    // Proceed to the next middleware or handler
    Ok(next.run(req).await)
}

fn from_token(header_value: &HeaderValue) -> Result<String, Box<dyn std::error::Error>> {
    // Convert HeaderValue to &str
    let header_str = header_value.to_str()?;

    // Convert &str to String
    let token = header_str.to_string();

    Ok(token)
}
