use crate::models::order_timeline::{OrderTimelineRequest, OrderTimelineResponse};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_order_timeline(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<OrderTimelineRequest>,
) -> Result<Json<OrderTimelineResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
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
        VALUES
        (
            $1, $2, $3, $4,
            $5, $6, $7
        )
        ",
    )
    .bind(id)
    .bind(&request.order_id)
    .bind(&request.status)
    .bind(&request.note)
    .bind(&request.actor_id)
    .bind(&request.actor_hint)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = OrderTimelineResponse {
        id,
        order_id: request.order_id,
        status: request.status,
        note: request.note,
        actor_id: request.actor_id,
        actor_hint: request.actor_hint,
        created_at: now,
    };

    Ok(Json(response))
}
