use crate::helpers::{TestApp, generate_address};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use chrono::{DateTime, Utc};
use serde_json::json;
use sqlx::Row;
use tokio::time::Duration;
use uuid::Uuid;
// use sqlx::Executor;

#[tokio::test]
async fn allocate_bounty_happy_path() {
    let app = TestApp::new().await;
    let db = &app.db;
    use bigdecimal::BigDecimal;
    use chrono::Utc;

    // Insert a user into escrow_users with a sufficient balance
    let wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address, balance) VALUES ($1, $2) ON CONFLICT DO NOTHING")
        .bind(&wallet)
        .bind(BigDecimal::from(1000))
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");

    // Insert a project owned by the user
    let contract_address = format!("0x{:0>64}", "1");
    let _project_id: String = sqlx::query_scalar(
        "INSERT INTO projects (owner_address, contract_address, name, description, contact_info) VALUES ($1, $2, $3, $4, $5) RETURNING id::TEXT"
    )
    .bind(&wallet)
    .bind(&contract_address)
    .bind("Test Project")
    .bind("A test project.")
    .bind("test@example.com")
    .fetch_one(&db.pool)
    .await
    .expect("Failed to insert project");

    // Prepare payload
    let payload = serde_json::json!({
        "wallet_address": wallet,
        "project_contract_address": contract_address,
        "amount": "100.0",
        "currency": "USD",
        "bounty_expiry_date": Utc::now().to_rfc3339(),
    });
    let req = axum::http::Request::post("/allocate_bounty")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), axum::http::StatusCode::OK);
}

