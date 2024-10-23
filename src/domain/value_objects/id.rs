use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

#[derive(PartialEq, Clone, Debug, Copy)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl TryFrom<Uuid> for Id {
    type Error = String;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        if value.is_nil() {
            return Err("Id is nil".to_string());
        }
        Ok(Self(value))
    }
}

impl TryFrom<String> for Id {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = Uuid::parse_str(&value).map_err(|err| err.to_string())?;
        Self::try_from(value)
    }
}

impl From<Id> for Uuid {
    fn from(id: Id) -> Self {
        id.0
    }
}

impl From<Id> for String {
    fn from(id: Id) -> Self {
        id.0.to_string()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.to_string().fmt(f)
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = String::deserialize(deserializer)?;
        Self::try_from(id).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests_id {
    use super::*;

    #[test]
    fn test_id() {
        let id = Id::try_from(Uuid::new_v4());
        assert!(id.is_ok());
        let id = Id::try_from(Uuid::nil());
        assert!(id.is_err());
    }
}
