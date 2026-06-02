use crate::handlers::order_timeline::create_order_timeline;
use axum::{Router, routing::post};

pub fn order_timeline_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/order_timeline", post(create_order_timeline))
}
