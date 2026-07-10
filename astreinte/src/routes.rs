// Ce fichier contient la logique des endpoints (GET, POST)

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST}; 
use sqlx::SqlitePool;

use crate::models::{CreateUserPayload, LoginPayload};

// Cette fonction est notre "Handler" pour la création d'utilisateur
pub async fn create_user(
    // On récupère le pool de connexion à la DB que l'on va passer via l'état de l'application
    State(pool): State<SqlitePool>,
    // Axum transforme automatiquement le JSON de la requête en notre structure Rust
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    
    // 1. Hachage du mot de passe provisoire
    let hashed_password = match hash(&payload.password_temp, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur de chiffrement du mot de passe".to_string()),
    };

    // 2. Insertion dans la base de données via sqlx
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

    // 3. Gestion de la réponse
    match insert_result {
        Ok(_) => (StatusCode::CREATED, "Utilisateur créé avec succès".to_string()),
        Err(e) => {
            // Gestion simple de l'erreur (ex: email déjà existant)
            (StatusCode::BAD_REQUEST, format!("Erreur lors de la création : {}", e))
        }
    }
}

pub async fn login_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    
    // 1. Chercher l'utilisateur dans la base via son email
    let user = sqlx::query!(
        "SELECT user_id, password_hash, must_change_password FROM users WHERE email = ?",
        payload.email
    )
    .fetch_optional(&pool)
    .await;

    match user {
        Ok(Some(record)) => {
            // 2. L'utilisateur existe, on compare le mot de passe en clair avec le hash en base
            let is_valid = verify(&payload.password, &record.password_hash).unwrap_or(false);
            
            if is_valid {
                // 3. Vérifier s'il doit changer son mot de passe (première connexion)
                if record.must_change_password == 1 {
                    (StatusCode::OK, "REQUIRES_PASSWORD_CHANGE".to_string())
                } else {
                    (StatusCode::OK, "Connexion réussie".to_string())
                }
            } else {
                (StatusCode::UNAUTHORIZED, "Mot de passe incorrect".to_string())
            }
        },
        Ok(None) => (StatusCode::NOT_FOUND, "Utilisateur introuvable".to_string()),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de la connexion à la base".to_string()),
    }
}