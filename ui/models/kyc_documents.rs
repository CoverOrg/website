use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "doc_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DocType {
    IDFront,
    IDBack,
    SelfieVideo,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "kyc_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum KycStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct KycDocuments {
    pub id: Uuid,
    pub user_id: Uuid,
    pub doc_type: DocType,
    pub file_url: String,
    pub status: KycStatus,
    pub reviewer_note: Option<String>,
    pub submitted_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct KycDocumentsRequest {
    pub user_id: Uuid,
    pub doc_type: DocType,
    pub file_url: String,
}

#[derive(Debug, Serialize)]
pub struct KycDocumentsResponse {
    pub id: Uuid,
    pub doc_type: DocType,
    pub file_url: String,
    pub status: KycStatus,
    pub reviewer_note: Option<String>,
    pub submitted_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
}
