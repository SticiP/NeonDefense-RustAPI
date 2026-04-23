// src/handlers/health.rs

use axum::Json;
use serde::Serialize;

// Definim forma răspunsului. 
// `#[derive(Serialize)]` este un macro care îi spune lui Rust să genereze automat 
// logica necesară pentru a transforma această structură în format JSON.
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

// `pub` face funcția vizibilă în afara acestui fișier (altfel `main.rs` nu o poate apela).
// `async` permite funcției să ruleze non-blocant în ecosistemul Tokio.
// Funcția returnează `Json<HealthResponse>`, adică un wrapper Axum peste structura noastră.
pub async fn health_check() -> Json<HealthResponse> {
    
    // Construim instanța structurii. Lipsa punctului și virgulei (;) la final 
    // înseamnă că returnăm automat acest obiect.
    Json(HealthResponse {
        // În Rust, textele scrise direct în cod ("online") sunt de tip `&str` (referințe statice).
        // Folosim `.to_string()` pentru a le transforma în obiecte `String` deținute de structură,
        // alocate dinamic pe memoria Heap.
        status: "online".to_string(),
        version: "0.1.0".to_string(),
    })
}