use mongodb::{bson::doc, Client, Collection};

use crate::{
    application::ports::user_repository::UserRepositoryTrait,
    domain::{entities::user::User, value_objects::id::Id},
};

#[derive(Clone)]
pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub async fn new(db_url: &String, db_name: &String, collection_name: &String) -> Self {
        let client = Client::with_uri_str(db_url)
            .await
            .expect("Failed to initialize client");
        let db = client.database(db_name);
        let collection = db.collection(collection_name);

        Self { collection }
    }
}

impl UserRepositoryTrait for UserRepository {
    async fn find_by_id(&self, id: Id) -> Result<User, String> {
        let filter = doc! { "_id": id.to_string() };
        match self.collection.find_one(filter).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err("User not found".to_string()),
            Err(err) => Err(err.to_string()),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<User, String> {
        let filter = doc! { "email": email };
        match self.collection.find_one(filter).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err("User not found".to_string()),
            Err(err) => Err(err.to_string()),
        }
    }

    async fn create(&self, user: User) -> Result<User, String> {
        match self.collection.insert_one(&user).await {
            Ok(_) => Ok(user),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::adapters::driven::user_repository::UserRepository;
    use crate::application::ports::user_repository::UserRepositoryTrait;
    use crate::domain::entities::user::User;
    use crate::domain::value_objects::email::Email;
    use mongodb::bson::doc;

    #[tokio::test]
    async fn test_create_user() {
        let email_test = Email::new("name@some.com".to_string()).expect("Failed to create email");

        let config = crate::adapters::config::Config::new();
        let user_repository =
            UserRepository::new(&config.db_url, &config.db_name, &"users".to_string()).await;

        let user = User::new(email_test.clone());

        let result = user_repository.create(user.clone()).await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.email, email_test);

        let client = mongodb::Client::with_uri_str(&config.db_url)
            .await
            .expect("Failed to initialize client");
        let db = client.database(&config.db_name);
        let collection = db.collection::<User>(&"users".to_string());

        let filter = doc! { "_id": user.id.to_string() };
        let result = collection.find_one(filter.clone()).await;
        assert!(result.is_ok());

        let user = result.unwrap().unwrap();
        assert_eq!(user.email, email_test);

        collection.delete_one(filter).await.unwrap();

        db.drop().await.unwrap();
    }
}
