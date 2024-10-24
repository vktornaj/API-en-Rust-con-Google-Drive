use crate::domain::{entities::user::User, value_objects::id::Id};

#[derive(Debug)]
pub enum Error {
    NotFound,
    ConnectionError(String),
    Unknown(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound => write!(f, "Not Found"),
            Error::ConnectionError(msg) => write!(f, "Connection Error: {}", msg),
            Error::Unknown(msg) => write!(f, "Unknown Error: {}", msg),
        }
    }
}

pub trait UserRepositoryTrait {
    async fn find_by_id(&self, id: Id) -> Result<User, Error>;
    async fn find_by_email(&self, email: &str) -> Result<User, Error>;
    async fn update(&self, user: User) -> Result<User, Error>;
    async fn create(&self, user: User) -> Result<User, Error>;
}
