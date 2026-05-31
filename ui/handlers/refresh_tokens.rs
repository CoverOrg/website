use crate::models::refresh_tokens::{RefreshTokensRequest, RefreshTokensResponse};
use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_refresh_token(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<RefreshTokensRequest>,
) -> Result<Json<RefreshTokensResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();
    let expires_at = now + Duration::days(30);
    let raw_token = Uuid::now_v7().to_string();
    let hash = Sha256::digest(raw_token.as_bytes());
    let token_hash = hex::encode(hash);

    sqlx::query(
        "
        INSERT INTO refresh_tokens
        (
            id,
            user_id,
            token_hash,
            device_hint,
            expires_at,
            revoked_at,
            created_at
        )
        VALUES
        (
            $1, $2, $3,
            $4, $5, $6, $7
        )
        ",
    )
    .bind(id)
    .bind(request.user_id)
    .bind(&token_hash)
    .bind(&request.device_hint)
    .bind(expires_at)
    .bind(None::<chrono::DateTime<Utc>>)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = RefreshTokensResponse {
        id,
        user_id: request.user_id,
        expires_at,
        created_at: now,
    };

    Ok(Json(response))
}
