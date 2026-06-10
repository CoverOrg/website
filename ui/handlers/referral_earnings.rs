use crate::models::{
    referral_earnings::{ReferralEarnings, ReferralEarningsResponse},
    types::EarningStatus,
};
use axum::{Json, extract::State};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_referral_earning(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<ReferralEarningsResponse>, String> {
    let id = Uuid::now_v7();

    let referral = sqlx::query_as::<_, ReferralEarnings>(
        "
        INSERT INTO referral_earnings
        (
            id,
            referrer_id,
            order_id,
            cover_fee,
            earning_amount,
            status,
            paid_out_at,
            created_at,
        )
        VALUES($1, $2, $3, $4, $5, $6, NULL, NOW())
        RETURNING *
        ",
    )
    .bind(id)
    .bind(Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap())
    .bind(Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap())
    .bind(0_i64)
    .bind(0_i64)
    .bind(EarningStatus::Pending)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(ReferralEarningsResponse::from(referral)))
}
