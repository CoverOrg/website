use crate::handlers::deal_timeline::create_deal_timeline;
use axum::{Router, routing::post};

pub fn deal_timeline_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/deal_timeline", post(create_deal_timeline))
}
