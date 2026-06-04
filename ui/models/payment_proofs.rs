use crate::models::types::MethodTypes;
use ::serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PaymentProofs {
    pub id: Uuid,
    pub order_id: Uuid,
    pub transaction_id: Uuid,
    pub method_type: MethodTypes,
    pub screenshot_url: String,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentProofsRequest {
    pub order_id: Uuid,
    pub transaction_id: Uuid,
    pub method_type: MethodTypes,
    pub screenshot_url: String,
}

#[derive(Debug, Serialize)]
pub struct PaymentProofsResponse {
    pub id: Uuid,
    pub order_id: Uuid,
    pub transaction_id: Uuid,
    pub method_type: MethodTypes,
    pub screenshot_url: String,
    pub submitted_at: DateTime<Utc>,
}
