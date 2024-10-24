use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{email::Email, id::Id};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Id,
    pub email: Email,
    pub access_token: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: Email, access_token: String) -> Self {
        Self {
            id: Id::new(),
            email,
            access_token,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
