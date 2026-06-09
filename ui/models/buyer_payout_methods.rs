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
    pub account_number: Option<String>,
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
    pub account_number: Option<String>,
    pub iban: Option<String>,
    pub account_holder: String,
    pub bank_name: Option<BankNames>,
    pub is_default: bool,
}

#[derive(Debug, Serialize)]
pub struct BuyerPayoutMethodsResponse {
    pub id: Uuid,
    pub method_type: MethodTypes,
    pub account_number: Option<String>,
    pub iban: Option<String>,
    pub account_holder: String,
    pub bank_name: Option<BankNames>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<BuyerPayoutMethods> for BuyerPayoutMethodsResponse {
    fn from(b: BuyerPayoutMethods) -> Self {
        Self {
            id: b.id,
            method_type: b.method_type,
            account_number: b.account_number,
            iban: b.iban,
            account_holder: b.account_holder,
            bank_name: b.bank_name,
            is_default: b.is_default,
            created_at: b.created_at,
            updated_at: b.updated_at,
        }
    }
}
