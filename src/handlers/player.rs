use axum::{extract::State, Json, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use crate::AppState;

#[derive(Deserialize)]
pub struct CreatePlayerPayload {
    pub user_id: Uuid, // Într-o variantă finală, asta vine din decodarea JWT-ului
    pub nickname: String,
}

// --- RUTA 3: CREATE PLAYER (Setare Nickname) ---
pub async fn create_profile(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePlayerPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    
    // Inserăm player-ul conform schemei din baza de date
    // (Resursele default precum coins=100 sunt puse automat de PostgreSQL)
    let player = sqlx::query!(
        "INSERT INTO players (user_id, nickname) VALUES ($1, $2) RETURNING id, nickname, coins, energy",
        payload.user_id,
        payload.nickname
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| (StatusCode::CONFLICT, "Acest nickname este deja luat!".to_string()))?;

    // Returnăm direct datele jucătorului create
    Ok(Json(serde_json::json!({
        "message": "Profil creat cu succes. Bun venit în NeonDefense!",
        "player": {
            "id": player.id,
            "nickname": player.nickname,
            "coins": player.coins,
            "energy": player.energy
        }
    })))
}