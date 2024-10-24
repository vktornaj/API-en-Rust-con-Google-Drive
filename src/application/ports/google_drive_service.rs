use crate::domain::value_objects::file_info::FileInfo;

pub trait GoogleDriveServiceTrait {
    async fn get_google_auth_url(&self) -> Result<(String, String), String>;
    async fn handle_google_callback(&self, code: String) -> Result<String, String>;
    async fn get_google_email(&self, access_token: String) -> Result<String, String>;
    async fn get_file(&self, access_token: String, file_id: &str) -> Result<Vec<u8>, String>;
    async fn list_files(
        &self,
        access_token: String,
        folder_id: &str,
    ) -> Result<Vec<FileInfo>, String>;
    async fn create_file(
        &self,
        access_token: String,
        file_name: &str,
        file_content: &[u8],
    ) -> Result<String, String>;
    async fn delete_file(&self, access_token: String, file_id: &str) -> Result<String, String>;
}
