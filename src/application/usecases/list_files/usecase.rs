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

pub struct Payload {
    pub path: String,
    pub user_id: Id,
}

pub async fn execute(
    user_repository: &impl UserRepositoryTrait,
    google_drive_service: &impl GoogleDriveServiceTrait,
    payload: Payload,
) -> Result<Vec<String>, Error> {
    let user = match user_repository.find_by_id(payload.user_id).await {
        Ok(user) => user,
        Err(err) => return Err(Error::ConnectionError(err.to_string())),
    };
    match google_drive_service.get_files(user.id, &payload.path).await {
        Ok(files_id) => Ok(files_id),
        Err(err) => Err(Error::ConnectionError(err.to_string())),
    }
}
