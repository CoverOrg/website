use crate::models::otp_codes::{OtpCodesRequest, OtpCodesResponse};
use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_otp_code(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<OtpCodesRequest>,
) -> Result<Json<OtpCodesResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();
    let expires_at = now + Duration::minutes(5);
    let otp_code = "123456";

    sqlx::query(
        "
        INSERT INTO otp_codes
        (
            id,
            phone,
            code,
            purpose,
            expires_at,
            used_at,
            attempts,
            created_at
        )
        VALUES
        (
            $1, $2, $3, $4,
            $5, $6, $7, $8
        )
        ",
    )
    .bind(id)
    .bind(&request.phone)
    .bind(otp_code)
    .bind(&request.purpose)
    .bind(expires_at)
    .bind(None::<chrono::DateTime<Utc>>)
    .bind(0_i16)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = OtpCodesResponse {
        id,
        phone: request.phone,
        purpose: request.purpose,
        expires_at,
        attempts: 0,
        created_at: now,
    };

    Ok(Json(response))
}
