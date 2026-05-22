#[path = "../db/mod.rs"]
pub mod db;

#[path = "../handlers/mod.rs"]
pub mod handlers;

#[path = "../models/mod.rs"]
pub mod models;

#[path = "../routes/mod.rs"]
pub mod routes;

#[path = "../pages/mod.rs"]
pub mod pages;

use crate::{
    db::{bootstrap::run_grants, connection::load_pool},
    pages::home::home,
    routes::{
        deal_timeline::deal_timeline_routes, deals::deal_routes, disputes::dispute_routes,
        notifications::notification_routes, otp_sessions::otp_routes, payment::payment_routes,
        risk_reports::risk_report_routes, scam_alerts::scam_alert_routes, sellers::seller_routes,
        sessions::session_routes, users::user_routes, verification::verification_routes,
    },
};
use axum::{Router, routing::get};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let admin_pool = load_pool("ADMIN_URL").await;
    let app_pool = load_pool("APP_URL").await;

    sqlx::migrate!("../migrations")
        .run(&admin_pool)
        .await
        .expect("migrations failed");

    run_grants(&admin_pool).await;

    let app = Router::new()
        // Pages
        .route("/", get(home))
        // APIs
        .nest("/users", user_routes())
        .nest("/sellers", seller_routes())
        .nest("/deals", deal_routes())
        .nest("/deal_timeline", deal_timeline_routes())
        .nest("/payment", payment_routes())
        .nest("/disputes", dispute_routes())
        .nest("/report", risk_report_routes())
        .nest("/notifications", notification_routes())
        .nest("/scams", scam_alert_routes())
        .nest("/otp", otp_routes())
        .nest("/sessions", session_routes())
        .nest("/verification", verification_routes())
        .nest_service(
            "/static",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .with_state(app_pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on port: 3000");

    axum::serve(listener, app).await.unwrap();
}
