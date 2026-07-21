use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::SqlitePool;

use crate::models::{CreateServicePayload, CreateUserPayload, Service};
use crate::routes::auth::generate_user_tag;

// Créer un service (ex: SSI, Réseau, Infra)
pub async fn create_service(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateServicePayload>,
) -> impl IntoResponse {
    let tag_formatted = format!("[{}]", payload.tag.trim().to_uppercase());

    let res = sqlx::query!(
        "INSERT INTO services (name, tag) VALUES (?, ?)",
        payload.name,
        tag_formatted
    )
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (StatusCode::CREATED, "Service créé avec succès").into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, format!("Erreur : {}", e)).into_response(),
    }
}

// Lister tous les services
pub async fn get_services(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let services = sqlx::query_as!(Service, "SELECT service_id, name, tag FROM services")
        .fetch_all(&pool)
        .await;

    match services {
        Ok(list) => (StatusCode::OK, Json(list)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur BDD").into_response(),
    }
}

// Créer un utilisateur (Admin, Manager ou User)
pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    let hashed = match hash(&payload.password_temp, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur hash").into_response(),
    };

    let user_tag = generate_user_tag(&payload.name);

    let res = sqlx::query!(
        "INSERT INTO users (service_id, name, email, password_hash, role, user_tag, must_change_password) VALUES (?, ?, ?, ?, ?, ?, 1)",
        payload.service_id,
        payload.name,
        payload.email,
        hashed,
        payload.role,
        user_tag
    )
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (StatusCode::CREATED, "Compte créé").into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, format!("Erreur : {}", e)).into_response(),
    }
}