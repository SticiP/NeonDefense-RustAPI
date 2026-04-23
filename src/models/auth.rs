// src/models/auth.rs

// `use` este echivalentul lui `import` sau `using`. Aducem uneltele din pachetul `serde`.
use serde::{Deserialize, Serialize};

// Macros: În Rust, tot ce începe cu `#[]` este un macro (un cod care generează alt cod).
// `#[derive(Deserialize)]` îi spune compilatorului să scrie automat codul necesar 
// pentru a transforma un text JSON în această structură (pentru Request-uri).
#[derive(Deserialize)]
pub struct GuestAuthRequest {
    // `pub` înseamnă public. În Rust, TOTUL este privat implicit (inclusiv variabilele din structuri).
    // `String` este un tip de date care deține textul și poate fi modificat (alocat pe Heap).
    pub nickname: String, 
}

// `#[derive(Serialize)]` face invers: transformă structura din Rust în JSON (pentru Răspunsuri).
#[derive(Serialize)]
pub struct UserProfile {
    pub nickname: String,
    pub account_id: String,
    // `u32` înseamnă "unsigned integer 32-bit" (număr întreg pozitiv).
    pub coins: u32,       
    pub energy: u32,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub token: String,
    pub profile: UserProfile,
}