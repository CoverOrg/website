use crate::handlers::otp_codes::create_otp_code;
use axum::{Router, routing::post};

pub fn otp_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/otp", post(create_otp_code))
}
