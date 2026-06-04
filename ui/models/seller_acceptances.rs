use crate::models::types::{BankNames, CourierServices, PayoutMethods, SellerDecision};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SellerAcceptances {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_id: Uuid,
    pub seller_name: String,
    pub payout_method: PayoutMethods,
    pub payout_account: Option<String>,
    pub iban: Option<String>,
    pub payout_holder: String,
    pub bank_name: Option<BankNames>,
    pub tracking_id: String,
    pub courier: CourierServices,
    pub decision: SellerDecision,
    pub rejection_reason: Option<String>,
    pub decision_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SellerAcceptancesRequest {
    pub user_id: Uuid,
    pub order_id: Uuid,
    pub seller_name: String,
    pub payout_method: PayoutMethods,
    pub payout_account: Option<String>,
    pub iban: Option<String>,
    pub payout_holder: String,
    pub bank_name: Option<BankNames>,
    pub tracking_id: String,
    pub courier: CourierServices,
    pub decision: SellerDecision,
    pub rejection_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SellerAcceptancesResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_id: Uuid,
    pub seller_name: String,
    pub payout_method: PayoutMethods,
    pub payout_account: Option<String>,
    pub iban: Option<String>,
    pub payout_holder: String,
    pub bank_name: Option<BankNames>,
    pub tracking_id: String,
    pub courier: CourierServices,
    pub decision: SellerDecision,
    pub rejection_reason: Option<String>,
    pub decision_at: DateTime<Utc>,
}
