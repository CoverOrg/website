use crate::handlers::verification::create_verification;
use axum::{Router, routing::post};

pub fn verification_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/verify", post(create_verification))
}
