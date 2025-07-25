use crate::helpers::TestApp;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;

#[tokio::test]
async fn test_deposit_successful_with_no_escrow_users() {
    let app = TestApp::new().await;
    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefcacdefabca";
    let tx_hash = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefcacdefabc";
    let payload = json!({
        "wallet_address": wallet,
        "amount": 10000000,
        "currency": "USDT",
        "notes": "Test project funding",
        "transaction_hash": tx_hash
    });
    let request = Request::post("/deposit")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(request).await;
    assert_eq!(res.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_deposit_successful_with_escrow_users_available() {
    let app = TestApp::new().await;
    let db = &app.db;
    // Create escrow account
    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabca";
    let tx_hash = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabc";
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to create escrow account");

    let payload = json!({
        "wallet_address": wallet,
        "amount": 10000000,
        "currency": "USDT",
        "notes": "Test project funding",
        "transaction_hash": tx_hash
    });
    let request = Request::post("/deposit")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(request).await;
    assert_eq!(res.status(), StatusCode::CREATED);
}
