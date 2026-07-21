use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::SqlitePool;

use crate::models::UpdateShiftStatusPayload;

pub async fn update_shift_status(
    State(pool): State<SqlitePool>,
    Json(payload): Json<UpdateShiftStatusPayload>,
) -> impl IntoResponse {
    // 1. Récupérer les infos de la demande ciblée
    let target = sqlx::query!(
        "SELECT service_id, period_type, start_date FROM shift_requests WHERE request_id = ?",
        payload.request_id
    )
    .fetch_optional(&pool)
    .await;

    if let Ok(Some(req)) = target {
        if payload.new_status == "Validée" {
            // RÈGLE : Si validée, on passe toutes les autres demandes du MÊME SERVICE et MÊME PÉRIODE en 'Refusée'
            let _ = sqlx::query!(
                "UPDATE shift_requests SET status = 'Refusée' WHERE service_id = ? AND period_type = ? AND start_date = ? AND status = 'En attente'",
                req.service_id,
                req.period_type,
                req.start_date
            )
            .execute(&pool)
            .await;
        }

        // On applique le statut à la demande ciblée
        let _ = sqlx::query!(
            "UPDATE shift_requests SET status = ? WHERE request_id = ?",
            payload.new_status,
            payload.request_id
        )
        .execute(&pool)
        .await;

        (StatusCode::OK, "Statut mis à jour").into_response()
    } else {
        (StatusCode::NOT_FOUND, "Demande introuvable").into_response()
    }
}