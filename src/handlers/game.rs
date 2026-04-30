use axum::{extract::State, Json, http::StatusCode};
use std::sync::Arc;
use crate::AppState;
use crate::models::auth::Claims;
use crate::models::game::{SyncRequest, SyncResponse};

// --- RUTA: SYNC MATCH DATA ---
pub async fn sync_match(
    State(state): State<Arc<AppState>>,
    claims: Claims, // Ne asigurăm că e logat și luăm user_id-ul
    Json(payload): Json<SyncRequest>,
) -> Result<Json<SyncResponse>, (StatusCode, String)> {

    // 1. Validare Anti-Cheat (Exemplu simplu)
    if payload.earned_coins > 10000 || payload.energy_used < 0 {
        return Err((StatusCode::BAD_REQUEST, "Date invalide. Posibil trișare!".to_string()));
    }

    // 2. Inițiem Tranzacția SQL
    let mut tx = state.db.begin().await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Eroare la pornirea tranzacției".to_string()))?;

    // 3. Actualizăm resurele Player-ului
    // Folosim user_id din Claims pentru a găsi jucătorul
    let updated_player = sqlx::query!(
        r#"
        UPDATE players 
        SET coins = coins + $1, 
            energy = energy - $2, 
            updated_at = NOW() 
        WHERE user_id = $3 
        RETURNING id, coins, energy
        "#,
        payload.earned_coins,
        payload.energy_used,
        claims.sub
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Eroare la actualizarea jucătorului".to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "Jucătorul nu există!".to_string()))?;

    // 4. Salvăm Itemele noi în Inventar
    for item in &payload.new_items {
        sqlx::query!(
            "INSERT INTO inventory (id, player_id, item_type, rarity, level) VALUES ($1, $2, $3, $4, $5)",
            item.id,
            updated_player.id, // Folosim ID-ul player-ului abia returnat
            item.item_type,
            item.rarity,
            item.level
        )
        .execute(&mut *tx)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Eroare la salvarea inventarului".to_string()))?;
    }

    // 5. Salvăm Acțiunile (Analytics)
    for action in &payload.actions {
        sqlx::query!(
            "INSERT INTO analytics_events (player_id, event_type, event_data) VALUES ($1, $2, $3)",
            updated_player.id,
            action.event_type,
            action.event_data.clone() // JSONB
        )
        .execute(&mut *tx)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Eroare la salvarea acțiunilor".to_string()))?;
    }

    // 6. COMMIT - Salvăm totul definitiv în baza de date
    tx.commit().await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Eroare la finalizarea tranzacției".to_string()))?;

    // 7. Trimitem răspunsul către Unity (Breakpoint-ul)
    Ok(Json(SyncResponse {
        message: "Progres salvat cu succes. Breakpoint creat.".to_string(),
        player_id: updated_player.id,
        current_coins: updated_player.coins.unwrap_or(0),
        current_energy: updated_player.energy.unwrap_or(0),
        synchronized_items_count: payload.new_items.len(),
    }))
}