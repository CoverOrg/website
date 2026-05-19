use crate::handlers::sellers::create_seller;
use axum::{Router, routing::post};

pub fn seller_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/seller", post(create_seller))
}
