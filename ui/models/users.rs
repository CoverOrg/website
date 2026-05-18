use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "role_types", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RoleTypes {
    Buyer,
    Seller,
    Admin,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_city", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum UserCity {
    Islamabad,
    Lahore,
    Karachi,
    Faisalabad,
    Quetta,
    Peshawar,
}

#[derive(Debug)]
pub struct Users {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub city: UserCity,
    pub role: RoleTypes,
    pub trust_score: u32,
    pub deal_count: u64,
    pub dispute_count: u64,
    pub created_at: DateTime<Utc>,
    pub last_active: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct UsersRequest {
    pub name: String,
    pub phone: String,
    pub city: UserCity,
}

#[derive(Serialize)]
pub struct UsersResponse {
    pub id: Uuid,
    pub name: String,
    pub city: UserCity,
    pub role: RoleTypes,
    pub trust_score: u32,
    pub deal_count: u64,
    pub dispute_count: u64,
    pub created_at: DateTime<Utc>,
    pub last_active: Option<DateTime<Utc>>,
}
