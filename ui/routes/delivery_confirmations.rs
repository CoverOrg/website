use crate::handlers::delivery_confirmations::create_delivery_confirmation;
use axum::{Router, routing::post};

pub fn delivery_confirmation_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/delivery_confirmation", post(create_delivery_confirmation))
}
