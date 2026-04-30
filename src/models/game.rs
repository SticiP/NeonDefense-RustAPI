use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Ce trimite Unity despre acțiunile din meci
#[derive(Deserialize, Debug)]
pub struct MatchActionPayload {
    pub event_type: String,
    pub event_data: serde_json::Value,
}

// Ce trimite Unity despre un item nou găsit
#[derive(Deserialize, Debug)]
pub struct NewItemPayload {
    pub id: Uuid, // Generat de Unity la drop
    pub item_type: String,
    pub rarity: i32,
    pub level: i32,
}

// Pachetul complet primit de la Unity
#[derive(Deserialize, Debug)]
pub struct SyncRequest {
    pub earned_coins: i64,
    pub energy_used: i32,
    pub new_items: Vec<NewItemPayload>,
    pub actions: Vec<MatchActionPayload>,
}

// Ce trimitem înapoi către Unity pentru a crea "Breakpoint-ul"
#[derive(Serialize)]
pub struct SyncResponse {
    pub message: String,
    pub player_id: Uuid,
    pub current_coins: i64,
    pub current_energy: i32,
    pub synchronized_items_count: usize,
}