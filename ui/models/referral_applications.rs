use crate::models::types::{
    ApplicationStatus, BankNames, MethodTypes, ReachEstimated, ReferralMethods,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

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

impl From<ReferralApplications> for ReferralApplicationsResponse {
    fn from(r: ReferralApplications) -> Self {
        Self {
            id: r.id,
            user_id: r.user_id,
            full_name: r.full_name,
            whatsapp: r.whatsapp,
            referral_method: r.referral_method,
            estimated_reach: r.estimated_reach,
            payout_method: r.payout_method,
            payout_account: r.payout_account,
            iban: r.iban,
            payout_holder: r.payout_holder,
            bank_name: r.bank_name,
            referral_code: r.referral_code,
            status: r.status,
            rejection_reason: r.rejection_reason,
            reviewed_at: r.reviewed_at,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}
