use crate::handlers::shipments::create_shipments;
use axum::{Router, routing::post};

pub fn shipment_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/shipments", post(create_shipments))
}
