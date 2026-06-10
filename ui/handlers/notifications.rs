use crate::models::notifications::{
    NotificationTypes, Notifications, NotificationsRequest, NotificationsResponse,
};
use axum::{Json, extract::State};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_notification(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<NotificationsRequest>,
) -> Result<Json<NotificationsResponse>, String> {
    let id = Uuid::now_v7();

    let notification = sqlx::query_as::<_, Notifications>(
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
        ($1, $2, $3, $4, $5, $6, NOW())
        RETURNING *
        ",
    )
    .bind(id)
    .bind(Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap())
    .bind(None::<Uuid>)
    .bind(NotificationTypes::OrderPaid)
    .bind(String::from("message"))
    .bind(&request.is_read)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(NotificationsResponse::from(notification)))
}
