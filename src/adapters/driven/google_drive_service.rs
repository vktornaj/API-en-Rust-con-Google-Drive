use crate::{
    application::ports::google_drive_service::GoogleDriveServiceTrait,
    domain::value_objects::id::Id,
};

#[derive(Clone)]
pub struct GoogleDriveService {
    // client: Client,
}

impl GoogleDriveService {
    pub fn new() -> Self {
        // Self {
        //     client: Client::new(),
        // }
        todo!()
    }
}

impl GoogleDriveServiceTrait for GoogleDriveService {
    async fn get_file(&self, user_id: Id, file_id: &str) -> Result<Vec<u8>, String> {
        todo!()
    }

    async fn get_files(&self, user_id: Id, path: &str) -> Result<Vec<String>, String> {
        todo!()
    }

    async fn create_file(
        &self,
        user_id: Id,
        file_name: &str,
        file_content: &[u8],
    ) -> Result<String, String> {
        todo!()
    }

    async fn update_file(
        &self,
        user_id: Id,
        file_id: &str,
        file_content: &[u8],
    ) -> Result<String, String> {
        todo!()
    }

    async fn delete_file(&self, user_id: Id, file_id: &str) -> Result<String, String> {
        todo!()
    }
}
