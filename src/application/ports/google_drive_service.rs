use crate::domain::value_objects::file_info::FileInfo;

#[derive(Debug)]
pub enum Error {
    GoogleUnauthenticated,
    Unknown(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::GoogleUnauthenticated => write!(f, "Google Unauthenticated"),
            Error::Unknown(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

pub trait GoogleDriveServiceTrait {
    async fn get_google_auth_url(&self) -> Result<(String, String), Error>;
    async fn handle_google_callback(&self, code: String) -> Result<String, Error>;
    async fn get_google_email(&self, access_token: String) -> Result<String, Error>;
    async fn download_p_d_f(&self, access_token: String, file_id: &str) -> Result<String, Error>;
    async fn list_files(
        &self,
        access_token: String,
        folder_id: &str,
    ) -> Result<Vec<FileInfo>, Error>;
    async fn create_p_d_f(
        &self,
        access_token: String,
        file_name: &str,
        file_path: String,
    ) -> Result<String, Error>;
}
