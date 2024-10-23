use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::id::Id;

#[derive(PartialEq, Clone, Debug)]
pub struct User {
    pub id: Id,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String) -> Self {
        Self {
            id: Id::new(),
            email: email,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("User", 5)?;
        // state.serialize_field("id", &self.id)?;
        // state.serialize_field("email", &self.email)?;
        // state.serialize_field("password", &self.password)?;
        // state.serialize_field("created_at", &self.created_at)?;
        // state.serialize_field("updated_at", &self.updated_at)?;
        // state.end()
        todo!()
    }
}

impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}
