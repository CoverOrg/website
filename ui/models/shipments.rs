use crate::models::types::{BankNames, CourierServices, PayoutMethods};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Shipments {
    pub id: Uuid,
    pub order_id: Uuid,
    pub seller_acceptance_id: Uuid,
    pub tracking_id: String,
    pub courier: CourierServices,
    pub handover_video_url: String,
    pub payout_method: PayoutMethods,
    pub payout_account: String,
    pub payout_holder: String,
    pub bank_name: Option<BankNames>,
    pub delivery_qr_token: String,
    pub shipped_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ShipmentsRequest {
    pub order_id: Uuid,
    pub seller_acceptance_id: Uuid,
    pub tracking_id: String,
    pub courier: CourierServices,
    pub handover_video_url: String,
    pub payout_method: PayoutMethods,
    pub payout_account: String,
    pub payout_holder: String,
    pub bank_name: Option<BankNames>,
}

#[derive(Debug, Serialize)]
pub struct ShipmentsResponse {
    pub id: Uuid,
    pub order_id: Uuid,
    pub seller_acceptance_id: Uuid,
    pub tracking_id: String,
    pub courier: CourierServices,
    pub handover_video_url: String,
    pub payout_method: PayoutMethods,
    pub payout_account: String,
    pub payout_holder: String,
    pub bank_name: Option<BankNames>,
    pub delivery_qr_token: String,
    pub shipped_at: DateTime<Utc>,
}

impl From<Shipments> for ShipmentsResponse {
    fn from(s: Shipments) -> Self {
        Self {
            id: s.id,
            order_id: s.order_id,
            seller_acceptance_id: s.seller_acceptance_id,
            tracking_id: s.tracking_id,
            courier: s.courier,
            handover_video_url: s.handover_video_url,
            payout_method: s.payout_method,
            payout_account: s.payout_account,
            payout_holder: s.payout_holder,
            bank_name: s.bank_name,
            delivery_qr_token: s.delivery_qr_token,
            shipped_at: s.shipped_at,
        }
    }
}
