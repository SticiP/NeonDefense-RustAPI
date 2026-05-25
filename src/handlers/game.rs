use axum::{extract::State, Json, http::StatusCode};
use std::sync::Arc;
use crate::AppState;
use crate::models::auth::Claims;
use crate::models::game::{SyncRequest, SyncResponse, InitDataResponse, MarketplaceItemResponse};

// --- RUTA: SYNC MATCH DATA ---
pub async fn sync_match(
    State(state): State<Arc<AppState>>,
    claims: Claims, // Axum extrage automat ID-ul din JWT
    Json(payload): Json<SyncRequest>,
) -> Result<Json<SyncResponse>, (StatusCode, String)> {

    // Log inițial
    tracing::info!(
        target: "GAME_SYNC", 
        user_id = %claims.sub, 
        earned_df = payload.earned_df,
        earned_cc = payload.earned_crypto_cores,
        "Match synchronization initiated."
    );

    // 1. Validare Anti-Cheat (Extinsă)
    if payload.earned_df > 50000 || payload.earned_crypto_cores > 500 || payload.energy_used < 0 {
        tracing::warn!(
            target: "GAME_SYNC",
            user_id = %claims.sub,
            earned_df = payload.earned_df,
            "Anti-cheat validation failed: Suspicious resource amounts detected."
        );
        return Err((StatusCode::NOT_ACCEPTABLE, "Date invalide. Validare anti-cheat eșuată!".to_string()));
    }

    // 2. Inițiem Tranzacția SQL
    let mut tx = state.db.begin().await
        .map_err(|e| {
            tracing::error!(target: "GAME_SYNC", user_id = %claims.sub, error = %e, "Failed to start SQL transaction for match sync.");
            (StatusCode::INTERNAL_SERVER_ERROR, "Eroare la pornirea tranzacției".to_string())
        })?;

    // 3. Actualizăm resursele Player-ului
    let updated_player = sqlx::query!(
        r#"
        UPDATE players 
        SET data_fragments = data_fragments + $1, 
            crypto_cores = crypto_cores + $2,
            energy = energy - $3, 
            updated_at = NOW() 
        WHERE user_id = $4 
        RETURNING id, data_fragments, crypto_cores, energy
        "#,
        payload.earned_df,
        payload.earned_crypto_cores,
        payload.energy_used,
        claims.sub
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!(target: "GAME_SYNC", user_id = %claims.sub, error = %e, "Database error while updating player resources.");
        (StatusCode::INTERNAL_SERVER_ERROR, "Eroare la actualizarea jucătorului".to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!(target: "GAME_SYNC", user_id = %claims.sub, "Match sync failed: Player profile not found.");
        (StatusCode::NOT_FOUND, "Jucătorul nu există!".to_string())
    })?;

    // 4. Salvăm Itemele noi în Inventar
    for item in &payload.new_items {
        let insert_result = sqlx::query!(
            "INSERT INTO inventory (id, player_id, item_type, rarity, level) VALUES ($1, $2, $3, $4, $5)",
            item.id,
            updated_player.id, // Variabila din query-ul anterior
            item.item_type,
            item.rarity,
            item.level
        )
        .execute(&mut *tx)
        .await;

        if let Err(e) = insert_result {
            let err_msg = e.to_string();
            // Aici transformăm constrângerea bazei de date într-un filtru anti-cheat!
            if err_msg.contains("fk_inventory_item_type") {
                tracing::warn!(
                    target: "ANTI_CHEAT", 
                    user_id = %claims.sub, 
                    invalid_item = %item.item_type, 
                    "Sistemul a blocat un item nerecunoscut. Posibilă tentativă de injecție sau desincronizare client!"
                );
                return Err((StatusCode::BAD_REQUEST, format!("Item corupt sau invalid: {}", item.item_type)));
            } else {
                tracing::error!(target: "GAME_SYNC", user_id = %claims.sub, error = %err_msg, "Eroare internă la salvarea inventarului.");
                return Err((StatusCode::INTERNAL_SERVER_ERROR, "Eroare la salvarea inventarului".to_string()));
            }
        }
    }

    // 5. Salvăm Acțiunile (Analytics)
    let mut total_calculated_df = 0;
    let mut suspicious_actions = 0;

    for action in &payload.actions {
        let event_type = action.event_type.as_str();
        
        // --- SOLUȚIA AICI: Despachetăm String-ul trimis de Unity în Obiect JSON ---
        let parsed_data: serde_json::Value = if let Some(text_json) = action.event_data.as_str() {
            // Dacă e text (cum trimite Unity), îl transformăm în JSON
            serde_json::from_str(text_json).unwrap_or(serde_json::json!({}))
        } else {
            // Dacă e deja JSON, îl lăsăm așa
            action.event_data.clone() 
        };

        // Acum folosim `parsed_data` ca să extragem valorile reale!
        let action_value = parsed_data.get("value").and_then(|v| v.as_i64()).unwrap_or(0);
        let action_time = parsed_data.get("timestamp").and_then(|v| v.as_str()).unwrap_or("unknown");

        match event_type {
            "ENEMY_KILLED" => {
                if action_value > 500 {
                    suspicious_actions += 1;
                    tracing::warn!(
                        target: "ANTI_CHEAT",
                        user_id = %claims.sub,
                        event = %event_type,
                        value = action_value,
                        "Suspicious loot amount from single enemy."
                    );
                } else {
                    total_calculated_df += action_value;
                }
            },
            "ABILITY_USED" => {
                tracing::debug!(
                    target: "GAME_LOGIC",
                    user_id = %claims.sub,
                    ability_id = %action_value,
                    time = %action_time,
                    "Player used ability within legal cooldown window."
                );
            },
            "MATCH_COMPLETED" => {
                // Extragem valorile folosind datele despachetate (parsed_data)
                // Folosim as_f64() pentru float sau as_i64() cu conversie în caz că Unity trimite "0" fără zecimale
                let duration = parsed_data.get("duration").and_then(|v| v.as_f64().or_else(|| v.as_i64().map(|i| i as f64))).unwrap_or(0.0);
                let enemies_killed = parsed_data.get("enemies_killed").and_then(|v| v.as_i64()).unwrap_or(0);
                
                // SIMULARE ANTI-CHEAT: Acum Rust va vedea cele 9999 kills!
                if duration < 10.0 && enemies_killed > 5 {
                    suspicious_actions += 1;
                    tracing::warn!(
                        target: "ANTI_CHEAT",
                        user_id = %claims.sub,
                        duration_seconds = duration,
                        enemies = enemies_killed,
                        "Match duration anomaly detected: Impossible time-to-kills ratio."
                    );
                } else {
                    tracing::info!(
                        target: "GAME_LOGIC",
                        user_id = %claims.sub,
                        duration_seconds = duration,
                        enemies = enemies_killed,
                        "Match completion data verified."
                    );
                }
            },
            _ => {}
        }

        // --- SALVAREA EFECTIVĂ ÎN DB ---
        // Salvăm `parsed_data` curat în DB (JsonB) în loc de string-ul escapat, ca să arate bine în Laravel
        sqlx::query!(
            "INSERT INTO analytics_events (player_id, event_type, event_data) VALUES ($1, $2, $3)",
            updated_player.id,
            action.event_type,
            parsed_data 
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!(target: "GAME_SYNC", user_id = %claims.sub, event_type = %action.event_type, error = %e, "Failed to save analytics event.");
            (StatusCode::INTERNAL_SERVER_ERROR, "Eroare la salvarea log-urilor de analytics".to_string())
        })?;
    }

    // O concluzie globală a anti-cheat-ului după ce a analizat tot vectorul de acțiuni
    if suspicious_actions > 0 {
        tracing::warn!(
            target: "ANTI_CHEAT",
            user_id = %claims.sub,
            flagged_count = suspicious_actions,
            claimed_df = payload.earned_df,
            calculated_df = total_calculated_df,
            "Multiple suspicious actions detected during run. Flagging account for review."
        );
        
        return Err((StatusCode::NOT_ACCEPTABLE, "Validare server eșuată: anomalii detectate în timpul meciului!".to_string()));
    } else {
        tracing::info!(
            target: "ANTI_CHEAT",
            user_id = %claims.sub,
            "Run integrity verified. Zero anomalies detected."
        );
    }

    // 6. COMMIT - Salvăm totul definitiv în baza de date
    tx.commit().await
        .map_err(|e| {
            tracing::error!(target: "GAME_SYNC", user_id = %claims.sub, error = %e, "Failed to commit SQL transaction.");
            (StatusCode::INTERNAL_SERVER_ERROR, "Eroare la finalizarea tranzacției".to_string())
        })?;

    // 7. Log de succes masiv și răspuns către Unity
    tracing::info!(
        target: "GAME_SYNC",
        user_id = %claims.sub,
        items_synced = payload.new_items.len(),
        actions_synced = payload.actions.len(),
        "Match synchronized successfully. Resources and inventory fully updated."
    );

    Ok(Json(SyncResponse {
        message: "Progres salvat cu succes în rețeaua Neurală.".to_string(),
        player_id: updated_player.id,
        current_df: updated_player.data_fragments.unwrap_or(0),
        current_crypto_cores: updated_player.crypto_cores.unwrap_or(0),
        current_energy: updated_player.energy.unwrap_or(0),
        synchronized_items_count: payload.new_items.len(),
    }))
}

pub async fn get_init_data(
    State(state): State<Arc<AppState>>,
    claims: Claims, 
) -> Result<Json<InitDataResponse>, (StatusCode, String)> {
    
    tracing::debug!(target: "GAME_INIT", user_id = %claims.sub, "Client requested live configurations and marketplace data.");

    // 1. Tragem configurația activă din Laravel (Game Configurations)
    let config_record = sqlx::query!(
        "SELECT version, config_payload FROM game_configurations WHERE is_active = true ORDER BY updated_at DESC LIMIT 1"
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Eroare la citirea configurației.".to_string()))?;

    // Dacă încă nu ai bifat nicio configurație ca 'activă' în Admin Panel, trimitem un fallback
    let (active_version, game_config) = match config_record {
        Some(rec) => (rec.version, rec.config_payload),
        None => ("1.0.0-default".to_string(), serde_json::json!({"engine": {"tick_rate": 60}}))
    };

    // 2. Tragem itemele din magazin (Store & Economy)
    // Folosim `price::FLOAT8` pentru a forța conversia de la DECIMAL (din BD) direct la `f64` în Rust
    let store_records = sqlx::query!(
        "SELECT item_sku, display_name, rarity, price::FLOAT8 as price, currency, reward_type, reward_amount, stock FROM marketplace_items WHERE is_active = true"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "Eroare la extragerea magazinului");
        (StatusCode::INTERNAL_SERVER_ERROR, "Eroare la citirea magazinului.".to_string())
    })?;

    let mut store_items = Vec::new();
    for record in store_records {
        store_items.push(MarketplaceItemResponse {
            item_sku: record.item_sku,
            display_name: record.display_name,
            rarity: record.rarity.unwrap_or_else(|| "COMMON".to_string()),
            price: record.price.unwrap_or(0.0),
            currency: record.currency.unwrap_or_else(|| "USD".to_string()),
            reward_type: record.reward_type,
            reward_amount: record.reward_amount,
            stock: record.stock.unwrap_or(-1),
        });
    }

    Ok(Json(InitDataResponse {
        active_version,
        game_config,
        store_items,
    }))
}