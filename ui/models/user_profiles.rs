use crate::models::types::{UserCity, UserKycStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ------------------------------------
//  USER INFORMATION IN THE DATABASE
// ------------------------------------
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserProfiles {
    pub id: Uuid,
    pub name: String,
    pub city: UserCity,
    pub avatar_url: Option<String>,
    pub is_buyer: bool,
    pub is_seller: bool,
    pub seller_handle: String,
    pub id_card: Option<String>,
    pub kyc_status: UserKycStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ----------------------------------------
//  REQUESTS - User request to the server
// ----------------------------------------
#[derive(Debug, Deserialize)]
pub struct UserProfilesRequest {
    pub name: String,
    pub city: UserCity,
    pub avatar_url: Option<String>,
    pub is_buyer: Option<bool>,
    pub is_seller: Option<bool>,
    pub seller_handle: String,
    pub id_card: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: String,
    pub city: UserCity,
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
//  RESPONSES - Server response to the frontend
// ---------------------------------------------
#[derive(Debug, Serialize)]
pub struct UserProfilesResponse {
    pub id: Uuid,
    pub name: String,
    pub city: UserCity,
    pub avatar_url: Option<String>,
    pub is_buyer: bool,
    pub is_seller: bool,
    pub seller_handle: String,
    pub id_card: Option<String>,
    pub kyc_status: UserKycStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UpdateProfileResponse {
    pub user: UserProfilesResponse,
}

#[derive(Debug, Serialize)]
pub struct ChangePhoneResponse {
    pub code: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ConfirmPhoneResponse {
    pub user: UserProfilesResponse,
}

impl From<UserProfiles> for UserProfilesResponse {
    fn from(u: UserProfiles) -> Self {
        Self {
            id: u.id,
            name: u.name,
            city: u.city,
            avatar_url: u.avatar_url,
            is_buyer: u.is_buyer,
            is_seller: u.is_seller,
            seller_handle: u.seller_handle,
            id_card: u.id_card,
            kyc_status: u.kyc_status,
            created_at: u.created_at,
        }
    }
}
