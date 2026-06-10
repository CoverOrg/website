use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "order_timeline_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OrderTimelineStatus {
    Created,
    Paid,
    SellerConfirmed,
    Shipped,
    Delivered,
    Released,
    Disputed,
    Cancelled,
    Refunded,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "actor_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    Buyer,
    Seller,
    Admin,
    System,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrderTimeline {
    pub id: Uuid,
    pub order_id: Uuid,
    pub status: OrderTimelineStatus,
    pub note: String,
    pub actor_id: Uuid,
    pub actor_hint: ActorType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct OrderTimelineRequest {
    pub order_id: Uuid,
    pub status: OrderTimelineStatus,
    pub note: String,
    pub actor_id: Uuid,
    pub actor_hint: ActorType,
}

#[derive(Debug, Serialize)]
pub struct OrderTimelineResponse {
    pub id: Uuid,
    pub order_id: Uuid,
    pub status: OrderTimelineStatus,
    pub note: String,
    pub actor_id: Uuid,
    pub actor_hint: ActorType,
    pub created_at: DateTime<Utc>,
}

impl From<OrderTimeline> for OrderTimelineResponse {
    fn from(o: OrderTimeline) -> Self {
        Self {
            id: o.id,
            order_id: o.order_id,
            status: o.status,
            note: o.note,
            actor_id: o.actor_id,
            actor_hint: o.actor_hint,
            created_at: o.created_at,
        }
    }
}
