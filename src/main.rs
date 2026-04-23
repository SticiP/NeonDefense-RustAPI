mod handlers;
mod models;

use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
// Adăugăm tool-urile de la sqlx
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // 1. Citim URL-ul bazei de date din mediul Docker (din docker-compose.yml)
    let db_url = std::env::var("DATABASE_URL")
        .expect("Variabila DATABASE_URL nu este setată!");

    // 2. Creăm un Pool de conexiuni către baza de date
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Nu m-am putut conecta la baza de date PostgreSQL!");

    println!("[>>] Conectat la baza de date cu succes.");

    // 3. MAGIA: Rulăm migrațiile din folderul /migrations automat!
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Eroare la rularea migrațiilor SQL!");

    println!("[>>] Migrațiile au fost validate/executate.");

    // 4. Setăm rutele
    let app = Router::new()
        .route("/v1/health", get(handlers::health::health_check))
        .route("/v1/auth/guest", post(handlers::auth::guest_auth))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("[>>] Rust API rulează pe: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}