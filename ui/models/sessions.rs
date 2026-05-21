use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct Sessions {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SessionsRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct SessionsResponse {
    pub id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}
