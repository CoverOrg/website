use crate::handlers::seller_payout_methods::create_seller_payout_methods;
use axum::{Router, routing::post};

pub fn seller_payout_method_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/seller_payout_methods", post(create_seller_payout_methods))
}
