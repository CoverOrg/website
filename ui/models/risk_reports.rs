use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "risk_sources", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RiskSource {
    Facebook,
    Reddit,
    Twitter,
    LinkedIn,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "risk_severities", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RiskSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug)]
pub struct RiskReports {
    pub id: Uuid,
    pub seller_id: Option<Uuid>,
    pub phone: String,
    pub risk_source: RiskSource,
    pub description: String,
    pub risk_severity: RiskSeverity,
    pub evidence_url: String,
    pub verified: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub reported_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RiskReportsRequest {
    pub phone: String,
    pub risk_source: RiskSource,
    pub description: String,
    pub risk_severity: RiskSeverity,
    pub evidence_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RiskReportsResponse {
    pub id: Uuid,
    pub seller_id: Option<Uuid>,
    pub phone: String,
    pub risk_source: RiskSource,
    pub description: String,
    pub risk_severity: RiskSeverity,
    pub evidence_url: String,
    pub verified_at: Option<DateTime<Utc>>,
    pub reported_at: DateTime<Utc>,
}
