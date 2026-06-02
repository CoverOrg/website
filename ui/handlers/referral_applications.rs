use crate::models::referral_applications::{
    ApplicationStatus, ReferralApplicationsRequest, ReferralApplicationsResponse,
};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_referral_application(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<ReferralApplicationsRequest>,
) -> Result<Json<ReferralApplicationsResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
        "
        INSERT INTO referral_applications
        (
            id,
            user_id,
            full_name,
            whatsapp,
            referral_method,
            estimated_reach,
            payout_method,
            payout_account,
            iban,
            payout_holder,
            bank_name,
            referral_code,
            status,
            rejection_reason,
            reviewed_at,
            created_at,
            updated_at,
        )
        VALUES
        (
            $1,  $2,  $3,  $4,  $5,
            $6,  $7,  $8,  $9,  $10,
            $11, $12, $13, $14, $15,
            $16, $17
        )
        ",
    )
    .bind(id)
    .bind(&request.user_id)
    .bind(&request.full_name)
    .bind(&request.whatsapp)
    .bind(&request.referral_method)
    .bind(&request.estimated_reach)
    .bind(&request.payout_method)
    .bind(&request.payout_account)
    .bind(&request.iban)
    .bind(&request.payout_holder)
    .bind(&request.bank_name)
    .bind(Some(String::from("referral code")))
    .bind(ApplicationStatus::Approved)
    .bind(Some(String::from("rejection reason")))
    .bind(now)
    .bind(now)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = ReferralApplicationsResponse {
        id,
        user_id: request.user_id,
        full_name: request.full_name,
        whatsapp: request.whatsapp,
        referral_method: request.referral_method,
        estimated_reach: request.estimated_reach,
        payout_method: request.payout_method,
        payout_account: request.payout_account,
        iban: request.iban,
        payout_holder: request.payout_holder,
        bank_name: request.bank_name,
        referral_code: Some(String::from("referral code")),
        status: ApplicationStatus::Approved,
        rejection_reason: Some(String::from("rejection reason")),
        reviewed_at: now,
        created_at: now,
        updated_at: now,
    };
    Ok(Json(response))
}
