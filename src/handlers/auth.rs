use axum::{extract::State, Json, http::StatusCode};
use std::sync::Arc;
use uuid::Uuid;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::AppState;

// Importăm modelele corecte
use crate::models::auth::{RegisterRequest, LoginRequest, AuthResponse, Claims};

// --- RUTA 1: REGISTER ---
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>, // Folosim modelul oficial
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    
    // 1. Hashuim parola cu Argon2
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .to_string();

    // 2. Inserăm utilizatorul în tabelul `users`
    let user_record = sqlx::query!(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id",
        payload.email,
        password_hash
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| (StatusCode::CONFLICT, "Email-ul este deja folosit!".to_string()))?;

    // 3. Generăm JWT
    let token = generate_jwt(user_record.id, &state.jwt_secret);

    Ok(Json(AuthResponse {
        token,
        user_id: user_record.id,
        message: "Cont creat cu succes. Setează-ți nickname-ul!".to_string(),
    }))
}

// --- RUTA 2: LOGIN ---
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>, // <-- Modifică aici din AuthPayload în LoginRequest
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    
    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE email = $1 AND is_deleted = false",
        payload.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Eroare DB".to_string()))?
    .ok_or((StatusCode::UNAUTHORIZED, "Email sau parolă greșită!".to_string()))?;

    // 2. Verificăm parola
    let hash_str = user.password_hash
        .as_deref()
        .ok_or((StatusCode::UNAUTHORIZED, "Contul nu are o parolă setată!".to_string()))?;

    let parsed_hash = PasswordHash::new(hash_str)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Eroare parsare hash".to_string()))?;
    
    Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Email sau parolă greșită!".to_string()))?;

    // 3. Generăm JWT nou
    let token = generate_jwt(user.id, &state.jwt_secret);

    Ok(Json(AuthResponse {
        token,
        user_id: user.id,
        message: "Login reușit!".to_string(),
    }))
}

// Funcție utilitară pentru generarea token-ului
fn generate_jwt(user_id: Uuid, secret: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .expect("Valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}