use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DealType {
    Delivery,
    Milestone,
    TimeWindow,
    Approval,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    Easypaisa,
    Nayapay,
    Raast,
    Bank,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DealStatus {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CourierService {
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
    pub deal_type: DealType,
    pub payment_method: PaymentMethod,
    pub deal_status: DealStatus,
    pub risk_score: u32,
    pub courier: Option<CourierService>,
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
    pub seller_phone: String,
    pub item_name: String,
    pub item_pics: Vec<u8>,
    pub delivery_video: Vec<u8>,
    pub amount: u64,
    pub deal_type: DealType,
    pub payment_method: PaymentMethod,
    pub courier: Option<CourierService>,
    pub tracking_number: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DealsResponse {
    pub id: Uuid,
    pub ref_code: String,
    pub seller_phone: String,
    pub item_name: String,
    pub amount: u64,
    pub deal_type: DealType,
    pub payment_method: PaymentMethod,
    pub courier: Option<CourierService>,
    pub tracking_number: Option<String>,

    pub status: DealStatus,
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
