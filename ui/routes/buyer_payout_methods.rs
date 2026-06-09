use crate::handlers::buyer_payout_methods::buyer_payout_methods;
use axum::{Router, routing::post};

pub fn buyer_payout_method_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/buyer_payout_methods", post(buyer_payout_methods))
}
