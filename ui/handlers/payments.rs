use crate::models::payments::{PaymentStatus, PaymentsRequest, PaymentsResponse};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_payment(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<PaymentsRequest>,
) -> Result<Json<PaymentsResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
        "
        INSERT INTO payments
        (
            id,
            deal_id,
            gateway,
            amount,
            fee,
            status,
            gateway_ref,
            screenshot_url,
            paid_at,
            released_at,
            created_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ",
    )
    .bind(id)
    .bind(&payload.deal_id)
    .bind(&payload.gateway)
    .bind(&payload.amount)
    .bind(0_i64)
    .bind(PaymentStatus::Pending)
    .bind(Some(String::from("gateway reference")))
    .bind(String::from("/path/image.png"))
    .bind(Some(now))
    .bind(Some(now))
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = PaymentsResponse {
        id,
        gateway: payload.gateway,
        amount: payload.amount,
        fee: 0,
        status: PaymentStatus::Pending,
        gateway_ref: Some(String::from("gateway reference")),
        screenshot_url: payload.screenshot_url,
        paid_at: Some(now),
        released_at: Some(now),
        created_at: now,
    };

    Ok(Json(response))
}
