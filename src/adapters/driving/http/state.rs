use crate::adapters::{
    config::Config,
    driven::{google_drive_service::GoogleDriveService, user_repository::UserRepository},
};

#[derive(Clone)]
pub struct AppState {
    pub user_repository: UserRepository,
    pub google_drive_service: GoogleDriveService,
    pub config: Config,
}

impl AppState {
    pub async fn new() -> AppState {
        let config = Config::new();
        AppState {
            user_repository: UserRepository::new(
                &config.db_url,
                &config.db_name,
                &"users".to_string(),
            )
            .await,
            google_drive_service: todo!(),
            config,
        }
    }
}
