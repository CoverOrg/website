use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "notification_types", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum NotificationTypes {
    OrderPaid,
    OrderConfirmed,
    OrderShipped,
    OrderRejected,
    DeliveryConfirmed,
    PaymentReleased,
    DisputeOpened,
    DisputeResolved,
    KycApproved,
    KycRejected,
    ReferralApproved,
    ReferralEarned,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Notifications {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_id: Option<Uuid>,
    pub notification_type: NotificationTypes,
    pub message: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct NotificationsRequest {
    pub is_read: bool,
}

#[derive(Debug, Serialize)]
pub struct NotificationsResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_id: Option<Uuid>,
    pub notification_type: NotificationTypes,
    pub message: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NotificationItems {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_id: Option<Uuid>,
    pub notification_type: NotificationTypes,
    pub message: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ListNotificationsResponse {
    pub data: Vec<NotificationItems>,
    pub unread_count: i64,
}

impl From<Notifications> for NotificationsResponse {
    fn from(n: Notifications) -> Self {
        Self {
            id: n.id,
            user_id: n.user_id,
            order_id: n.order_id,
            notification_type: n.notification_type,
            message: n.message,
            is_read: n.is_read,
            created_at: n.created_at,
        }
    }
}
