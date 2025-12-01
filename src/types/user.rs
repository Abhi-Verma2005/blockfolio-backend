use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: Option<String>,
    #[sqlx(default)]
    pub created_at: DateTime<Utc>,
    #[sqlx(default)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct UserWallet {
    pub id: i32,
    pub user_id: i32,
    pub address: String,
    pub chain: String,
    pub label: Option<String>,
    pub is_primary: bool,
    #[sqlx(default)]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddWalletRequest {
    pub address: String,
    pub chain: String,
    pub label: Option<String>,
    pub is_primary: Option<bool>,
}

