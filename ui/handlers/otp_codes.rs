use crate::models::{
    otp_codes::{
        OtpCodes, OtpCodesRequest, OtpCodesResponse, SendOtpRequest, SendOtpResponse,
        VerifyOtpRequest, VerifyOtpResponse,
    },
    types::{OtpPurpose, UserKycStatus},
    users::UsersResponse,
};
use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

pub async fn send_otp(
    State(pool): State<PgPool>,
    Json(request): Json<SendOtpRequest>,
) -> Result<Json<SendOtpResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();
    let num: i32 = rand::random_range(100000..999999);
    sqlx::query(
        r#"
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
            $1, $2, $3, $4, $5, $6, $7, $8
        )
        "#,
    )
    .bind(id)
    .bind(&request.phone)
    .bind(num)
    .bind(&request.purpose)
    .bind(now + chrono::Duration::minutes(5))
    .bind::<Option<chrono::DateTime<Utc>>>(None)
    .bind(0_i16)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = SendOtpResponse {
        code: num,
        expires_at: now,
    };

    print!(
        "
Cover verification code: {}
Do not share this code with anyone.
This code will expire in 5 minutes.
        ",
        num
    );

    Ok(Json(response))
}

pub async fn verify_otp(
    State(pool): State<PgPool>,
    Json(request): Json<VerifyOtpRequest>,
) -> Result<Json<VerifyOtpResponse>, String> {
    let _otp: OtpCodes = sqlx::query_as::<_, OtpCodes>(
        "
            SELECT * FROM otp_codes WHERE
            phone = $1 AND
            code = $2 AND
            purpose = $3 AND
            expires_at > NOW() AND
            used_at IS NULL AND
            attempts < 5
            LIMIT 1
        ",
    )
    .bind(&request.phone)
    .bind(&request.code)
    .bind(request.purpose as OtpPurpose)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(VerifyOtpResponse {
        access_token: "generate_jwt_token".to_string(),
        expires_token: "generate_refresh_token".to_string(),
        code: 900,
        user: UsersResponse {
            id: Uuid::nil(), // empty uuid — all zeros
            phone: request.phone,
            name: None,
            city: None,
            avatar_url: None,
            is_buyer: true,
            is_seller: false,
            kyc_status: UserKycStatus::NotSubmitted,
            id_card: None,
            seller_handle: None,
            phone_verified: false,
            created_at: Utc::now(),
        },
    }))
}

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
