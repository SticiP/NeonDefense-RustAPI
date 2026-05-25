use axum::{extract::State, Json, http::StatusCode};
use serde::{Deserialize, Serialize}; 
use std::sync::Arc;
use crate::AppState;
use crate::models::auth::Claims; 
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreatePlayerPayload {
    pub nickname: String,
}

// Structura pentru a trimite un singur item înapoi către Unity
#[derive(Serialize)]
pub struct InventoryItemResponse {
    pub id: Uuid,
    pub item_type: String,
    pub rarity: i32,
    pub level: i32,
    pub is_equipped: bool,
}

// Am adăugat vectorul de inventar aici
#[derive(Serialize)]
#[allow(non_snake_case)] 
pub struct PlayerProfileResponse {
    pub nickname: String,
    pub accountId: String,
    pub data_fragments: i64,
    pub crypto_cores: i32,
    pub energy: i32,
    pub isOfflineMode: bool,
    pub inventory: Vec<InventoryItemResponse>, // <-- Lista de iteme!
}

// --- RUTA 3: CREATE PLAYER (Setare Nickname) ---
pub async fn create_profile(
    State(state): State<Arc<AppState>>,
    claims: Claims, 
    Json(payload): Json<CreatePlayerPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    
    tracing::info!(
        target: "PLAYER_API", 
        user_id = %claims.sub, 
        nickname = %payload.nickname, 
        "Player profile creation requested."
    );

    let player = sqlx::query!(
        "INSERT INTO players (user_id, nickname) VALUES ($1, $2) RETURNING id, nickname, data_fragments, energy",
        claims.sub,
        payload.nickname
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::warn!(
            target: "PLAYER_API", 
            user_id = %claims.sub, 
            nickname = %payload.nickname, 
            error = %e, 
            "Profile creation failed: Database conflict (nickname likely already taken)."
        );
        (StatusCode::CONFLICT, "Acest nickname este deja luat!".to_string())
    })?;

    tracing::info!(
        target: "PLAYER_API", 
        user_id = %claims.sub, 
        player_id = %player.id, 
        nickname = %player.nickname, 
        "Player profile successfully created."
    );

    Ok(Json(serde_json::json!({
        "message": "Profil creat cu succes. Bun venit în NeonDefense!",
        "player": {
            "id": player.id,
            "nickname": player.nickname,
            "data_fragments": player.data_fragments, 
            "energy": player.energy
        }
    })))
}

// --- RUTA: GET PROFILE ---
pub async fn get_profile(
    State(state): State<Arc<AppState>>,
    claims: Claims, 
) -> Result<Json<PlayerProfileResponse>, (StatusCode, String)> {
    
    tracing::debug!(
        target: "PLAYER_API", 
        user_id = %claims.sub, 
        "Player profile and inventory fetch requested."
    );

    // 1. Extragem datele de bază ale jucătorului
    let player = sqlx::query!(
        "SELECT id, nickname, data_fragments, crypto_cores, energy FROM players WHERE user_id = $1 AND is_deleted = false",
        claims.sub
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!(
            target: "PLAYER_API", 
            user_id = %claims.sub, 
            error = %e, 
            "Database error while fetching player profile."
        );
        (StatusCode::INTERNAL_SERVER_ERROR, "Eroare DB la profil".to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!(
            target: "PLAYER_API", 
            user_id = %claims.sub, 
            "Profile fetch failed: Player not found or deleted."
        );
        (StatusCode::NOT_FOUND, "Profilul nu a fost găsit!".to_string())
    })?;

    // 2. Extragem TOATE itemele din inventarul acestui jucător
    let inventory_records = sqlx::query!(
        "SELECT id, item_type, rarity, level, is_equipped FROM inventory WHERE player_id = $1",
        player.id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!(
            target: "PLAYER_API", 
            user_id = %claims.sub, 
            player_id = %player.id, 
            error = %e, 
            "Database error while fetching inventory records."
        );
        (StatusCode::INTERNAL_SERVER_ERROR, "Eroare DB la inventar".to_string())
    })?;

    // 3. Mapăm rezultatele din baza de date în structura pe care o așteaptă Unity
    let mut mapped_inventory = Vec::new();
    for record in inventory_records {
        mapped_inventory.push(InventoryItemResponse {
            id: record.id,
            item_type: record.item_type,
            rarity: record.rarity.unwrap_or(0), 
            level: record.level.unwrap_or(1),
            is_equipped: record.is_equipped.unwrap_or(false),
        });
    }

    tracing::info!(
        target: "PLAYER_API", 
        user_id = %claims.sub, 
        player_id = %player.id, 
        items_count = mapped_inventory.len(), 
        "Player profile and inventory fetched successfully."
    );

    // 4. Returnăm pachetul complet
    Ok(Json(PlayerProfileResponse {
        nickname: player.nickname,
        accountId: player.id.to_string(),
        data_fragments: player.data_fragments.unwrap_or(0),
        crypto_cores: player.crypto_cores.unwrap_or(0),
        energy: player.energy.unwrap_or(0),
        isOfflineMode: false,
        inventory: mapped_inventory,
    }))
}