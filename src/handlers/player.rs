use axum::{extract::State, Json, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;
use crate::AppState;
use crate::models::auth::Claims; // Importăm Extractorul

#[derive(Deserialize)]
pub struct CreatePlayerPayload {
    // AM ELIMINAT user_id! Clientul nu mai poate trimite ce ID vrea el.
    pub nickname: String,
}

// --- RUTA 3: CREATE PLAYER (Setare Nickname) ---
pub async fn create_profile(
    State(state): State<Arc<AppState>>,
    claims: Claims, // AXUM extrage automat datele din JWT aici! Dacă JWT-ul lipsește sau e invalid, ruta dă respingere automat.
    Json(payload): Json<CreatePlayerPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    
    let player = sqlx::query!(
        "INSERT INTO players (user_id, nickname) VALUES ($1, $2) RETURNING id, nickname, coins, energy",
        claims.sub, // Folosim 100% sigur ID-ul extras din token-ul validat criptografic
        payload.nickname
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| (StatusCode::CONFLICT, "Acest nickname este deja luat!".to_string()))?;

    // ... Restul funcției rămâne la fel
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