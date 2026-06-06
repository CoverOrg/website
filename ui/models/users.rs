use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ------------------------------------
//  USER INFORMATION IN THE DATABASE
// ------------------------------------
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Users {
    pub id: Uuid,
    pub phone: String,
    pub phone_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ----------------------------------------
//  REQUESTS - User request to the server
// ----------------------------------------
#[derive(Debug, Deserialize)]
pub struct UsersRequest {
    pub phone: String,
}

// ---------------------------------------------
//  RESPONSES - Server response to the frontend
// ---------------------------------------------
#[derive(Debug, Serialize)]
pub struct UsersResponse {
    pub id: Uuid,
    pub phone: String,
    pub phone_verified: bool,
    pub created_at: DateTime<Utc>,
}

impl From<Users> for UsersResponse {
    fn from(u: Users) -> Self {
        Self {
            id: u.id,
            phone: u.phone,
            phone_verified: u.phone_verified,
            created_at: u.created_at,
        }
    }
}
