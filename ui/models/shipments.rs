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
