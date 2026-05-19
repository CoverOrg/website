use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "deal_event_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DealEventType {
    DealCreated,
    PaymentReceived,
    PaymentFailed,
    SellerNotified,
    SellerAccepted,
    SellerRejected,
    Shipped,
    InTransit,
    OutForDelivery,
    Delivered,
    BuyerConfirmed,
    DisputeRaised,
    DisputeResolved,
    Refunded,
    DealCancelled,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "deal_actor", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DealActor {
    Buyer,
    Seller,
    Admin,
    System,
}

#[derive(Debug)]
pub struct DealTimeline {
    pub id: Uuid,
    pub deal_id: Uuid,
    pub event_type: DealEventType,
    pub description: Option<String>, // Description is required. Let's say "Seller uploaded tracking number" "Buyer raised dispute for damaged item"
    pub actor: DealActor,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct DealTimelineRequests {
    pub deal_id: Uuid,
    pub event_type: DealEventType,
    pub description: Option<String>,
    pub actor: DealActor,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct DealTimelineResponse {
    pub id: Uuid,
    pub event_type: DealEventType,
    pub description: Option<String>,
    pub actor: DealActor,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}
