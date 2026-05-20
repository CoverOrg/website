use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "issue_types", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum IssueType {
    NotDelivered,
    WrongItem,
    Damaged,
    FakeCounterfeit,
    NotAsDescribed,
    Other,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "dispute_statuses", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DisputeStatus {
    Open,
    AwaitingSeller,
    UnderReview,
    Resolved,
    Appealed,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "verdicts", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    BuyerWins,
    SellerWins,
    Partial,
    ReturnRefund,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "role_typess", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RoleTypess {
    Buyer,
    Seller,
    Admin,
}

#[derive(Debug)]
pub struct Disputes {
    pub id: Uuid,
    pub deal_id: Uuid,
    pub raised_by: RoleTypess,
    pub admin_id: Option<Uuid>,
    pub issue_type: IssueType,
    pub description: String,
    pub buyer_evidence: Option<Value>,
    pub seller_evidence: Option<Value>,
    pub status: DisputeStatus,
    pub verdict: Option<Verdict>,
    pub verdict_note: Option<String>,
    pub verdict_amount: Option<u64>,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct DisputesRequest {
    pub deal_id: Uuid,
    pub issue_type: IssueType,
    pub description: String,
    pub buyer_evidence: Option<Value>,
    pub seller_evidence: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct DisputesResponse {
    pub id: Uuid,
    pub raised_by: RoleTypess,
    pub admin_id: Option<Uuid>,
    pub issue_type: IssueType,
    pub description: String,
    pub buyer_evidence: Option<Value>,
    pub seller_evidence: Option<Value>,
    pub status: DisputeStatus,
    pub verdict: Option<Verdict>,
    pub verdict_note: Option<String>,
    pub verdict_amount: Option<u64>,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}
