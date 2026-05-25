// src/handlers/health.rs

use axum::Json;
use serde::Serialize;

// Definim forma răspunsului. 
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

// Funcția returnează Json<HealthResponse>
pub async fn health_check() -> Json<HealthResponse> {
    let current_version = "0.1.0";
    
    // Logăm verificarea stării sistemului
    // Fiind o rută publică, marcăm log-ul ca un eveniment de sistem (heartbeat)
    tracing::info!(
        target: "SYSTEM_HEALTH",
        version = %current_version,
        "Public heartbeat check received. System state: operational."
    );
    
    Json(HealthResponse {
        status: "online".to_string(),
        version: current_version.to_string(),
    })
}