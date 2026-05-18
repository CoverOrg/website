use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn admin_pool() -> Pool<Postgres> {
    dotenvy::dotenv().expect("the .env file should be access");
    // let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let database_url =
        std::env::var("ADMIN_URL").expect("DATABASE_URL needs to be found in the .env file");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Database connection needs to be established");

    db_pool
}

pub async fn app_pool() -> Pool<Postgres> {
    dotenvy::dotenv().expect("the .env file should be access");
    // let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let database_url =
        std::env::var("APP_URL").expect("DATABASE_URL needs to be found in the .env file");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Database connection needs to be established");

    db_pool
}
