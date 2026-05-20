use crate::models::disputes::{
    DisputeStatus, DisputesRequest, DisputesResponse, RoleTypess, Verdict,
};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_dispute(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<DisputesRequest>,
) -> Result<Json<DisputesResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
        "
        INSERT INTO disputes
        (
            id,
            deal_id,
            raised_by,
            admin_id,
            issue_type,
            description,
            buyer_evidence,
            seller_evidence,
            status,
            verdict,
            verdict_note,
            verdict_amount,
            created_at,
            resolved_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        ",
    )
    .bind(id)
    .bind(&payload.deal_id)
    .bind(RoleTypess::Buyer)
    .bind(None::<Uuid>)
    .bind(&payload.issue_type)
    .bind(&payload.description)
    .bind(&payload.buyer_evidence)
    .bind(&payload.seller_evidence)
    .bind(DisputeStatus::Open)
    .bind(Some(Verdict::BuyerWins))
    .bind(Some(String::from("verdict note")))
    .bind(Some(0_i64))
    .bind(now)
    .bind(Some(now))
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = DisputesResponse {
        id,
        raised_by: RoleTypess::Buyer,
        admin_id: None,
        issue_type: payload.issue_type,
        description: payload.description,
        buyer_evidence: payload.buyer_evidence,
        seller_evidence: payload.seller_evidence,
        status: DisputeStatus::Open,
        verdict: Some(Verdict::BuyerWins),
        verdict_note: Some(String::from("verdict note")),
        verdict_amount: Some(0),
        created_at: now,
        resolved_at: Some(now),
    };

    Ok(Json(response))
}
