use crate::models::payment_proofs::{PaymentProofsRequest, PaymentProofsResponse};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_payment_proofs(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<PaymentProofsRequest>,
) -> Result<Json<PaymentProofsResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
        "
        INSERT INTO payment_proofs
        (
            id,
            order_id,
            method_type,
            transaction_id,
            screenshot_url,
            submitted_at
        )
        VALUES
        (
            $1, $2, $3,
            $4, $5, $6
        )
        ",
    )
    .bind(id)
    .bind(request.order_id)
    .bind(&request.method_type)
    .bind(Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap())
    .bind(&request.screenshot_url)
    .bind(now)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = PaymentProofsResponse {
        id,
        order_id: request.order_id,
        transaction_id: Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap(),
        method_type: request.method_type,
        screenshot_url: request.screenshot_url,
        submitted_at: now,
    };

    Ok(Json(response))
}
