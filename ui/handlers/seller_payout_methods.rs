use crate::models::seller_payout_methods::{
    SellerPayoutMethods, SellerPayoutMethodsRequest, SellerPayoutMethodsResponse,
};
use axum::{Json, extract::State};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_seller_payout_methods(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<SellerPayoutMethodsRequest>,
) -> Result<Json<SellerPayoutMethodsResponse>, String> {
    let id = Uuid::now_v7();

    let seller_payout = sqlx::query_as::<_, SellerPayoutMethods>(
        "
        INSERT INTO seller_payout_methods
        (
            id,
            user_id,
            method_type,
            account_number,
            iban,
            account_holder,
            bank_name,
            is_default,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())
        RETURNING *
        ",
    )
    .bind(id)
    .bind(request.user_id)
    .bind(&request.method_type)
    .bind(request.account_number)
    .bind(&request.iban)
    .bind(&request.account_holder)
    .bind(&request.bank_name)
    .bind(request.is_default)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(SellerPayoutMethodsResponse::from(seller_payout)))
}
