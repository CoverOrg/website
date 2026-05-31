use crate::handlers::payout_methods::create_payout_methods;
use axum::{Router, routing::post};

pub fn payout_method_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/payout_methods", post(create_payout_methods))
}
