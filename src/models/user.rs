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
    // Aici vom adăuga logica pentru a valida parolele sau a iniția procesul de ștergere GDPR
    pub fn verify_password(&self, _input_password: &str) -> bool {
        // TODO: Logica cu Argon2 pentru verificarea parolei
        false
    }

    pub fn mark_as_deleted(&mut self) {
        // TODO: Logica pentru a anonimiza contul
    }
}