use crate::models::types::OtpPurpose;
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
