use crate::helpers::{TestApp, generate_address};
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
    let wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&wallet)
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
    let wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&wallet)
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
    let wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&wallet)
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
    let wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&wallet)
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
    let wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&wallet)
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
    let wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&wallet)
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
    let wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&wallet)
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
    let wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&wallet)
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
    let wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&wallet)
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

#[tokio::test]
async fn assign_ticket_happy_path() {
    let app = TestApp::new().await;
    let db = &app.db;
    // Insert a support agent
    let agent_wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address, type) VALUES ($1, 'support_agent');")
        .bind(&agent_wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert support agent");
    // Insert a user
    let user_wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&user_wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");
    // Create a ticket
    let ticket_id: String = sqlx::query_scalar(
        "INSERT INTO request_ticket (subject, message, opened_by, response_subject) VALUES ($1, $2, $3, $4) RETURNING id::TEXT"
    )
    .bind("Assignment Test")
    .bind("Please assign this ticket.")
    .bind(&user_wallet)
    .bind("Assignment Test")
    .fetch_one(&db.pool)
    .await
    .expect("Failed to create ticket");

    // Assign the ticket
    let payload = json!({
        "ticket_id": ticket_id,
        "support_agent_wallet": agent_wallet
    });
    let req = Request::post("/assign_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn assign_ticket_nonexistent_ticket() {
    let app = TestApp::new().await;
    let db = &app.db;
    // Insert a support agent
    let agent_wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address, type) VALUES ($1, 'support_agent');")
        .bind(&agent_wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert support agent");
    // Try to assign a non-existent ticket
    let payload = json!({
        "ticket_id": "99999999-9999-4999-9999-999999999999",
        "support_agent_wallet": agent_wallet
    });
    let req = Request::post("/assign_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn assign_ticket_already_assigned() {
    let app = TestApp::new().await;
    let db = &app.db;
    let agent_wallet = generate_address();
    let user_wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address, type) VALUES ($1, 'support_agent');")
        .bind(&agent_wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert support agent");
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&user_wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");
    // Create and assign a ticket
    let ticket_id: String = sqlx::query_scalar(
        "INSERT INTO request_ticket (subject, message, opened_by, response_subject, status, assigned_to) VALUES ($1, $2, $3, $4, 'assigned', $5) RETURNING id::TEXT"
    )
    .bind("Assignment Test")
    .bind("Please assign this ticket.")
    .bind(&user_wallet)
    .bind("Assignment Test")
    .bind(&agent_wallet)
    .fetch_one(&db.pool)
    .await
    .expect("Failed to create ticket");
    // Try to assign again
    let payload = json!({
        "ticket_id": ticket_id,
        "support_agent_wallet": agent_wallet
    });
    let req = Request::post("/assign_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn assign_ticket_non_agent() {
    let app = TestApp::new().await;
    let db = &app.db;
    let not_agent_wallet = generate_address();
    let user_wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&not_agent_wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user (not agent)");
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&user_wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");
    // Create a ticket
    let ticket_id: String = sqlx::query_scalar(
        "INSERT INTO request_ticket (subject, message, opened_by, response_subject) VALUES ($1, $2, $3, $4) RETURNING id::TEXT"
    )
    .bind("Assignment Test")
    .bind("Please assign this ticket.")
    .bind(&user_wallet)
    .bind("Assignment Test")
    .fetch_one(&db.pool)
    .await
    .expect("Failed to create ticket");
    // Try to assign to a non-agent
    let payload = json!({
        "ticket_id": ticket_id,
        "support_agent_wallet": not_agent_wallet
    });
    let req = Request::post("/assign_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn assign_ticket_agent_busy() {
    let app = TestApp::new().await;
    let db = &app.db;
    let agent_wallet = generate_address();
    let user_wallet1 = generate_address();
    let user_wallet2 = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address, type) VALUES ($1, 'support_agent');")
        .bind(&agent_wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert support agent");
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&user_wallet1)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user1");
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&user_wallet2)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user2");
    // Create and assign a ticket to agent
    let _ticket_id1: String = sqlx::query_scalar(
        "INSERT INTO request_ticket (subject, message, opened_by, response_subject, status, assigned_to) VALUES ($1, $2, $3, $4, 'assigned', $5) RETURNING id::TEXT"
    )
    .bind("Assignment Test 1")
    .bind("Please assign this ticket.")
    .bind(&user_wallet1)
    .bind("Assignment Test 1")
    .bind(&agent_wallet)
    .fetch_one(&db.pool)
    .await
    .expect("Failed to create ticket 1");
    // Create a second ticket
    let ticket_id2: String = sqlx::query_scalar(
        "INSERT INTO request_ticket (subject, message, opened_by, response_subject) VALUES ($1, $2, $3, $4) RETURNING id::TEXT"
    )
    .bind("Assignment Test 2")
    .bind("Please assign this ticket.")
    .bind(&user_wallet2)
    .bind("Assignment Test 2")
    .fetch_one(&db.pool)
    .await
    .expect("Failed to create ticket 2");
    // Try to assign the busy agent to the second ticket
    let payload = json!({
        "ticket_id": ticket_id2,
        "support_agent_wallet": agent_wallet
    });
    let req = Request::post("/assign_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn assign_ticket_nonexistent_agent() {
    let app = TestApp::new().await;
    let db = &app.db;
    let user_wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&user_wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");
    // Create a ticket
    let ticket_id: String = sqlx::query_scalar(
        "INSERT INTO request_ticket (subject, message, opened_by, response_subject) VALUES ($1, $2, $3, $4) RETURNING id::TEXT"
    )
    .bind("Assignment Test")
    .bind("Please assign this ticket.")
    .bind(&user_wallet)
    .bind("Assignment Test")
    .fetch_one(&db.pool)
    .await
    .expect("Failed to create ticket");
    // Try to assign to a non-existent agent
    let agent_wallet = generate_address();
    let payload = json!({
        "ticket_id": ticket_id,
        "support_agent_wallet": agent_wallet
    });
    let req = Request::post("/assign_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn assign_ticket_invalid_ticket_id() {
    let app = TestApp::new().await;
    let db = &app.db;
    let agent_wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address, type) VALUES ($1, 'support_agent');")
        .bind(&agent_wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert support agent");
    // Try to assign with an invalid ticket id format
    let payload = json!({
        "ticket_id": "not-a-uuid",
        "support_agent_wallet": agent_wallet
    });
    let req = Request::post("/assign_ticket")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert!(!res.status().is_success());
}
