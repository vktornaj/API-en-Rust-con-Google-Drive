use crate::application::ports::google_drive_service::GoogleDriveServiceTrait;

pub enum Error {
    NotFound(String),
    ConnectionError(String),
}

pub async fn execute(google_drive_service: &impl GoogleDriveServiceTrait) -> Result<String, Error> {
    match google_drive_service.get_google_auth_url().await {
        Ok(files_id) => Ok(files_id.0),
        Err(err) => Err(Error::ConnectionError(err.to_string())),
    }
}
