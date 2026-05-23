use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub id: Uuid,
    pub full_name: String,
    pub username: String,
    pub phone: String,
    pub email: String,
    pub city: UserCity,
    pub bio: String,
    pub avatar_url: Option<String>,
    pub trust_score: u32,
    pub kyc_level: u32,
    pub is_seller: bool,
    pub is_verified: bool,
    pub deals_completed: u32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UsersRequest {
    pub full_name: String,
    pub username: String,
    pub phone: String,
    pub email: String,
    pub city: UserCity,
    pub bio: String,
    pub avatar_url: Option<String>,
    pub is_seller: bool,
}

#[derive(Debug, Serialize)]
pub struct UsersResponse {
    pub id: Uuid,
    pub full_name: String,
    pub username: String,
    pub phone: String,
    pub email: String,
    pub city: UserCity,
    pub bio: String,
    pub avatar_url: Option<String>,
    pub trust_score: u32,
    pub kyc_level: u32,
    pub is_seller: bool,
    pub is_verified: bool,
    pub deals_completed: u32,
    pub created_at: Option<DateTime<Utc>>,
}
