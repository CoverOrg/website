#[path = "../db/mod.rs"]
pub mod db;

#[path = "../handlers/mod.rs"]
pub mod handlers;

#[path = "../models/mod.rs"]
pub mod models;

#[path = "../routes/mod.rs"]
pub mod routes;

use crate::routes::{
    deal_timeline::deal_timeline_routes, deals::deal_routes, disputes::dispute_routes,
    payment::payment_routes, risk_reports::risk_report_routes, sellers::seller_routes,
    users::user_routes,
};
use axum::Router;
use db::{
    bootstrap::run_grants,
    connection::{admin_pool, app_pool},
};

#[tokio::main]
async fn main() {
    let admin_pool = admin_pool().await;
    let app_pool = app_pool().await;

    sqlx::migrate!("../migrations")
        .run(&admin_pool)
        .await
        .expect("migrations failed");

    run_grants(&admin_pool).await;

    let app = Router::new()
        .nest("/users", user_routes())
        .nest("/sellers", seller_routes())
        .nest("/deals", deal_routes())
        .nest("/deal_timeline", deal_timeline_routes())
        .nest("/payment", payment_routes())
        .nest("/disputes", dispute_routes())
        .nest("/report", risk_report_routes())
        .with_state(app_pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on port: 3000");

    axum::serve(listener, app).await.unwrap();
}
