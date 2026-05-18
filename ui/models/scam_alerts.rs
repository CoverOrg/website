use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScamPlatform {
    Facebook,
    WhatsApp,
    Instagram,
    OLX,
    Daraz,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScamCity {
    Islamabad,
    Lahore,
    Karachi,
    Faisalabad,
    Quetta,
    Peshawar,
}

#[derive(Debug)]
pub struct ScamAlerts {
    pub id: Uuid,
    pub title: String,
    pub city: ScamCity,
    pub platform: ScamPlatform,
    pub description: String,
    pub scammer_phone: Option<String>,
    pub amount_lost: Option<u64>,
    pub victim_count: u32,
    pub severity: Severity,
    pub is_published: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ScamAlertsRequest {
    pub title: String,
    pub city: ScamCity,
    pub platform: ScamPlatform,
    pub description: String,
    pub scammer_phone: Option<String>,
    pub amount_lost: Option<u64>,
    pub victim_count: Option<u32>,
    pub severity: Severity,
}

#[derive(Debug, Serialize)]
pub struct ScamAlertsResponse {
    pub id: Uuid,
    pub title: String,
    pub city: ScamCity,
    pub platform: ScamPlatform,
    pub description: String,
    pub scammer_phone: Option<String>,
    pub amount_lost: Option<u64>,
    pub victim_count: u32,
    pub severity: Severity,
    pub is_published: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
