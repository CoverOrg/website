use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "pay_out_methods", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PayoutMethods {
    Easypaisa,
    Jazzcash,
    Nayapay,
    Sadapay,
    Raast,
    Bank,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "courier_services", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CourierServices {
    TCS,
    Leopard,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "seller_decision", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SellerDecision {
    Accepted,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "bank_name", rename_all = "snake_case")]
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
