use crate::models::disputes::{DisputeStatus, Disputes, DisputesRequest, DisputesResponse};
use axum::{Json, extract::State};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_dispute(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<DisputesRequest>,
) -> Result<Json<DisputesResponse>, String> {
    let id = Uuid::now_v7();

    let disputes = sqlx::query_as::<_, Disputes>(
        "
        INSERT INTO disputes
        (
            id,
            order_id,
            raised_by,
            admin_id,
            reason,
            description,
            proof_urls,
            status,
            resolution_notes,
            created_at,
            updated_at,
            resolved_at,
        )
        VALUES
        (
            $1, $2,    $3,    $4,
            $5, $6,    $7,    $8,
            $9  NOW(), NOW(), NULL
        )
        RETURNING *
        ",
    )
    .bind(id)
    .bind(&request.order_id)
    .bind(&request.raised_by)
    .bind(Some(
        Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap(),
    ))
    .bind(&request.reason)
    .bind(&request.description)
    .bind(&request.proof_urls)
    .bind(DisputeStatus::Open)
    .bind(Some(String::from("resolution notes")))
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(DisputesResponse::from(disputes)))
}
