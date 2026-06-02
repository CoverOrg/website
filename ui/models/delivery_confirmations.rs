use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeliveryConfirmations {
    pub id: Uuid,
    pub order_id: Uuid,
    pub video_url: String,
    pub notes: String,
    pub qr_scanned_at: DateTime<Utc>,
    pub confirmed_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct DeliveryConfirmationsRequest {
    pub order_id: Uuid,
    pub video_url: String,
    pub notes: String,
}

#[derive(Debug, Serialize)]
pub struct DeliveryConfirmationsResponse {
    pub id: Uuid,
    pub order_id: Uuid,
    pub video_url: String,
    pub notes: String,
    pub qr_scanned_at: DateTime<Utc>,
    pub confirmed_at: DateTime<Utc>,
}
