use crate::models::seller_acceptances::{
    SellerAcceptances, SellerAcceptancesRequest, SellerAcceptancesResponse,
};
use axum::{Json, extract::State};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_seller_acceptances(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<SellerAcceptancesRequest>,
) -> Result<Json<SellerAcceptancesResponse>, String> {
    let id = Uuid::now_v7();

    let seller = sqlx::query_as::<_, SellerAcceptances>(
        "
        INSERT INTO seller_acceptances
        (
            id,
            user_id,
            order_id,
            seller_name,
            payout_method,
            payout_account,
            iban,
            payout_holder,
            bank_name,
            tracking_id,
            courier,
            decision,
            rejection_reason,
            decision_at,
        )
        VALUES
        (
            $1,  $2,  $3,  $4,
            $5,  $6,  $7,  $8,
            $9,  $10, $11, $12,
            $13, NOW()
        )
        RETURNING *
        ",
    )
    .bind(id)
    .bind(&request.user_id)
    .bind(&request.order_id)
    .bind(&request.seller_name)
    .bind(&request.payout_method)
    .bind(&request.payout_account)
    .bind(&request.iban)
    .bind(&request.payout_holder)
    .bind(&request.bank_name)
    .bind(&request.tracking_id)
    .bind(&request.courier)
    .bind(&request.decision)
    .bind(&request.rejection_reason)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(SellerAcceptancesResponse::from(seller)))
}
