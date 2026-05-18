use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct Verification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub image: Option<String>,
    pub id_card_front: Option<String>,
    pub id_card_back: Option<String>,
    pub is_verified: bool,
}

#[derive(Debug, Deserialize)]
pub struct VerificationRequest {
    pub image: Option<String>,
    pub id_card_front: Option<String>,
    pub id_card_back: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VerificationResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub image: Option<String>,
    pub is_verified: bool,
}
