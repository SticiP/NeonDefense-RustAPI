use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub auth_provider: String,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
}

impl User {
    #[allow(dead_code)]
    pub fn verify_password(&self, _input_password: &str) -> bool {
        false
    }

    #[allow(dead_code)]
    pub fn mark_as_deleted(&mut self) {}
}