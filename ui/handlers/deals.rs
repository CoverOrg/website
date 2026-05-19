use crate::models::deals::{DealStatuses, DealsRequest, DealsResponse};
use axum::extract::{Json, State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_deal(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<DealsRequest>,
) -> Result<Json<DealsResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
        "
        INSERT INTO deals
        (
            id,
            ref_code,
            buyer_id,
            seller_id,
            seller_phone,
            item_name,
            item_pics,
            delivery_video,
            amount,
            fee_amount,
            fee_percent,
            deal_type,
            payment_method,
            deal_status,
            risk_score,
            courier,
            tracking_number,
            tracking_verified,
            expected_delivery,
            notes,
            created_at,
            updated_at,
            confirmed_at,
            disputed_at,
            refunded_at,
            cancelled_at,
            delivered_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27)
        ",
    )
    .bind(id)
    .bind("RTEW-5423654235")
    .bind(&payload.buyer_id)
    .bind(&payload.seller_id)
    .bind(&payload.seller_phone)
    .bind(&payload.item_name)
    .bind(&payload.item_pics)
    .bind(&payload.delivery_video)
    .bind(payload.amount as i64)
    .bind(0_i64)
    .bind(0_i32)
    .bind(&payload.deal_type)
    .bind(&payload.payment_method)
    .bind(DealStatuses::Draft)
    .bind(0_i32)
    .bind(&payload.courier)
    .bind(&payload.tracking_number)
    .bind(true)
    .bind(Some(now))
    .bind(Some("notes"))
    .bind(now)
    .bind(now)
    .bind(Some(now))
    .bind(Some(now))
    .bind(Some(now))
    .bind(Some(now))
    .bind(Some(now))
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = DealsResponse {
        id,
        ref_code: String::from("RTEW-5423654235"),
        seller_phone: payload.seller_phone,
        item_name: payload.item_name,
        amount: payload.amount,
        deal_type: payload.deal_type,
        payment_method: payload.payment_method,
        courier: payload.courier,
        tracking_number: payload.tracking_number,
        status: DealStatuses::Draft,
        fee_amount: 0,
        fee_percent: 0,
        risk_score: 0,
        tracking_verified: true,
        expected_delivery: Some(now),
        notes: Some(String::from("notes")),
        created_at: now,
        confirmed_at: None,
        disputed_at: None,
        refunded_at: None,
        cancelled_at: None,
        delivered_at: None,
    };

    Ok(Json(response))
}
