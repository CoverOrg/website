use crate::handlers::referral_applications::create_referral_application;
use axum::{Router, routing::post};

pub fn referral_application_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/referral_applications", post(create_referral_application))
}
