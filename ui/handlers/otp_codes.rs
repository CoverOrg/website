use crate::models::{
    otp_codes::{
        Claims, OtpCodes, OtpCodesRequest, OtpCodesResponse, VerifyOtpRequest, VerifyOtpResponse,
    },
    users::{Users, UsersResponse},
};
use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn send_otp(
    State(pool): State<PgPool>,
    Json(request): Json<OtpCodesRequest>,
) -> Result<Json<OtpCodesResponse>, String> {
    let id = Uuid::now_v7();
    let num: i32 = rand::random_range(100000..999999);

    let otp = sqlx::query_as::<_, OtpCodes>(
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
        VALUES ($1, $2, $3, $4, NOW() + INTERVAL '5 minutes', $5, $6, NOW())
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(&request.phone)
    .bind(num)
    .bind(&request.purpose)
    .bind::<Option<chrono::DateTime<Utc>>>(None)
    .bind(0_i16)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    print!(
        "
    Cover verification code: {}
    Do not share this code with anyone.
    This code will expire in 5 minutes.
        ",
        num
    );

    Ok(Json(OtpCodesResponse::from(otp)))
}

pub fn generate_jwt_token(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let key = EncodingKey::from_secret(secret.as_bytes());

    let claims = Claims {
        sub: user_id.to_string(),
        exp: (now + Duration::minutes(15)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    let token = encode(&Header::default(), &claims, &key)?;

    Ok(token)
}

pub async fn verify_otp(
    State(pool): State<PgPool>,
    Json(request): Json<VerifyOtpRequest>,
) -> Result<Json<VerifyOtpResponse>, String> {
    let now = Utc::now();
    let otp = sqlx::query_as::<_, OtpCodes>(
        "
            SELECT * FROM otp_codes
            WHERE phone = $1
            AND code = $2
            AND purpose = $3
            AND expires_at > NOW()
            AND used_at IS NULL
            AND attempts < 5
            LIMIT 1
        ",
    )
    .bind(&request.phone)
    .bind(request.code)
    .bind(request.purpose)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    // mark otp as used
    sqlx::query("UPDATE otp_codes SET used_at = NOW() WHERE id = $1")
        .bind(otp.id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    // fetch or create the user
    let user = sqlx::query_as::<_, Users>(
        "
            INSERT INTO users (id, phone, phone_verified, created_at, updated_at)
            VALUES ($1, $2, true, NOW(), NOW())
            ON CONFLICT (phone) DO UPDATE SET phone_verified = true, updated_at = NOW()
            RETURNING *
        ",
    )
    .bind(Uuid::now_v7())
    .bind(request.phone)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let access_token = generate_jwt_token(user.id).map_err(|e| e.to_string())?;

    println!("access token: {}", access_token);

    Ok(Json(VerifyOtpResponse {
        access_token,
        refresh_token: "generate_refresh_here".to_string(),
        expires_at: now,
        user: UsersResponse::from(user),
    }))
}
