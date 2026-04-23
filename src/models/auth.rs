// src/models/auth.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct GuestAuthRequest {
    pub nickname: String, 
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub token: String,
    pub user_id: Uuid,
}