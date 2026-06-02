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
