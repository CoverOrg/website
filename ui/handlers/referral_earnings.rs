use crate::models::{referral_earnings::ReferralEarningsResponse, types::EarningStatus};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_referral_earning(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<ReferralEarningsResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
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
        VALUES
        (
            $1,  $2,  $3,  $4,
            $5,  $6,  $7,  $8
        )
        ",
    )
    .bind(id)
    .bind(Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap())
    .bind(Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap())
    .bind(0_i64)
    .bind(0_i64)
    .bind(EarningStatus::Pending)
    .bind(Some(now))
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = ReferralEarningsResponse {
        id,
        referrer_id: Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap(),
        order_id: Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap(),
        cover_fee: 0,
        earning_amount: 0,
        status: EarningStatus::Pending,
        paid_out_at: Some(now),
        created_at: now,
    };
    Ok(Json(response))
}
