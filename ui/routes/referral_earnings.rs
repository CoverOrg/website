use crate::handlers::referral_earnings::create_referral_earning;
use axum::{Router, routing::post};

pub fn referral_earning_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/referral_earnings", post(create_referral_earning))
}
