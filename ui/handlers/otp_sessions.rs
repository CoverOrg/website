use crate::models::otp_sessions::{OtpSessionsRequest, OtpSessionsResponse};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_otp_session(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<OtpSessionsRequest>,
) -> Result<Json<OtpSessionsResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
        "
        INSERT INTO otp_sessions
        (
            id,
            phone,
            code,
            expires_at,
            used,
            created_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6)
        ",
    )
    .bind(id)
    .bind(&payload.phone)
    .bind(0_i32)
    .bind(now)
    .bind(true)
    .bind(Some(now))
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = OtpSessionsResponse {
        id,
        expires_at: now,
        used: true,
        created_at: Some(now),
    };

    Ok(Json(response))
}
