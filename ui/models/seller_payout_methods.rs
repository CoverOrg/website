use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "method_types", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MethodTypes {
    Easypaisa,
    Jazzcash,
    Nayapay,
    Sadapay,
    Raast,
    Bank,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "bank_names", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BankNames {
    Hbl,
    Ubl,
    Mcb,
    AlliedBank,
    BankAlfalah,
    MeezanBank,
    AskariBank,
    BankAlHabib,
    FaysalBank,
    SoneriBank,
    JsBank,
    Silkbank,
    SummitBank,
    Bankislami,
    DubaiIslamicBank,
    StandardChartered,
    SambaBank,
    Nbp,
    HabibMetropolitan,
}

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
