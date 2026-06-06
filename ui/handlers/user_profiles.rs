use crate::models::{
    types::UserKycStatus,
    user_profiles::{UserProfilesRequest, UserProfilesResponse},
};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_user_profile(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<UserProfilesRequest>,
) -> Result<Json<UserProfilesResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();
    let is_buyer = request.is_buyer.unwrap_or(true);
    let is_seller = request.is_seller.unwrap_or(false);

    sqlx::query(
        "
        INSERT INTO user_profiles
        (
            id,
            name,
            city,
            avatar_url,
            is_buyer,
            is_seller,
            seller_handle,
            id_card,
            kyc_status,
            created_at,
            updated_at
        )
        VALUES
        (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9, $10,
            $11
        )
        ",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.city)
    .bind(&request.avatar_url)
    .bind(is_buyer)
    .bind(is_seller)
    .bind(&request.seller_handle)
    .bind(&request.id_card)
    .bind(UserKycStatus::NotSubmitted)
    .bind(now)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = UserProfilesResponse {
        id,
        name: request.name,
        city: request.city,
        avatar_url: request.avatar_url,
        is_buyer,
        is_seller,
        kyc_status: UserKycStatus::NotSubmitted,
        id_card: request.id_card,
        seller_handle: request.seller_handle,
        created_at: now,
    };

    Ok(Json(response))
}
