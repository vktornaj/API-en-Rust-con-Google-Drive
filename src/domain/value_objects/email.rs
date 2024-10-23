use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err("Email cannot be empty".to_string());
        }

        if !value.contains('@') {
            return Err("Email must contain @".to_string());
        }

        Ok(Email(value))
    }
}
