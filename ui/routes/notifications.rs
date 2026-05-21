use crate::handlers::notifications::create_notification;
use axum::{Router, routing::post};

pub fn notification_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/notifications", post(create_notification))
}
