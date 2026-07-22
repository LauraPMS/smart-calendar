use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
pub struct UpdateShiftRequest {
    pub request_id: i64,
    pub new_status: String,
}

pub async fn update_shift_status(
    State(pool): State<SqlitePool>,
    Json(payload): Json<UpdateShiftRequest>,
) -> impl IntoResponse {
    
    // 1. Mettre à jour la demande cliquée par le manager
    let res = sqlx::query!(
        "UPDATE shift_requests SET status = ? WHERE request_id = ?",
        payload.new_status,
        payload.request_id
    )
    .execute(&pool)
    .await;

    if res.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur BDD").into_response();
    }

    // 2. LA MAGIE : Si le manager valide cette demande, on refuse les autres pour le même bloc
    if payload.new_status == "Validée" {
        // On récupère les infos du bloc qu'on vient de valider
        let shift_info = sqlx::query!(
            "SELECT service_id, period_type, start_date FROM shift_requests WHERE request_id = ?",
            payload.request_id
        )
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);

        if let Some(info) = shift_info {
            // On passe en "Refusée" toutes les autres demandes du même service, même date, même type
            let _ = sqlx::query!(
                "UPDATE shift_requests SET status = 'Refusée' 
                 WHERE service_id = ? AND period_type = ? AND start_date = ? AND request_id != ?",
                info.service_id,
                info.period_type,
                info.start_date,
                payload.request_id
            )
            .execute(&pool)
            .await;
        }
    }

    (StatusCode::OK, "Statut mis à jour avec succès").into_response()
}