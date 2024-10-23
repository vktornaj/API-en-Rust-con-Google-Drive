use crate::domain::{entities::user::User, value_objects::id::Id};

pub trait UserRepositoryTrait {
    async fn find_by_id(&self, id: Id) -> Result<User, String>;
    async fn find_by_email(&self, email: &str) -> Result<User, String>;
    async fn create(&self, user: User) -> Result<User, String>;
}
