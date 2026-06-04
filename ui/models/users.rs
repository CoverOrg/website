use crate::models::types::{OtpPurpose, UserCity, UserKycStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ------------------------------------
//  USER INFORMATION IN THE DATABASE
// ------------------------------------
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Users {
    pub id: Uuid,
    pub phone: String,
    pub name: Option<String>,
    pub city: Option<UserCity>,
    pub avatar_url: Option<String>,
    pub is_buyer: bool,
    pub is_seller: bool,
    pub kyc_status: UserKycStatus,
    pub seller_handle: Option<String>,
    pub id_card: Option<String>,
    pub phone_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ----------------------------------------
//  REQUESTS - User request to the server
// ----------------------------------------
#[derive(Debug, Deserialize)]
pub struct UsersRequest {
    pub phone: String,
    pub name: Option<String>,
    pub city: Option<UserCity>,
    pub avatar_url: Option<String>,
    pub is_buyer: Option<bool>,
    pub is_seller: Option<bool>,
    pub id_card: Option<String>,
    pub seller_handle: String,
}

#[derive(Debug, Deserialize)]
pub struct SendOtpRequest {
    pub phone: String,
    pub purpose: OtpPurpose,
}

#[derive(Debug, Deserialize)]
pub struct VerifyOtpRequest {
    pub phone: String,
    pub code: i32,
    pub purpose: OtpPurpose,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub city: Option<UserCity>,
}

#[derive(Debug, Deserialize)]
pub struct ChangePhoneRequest {
    pub new_phone: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmPhoneRequest {
    pub new_phone: String,
    pub code: String,
}

// ---------------------------------------------
//  REQUESTS - Server response to the frontend
// ---------------------------------------------
#[derive(Debug, Serialize)]
pub struct UsersResponse {
    pub id: Uuid,
    pub phone: String,
    pub name: Option<String>,
    pub city: Option<UserCity>,
    pub avatar_url: Option<String>,
    pub is_buyer: bool,
    pub is_seller: bool,
    pub kyc_status: UserKycStatus,
    pub id_card: Option<String>,
    pub seller_handle: Option<String>,
    pub phone_verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SendOtpResponse {
    pub code: i32,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct VerifyOtpResponse {
    pub access_token: String,
    pub expires_token: String,
    pub code: i32,
    pub user: UsersResponse,
}

#[derive(Debug, Serialize)]
pub struct UpdateProfileResponse {
    pub user: UsersResponse,
}

#[derive(Debug, Serialize)]
pub struct ChangePhoneResponse {
    pub code: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ConfirmPhoneResponse {
    pub user: UsersResponse,
}

impl From<Users> for UsersResponse {
    fn from(u: Users) -> Self {
        Self {
            id: u.id,
            phone: u.phone,
            name: u.name,
            city: u.city,
            avatar_url: u.avatar_url,
            is_buyer: u.is_buyer,
            is_seller: u.is_seller,
            kyc_status: u.kyc_status,
            seller_handle: u.seller_handle,
            id_card: u.id_card,
            phone_verified: u.phone_verified,
            created_at: u.created_at,
        }
    }
}
