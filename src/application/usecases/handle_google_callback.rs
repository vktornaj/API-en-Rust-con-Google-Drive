use crate::{
    adapters::config,
    application::ports::{
        google_drive_service::GoogleDriveServiceTrait, user_repository::UserRepositoryTrait,
    },
    domain::entities::token_data::TokenData,
};

pub enum Error {
    NotFound(String),
    ConnectionError(String),
}

pub struct Payload {
    pub code: String,
}

pub async fn execute(
    user_repository: &impl UserRepositoryTrait,
    google_drive_service: &impl GoogleDriveServiceTrait,
    secret: &[u8],
    payload: Payload,
) -> Result<String, Error> {
    let access_token = match google_drive_service
        .handle_google_callback(payload.code)
        .await
    {
        Ok(access_token) => access_token,
        Err(err) => return Err(Error::ConnectionError(err.to_string())),
    };

    let email = match google_drive_service.get_google_email(access_token).await {
        Ok(email) => email,
        Err(err) => return Err(Error::ConnectionError(err.to_string())),
    };

    let user = match user_repository.find_by_email(&email).await {
        Ok(user) => user,
        Err(err) => return Err(Error::ConnectionError(err.to_string())),
    };

    let user = user_repository
        .update(user)
        .await
        .map_err(|x| Error::ConnectionError(x.to_string()))?;

    Ok(TokenData::new(&user.id).token(secret))
}
