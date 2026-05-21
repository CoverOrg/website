use crate::models::verification::{VerificationRequest, VerificationResponse};
use axum::{Json, extract::State};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_verification(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<VerificationRequest>,
) -> Result<Json<VerificationResponse>, String> {
    let id = Uuid::now_v7();

    sqlx::query(
        "
        INSERT INTO verifications
        (
            id,
            user_id,
            image,
            id_card_front,
            id_card_back,
            is_verified
        )
        VALUES
        ($1, $2, $3, $4, $5, $6)
        ",
    )
    .bind(id)
    .bind(Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap())
    .bind(&payload.image)
    .bind(String::from("id_card_front_path.png"))
    .bind(String::from("id_card_back_path.png"))
    .bind(true)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = VerificationResponse {
        id,
        user_id: Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap(),
        image: payload.image,
        is_verified: true,
    };

    Ok(Json(response))
}
