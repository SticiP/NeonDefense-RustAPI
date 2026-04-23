use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: Uuid,
    pub player_id: Uuid,
    pub item_type: String,
    pub rarity: i32,
    pub level: i32,
    pub is_equipped: bool,
    pub acquired_at: DateTime<Utc>,
}

impl InventoryItem {
    pub fn equip(&mut self) {
        // TODO: Logica pentru a marca item-ul ca instalat pe AI
    }

    pub fn can_be_merged_with(&self, _other_item: &InventoryItem) -> bool {
        // TODO: Verifică dacă au același tip și raritate pentru sistemul N+1
        false
    }
}