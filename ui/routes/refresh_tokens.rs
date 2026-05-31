use crate::handlers::refresh_tokens::create_refresh_token;
use axum::{Router, routing::post};

pub fn refresh_token_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/refresh_tokens", post(create_refresh_token))
}
