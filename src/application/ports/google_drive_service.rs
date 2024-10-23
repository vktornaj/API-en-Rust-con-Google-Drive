use crate::domain::value_objects::id::Id;

pub trait GoogleDriveServiceTrait {
    async fn get_file(&self, user_id: Id, file_id: &str) -> Result<Vec<u8>, String>;
    async fn get_files(&self, user_id: Id, path: &str) -> Result<Vec<String>, String>;
    async fn create_file(
        &self,
        user_id: Id,
        file_name: &str,
        file_content: &[u8],
    ) -> Result<String, String>;
    async fn update_file(
        &self,
        user_id: Id,
        file_id: &str,
        file_content: &[u8],
    ) -> Result<String, String>;
    async fn delete_file(&self, user_id: Id, file_id: &str) -> Result<String, String>;
}
