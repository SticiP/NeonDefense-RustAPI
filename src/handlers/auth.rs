use axum::Json;
use uuid::Uuid;
use crate::models::auth::{AuthResponse, GuestAuthRequest};

pub async fn guest_auth(Json(payload): Json<GuestAuthRequest>) -> Json<AuthResponse> {
    // Generăm un UUID fals pentru moment
    let fake_user_id = Uuid::new_v4();

    Json(AuthResponse {
        message: format!("Salut {}, cont creat cu succes!", payload.nickname),
        token: "fake_jwt_token".to_string(),
        user_id: fake_user_id,
    })
}