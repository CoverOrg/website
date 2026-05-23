use crate::models::users::{UsersRequest, UsersResponse};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<UsersRequest>,
) -> Result<Json<UsersResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
        "
        INSERT INTO users
        (
            id,
            full_name,
            username,
            phone,
            email,
            city,
            bio,
            avatar_url,
            trust_score,
            kyc_level,
            is_seller,
            is_verified,
            deals_completed,
            created_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        ",
    )
    .bind(id)
    .bind(&payload.full_name)
    .bind(&payload.username)
    .bind(&payload.phone)
    .bind(&payload.email)
    .bind(&payload.city)
    .bind(&payload.bio)
    .bind(&payload.avatar_url)
    .bind(0_i32)
    .bind(0_i32)
    .bind(&payload.is_seller)
    .bind(false)
    .bind(0_i32)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = UsersResponse {
        id,
        full_name: payload.full_name,
        username: payload.username,
        phone: payload.phone,
        email: payload.email,
        city: payload.city,
        bio: payload.bio,
        avatar_url: payload.avatar_url,
        trust_score: 0,
        kyc_level: 0,
        is_seller: false,
        is_verified: false,
        deals_completed: 0,
        created_at: Some(now),
    };

    Ok(Json(response))
}
