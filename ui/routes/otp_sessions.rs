use crate::handlers::otp_sessions::create_otp_session;
use axum::{Router, routing::post};

pub fn otp_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/sessions", post(create_otp_session))
}
