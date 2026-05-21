use crate::models::notifications::{Channels, NotificationsRequest, NotificationsResponse, Types};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_notification(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<NotificationsRequest>,
) -> Result<Json<NotificationsResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
        "
        INSERT INTO notifications
        (
            id,
            user_id,
            deal_id,
            title,
            notification_type,
            message,
            channel,
            is_read,
            sent_at,
            created_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ",
    )
    .bind(id)
    .bind(Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap())
    .bind(None::<Uuid>)
    .bind(String::from("title"))
    .bind(Types::DealUpdate)
    .bind(String::from("message"))
    .bind(Channels::InApp)
    .bind(&payload.is_read)
    .bind(Some(now))
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = NotificationsResponse {
        id,
        deal_id: None::<Uuid>,
        title: String::from("title"),
        notification_type: Types::DealUpdate,
        message: String::from("message"),
        channel: Channels::InApp,
        is_read: payload.is_read,
        sent_at: Some(now),
        created_at: now,
    };

    Ok(Json(response))
}
