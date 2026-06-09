use crate::models::{
    types::UserKycStatus,
    user_profiles::{UserProfiles, UserProfilesRequest, UserProfilesResponse},
};
use axum::{Json, extract::State};
use sqlx::{Pool, Postgres};

pub async fn create_user_profile(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<UserProfilesRequest>,
) -> Result<Json<UserProfilesResponse>, String> {
    let is_buyer = request.is_buyer.unwrap_or(true);
    let is_seller = request.is_seller.unwrap_or(false);

    let profile = sqlx::query_as::<_, UserProfiles>(
        "
        INSERT INTO user_profiles
        (
            user_id,
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
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())
        RETURNING *
        ",
    )
    .bind(request.user_id)
    .bind(&request.name)
    .bind(&request.city)
    .bind(&request.avatar_url)
    .bind(is_buyer)
    .bind(is_seller)
    .bind(&request.seller_handle)
    .bind(&request.id_card)
    .bind(UserKycStatus::NotSubmitted)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(UserProfilesResponse::from(profile)))
}
