use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RefreshTokens {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub device_hint: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokensRequest {
    pub user_id: Uuid,
    pub device_hint: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokensResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<RefreshTokens> for RefreshTokensResponse {
    fn from(r: RefreshTokens) -> Self {
        Self {
            id: r.id,
            user_id: r.user_id,
            expires_at: r.expires_at,
            created_at: r.created_at,
        }
    }
}
