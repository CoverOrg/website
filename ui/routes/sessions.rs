use crate::handlers::sessions::create_session;
use axum::{Router, routing::post};

pub fn session_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/session", post(create_session))
}
