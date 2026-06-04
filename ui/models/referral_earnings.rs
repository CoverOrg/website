use crate::models::types::EarningStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ReferralEarnings {
    pub id: Uuid,
    pub referrer_id: Uuid,
    pub order_id: Uuid,
    pub cover_fee: i64,
    pub earning_amount: i64,
    pub status: EarningStatus,
    pub paid_out_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ReferralEarningsResponse {
    pub id: Uuid,
    pub referrer_id: Uuid,
    pub order_id: Uuid,
    pub cover_fee: i64,
    pub earning_amount: i64,
    pub status: EarningStatus,
    pub paid_out_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
