use crate::models::notifications::{NotificationTypes, NotificationsRequest, NotificationsResponse};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_notification(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<NotificationsRequest>,
) -> Result<Json<NotificationsResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
        "
        INSERT INTO notifications
        (
            id,
            user_id,
            order_id,
            notification_type,
            message,
            is_read,
            created_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7)
        ",
    )
    .bind(id)
    .bind(Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap())
    .bind(None::<Uuid>)
    .bind(NotificationTypes::OrderPaid)
    .bind(String::from("message"))
    .bind(&request.is_read)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = NotificationsResponse {
        id,
        user_id: Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap(),
        order_id: None::<Uuid>,
        notification_type: NotificationTypes::OrderPaid,
        message: String::from("message"),
        is_read: request.is_read,
        created_at: now,
    };

    Ok(Json(response))
}
