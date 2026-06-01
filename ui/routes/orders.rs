use crate::handlers::orders::create_order;
use axum::{Router, routing::post};

pub fn order_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/orders", post(create_order))
}
