use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "gateways", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Gateways {
    Easypaisa,
    Nayapay,
    Raast,
    Bank,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Received,
    Held,
    Released,
    Refunded,
}

#[derive(Debug)]
pub struct Payments {
    pub id: Uuid,
    pub deal_id: Uuid,
    pub gateway: Gateways,
    pub amount: i64,
    pub fee: u64,
    pub status: PaymentStatus,
    pub gateway_ref: Option<String>,
    pub screenshot_url: String,
    pub paid_at: Option<DateTime<Utc>>,
    pub released_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentsRequest {
    pub deal_id: Uuid,
    pub gateway: Gateways,
    pub amount: i64,
    pub screenshot_url: String,
}

#[derive(Debug, Serialize)]
pub struct PaymentsResponse {
    pub id: Uuid,
    pub gateway: Gateways,
    pub amount: i64,
    pub fee: u64,
    pub status: PaymentStatus,
    pub gateway_ref: Option<String>,
    pub screenshot_url: String,
    pub paid_at: Option<DateTime<Utc>>,
    pub released_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
