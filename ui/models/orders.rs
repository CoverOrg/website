use crate::models::types::OrderStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

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

impl From<Orders> for OrdersResponse {
    fn from(o: Orders) -> Self {
        Self {
            id: o.id,
            order_number: o.order_number,
            buyer_id: o.buyer_id,
            seller_id: o.seller_id,
            status: o.status,
            product_name: o.product_name,
            product_link: o.product_link,
            product_image_url: o.product_image_url,
            product_amount: o.product_amount,
            delivery_charges: o.delivery_charges,
            cover_fee: o.cover_fee,
            total_amount: o.total_amount,
            seller_payout: o.seller_payout,
            currency: o.currency,
            seller_name: o.seller_name,
            seller_whatsapp: o.seller_whatsapp,
            seller_handle: o.seller_handle,
            delivery_qr_token: o.delivery_qr_token,
            last_seller_notified_at: o.last_seller_notified_at,
            delivery_address: o.delivery_address,
            referred_by: o.referred_by,
            created_at: o.created_at,
            updated_at: o.updated_at,
            paid_at: o.paid_at,
            confirmed_at: o.confirmed_at,
            shipped_at: o.shipped_at,
            delivered_at: o.delivered_at,
            released_at: o.released_at,
        }
    }
}
