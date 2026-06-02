use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "referral_methods", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ReferralMethods {
    SocialMedia,
    WhatsappGroups,
    YouTube,
    Blog,
    WordOfMouth,
    MarketplaceCommunities,
    Other,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "reach_estimated", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ReachEstimated {
    Under100,
    From100To500,
    From500To2000,
    From2000To10000,
    From10000AndPlus,
}

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

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "application_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ApplicationStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ReferralApplications {
    pub id: Uuid,
    pub user_id: Uuid,
    pub full_name: String,
    pub whatsapp: String,
    pub referral_method: ReferralMethods,
    pub estimated_reach: ReachEstimated,
    pub payout_method: MethodTypes,
    pub payout_account: String,
    pub iban: Option<String>,
    pub payout_holder: String,
    pub bank_name: Option<BankNames>,
    pub referral_code: Option<String>,
    pub status: ApplicationStatus,
    pub rejection_reason: Option<String>,
    pub reviewed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ReferralApplicationsRequest {
    pub user_id: Uuid,
    pub full_name: String,
    pub whatsapp: String,
    pub referral_method: ReferralMethods,
    pub estimated_reach: ReachEstimated,
    pub payout_method: MethodTypes,
    pub payout_account: String,
    pub iban: Option<String>,
    pub payout_holder: String,
    pub bank_name: Option<BankNames>,
}

#[derive(Debug, Serialize)]
pub struct ReferralApplicationsResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub full_name: String,
    pub whatsapp: String,
    pub referral_method: ReferralMethods,
    pub estimated_reach: ReachEstimated,
    pub payout_method: MethodTypes,
    pub payout_account: String,
    pub iban: Option<String>,
    pub payout_holder: String,
    pub bank_name: Option<BankNames>,
    pub referral_code: Option<String>,
    pub status: ApplicationStatus,
    pub rejection_reason: Option<String>,
    pub reviewed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
