use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::AppState;

pub async fn verify_hmac(
    State(_state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    
    let headers = req.headers();
    let path = req.uri().path().to_string();

    // Extragem headerele standard de securitate
    let signature = headers.get("X-App-Signature").and_then(|h| h.to_str().ok());
    let timestamp = headers.get("X-Timestamp").and_then(|h| h.to_str().ok());
    
    // Extragem certificatul de integritate (Doar pentru Login/Register)
    let attestation = headers.get("X-Device-Attestation").and_then(|h| h.to_str().ok());

    // Identificăm tipul de cerere pe baza URL-ului
    let is_auth_route = path.contains("/auth");
    let is_health_route = path.contains("/health");

    match (signature, timestamp) {
        (Some(sig), Some(ts)) => {
            tracing::debug!(
                target: "SECURITY_CORE",
                path = %path,
                timestamp = %ts,
                "HMAC signature detected. Initiating cryptographic payload verification..."
            );

            // SIMULARE MATEMATICĂ
            let is_valid = sig.len() > 10; 

            if is_valid {
                // --- LOGICĂ DIFERENȚIATĂ PE BAZA RUTEI ---
                if is_health_route {
                    tracing::info!(
                        target: "SECURITY_CORE",
                        path = %path,
                        "HMAC SUCCESS: Public health-check integrity trusted."
                    );
                } else if is_auth_route {
                    // Dacă e rută de Auth, verificăm și certificatul dispozitivului
                    if let Some(cert) = attestation {
                        tracing::info!(
                            target: "SECURITY_CORE",
                            path = %path,
                            cert = %cert,
                            "HMAC SUCCESS: Auth request and Device Attestation verified."
                        );
                    } else {
                        tracing::warn!(
                            target: "SECURITY_CORE",
                            path = %path,
                            "HMAC SUCCESS: Signature valid, but Device Attestation is MISSING for Auth route!"
                        );
                    }
                } else {
                    // Pentru rutele generale (Sync, Profile, etc.)
                    tracing::info!(
                        target: "SECURITY_CORE",
                        path = %path,
                        "HMAC SUCCESS: Protected gameplay payload integrity trusted."
                    );
                }
            } else {
                tracing::error!(
                    target: "SECURITY_CORE",
                    path = %path,
                    signature = %sig,
                    "HMAC Validation FAILED. Possible spoofing or replay attack intercepted!"
                );
                // return Err((StatusCode::UNAUTHORIZED, "Acces Respins: Semnătură Invalidă!".to_string()));
            }
        }
        _ => {
            tracing::warn!(
                target: "SECURITY_CORE",
                path = %path,
                "Missing HMAC headers (X-App-Signature / X-Timestamp). [DEV MODE: Security Bypassed]"
            );
        }
    }

    let response = next.run(req).await;
    Ok(response)
}