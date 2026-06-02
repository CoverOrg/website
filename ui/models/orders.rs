use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Created,
    Paid,
    SellerConfirmed,
    Shipped,
    Delivered,
    Released,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Orders {
    pub id: Uuid,
    pub order_number: String,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub status: OrderStatus,
    pub product_name: String,
    pub product_link: Option<String>,
    pub product_image_url: String,
    pub product_amount: i64,
    pub delivery_charges: i64,
    pub cover_fee: i64,
    pub total_amount: i64,
    pub seller_payout: i64,
    pub currency: String, // it can be converted to enum
    pub seller_name: String,
    pub seller_whatsapp: String,
    pub seller_handle: String,
    pub seller_accept_token: String,
    pub delivery_qr_token: String,
    pub last_seller_notified_at: DateTime<Utc>,
    pub delivery_address: String,
    pub referred_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub released_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct OrdersRequest {
    pub seller_id: Uuid,
    pub product_name: String,
    pub product_link: Option<String>,
    pub product_image_url: String,
    pub product_amount: i64,
    pub delivery_charges: i64,
    pub seller_name: String,
    pub seller_whatsapp: String,
    pub seller_handle: String,
    pub delivery_address: String,
}

#[derive(Debug, Serialize)]
pub struct OrdersResponse {
    pub id: Uuid,
    pub order_number: String,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub status: OrderStatus,
    pub product_name: String,
    pub product_link: Option<String>,
    pub product_image_url: String,
    pub product_amount: i64,
    pub delivery_charges: i64,
    pub cover_fee: i64,
    pub total_amount: i64,
    pub seller_payout: i64,
    pub currency: String,
    pub seller_name: String,
    pub seller_whatsapp: String,
    pub seller_handle: String,
    pub delivery_qr_token: String,
    pub last_seller_notified_at: DateTime<Utc>,
    pub delivery_address: String,
    pub referred_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub released_at: Option<DateTime<Utc>>,
}
