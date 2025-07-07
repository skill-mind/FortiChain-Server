use crate::helpers::TestApp;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
// use sqlx::Executor;

#[tokio::test]
async fn open_ticket_happy_path() {
    let app = TestApp::new().await;
    let db = &app.db;
    // Insert a user into escrow_users
    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabca";
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");

    let payload = json!({
        "subject": "Help with my account",
        "message": "I can't access my account. Please assist!",
        "opened_by": wallet
    });
    let req = Request::post("/open_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn open_ticket_subject_too_short() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabce";
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");
    let payload = json!({
        "subject": "Hi",
        "message": "This is a valid message with enough length.",
        "opened_by": wallet
    });
    let req = Request::post("/open_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn open_ticket_message_too_short() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabce";
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");
    let payload = json!({
        "subject": "Valid Subject",
        "message": "short",
        "opened_by": wallet
    });
    let req = Request::post("/open_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn open_ticket_empty_wallet() {
    let app = TestApp::new().await;
    let payload = json!({
        "subject": "Valid Subject",
        "message": "This is a valid message with enough length.",
        "opened_by": ""
    });
    let req = Request::post("/open_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn open_ticket_nonexistent_user() {
    let app = TestApp::new().await;
    let payload = json!({
        "subject": "Valid Subject",
        "message": "This is a valid message with enough length.",
        "opened_by": "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
    });
    let req = Request::post("/open_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn open_ticket_subject_max_length() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd";
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");
    let subject = "a".repeat(100);
    let payload = json!({
        "subject": subject,
        "message": "This is a valid message with enough length.",
        "opened_by": wallet
    });
    let req = Request::post("/open_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn open_ticket_subject_too_long() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd";
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");
    let subject = "a".repeat(101);
    let payload = json!({
        "subject": subject,
        "message": "This is a valid message with enough length.",
        "opened_by": wallet
    });
    let req = Request::post("/open_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn open_ticket_message_max_length() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd";
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");
    let message = "a".repeat(4999);
    let payload = json!({
        "subject": "Valid Subject",
        "message": message,
        "opened_by": wallet
    });
    let req = Request::post("/open_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn open_ticket_message_too_long() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd";
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");
    let message = "a".repeat(5001);
    let payload = json!({
        "subject": "Valid Subject",
        "message": message,
        "opened_by": wallet
    });
    let req = Request::post("/open_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn open_ticket_trimming() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd";
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");
    let payload = json!({
        "subject": "   Valid Subject   ",
        "message": "   This is a valid message with enough length.   ",
        "opened_by": wallet
    });
    let req = Request::post("/open_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn open_ticket_invalid_wallet_format() {
    let app = TestApp::new().await;

    // Not inserting user, and using an invalid wallet format
    let payload = json!({
        "subject": "Valid Subject",
        "message": "This is a valid message with enough length.",
        "opened_by": "not_a_wallet"
    });
    let req = Request::post("/open_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    // Should fail due to DB constraint
    assert!(
        res.status() == StatusCode::BAD_REQUEST
            || res.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn open_ticket_sql_injection() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd";
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");
    let payload = json!({
        "subject": "Valid Subject; DROP TABLE users; --",
        "message": "This is a valid message; DROP TABLE request_ticket; --",
        "opened_by": wallet
    });
    let req = Request::post("/open_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::CREATED);
}
