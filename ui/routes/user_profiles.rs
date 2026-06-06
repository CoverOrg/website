use crate::handlers::user_profiles::create_user_profile;
use axum::{Router, routing::post};

pub fn user_profile_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/users/profile", post(create_user_profile))
}
