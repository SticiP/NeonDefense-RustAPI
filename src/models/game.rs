use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Ce trimite Unity despre acțiunile din meci (Loguri)
#[derive(Deserialize, Debug)]
pub struct MatchActionPayload {
    pub event_type: String,
    pub event_data: serde_json::Value,
}

// Ce trimite Unity despre un item nou găsit la final de meci
#[derive(Deserialize, Debug)]
pub struct NewItemPayload {
    pub id: Uuid,
    pub item_type: String,
    pub rarity: i32,
    pub level: i32,
}

// Pachetul complet primit de la Unity la apăsarea butonului "Return to Hub"
#[derive(Deserialize, Debug)]
pub struct SyncRequest {
    pub earned_df: i64,             // Valuta soft (Data Fragments)
    pub earned_crypto_cores: i32,   // Valuta premium (în caz că a picat din vreo realizare/boss)
    pub energy_used: i32,           // Câtă energie a costat meciul
    pub new_items: Vec<NewItemPayload>,
    pub actions: Vec<MatchActionPayload>,
}

// Ce trimitem înapoi către Unity pentru a confirma salvarea
#[derive(Serialize)]
pub struct SyncResponse {
    pub message: String,
    pub player_id: Uuid,
    pub current_df: i64,            // Balanța finală din DB
    pub current_crypto_cores: i32,  // Balanța finală din DB
    pub current_energy: i32,        // Balanța finală din DB
    pub synchronized_items_count: usize,
}

#[derive(Serialize)]
pub struct MarketplaceItemResponse {
    pub item_sku: String,
    pub display_name: String,
    pub rarity: String,
    pub price: f64,
    pub currency: String,
    pub reward_type: String,
    pub reward_amount: i32,
    pub stock: i32,
}

#[derive(Serialize)]
pub struct InitDataResponse {
    pub active_version: String,
    pub game_config: serde_json::Value, // Trimitem JSON-ul exact așa cum l-ai generat în Laravel
    pub store_items: Vec<MarketplaceItemResponse>,
}