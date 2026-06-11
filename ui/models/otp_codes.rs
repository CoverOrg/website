use crate::models::{types::OtpPurpose, users::UsersResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OtpCodes {
    pub id: Uuid,
    pub phone: String,
    pub code: i32,
    pub purpose: OtpPurpose,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub attempts: i16,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct OtpCodesRequest {
    pub phone: String,
    pub purpose: OtpPurpose,
}

#[derive(Debug, Deserialize)]
pub struct VerifyOtpRequest {
    pub phone: String,
    pub code: i32,
    pub purpose: OtpPurpose,
}

#[derive(Debug, Serialize)]
pub struct OtpCodesResponse {
    pub code: i32,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct VerifyOtpResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub user: UsersResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

impl From<OtpCodes> for OtpCodesResponse {
    fn from(o: OtpCodes) -> Self {
        Self {
            code: o.code,
            expires_at: o.expires_at,
        }
    }
}
