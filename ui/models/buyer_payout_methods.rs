use crate::models::types::{BankNames, MethodTypes};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BuyerPayoutMethods {
    pub id: Uuid,
    pub user_id: Uuid,
    pub method_type: MethodTypes,
    pub account_number: Option<i32>,
    pub iban: Option<String>,
    pub account_holder: String,
    pub bank_name: Option<BankNames>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct BuyerPayoutMethodsRequest {
    pub user_id: Uuid,
    pub method_type: MethodTypes,
    pub account_number: Option<i32>,
    pub iban: Option<String>,
    pub account_holder: String,
    pub bank_name: Option<BankNames>,
    pub is_default: bool,
}

#[derive(Debug, Serialize)]
pub struct BuyerPayoutMethodsResponse {
    pub id: Uuid,
    pub method_type: MethodTypes,
    pub account_number: Option<i32>,
    pub iban: Option<String>,
    pub account_holder: String,
    pub bank_name: Option<BankNames>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
