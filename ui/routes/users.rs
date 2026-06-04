use crate::handlers::users::{create_user, send_otp};
use axum::{Router, routing::post};

pub fn router() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/auth/otp/send", post(send_otp))
}

pub fn user_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/signup", post(create_user))
}
