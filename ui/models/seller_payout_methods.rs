use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::types::{BankNames, MethodTypes};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SellerPayoutMethods {
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
pub struct SellerPayoutMethodsRequest {
    pub user_id: Uuid,
    pub method_type: MethodTypes,
    pub account_number: Option<i32>,
    pub iban: Option<String>,
    pub account_holder: String,
    pub bank_name: Option<BankNames>,
    pub is_default: bool,
}

#[derive(Debug, Serialize)]
pub struct SellerPayoutMethodsResponse {
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

impl From<SellerPayoutMethods> for SellerPayoutMethodsResponse {
    fn from(s: SellerPayoutMethods) -> Self {
        Self {
            id: s.id,
            method_type: s.method_type,
            account_number: s.account_number,
            iban: s.iban,
            account_holder: s.account_holder,
            bank_name: s.bank_name,
            is_default: s.is_default,
            created_at: s.created_at,
            updated_at: s.updated_at,
        }
    }
}
