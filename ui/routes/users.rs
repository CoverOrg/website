use crate::handlers::users::create_user;
use axum::{Router, routing::post};

pub fn user_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/signup", post(create_user))
}
