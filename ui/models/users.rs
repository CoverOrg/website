use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
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

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_kyc_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum UserKycStatus {
    None,
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Users {
    pub id: Uuid,
    pub phone: String,
    pub name: Option<String>,
    pub city: UserCity,
    pub avatar_url: Option<String>,
    pub is_buyer: bool,
    pub is_seller: bool,
    pub kyc_status: UserKycStatus,
    pub id_card: Option<String>,
    pub phone_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UsersRequest {
    pub phone: String,
    pub name: Option<String>,
    pub city: UserCity,
    pub avatar_url: Option<String>,
    pub is_buyer: Option<bool>,
    pub is_seller: Option<bool>,
    pub id_card: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UsersResponse {
    pub id: Uuid,
    pub phone: String,
    pub name: Option<String>,
    pub city: UserCity,
    pub avatar_url: Option<String>,
    pub is_buyer: bool,
    pub is_seller: bool,
    pub kyc_status: UserKycStatus,
    pub id_card: Option<String>,
    pub phone_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
