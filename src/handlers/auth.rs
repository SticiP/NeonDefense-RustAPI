// src/handlers/auth.rs

use axum::Json;
// Importăm structurile create mai devreme. `crate` înseamnă "rădăcina proiectului nostru".
use crate::models::auth::{AuthResponse, GuestAuthRequest, UserProfile};

// `async fn` declară o funcție asincronă, necesară pentru serverele web ca să nu blocheze execuția.
// Parametrul: `Json(payload): Json<GuestAuthRequest>`. Axum extrage automat JSON-ul din request
// și îl transformă în structura noastră. Dacă JSON-ul e invalid, Axum returnează eroare automat.
// Returnează: `Json<AuthResponse>`, adică promite să dea înapoi un JSON cu structura specificată.
pub async fn guest_auth(Json(payload): Json<GuestAuthRequest>) -> Json<AuthResponse> {
    
    // `let` declară o variabilă. Implicit, toate variabilele sunt IMUTABILE (nu pot fi modificate).
    // Funcția `format!` este un macro (vezi semnul exclamării) folosit pentru a concatena string-uri.
    let account_id = format!("USR_{}", 12345); 
    
    // Creăm o instanță a structurii UserProfile.
    let profile = UserProfile {
        nickname: payload.nickname, // Mutăm valoarea din payload aici.
        // `.clone()` e important în Rust! Fiindcă Rust nu are Garbage Collector, el folosește 
        // un sistem de "Ownership". O variabilă are un singur proprietar. Dacă folosim `account_id` aici, 
        // îl "mutăm", și nu l-am mai putea folosi mai jos la token. Prin `.clone()` creăm o copie în memorie.
        account_id: account_id.clone(), 
        coins: 500,
        energy: 50,
    };

    // Ultima expresie dintr-o funcție, dacă NU are punct și virgulă (;) la final, 
    // este automat returnată. Este la fel ca și cum ai scrie `return Json(...);`
    Json(AuthResponse {
        // `.to_string()` convertește un text static ("Cont creat...") într-un obiect String dinamic.
        message: "Cont creat cu succes!".to_string(),
        token: format!("fake_jwt_token_{}", account_id),
        profile, // Aici dăm "pass" la structura de mai sus (prescurtare de la profile: profile)
    })
}