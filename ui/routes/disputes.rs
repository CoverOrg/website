use crate::handlers::disputes::create_dispute;
use axum::{Router, routing::post};

pub fn dispute_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/disputes", post(create_dispute))
}
