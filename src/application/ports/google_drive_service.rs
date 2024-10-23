
pub trait GoogleDriveServiceTrait {
    async fn get_google_auth_url(&self) -> Result<(String, String), String>;
    async fn get_file(&self, access_token: String, file_id: &str) -> Result<Vec<u8>, String>;
    async fn get_files(&self, access_token: String, path: &str) -> Result<Vec<String>, String>;
    async fn create_file(
        &self,
        access_token: String,
        file_name: &str,
        file_content: &[u8],
    ) -> Result<String, String>;
    async fn delete_file(&self, access_token: String, file_id: &str) -> Result<String, String>;
}
