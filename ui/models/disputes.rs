use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "dispute_reason", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DisputeReason {
    ItemNotReceived,
    NotAsDescribed,
    Damaged,
    SellerNotResponding,
    SuspectedScam,
    Other,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "dispute_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DisputeStatus {
    Open,
    UnderReview,
    ResolvedRefund,
    ResolvedRelease,
    Closed,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Disputes {
    pub id: Uuid,
    pub order_id: Uuid,
    pub raised_by: Uuid,
    pub admin_id: Option<Uuid>,
    pub reason: DisputeReason,
    pub description: String,
    pub proof_urls: Vec<String>,
    pub status: DisputeStatus,
    pub resolution_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct DisputesRequest {
    pub order_id: Uuid,
    pub raised_by: Uuid,
    pub reason: DisputeReason,
    pub description: String,
    pub proof_urls: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DisputesResponse {
    pub id: Uuid,
    pub order_id: Uuid,
    pub raised_by: Uuid,
    pub admin_id: Option<Uuid>,
    pub reason: DisputeReason,
    pub description: String,
    pub proof_urls: Vec<String>,
    pub status: DisputeStatus,
    pub resolution_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

impl From<Disputes> for DisputesResponse {
    fn from(d: Disputes) -> Self {
        Self {
            id: d.id,
            order_id: d.order_id,
            raised_by: d.raised_by,
            admin_id: d.admin_id,
            reason: d.reason,
            description: d.description,
            proof_urls: d.proof_urls,
            status: d.status,
            resolution_notes: d.resolution_notes,
            created_at: d.created_at,
            updated_at: d.updated_at,
            resolved_at: d.resolved_at,
        }
    }
}
