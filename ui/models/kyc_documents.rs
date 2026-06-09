use crate::models::types::{DocType, KycStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

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

impl From<KycDocuments> for KycDocumentsResponse {
    fn from(k: KycDocuments) -> Self {
        Self {
            id: k.id,
            doc_type: k.doc_type,
            file_url: k.file_url,
            status: k.status,
            reviewer_note: k.reviewer_note,
            submitted_at: k.submitted_at,
            reviewed_at: k.reviewed_at,
        }
    }
}
