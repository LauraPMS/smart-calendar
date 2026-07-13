use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Admin,
    User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub role: Role,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateUserPayload {
    pub name: String,
    pub email: String,
    pub role: String,
    pub password_temp: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct ChangePasswordPayload {
    pub email: String,
    pub old_password: String,
    pub new_password: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateRequestPayload {
    pub user_email: String,
    pub date: String,
    pub shift_type: String,
    pub preference: i64, // <-- Modifié ici (i64 au lieu de i32)
}

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct ShiftRequest {
    pub request_id: i64, // <-- Modifié ici (i64 au lieu de i32)
    pub user_email: String,
    pub date: String,
    pub shift_type: String,
    pub preference: i64, // <-- Modifié ici (i64 au lieu de i32)
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateRequestPayload {
    pub request_id: i64,
    pub new_status: String,
}