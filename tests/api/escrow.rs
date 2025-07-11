use crate::helpers::{TestApp, generate_address};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use serde_json::json;

#[tokio::test]
async fn allocate_bounty_happy_path() {
    let app = TestApp::new().await;
    let db = &app.db;
    use bigdecimal::BigDecimal;
    use chrono::Utc;

    // Insert a user into escrow_users with a sufficient balance
    let wallet = generate_address();
    sqlx::query(
        "INSERT INTO escrow_users (wallet_address, balance) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(&wallet)
    .bind(BigDecimal::from(1000))
    .execute(&db.pool)
    .await
    .expect("Failed to insert user");

    // Insert a project owned by the user
    let contract_address = generate_address();
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

#[tokio::test]
async fn allocate_bounty_invalid_amount() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = generate_address();
    let contract_address = generate_address();
    // Insert user and project
    sqlx::query(
        "INSERT INTO escrow_users (wallet_address, balance) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(&wallet)
    .bind(bigdecimal::BigDecimal::from(1000))
    .execute(&db.pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO projects (owner_address, contract_address, name, description, contact_info) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
    )
    .bind(&wallet)
    .bind(&contract_address)
    .bind("Test Project")
    .bind("A test project.")
    .bind("test@example.com")
    .execute(&db.pool)
    .await
    .unwrap();
    // Zero amount
    let payload = json!({
        "wallet_address": wallet,
        "project_contract_address": contract_address,
        "amount": "0",
        "currency": "USD",
        "bounty_expiry_date": chrono::Utc::now().to_rfc3339(),
    });
    let req = Request::post("/allocate_bounty")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn allocate_bounty_invalid_address_format() {
    let app: TestApp = TestApp::new().await;
    let wallet = "not_a_valid_address".to_string();
    let contract_address = "0x123".to_string();

    let payload = json!({
        "wallet_address": wallet,
        "project_contract_address": contract_address,
        "amount": "100.0",
        "currency": "USD",
        "bounty_expiry_date": chrono::Utc::now().to_rfc3339(),
    });
    let req = Request::post("/allocate_bounty")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn allocate_bounty_escrow_user_not_found() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = generate_address();
    let contract_address = generate_address();
    // Only insert project
    sqlx::query(
        "INSERT INTO projects (owner_address, contract_address, name, description, contact_info) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
    )
    .bind(&wallet)
    .bind(&contract_address)
    .bind("Test Project")
    .bind("A test project.")
    .bind("test@example.com")
    .execute(&db.pool)
    .await
    .unwrap();
    let payload = json!({
        "wallet_address": wallet,
        "project_contract_address": contract_address,
        "amount": "100.0",
        "currency": "USD",
        "bounty_expiry_date": chrono::Utc::now().to_rfc3339(),
    });
    let req = Request::post("/allocate_bounty")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn allocate_bounty_insufficient_balance() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = generate_address();
    let contract_address = generate_address();
    // Insert user with low balance and project
    sqlx::query(
        "INSERT INTO escrow_users (wallet_address, balance) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(&wallet)
    .bind(bigdecimal::BigDecimal::from(10))
    .execute(&db.pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO projects (owner_address, contract_address, name, description, contact_info) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
    )
    .bind(&wallet)
    .bind(&contract_address)
    .bind("Test Project")
    .bind("A test project.")
    .bind("test@example.com")
    .execute(&db.pool)
    .await
    .unwrap();
    let payload = json!({
        "wallet_address": wallet,
        "project_contract_address": contract_address,
        "amount": "100.0",
        "currency": "USD",
        "bounty_expiry_date": chrono::Utc::now().to_rfc3339(),
    });
    let req = Request::post("/allocate_bounty")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn allocate_bounty_project_not_found() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = generate_address();
    let contract_address = generate_address();
    // Insert user only
    sqlx::query(
        "INSERT INTO escrow_users (wallet_address, balance) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(&wallet)
    .bind(bigdecimal::BigDecimal::from(1000))
    .execute(&db.pool)
    .await
    .unwrap();
    let payload = json!({
        "wallet_address": wallet,
        "project_contract_address": contract_address,
        "amount": "100.0",
        "currency": "USD",
        "bounty_expiry_date": chrono::Utc::now().to_rfc3339(),
    });
    let req = Request::post("/allocate_bounty")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn allocate_bounty_user_not_owner() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = generate_address();
    let other_wallet = generate_address();
    let contract_address = generate_address();
    // Insert user and project owned by someone else
    sqlx::query(
        "INSERT INTO escrow_users (wallet_address, balance) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(&wallet)
    .bind(bigdecimal::BigDecimal::from(1000))
    .execute(&db.pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO projects (owner_address, contract_address, name, description, contact_info) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
    )
    .bind(&other_wallet)
    .bind(&contract_address)
    .bind("Test Project")
    .bind("A test project.")
    .bind("test@example.com")
    .execute(&db.pool)
    .await
    .unwrap();
    let payload = json!({
        "wallet_address": wallet,
        "project_contract_address": contract_address,
        "amount": "100.0",
        "currency": "USD",
        "bounty_expiry_date": chrono::Utc::now().to_rfc3339(),
    });
    let req = Request::post("/allocate_bounty")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}
