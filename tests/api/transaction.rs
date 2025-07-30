use crate::helpers::TestApp;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use bigdecimal::BigDecimal;
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

// use crate::helpers::TestApp;
// use axum::{
//     body::Body,
//     http::{Request, StatusCode},
// };
// use serde_json::json;
#[tokio::test]
async fn test_withdraw_successful() {
    let app = TestApp::new().await;
    let db = &app.db;

    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd";
    let initial_balance = BigDecimal::from(20000000);
    sqlx::query!(
        r#"
        INSERT INTO escrow_users (wallet_address, balance)
        VALUES ($1, $2)
        ON CONFLICT (wallet_address) DO UPDATE
        SET balance = EXCLUDED.balance
        "#,
        wallet,
        initial_balance
    )
    .execute(&db.pool)
    .await
    .expect("Failed to create escrow account");

    let tx_hash = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabc";
    let withdrawal_amount = BigDecimal::from(10000000);
    let payload = json!({
        "wallet_address": wallet,
        "amount": withdrawal_amount.to_string(),
        "currency": "USDT",
        "notes": "Project withdrawal",
        "transaction_hash": tx_hash
    });

    let request = Request::post("/withdraw")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(request).await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let updated_balance: BigDecimal = sqlx::query_scalar!(
        "SELECT balance FROM escrow_users WHERE wallet_address = $1",
        wallet
    )
    .fetch_one(&db.pool)
    .await
    .expect("Failed to fetch balance");

    assert_eq!(updated_balance, initial_balance - withdrawal_amount.clone());

    #[derive(Debug, sqlx::FromRow)]
    struct Transaction {
        amount: BigDecimal,
        r#type: String,
    }

    let transaction = sqlx::query_as!(
        Transaction,
        r#"
        SELECT amount, type as "type!: String"
        FROM escrow_transactions
        WHERE wallet_address = $1 AND transaction_hash = $2
        "#,
        wallet,
        tx_hash
    )
    .fetch_one(&db.pool)
    .await
    .expect("Failed to fetch transaction");

    assert_eq!(transaction.amount, withdrawal_amount);
    assert_eq!(transaction.r#type, "withdrawal".to_string());
}

#[tokio::test]
async fn test_withdraw_insufficient_balance() {
    let app = TestApp::new().await;
    let db = &app.db;

    // Setup: Create escrow user with minimal balance
    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabce";
    let initial_balance = BigDecimal::from(5000000);
    sqlx::query!(
        "INSERT INTO escrow_users (wallet_address, balance) VALUES ($1, $2)",
        wallet,
        initial_balance
    )
    .execute(&db.pool)
    .await
    .expect("Failed to create escrow account");

    let tx_hash = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabe";
    let withdrawal_amount = 10000000; // More than balance
    let payload = json!({
        "wallet_address": wallet,
        "amount": withdrawal_amount,
        "currency": "USDT",
        "notes": "Project withdrawal",
        "transaction_hash": tx_hash
    });

    let request = Request::post("/withdraw")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(request).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    // Verify balance wasn't changed
    let current_balance = sqlx::query!(
        "SELECT balance FROM escrow_users WHERE wallet_address = $1",
        wallet
    )
    .fetch_one(&db.pool)
    .await
    .expect("Failed to fetch balance");

    assert_eq!(current_balance.balance, initial_balance);
}

#[tokio::test]
async fn test_withdraw_nonexistent_wallet() {
    let app = TestApp::new().await;

    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcf";
    let tx_hash = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabf";
    let payload = json!({
        "wallet_address": wallet,
        "amount": 10000000,
        "currency": "USDT",
        "notes": "Project withdrawal",
        "transaction_hash": tx_hash
    });

    let request = Request::post("/withdraw")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(request).await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_withdraw_invalid_amount() {
    let app = TestApp::new().await;

    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabca";
    let tx_hash = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabc";
    let payload = json!({
        "wallet_address": wallet,
        "amount": 0,  // Invalid amount
        "currency": "USDT",
        "notes": "Project withdrawal",
        "transaction_hash": tx_hash
    });

    let request = Request::post("/withdraw")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(request).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}
