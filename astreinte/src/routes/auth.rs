use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use rand::Rng;
use sqlx::SqlitePool;

use crate::models::{LoginPayload, LoginResponse, ChangePasswordPayload};

// Fonction helper pour générer un tag unique court (ex: LPM-A4)
pub fn generate_user_tag(name: &str) -> String {
    let parts: Vec<&str> = name.split_whitespace().collect();
    let prefix = if parts.len() >= 2 {
        format!(
            "{}{}",
            parts[0].chars().next().unwrap_or('X'),
            parts[1].chars().next().unwrap_or('X')
        )
    } else if let Some(first) = parts.first() {
        first.chars().take(2).collect::<String>()
    } else {
        "XX".to_string()
    };

    let mut rng = rand::thread_rng();
    let random_hex: u8 = rng.r#gen();
    format!("{}-{:02X}", prefix.to_uppercase(), random_hex)
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