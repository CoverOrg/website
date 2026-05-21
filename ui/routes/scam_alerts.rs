use crate::handlers::scam_alerts::create_scam_alert;
use axum::{Router, routing::post};

pub fn scam_alert_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/alerts", post(create_scam_alert))
}
