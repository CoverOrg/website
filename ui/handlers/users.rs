use crate::models::users::{RoleTypes, UsersRequest, UsersResponse};
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
            name,
            phone,
            city,
            role,
            trust_score,
            deal_count,
            dispute_count,
            created_at,
            last_active
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ",
    )
    .bind(id)
    .bind(&payload.name)
    .bind(&payload.phone)
    .bind(&payload.city)
    .bind(RoleTypes::Buyer)
    .bind(0_i32)
    .bind(0_i32)
    .bind(0_i32)
    .bind(now)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = UsersResponse {
        id,
        name: payload.name,
        city: payload.city,
        role: RoleTypes::Buyer,
        trust_score: 0,
        deal_count: 0,
        dispute_count: 0,
        created_at: now,
        last_active: Some(now),
    };

    Ok(Json(response))
}
