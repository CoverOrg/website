use crate::models::scam_alerts::{ScamAlertsRequest, ScamAlertsResponse};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_scam_alert(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<ScamAlertsRequest>,
) -> Result<Json<ScamAlertsResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
        "
        INSERT INTO scam_alerts
        (
            id,
            title,
            city,
            platform,
            description,
            scammer_phone,
            amount_lost,
            victim_count,
            severity,
            is_published,
            published_at,
            created_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        ",
    )
    .bind(id)
    .bind(&payload.title)
    .bind(&payload.city)
    .bind(&payload.platform)
    .bind(&payload.description)
    .bind(&payload.scammer_phone)
    .bind(&payload.amount_lost)
    .bind(&payload.victim_count)
    .bind(&payload.severity)
    .bind(true)
    .bind(Some(now))
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = ScamAlertsResponse {
        id,
        title: payload.title,
        city: payload.city,
        platform: payload.platform,
        description: payload.description,
        scammer_phone: payload.scammer_phone,
        amount_lost: payload.amount_lost,
        victim_count: payload.victim_count,
        severity: payload.severity,
        is_published: true,
        published_at: Some(now),
        created_at: now,
    };

    Ok(Json(response))
}
