use crate::models::sessions::{SessionsRequest, SessionsResponse};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_session(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<SessionsRequest>,
) -> Result<Json<SessionsResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
        "
        INSERT INTO sessions
        (
            id,
            user_id,
            token,
            created_at,
            expires_at
        )
        VALUES
        ($1, $2, $3, $4, $5)
        ",
    )
    .bind(id)
    .bind(&payload.user_id)
    .bind(String::from("token"))
    .bind(now)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = SessionsResponse {
        id,
        token: String::from("token"),
        expires_at: now,
    };

    Ok(Json(response))
}
