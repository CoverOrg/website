use crate::models::deal_timeline::{DealTimelineRequests, DealTimelineResponse};
use axum::{Json, extract::State};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_deal_timeline(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<DealTimelineRequests>,
) -> Result<Json<DealTimelineResponse>, String> {
    let id = Uuid::now_v7();

    sqlx::query(
        "
        INSERT INTO deal_timeline
        (
            id,
            deal_id,
            event_type,
            description,
            actor,
            metadata,
            created_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7)
        ",
    )
    .bind(id)
    .bind(&payload.deal_id)
    .bind(&payload.event_type)
    .bind(&payload.description)
    .bind(&payload.actor)
    .bind(&payload.metadata)
    .bind(&payload.created_at)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = DealTimelineResponse {
        id,
        event_type: payload.event_type,
        description: payload.description,
        actor: payload.actor,
        metadata: payload.metadata,
        created_at: payload.created_at,
    };

    Ok(Json(response))
}
