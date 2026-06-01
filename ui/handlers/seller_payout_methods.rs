use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::models::seller_payout_methods::{SellerPayoutMethodsRequest, SellerPayoutMethodsResponse};

pub async fn create_seller_payout_methods(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<SellerPayoutMethodsRequest>,
) -> Result<Json<SellerPayoutMethodsResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
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
        VALUES
        (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9, $10
        )
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
    .bind(now)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = SellerPayoutMethodsResponse {
        id,
        method_type: request.method_type,
        account_number: request.account_number,
        iban: request.iban,
        account_holder: request.account_holder,
        bank_name: request.bank_name,
        is_default: request.is_default,
        created_at: now,
        updated_at: now,
    };

    Ok(Json(response))
}
