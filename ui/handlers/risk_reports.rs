use crate::models::risk_reports::{RiskReportsRequest, RiskReportsResponse};
use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_risk_report(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<RiskReportsRequest>,
) -> Result<Json<RiskReportsResponse>, String> {
    let id = Uuid::now_v7();
    let now = Utc::now();

    sqlx::query(
        "
        INSERT INTO risk_reports
        (
            id,
            seller_id,
            phone,
            risk_source,
            description,
            risk_severity,
            evidence_url,
            verified,
            verified_at,
            reported_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ",
    )
    .bind(id)
    .bind(None::<Uuid>)
    .bind(&payload.phone)
    .bind(&payload.risk_source)
    .bind(&payload.description)
    .bind(&payload.risk_severity)
    .bind(&payload.evidence_url)
    .bind(true)
    .bind(Some(now))
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let response = RiskReportsResponse {
        id,
        seller_id: None::<Uuid>,
        phone: payload.phone,
        risk_source: payload.risk_source,
        description: payload.description,
        risk_severity: payload.risk_severity,
        evidence_url: payload.evidence_url,
        verified_at: Some(now),
        reported_at: now,
    };

    Ok(Json(response))
}
