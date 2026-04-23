use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub user_id: Uuid,
    pub nickname: String,
    pub coins: i64,
    pub crypto_cores: i32,
    pub energy: i32,
    pub is_shadowbanned: bool,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Player {
    // Funcții utilitare pentru economia jocului
    pub fn add_coins(&mut self, _amount: i64) {
        // TODO: Adaugă Data Fragments
    }

    pub fn deduct_energy(&mut self, _amount: i32) -> Result<(), String> {
        // TODO: Verifică dacă are suficientă energie pentru a porni un meci
        Ok(())
    }

    pub fn trigger_hard_reset(&mut self) {
        // TODO: Eliberează nickname-ul și resetează resursele
    }
}