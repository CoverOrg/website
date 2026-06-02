use crate::models::orders::{OrderStatus, OrdersRequest, OrdersResponse};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_order(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<OrdersRequest>,
) -> Result<Json<OrdersResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
        "
        INSERT INTO orders
        (
            id,
            order_number,
            buyer_id,
            seller_id,
            status,
            product_name,
            product_link,
            product_image_url,
            product_amount,
            delivery_charges,
            cover_fee,
            total_amount,
            seller_payout,
            currency,
            seller_name,
            seller_whatsapp,
            seller_handle,
            seller_accept_token,
            delivery_qr_token,
            last_seller_notified_at,
            delivery_address,
            referred_by,
            created_at,
            updated_at,
            paid_at,
            confirmed_at,
            shipped_at,
            delivered_at,
            released_at,
        )
        VALUES
        (
            $1,  $2,  $3,  $4,  $5,
            $6,  $7,  $8,  $9,  $10,
            $11, $12, $13, $14, $15,
            $16, $17, $18, $19, $20,
            $21, $22, $23, $24, $25,
            $26, $27, $28, $29,
        )
        ",
    )
    .bind(id)
    .bind(String::from("CVR-1234-5678"))
    .bind(Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap())
    .bind(&request.seller_id)
    .bind(OrderStatus::Paid)
    .bind(&request.product_name)
    .bind(&request.product_link)
    .bind(&request.product_image_url)
    .bind(&request.product_amount)
    .bind(&request.delivery_charges)
    .bind(0_i64)
    .bind(0_i64)
    .bind(0_i64)
    .bind(String::from("PKR"))
    .bind(&request.seller_name)
    .bind(&request.seller_whatsapp)
    .bind(&request.seller_handle)
    .bind(String::from("accept token"))
    .bind(String::from("delivery_qr_token"))
    .bind(now)
    .bind(&request.delivery_address)
    .bind(Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap())
    .bind(now)
    .bind(now)
    .bind(Some(now))
    .bind(Some(now))
    .bind(Some(now))
    .bind(Some(now))
    .bind(Some(now))
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = OrdersResponse {
        id,
        order_number: String::from("CVR-1234-5678"),
        buyer_id: Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap(),
        seller_id: request.seller_id,
        status: OrderStatus::Paid,
        product_name: request.product_name,
        product_link: request.product_link,
        product_image_url: request.product_image_url,
        product_amount: request.product_amount,
        delivery_charges: request.delivery_charges,
        cover_fee: 0,
        total_amount: 0,
        seller_payout: 0,
        currency: String::from("PKR"),
        seller_name: request.seller_name,
        seller_whatsapp: request.seller_whatsapp,
        seller_handle: request.seller_handle,
        delivery_qr_token: String::from("delivery qr token"),
        last_seller_notified_at: now,
        delivery_address: request.delivery_address,
        referred_by: Uuid::parse_str("019e458c-23cc-7591-ad6c-25930e2ef0d8").unwrap(),
        created_at: now,
        updated_at: now,
        paid_at: Some(now),
        confirmed_at: Some(now),
        shipped_at: Some(now),
        delivered_at: Some(now),
        released_at: Some(now),
    };

    Ok(Json(response))
}
