use crate::handlers::risk_reports::create_risk_report;
use axum::{Router, routing::post};

pub fn risk_report_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/risk_reports", post(create_risk_report))
}
