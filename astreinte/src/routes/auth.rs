use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use rand::Rng;
use sqlx::SqlitePool;

use crate::models::{LoginPayload, LoginResponse, ChangePasswordPayload};

pub fn generate_user_tag(name: &str) -> String {
    let first_name = name.split_whitespace().next().unwrap_or("X");
    
    // Création d'une empreinte unique avec un multiplicateur pour bien disperser les prénoms proches
    let mut hash: u32 = 0;
    for (i, c) in first_name.chars().enumerate() {
        hash = hash.wrapping_add((c as u32).wrapping_mul((i as u32) + 1) * 37);
    }
    
    // Modèle HSL : Teinte, Saturation (75%), Luminosité (55%)
    // On force le type f32 pour que la méthode .abs() fonctionne sans ambiguïté
    let h: f32 = (hash % 360) as f32;
    let s: f32 = 0.75; 
    let l: f32 = 0.55; 

    let c: f32 = (1.0_f32 - (2.0_f32 * l - 1.0_f32).abs()) * s;
    let x: f32 = c * (1.0_f32 - ((h / 60.0_f32) % 2.0_f32 - 1.0_f32).abs());
    let m: f32 = l - c / 2.0_f32;

    let (r1, g1, b1) = if h < 60.0 { (c, x, 0.0) }
    else if h < 120.0 { (x, c, 0.0) }
    else if h < 180.0 { (0.0, c, x) }
    else if h < 240.0 { (0.0, x, c) }
    else if h < 300.0 { (x, 0.0, c) }
    else { (c, 0.0, x) };

    let r = ((r1 + m) * 255.0).round() as u8;
    let g = ((g1 + m) * 255.0).round() as u8;
    let b = ((b1 + m) * 255.0).round() as u8;

    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub async fn login_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let email = payload.email.trim();

    let user = sqlx::query!(
        "SELECT user_id, name, email, role, service_id, password_hash, user_tag, must_change_password FROM users WHERE email = ?",
        email
    )
    .fetch_optional(&pool)
    .await;

    match user {
        Ok(Some(record)) => {
            if verify(&payload.password, &record.password_hash).unwrap_or(false) {
                let response = LoginResponse {
                    user_id: record.user_id,
                    name: record.name,
                    email: record.email,
                    role: record.role,
                    service_id: record.service_id,
                    user_tag: record.user_tag,
                    must_change_password: record.must_change_password,
                };
                (StatusCode::OK, Json(response)).into_response()
            } else {
                (StatusCode::UNAUTHORIZED, "Identifiants invalides").into_response()
            }
        }
        _ => (StatusCode::UNAUTHORIZED, "Identifiants invalides").into_response(),
    }
}

pub async fn change_password(
    State(pool): State<SqlitePool>,
    Json(payload): Json<ChangePasswordPayload>,
) -> impl IntoResponse {
    let user = sqlx::query!(
        "SELECT password_hash FROM users WHERE email = ?",
        payload.email
    )
    .fetch_optional(&pool)
    .await;

    match user {
        Ok(Some(record)) => {
            // Vérifie l'ancien mot de passe
            if !bcrypt::verify(&payload.old_password, &record.password_hash).unwrap_or(false) {
                return (StatusCode::UNAUTHORIZED, "Ancien mot de passe incorrect".to_string()).into_response();
            }

            // Hache le nouveau
            let new_hashed = match bcrypt::hash(&payload.new_password, bcrypt::DEFAULT_COST) {
                Ok(h) => h,
                Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur de chiffrement").into_response(),
            };

            // Met à jour en base
            let update_result = sqlx::query!(
                "UPDATE users SET password_hash = ?, must_change_password = 0 WHERE email = ?",
                new_hashed,
                payload.email
            )
            .execute(&pool)
            .await;

            match update_result {
                Ok(_) => (StatusCode::OK, "Mot de passe mis à jour").into_response(),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur de base de données").into_response(),
            }
        },
        _ => (StatusCode::NOT_FOUND, "Utilisateur introuvable".to_string()).into_response(),
    }
}