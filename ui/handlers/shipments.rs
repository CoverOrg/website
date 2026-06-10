use crate::models::shipments::{Shipments, ShipmentsRequest, ShipmentsResponse};
use axum::{Json, extract::State};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_shipments(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<ShipmentsRequest>,
) -> Result<Json<ShipmentsResponse>, String> {
    let id = Uuid::now_v7();

    let shipment = sqlx::query_as::<_, Shipments>(
        "
        INSERT INTO shipments
        (
            id,
            order_id,
            seller_acceptance_id,
            tracking_id,
            courier,
            handover_video_url,
            payout_method,
            payout_account,
            payout_holder,
            bank_name,
            delivery_qr_token,
            shipped_at,
        )
        VALUES
        (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9, $10,
            $11, NOW()
        )
        RETURNING *
        ",
    )
    .bind(id)
    .bind(&request.order_id)
    .bind(&request.seller_acceptance_id)
    .bind(&request.tracking_id)
    .bind(&request.courier)
    .bind(&request.handover_video_url)
    .bind(&request.payout_method)
    .bind(&request.payout_account)
    .bind(&request.payout_holder)
    .bind(&request.bank_name)
    .bind(String::from("delivery qr token"))
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(ShipmentsResponse::from(shipment)))
}
