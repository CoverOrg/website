use crate::models::delivery_confirmations::{
    DeliveryConfirmations, DeliveryConfirmationsRequest, DeliveryConfirmationsResponse,
};
use axum::{Json, extract::State};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_delivery_confirmation(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<DeliveryConfirmationsRequest>,
) -> Result<Json<DeliveryConfirmationsResponse>, String> {
    let id = Uuid::now_v7();

    let delivery = sqlx::query_as::<_, DeliveryConfirmations>(
        "
        INSERT INTO delivery_confirmations
        (
            id,
            order_id,
            video_url,
            notes,
            qr_scanned_at,
            confirmed_at,
        )
        VALUES ($1, $2, $3, $4, NOW(), NOW())
        RETURNING *
        ",
    )
    .bind(id)
    .bind(&request.order_id)
    .bind(&request.video_url)
    .bind(&request.notes)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(DeliveryConfirmationsResponse::from(delivery)))
}
