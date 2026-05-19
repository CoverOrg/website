use crate::handlers::deals::create_deal;
use axum::{Router, routing::post};

pub fn deal_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/deal", post(create_deal))
}
