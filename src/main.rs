mod handlers;
mod models;

use axum::{routing::{get, post}, Router};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use std::sync::Arc;

// Structura care va fi partajată între toate rutele
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub hmac_secret: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // 1. Citim variabilele injectate de Docker
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL lipsă!");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET lipsă!");
    let hmac_secret = std::env::var("HMAC_SECRET").expect("HMAC_SECRET lipsă!");

    // 2. Creăm Pool-ul de conexiuni
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Eroare conexiune DB");

    println!("[>>] Conectat la baza de date cu succes.");

    // 3. MAGIA: Rulăm migrațiile automate
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Eroare la rularea migrațiilor SQL!");

    println!("[>>] Migrațiile au fost validate/executate.");

    // 4. Asamblăm starea aplicației
    let state = Arc::new(AppState {
        db: pool,
        jwt_secret,
        hmac_secret,
    });

    // 5. Definim rutele și injectăm state-ul
    let app = Router::new()
        .route("/v1/health", get(handlers::health::health_check))
        .route("/v1/auth/register", post(handlers::auth::register))
        .route("/v1/auth/login", post(handlers::auth::login))
        .route("/v1/player/create", post(handlers::player::create_profile))
        .layer(CorsLayer::permissive())
        .with_state(state); 

    // 6. Pornim serverul
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("[>>] Rust API rulează pe: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}