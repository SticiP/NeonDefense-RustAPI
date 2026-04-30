use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode, header},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;

use crate::AppState;
use crate::models::auth::Claims;

#[async_trait]
impl FromRequestParts<Arc<AppState>> for Claims {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &Arc<AppState>) -> Result<Self, Self::Rejection> {
        // 1. Extragem header-ul "Authorization"
        let auth_header = parts.headers.get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Header de autorizare lipsă!".to_string()))?;

        // 2. Verificăm dacă respectă formatul "Bearer <token>"
        if !auth_header.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "Format token invalid! Folosește 'Bearer <token>'".to_string()));
        }
        let token = &auth_header["Bearer ".len()..];

        // 3. Decodăm și validăm token-ul
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default()
        ).map_err(|_| (StatusCode::UNAUTHORIZED, "Token invalid sau expirat!".to_string()))?;

        // 4. Returnăm doar informațiile utile (Claims)
        Ok(token_data.claims)
    }
}