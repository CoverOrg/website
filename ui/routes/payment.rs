use crate::handlers::payments::create_payment;
use axum::{Router, routing::post};

pub fn payment_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/payment", post(create_payment))
}
