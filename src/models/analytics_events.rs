use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json; // Ajută la maparea JSONB-ului din Postgres
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub id: i64,
    pub player_id: Uuid,
    pub event_type: String,
    pub event_data: serde_json::Value, // Permite stocarea oricărui format de JSON
    pub created_at: DateTime<Utc>,
}

impl AnalyticsEvent {
    pub fn new_match_completed(_player_id: Uuid, _duration: i32, _enemies_killed: i32) -> Self {
        // TODO: Constructor specializat care generează structura JSON direct aici
        unimplemented!()
    }
    
    pub fn new_purchase(_player_id: Uuid, _item_bought: &str, _cost: i64) -> Self {
        // TODO: Constructor pentru tranzacții în Shop
        unimplemented!()
    }
}