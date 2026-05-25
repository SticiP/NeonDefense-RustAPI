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
        // Extragem ruta pentru a oferi context log-urilor
        let path = parts.uri.path().to_string();

        // 1. Extragem header-ul "Authorization"
        let auth_header = match parts.headers.get(header::AUTHORIZATION).and_then(|value| value.to_str().ok()) {
            Some(header_value) => header_value,
            None => {
                tracing::warn!(
                    target: "AUTH_MIDDLEWARE",
                    path = %path,
                    "JWT Extraction FAILED: Missing Authorization header."
                );
                return Err((StatusCode::UNAUTHORIZED, "Header de autorizare lipsă!".to_string()));
            }
        };

        // 2. Verificăm dacă respectă formatul "Bearer <token>"
        if !auth_header.starts_with("Bearer ") {
            tracing::warn!(
                target: "AUTH_MIDDLEWARE",
                path = %path,
                "JWT Extraction FAILED: Invalid token format. Expected 'Bearer <token>'."
            );
            return Err((StatusCode::UNAUTHORIZED, "Format token invalid! Folosește 'Bearer <token>'".to_string()));
        }
        
        let token = &auth_header["Bearer ".len()..];

        // 3. Decodăm și validăm token-ul
        // Folosim |e| pentru a prinde eroarea exactă generată de jsonwebtoken (ex: ExpiredSignature, InvalidSignature)
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default()
        ).map_err(|e| {
            tracing::warn!(
                target: "AUTH_MIDDLEWARE",
                path = %path,
                error = %e,
                "JWT Validation FAILED: Token is invalid, tampered, or expired."
            );
            (StatusCode::UNAUTHORIZED, "Token invalid sau expirat!".to_string())
        })?;

        // 4. Logăm succesul (opțional, poate fi schimbat în tracing::debug! dacă devine prea aglomerat log-ul)
        tracing::info!(
            target: "AUTH_MIDDLEWARE",
            path = %path,
            user_id = %token_data.claims.sub,
            "JWT Validation SUCCESS. Session authenticated."
        );

        // Returnăm doar informațiile utile (Claims)
        Ok(token_data.claims)
    }
}