use crate::{
    application::ports::{
        google_drive_service::{self, GoogleDriveServiceTrait},
        user_repository::UserRepositoryTrait,
    },
    domain::value_objects::{file_info::FileInfo, id::Id},
};

pub enum Error {
    ConnectionError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConnectionError(e) => write!(f, "Connection error: {}", e),
        }
    }
}

pub struct Payload {
    pub path: String,
    pub user_id: Id,
}

pub async fn execute(
    user_repository: &impl UserRepositoryTrait,
    google_drive_service: &impl GoogleDriveServiceTrait,
    payload: Payload,
) -> Result<Vec<FileInfo>, Error> {
    let user = match user_repository.find_by_id(payload.user_id).await {
        Ok(user) => user,
        Err(err) => return Err(Error::ConnectionError(err.to_string())),
    };

    match google_drive_service
        .list_files(user.access_token, &payload.path)
        .await
    {
        Ok(files_id) => Ok(files_id),
        Err(google_drive_service::Error::GoogleUnauthenticated) => Err(Error::ConnectionError(
            "Google access token expired".to_string(),
        )),
        Err(err) => Err(Error::ConnectionError(err.to_string())),
    }
}
