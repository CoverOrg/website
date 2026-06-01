use crate::handlers::payment_proofs::create_payment_proofs;
use axum::{Router, routing::post};

pub fn payment_proof_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/payment_proofs", post(create_payment_proofs))
}
