use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String, // Corectat din password_hash
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String, // Corectat din password_hash
}

#[derive(Deserialize)]
#[allow(dead_code)] // Oprim warning-ul până vom folosi acest struct
pub struct GuestAuthRequest {
    pub nickname: String, 
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: Uuid,
    pub message: String,
}

// Mutăm Claims aici pentru a fi vizibil global
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: Uuid, // Subject (User ID)
    pub exp: usize, // Expiration
}