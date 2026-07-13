use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::SqlitePool;

use crate::models::{CreateUserPayload, LoginPayload, ChangePasswordPayload};

pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    
    let hashed_password = match hash(&payload.password_temp, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur de chiffrement du mot de passe".to_string()),
    };

    let insert_result = sqlx::query!(
        r#"
        INSERT INTO users (name, role, email, password_hash, must_change_password)
        VALUES (?, ?, ?, ?, 1)
        "#,
        payload.name,
        payload.role,
        payload.email,
        hashed_password
    )
    .execute(&pool)
    .await;

    match insert_result {
        Ok(_) => (StatusCode::CREATED, "Utilisateur créé avec succès".to_string()),
        Err(e) => (StatusCode::BAD_REQUEST, format!("Erreur lors de la création : {}", e))
    }
}

pub async fn login_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    
    let user = sqlx::query!(
        "SELECT user_id, name, role, password_hash, must_change_password FROM users WHERE email = ?",
        payload.email
    )
    .fetch_optional(&pool)
    .await;

    match user {
        Ok(Some(record)) => {
            let is_valid = verify(&payload.password, &record.password_hash).unwrap_or(false);
            
            if is_valid {
                let status = if record.must_change_password == 1 {
                    "REQUIRES_PASSWORD_CHANGE"
                } else {
                    "OK"
                };
                
                let json_response = serde_json::json!({
                    "status": status,
                    "name": record.name,
                    "role": record.role
                });

                (StatusCode::OK, Json(json_response)).into_response()
            } else {
                (StatusCode::UNAUTHORIZED, "Mot de passe incorrect".to_string()).into_response()
            }
        },
        Ok(None) => (StatusCode::NOT_FOUND, "Utilisateur introuvable".to_string()).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur serveur".to_string()).into_response(),
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
            if !verify(&payload.old_password, &record.password_hash).unwrap_or(false) {
                return (StatusCode::UNAUTHORIZED, "Ancien mot de passe incorrect".to_string());
            }

            let new_hashed = match hash(&payload.new_password, DEFAULT_COST) {
                Ok(h) => h,
                Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur de chiffrement".to_string()),
            };

            let update_result = sqlx::query!(
                "UPDATE users SET password_hash = ?, must_change_password = 0 WHERE email = ?",
                new_hashed,
                payload.email
            )
            .execute(&pool)
            .await;

            match update_result {
                Ok(_) => (StatusCode::OK, "Mot de passe mis à jour avec succès".to_string()),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de la mise à jour".to_string()),
            }
        },
        Ok(None) => (StatusCode::NOT_FOUND, "Utilisateur introuvable".to_string()),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur de base de données".to_string()),
    }
}