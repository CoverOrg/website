use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct OtpSessions {
    pub id: Uuid,
    pub phone: String,
    pub code: u32,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct OtpSessionsRequest {
    pub phone: String,
}

#[derive(Debug, Serialize)]
pub struct OtpSessionsResponse {
    pub id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub created_at: Option<DateTime<Utc>>,
}
