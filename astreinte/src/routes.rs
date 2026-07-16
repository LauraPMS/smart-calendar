use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::SqlitePool;

use crate::models::{CreateUserPayload, LoginPayload, ChangePasswordPayload};
use crate::models::{CreateRequestPayload, ShiftRequest, UpdateRequestPayload};


pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    println!("\n--- [DEBUG] TENTATIVE DE CREATION DE COMPTE ---");
    println!("1. Payload reçu depuis le front : {:?}", payload);
    
    let hashed_password = match hash(&payload.password_temp, DEFAULT_COST) {
        Ok(h) => h,
        Err(e) => {
            println!("❌ Erreur de hachage : {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur serveur").into_response();
        }
    };

    println!("2. Hachage réussi, tentative d'insertion SQLite...");

    let insert_result = sqlx::query!(
        "INSERT INTO users (name, role, email, password_hash, must_change_password) VALUES (?, ?, ?, ?, 1)",
        payload.name,
        payload.role,
        payload.email,
        hashed_password
    )
    .execute(&pool)
    .await;

    match insert_result {
        Ok(result) => {
            println!("✅ Résultat SQL : Ok");
            println!("📊 Lignes réellement modifiées : {}", result.rows_affected());
            
            if result.rows_affected() == 0 {
                println!("⚠️ ATTENTION BIZARRE : Requête acceptée mais 0 ligne ajoutée !");
            }
            
            (StatusCode::CREATED, "Utilisateur créé avec succès".to_string()).into_response()
        },
        Err(e) => {
            println!("❌ ERREUR FATALE SQL : {:?}", e);
            (StatusCode::BAD_REQUEST, format!("Erreur DB : {}", e)).into_response()
        }
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
    
    // On récupère le hash, mais aussi le nom et le rôle pour la connexion auto
    let user = sqlx::query!(
        "SELECT name, role, password_hash FROM users WHERE email = ?",
        payload.email
    )
    .fetch_optional(&pool)
    .await;

    match user {
        Ok(Some(record)) => {
            if !verify(&payload.old_password, &record.password_hash).unwrap_or(false) {
                return (StatusCode::UNAUTHORIZED, "Ancien mot de passe incorrect".to_string()).into_response();
            }

            let new_hashed = match hash(&payload.new_password, DEFAULT_COST) {
                Ok(h) => h,
                Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur de chiffrement".to_string()).into_response(),
            };

            let update_result = sqlx::query!(
                "UPDATE users SET password_hash = ?, must_change_password = 0 WHERE email = ?",
                new_hashed,
                payload.email
            )
            .execute(&pool)
            .await;

            match update_result {
                Ok(_) => {
                    // CONNEXION AUTOMATIQUE : On renvoie les données JSON
                    let json_response = serde_json::json!({
                        "name": record.name,
                        "role": record.role
                    });
                    (StatusCode::OK, Json(json_response)).into_response()
                },
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de la mise à jour".to_string()).into_response(),
            }
        },
        Ok(None) => (StatusCode::NOT_FOUND, "Utilisateur introuvable".to_string()).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur de base de données".to_string()).into_response(),
    }
}

pub async fn create_request(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateRequestPayload>,
) -> impl IntoResponse {
    let insert_result = sqlx::query!(
        "INSERT INTO requests (user_email, date, shift_type, preference, status) VALUES (?, ?, ?, ?, 'En attente')",
        payload.user_email,
        payload.date,
        payload.shift_type,
        payload.preference
    )
    .execute(&pool)
    .await;

    match insert_result {
        Ok(_) => (StatusCode::CREATED, "Demande enregistrée avec succès".to_string()).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur d'écriture en base".to_string()).into_response(),
    }
}

pub async fn get_requests(
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    let requests = sqlx::query_as!(
        ShiftRequest,
        "SELECT request_id, user_email, date, shift_type, preference, status FROM requests"
    )
    .fetch_all(&pool)
    .await;

    match requests {
        Ok(list) => (StatusCode::OK, Json(list)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur de lecture de la base".to_string()).into_response(),
    }
}

pub async fn update_request_status(
    State(pool): State<SqlitePool>,
    Json(payload): Json<UpdateRequestPayload>,
) -> impl IntoResponse 
{
    
    let update_result = sqlx::query!(
        "UPDATE requests SET status = ? WHERE request_id = ?",
        payload.new_status,
        payload.request_id
    )
    .execute(&pool)
    .await;

    match update_result {
        Ok(_) => (StatusCode::OK, "Statut mis à jour").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur de mise à jour").into_response(),
    }
}