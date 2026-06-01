use ::serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "method_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MethodType {
    Easypaisa,
    Jazzcash,
    Nayapay,
    Sadapay,
    Raast,
    Bank,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PaymentProofs {
    pub id: Uuid,
    pub order_id: Uuid,
    pub method_type: MethodType,
    pub transaction_id: String,
    pub screenshot_url: String,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentProofsRequest {
    pub order_id: Uuid,
    pub method_type: MethodType,
    pub transaction_id: String,
    pub screenshot_url: String,
}

#[derive(Debug, Serialize)]
pub struct PaymentProofsResponse {
    pub id: Uuid,
    pub order_id: Uuid,
    pub method_type: MethodType,
    pub transaction_id: String,
    pub screenshot_url: String,
    pub submitted_at: DateTime<Utc>,
}
