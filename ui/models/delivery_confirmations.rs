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

impl From<DeliveryConfirmations> for DeliveryConfirmationsResponse {
    fn from(d: DeliveryConfirmations) -> Self {
        Self {
            id: d.id,
            order_id: d.order_id,
            video_url: d.video_url,
            notes: d.notes,
            qr_scanned_at: d.qr_scanned_at,
            confirmed_at: d.confirmed_at,
        }
    }
}
