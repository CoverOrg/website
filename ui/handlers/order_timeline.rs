use crate::models::order_timeline::{OrderTimeline, OrderTimelineRequest, OrderTimelineResponse};
use axum::{Json, extract::State};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_order_timeline(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<OrderTimelineRequest>,
) -> Result<Json<OrderTimelineResponse>, String> {
    let id = Uuid::now_v7();

    let order = sqlx::query_as::<_, OrderTimeline>(
        "
        INSERT INTO order_timeline
        (
            id,
            order_id,
            status,
            note,
            actor_id,
            actor_hint,
            created_at,
        )
        VALUES ($1, $2, $3, $4, $5, $6, NOW())
        RETURNING *
        ",
    )
    .bind(id)
    .bind(&request.order_id)
    .bind(&request.status)
    .bind(&request.note)
    .bind(&request.actor_id)
    .bind(&request.actor_hint)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(OrderTimelineResponse::from(order)))
}
