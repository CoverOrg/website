use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "deal_types", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DealTypes {
    Delivery,
    Milestone,
    TimeWindow,
    Approval,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_methods", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethods {
    Easypaisa,
    Nayapay,
    Raast,
    Bank,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "deal_statuses", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DealStatuses {
    Draft,
    PendingPayment,
    PaymentReceived,
    SellerNotified,
    InTransit,
    Delivered,
    Confirmed,
    Disputed,
    Refunded,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "courier_services", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CourierServices {
    Tcs,
    Leopard,
}

#[derive(Debug)]
pub struct Deals {
    pub id: Uuid,
    pub ref_code: String,
    pub buyer_id: Uuid,
    pub seller_id: Option<Uuid>,
    pub seller_phone: String,
    pub item_name: String,
    pub item_pics: Vec<u8>,
    pub delivery_video: Vec<u8>,
    pub amount: u64,
    pub fee_amount: u64,
    pub fee_percent: u32,
    pub deal_type: DealTypes,
    pub payment_method: PaymentMethods,
    pub deal_status: DealStatuses,
    pub risk_score: u32,
    pub courier: Option<CourierServices>,
    pub tracking_number: Option<String>,
    pub tracking_verified: bool,
    pub expected_delivery: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub disputed_at: Option<DateTime<Utc>>,
    pub refunded_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct DealsRequest {
    pub buyer_id: Uuid,
    pub seller_id: Option<Uuid>,
    pub seller_phone: String,
    pub item_name: String,
    pub item_pics: Vec<u8>,
    pub delivery_video: Vec<u8>,
    pub amount: u64,
    pub deal_type: DealTypes,
    pub payment_method: PaymentMethods,
    pub courier: Option<CourierServices>,
    pub tracking_number: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DealsResponse {
    pub id: Uuid,
    pub ref_code: String,
    pub seller_phone: String,
    pub item_name: String,
    pub amount: u64,
    pub deal_type: DealTypes,
    pub payment_method: PaymentMethods,
    pub courier: Option<CourierServices>,
    pub tracking_number: Option<String>,

    pub status: DealStatuses,
    pub fee_amount: u64,
    pub fee_percent: u32,
    pub risk_score: u32,
    pub tracking_verified: bool,
    pub expected_delivery: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub disputed_at: Option<DateTime<Utc>>,
    pub refunded_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
}
