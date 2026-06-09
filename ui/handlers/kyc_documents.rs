use crate::models::{
    kyc_documents::{KycDocuments, KycDocumentsRequest, KycDocumentsResponse},
    types::KycStatus,
};
use axum::{Json, extract::State};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_kyc_documents(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<KycDocumentsRequest>,
) -> Result<Json<KycDocumentsResponse>, String> {
    let id = Uuid::now_v7();

    let kyc = sqlx::query_as::<_, KycDocuments>(
        "
        INSERT INTO kyc_documents
        (
            id,
            user_id,
            doc_type,
            file_url,
            status,
            reviewer_note,
            submitted_at,
            reviewed_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
        RETURNING *
        ",
    )
    .bind(id)
    .bind(Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap())
    .bind(&request.doc_type)
    .bind(&request.file_url)
    .bind(KycStatus::Approved)
    .bind(Some(String::from("reviewer note")))
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(KycDocumentsResponse::from(kyc)))
}
