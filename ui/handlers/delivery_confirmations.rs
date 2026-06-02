use crate::models::delivery_confirmations::{
    DeliveryConfirmationsRequest, DeliveryConfirmationsResponse,
};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_delivery_confirmation(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<DeliveryConfirmationsRequest>,
) -> Result<Json<DeliveryConfirmationsResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
        "
        INSERT INTO delivery_confirmations
        (
            id,
            order_id,
            video_url,
            notes,
            confirmed_at,
        )
        VALUES
        (
            $1, $2, $3,
            $4, $5
        )
        ",
    )
    .bind(id)
    .bind(&request.order_id)
    .bind(&request.video_url)
    .bind(&request.notes)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = DeliveryConfirmationsResponse {
        id,
        order_id: request.order_id,
        video_url: request.video_url,
        notes: request.notes,
        confirmed_at: now,
    };

    Ok(Json(response))
}
