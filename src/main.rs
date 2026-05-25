mod handlers;
mod models;
mod middlewares;

use axum::{routing::{get, post}, Router};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Structura care va fi partajată între toate rutele
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub hmac_secret: String,
}

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::daily("/app/logs", "neon_api.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        // MAGIA: Arată logurile INFO globale, dar de la sqlx arată doar WARNING-urile, ignorând DEBUG-ul
        .with(EnvFilter::new("info,sqlx=warn")) 
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
        .with(tracing_subscriber::fmt::layer().json().with_writer(non_blocking))
        .init();

    tracing::info!("Server Rust API inițializat cu succes!");

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
        .route("/v1/player/profile", get(handlers::player::get_profile))
        .route("/v1/game/sync", post(handlers::game::sync_match))
        .route("/v1/game/init", get(handlers::game::get_init_data))
        .layer(axum::middleware::from_fn_with_state(state.clone(), middlewares::hmac_verify::verify_hmac))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state); 

    // 6. Pornim serverul
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("[>>] Rust API rulează pe: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}