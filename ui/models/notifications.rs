use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    InApp,
    WhatsApp,
    SMS,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Types {
    DealUpdate,
    ScamAlert,
}

#[derive(Debug)]
pub struct Notifications {
    pub id: Uuid,
    pub user_id: Uuid,
    pub deal_id: Option<Uuid>,
    pub title: String,
    pub notification_type: Types,
    pub message: String,
    pub channel: Channel,
    pub is_read: bool,
    pub sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct NotificationsRequest {
    pub is_read: bool,
}

#[derive(Debug, Serialize)]
pub struct NotificationsResponse {
    pub id: Uuid,
    pub deal_id: Option<Uuid>,
    pub title: String,
    pub notification_type: Types,
    pub message: String,
    pub channel: Channel,
    pub is_read: bool,
    pub sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
