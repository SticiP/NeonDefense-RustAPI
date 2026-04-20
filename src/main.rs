use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Deserialize)]
struct GuestAuthRequest {
    nickname: String,
}

#[derive(Serialize)]
struct UserProfile {
    nickname: String,
    account_id: String,
    coins: u32,
    energy: u32,
}

#[derive(Serialize)]
struct AuthResponse {
    message: String,
    token: String,
    profile: UserProfile,
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "online".to_string(),
        version: "0.1.0".to_string(),
    })
}

async fn guest_auth(Json(payload): Json<GuestAuthRequest>) -> Json<AuthResponse> {
    let account_id = format!("USR_{}", 12345); // Aici vei genera un ID unic real
    
    let profile = UserProfile {
        nickname: payload.nickname,
        account_id: account_id.clone(),
        coins: 500,
        energy: 50,
    };

    Json(AuthResponse {
        message: "Cont creat cu succes!".to_string(),
        token: format!("fake_jwt_token_{}", account_id),
        profile,
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/v1/health", get(health_check))
        .route("/v1/auth/guest", post(guest_auth))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("[>>] Rust API rulează pe: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}