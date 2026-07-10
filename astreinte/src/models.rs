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