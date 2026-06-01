use crate::handlers::seller_acceptances::create_seller_acceptances;
use axum::{Router, routing::post};

pub fn seller_acceptances_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/seller_acceptances", post(create_seller_acceptances))
}
