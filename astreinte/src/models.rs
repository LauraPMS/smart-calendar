use serde::{Deserialize, Serialize};

// --- SERVICES ---
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Service {
    pub service_id: i64,
    pub name: String,
    pub tag: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateServicePayload {
    pub name: String,
    pub tag: String,
}

// --- UTILISATEURS ---
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub user_id: i64,
    pub service_id: Option<i64>,
    pub name: String,
    pub email: String,
    pub role: String,
    pub user_tag: String,
    pub must_change_password: bool,
}

#[derive(Deserialize, Debug)]
pub struct CreateUserPayload {
    pub name: String,
    pub email: String,
    pub role: String,
    pub service_id: Option<i64>,
    pub password_temp: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub user_id: i64,
    pub name: String,
    pub email: String,
    pub role: String,
    pub service_id: Option<i64>,
    pub user_tag: String,
    pub must_change_password: bool,
}

#[derive(Deserialize, Debug)]
pub struct ChangePasswordPayload {
    pub email: String,
    pub old_password: String,
    pub new_password: String,
}

// --- ASTREINTES (Par Blocs) ---
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ShiftRequest {
    pub request_id: i64,
    pub user_id: i64,
    pub service_id: i64,
    pub period_type: String, // 'Semaine' ou 'Weekend'
    pub start_date: String,  // YYYY-MM-DD
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateShiftRequestPayload {
    pub user_id: i64,
    pub service_id: i64,
    pub period_type: String,
    pub start_date: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateShiftStatusPayload {
    pub request_id: i64,
    pub new_status: String, // 'Validée' ou 'Refusée'
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ShiftResponse {
    pub request_id: i64,
    pub user_id: i64,
    pub service_id: i64,
    pub period_type: String,
    pub start_date: String,
    pub status: String,
    pub user_tag: String,
    pub user_name: String,
}