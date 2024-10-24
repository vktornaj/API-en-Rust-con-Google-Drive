use std::fmt::Display;

use crate::{
    application::ports::{
        google_drive_service::GoogleDriveServiceTrait, user_repository::UserRepositoryTrait,
    },
    domain::value_objects::id::Id,
};

pub enum Error {
    NotFound(String),
    ConnectionError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound(e) => write!(f, "Not found: {}", e),
            Error::ConnectionError(e) => write!(f, "Connection error: {}", e),
        }
    }
}

pub struct Payload {
    pub file_name: String,
    pub user_id: Id,
    pub file_path: String,
}

pub async fn execute(
    user_repository: &impl UserRepositoryTrait,
    google_drive_service: &impl GoogleDriveServiceTrait,
    payload: Payload,
) -> Result<String, Error> {
    let user = match user_repository.find_by_id(payload.user_id).await {
        Ok(user) => user,
        Err(err) => return Err(Error::ConnectionError(err.to_string())),
    };

    match google_drive_service
        .create_p_d_f(user.access_token, &payload.file_name, payload.file_path)
        .await
    {
        Ok(msg) => Ok(msg),
        Err(err) => Err(Error::ConnectionError(err.to_string())),
    }
}
