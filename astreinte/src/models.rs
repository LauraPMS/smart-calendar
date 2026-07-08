// Ce fichier servira a contenir la structure des données

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