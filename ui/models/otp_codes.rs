use crate::models::{types::OtpPurpose, users::UsersResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OtpCodes {
    pub id: Uuid,
    pub phone: String,
    pub code: String,
    pub purpose: OtpPurpose,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub attempts: u16,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct OtpCodesRequest {
    pub phone: String,
    pub purpose: OtpPurpose,
}

#[derive(Debug, Serialize)]
pub struct OtpCodesResponse {
    pub id: Uuid,
    pub phone: String,
    pub purpose: OtpPurpose,
    pub expires_at: DateTime<Utc>,
    pub attempts: u16,
    pub created_at: DateTime<Utc>,
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
