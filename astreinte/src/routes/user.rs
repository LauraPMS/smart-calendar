use axum::{extract::{State, Path}, http::StatusCode, response::IntoResponse, Json};
use sqlx::SqlitePool;

use crate::models::{CreateShiftRequestPayload, ShiftResponse};

pub async fn create_shift_request(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateShiftRequestPayload>,
) -> impl IntoResponse {
    let res = sqlx::query!(
        "INSERT INTO shift_requests (user_id, service_id, period_type, start_date, status) VALUES (?, ?, ?, ?, 'En attente')",
        payload.user_id,
        payload.service_id,
        payload.period_type,
        payload.start_date
    )
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (StatusCode::CREATED, "Demande enregistrée").into_response(),
        Err(_) => (StatusCode::BAD_REQUEST, "Demande déjà existante pour ce bloc").into_response(),
    }
}

pub async fn get_service_shifts(
    State(pool): State<SqlitePool>,
    Path(service_id): Path<i64>,
) -> impl IntoResponse {
    let shifts = sqlx::query_as!(
        ShiftResponse,
        "SELECT r.request_id, r.user_id, r.service_id, r.period_type, r.start_date, r.status, u.user_tag 
         FROM shift_requests r 
         JOIN users u ON r.user_id = u.user_id 
         WHERE r.service_id = ?",
        service_id
    )
    .fetch_all(&pool)
    .await;

    match shifts {
        Ok(list) => (StatusCode::OK, Json(list)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur BDD").into_response(),
    }
}