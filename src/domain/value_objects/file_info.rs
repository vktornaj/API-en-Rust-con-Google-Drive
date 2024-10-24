use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FileInfo {
    pub name: String,
    pub file_type: String,
    pub created_at: Option<DateTime<Utc>>,
}
