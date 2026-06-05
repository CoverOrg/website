use crate::handlers::otp_codes::{create_otp_code, send_otp, verify_otp};
use axum::{Router, routing::post};

pub fn router() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new()
        .route("/auth/otp/send", post(send_otp))
        .route("/auth/otp/verify", post(verify_otp))
}

pub fn otp_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/otp", post(create_otp_code))
}
