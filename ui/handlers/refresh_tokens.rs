use crate::models::refresh_tokens::{RefreshTokens, RefreshTokensRequest, RefreshTokensResponse};
use axum::{Json, extract::State};
use chrono::Utc;
use sha2::{Digest, Sha256};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_refresh_token(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<RefreshTokensRequest>,
) -> Result<Json<RefreshTokensResponse>, String> {
    let id = Uuid::now_v7();
    let raw_token = Uuid::now_v7().to_string();
    let hash = Sha256::digest(raw_token.as_bytes());
    let token_hash = hex::encode(hash);

    let refresh = sqlx::query_as::<_, RefreshTokens>(
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
        VALUES ($1, $2, $3, $4, NOW() + INTERVAL '30 days', $5, NOW())
        RETURNING *
        ",
    )
    .bind(id)
    .bind(request.user_id)
    .bind(&token_hash)
    .bind(&request.device_hint)
    .bind(None::<chrono::DateTime<Utc>>)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(RefreshTokensResponse::from(refresh)))
}
