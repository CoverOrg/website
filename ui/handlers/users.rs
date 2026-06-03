use crate::models::users::{UserKycStatus, UsersRequest, UsersResponse};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<UsersRequest>,
) -> Result<Json<UsersResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();
    let is_buyer = request.is_buyer.unwrap_or(true);
    let is_seller = request.is_seller.unwrap_or(false);

    sqlx::query(
        "
        INSERT INTO users
        (
            id,
            phone,
            name,
            city,
            avatar_url,
            is_buyer,
            is_seller,
            kyc_status,
            id_card,
            seller_handle,
            phone_verified,
            created_at,
            updated_at
        )
        VALUES
        (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9, $10,
            $11, $12, $13
        )
        ",
    )
    .bind(id)
    .bind(&request.phone)
    .bind(&request.name)
    .bind(&request.city)
    .bind(&request.avatar_url)
    .bind(is_buyer)
    .bind(is_seller)
    .bind(UserKycStatus::None)
    .bind(&request.id_card)
    .bind(&request.seller_handle)
    .bind(false)
    .bind(now)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = UsersResponse {
        id,

        phone: request.phone,
        name: request.name,
        city: request.city,
        avatar_url: request.avatar_url,
        is_buyer,
        is_seller,
        kyc_status: UserKycStatus::None,
        id_card: request.id_card,
        seller_handle: Some(request.seller_handle),
        phone_verified: false,
        created_at: now,
        updated_at: now,
    };

    Ok(Json(response))
}
