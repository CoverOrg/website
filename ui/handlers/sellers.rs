use crate::models::sellers::{SellersRequest, SellersResponse};
use axum::extract::{Json, State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_seller(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<SellersRequest>,
) -> Result<Json<SellersResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
        "
        INSERT INTO sellers
        (
            id,
            user_id,
            shop_name,
            banner_url,
            address,
            category,
            description,
            risk_score,
            risk_checked_at,
            is_featured,
            deal_count,
            became_seller
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        ",
    )
    .bind(id)
    .bind(&payload.user_id)
    .bind(&payload.shop_name)
    .bind(&payload.banner_url)
    .bind(&payload.address)
    .bind(&payload.category)
    .bind(&payload.description)
    .bind(0_i32)
    .bind(now)
    .bind(true)
    .bind(0_i32)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = SellersResponse {
        id,
        shop_name: payload.shop_name,
        banner_url: payload.banner_url,
        address: payload.address,
        category: payload.category,
        description: payload.description,
        risk_score: 0,
        is_featured: true,
        deal_count: 0,
        became_seller: now,
    };

    Ok(Json(response))
}
