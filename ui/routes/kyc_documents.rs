use crate::handlers::kyc_documents::create_kyc_documents;
use axum::{Router, routing::post};

pub fn kyc_documents_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    Router::new().route("/kyc", post(create_kyc_documents))
}
