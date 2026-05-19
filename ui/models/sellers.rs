use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "select_category", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SelectCategory {
    Clothing,
    Jewellery,
    Shoes,
}

#[derive(Debug)]
pub struct Sellers {
    pub id: Uuid,
    pub user_id: Uuid,
    pub shop_name: String,
    pub banner_url: String,
    pub address: String,
    pub category: SelectCategory,
    pub description: String,
    pub risk_score: u32,
    pub risk_checked_at: Option<DateTime<Utc>>,
    pub is_featured: bool,
    pub deal_count: u64,
    pub became_seller: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct SellersRequest {
    pub user_id: Uuid,
    pub shop_name: String,
    pub banner_url: String,
    pub address: String,
    pub category: SelectCategory,
    pub description: String,
}

#[derive(Serialize)]
pub struct SellersResponse {
    pub id: Uuid,
    pub shop_name: String,
    pub banner_url: String,
    pub address: String,
    pub category: SelectCategory,
    pub description: String,
    pub risk_score: u32,
    pub is_featured: bool,
    pub deal_count: u64,
    pub became_seller: DateTime<Utc>,
}
