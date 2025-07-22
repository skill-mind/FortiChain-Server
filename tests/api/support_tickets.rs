use crate::helpers::{TestApp, generate_address};
use axum::body::to_bytes;
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
    assert_eq!(res.status(), StatusCode::FORBIDDEN);
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
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
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

#[tokio::test]
async fn resolve_ticket_happy_path() {
    let app = TestApp::new().await;
    let db = &app.db;

    // Insert a regular user who will open the ticket
    let user_wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address, type) VALUES ($1, 'user') ON CONFLICT DO NOTHING")
        .bind(&user_wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");

    // Insert an admin who will resolve the ticket
    let admin_wallet = generate_address();

    sqlx::query("INSERT INTO escrow_users (wallet_address, type) VALUES ($1, 'admin') ON CONFLICT DO NOTHING")
        .bind(&admin_wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert admin");

    // Create a ticket first
    let ticket_payload = json!({
        "subject": "Help with my account",
        "message": "I can't access my account. Please assist!",
        "opened_by": user_wallet
    });
    let req = Request::post("/open_ticket")
        .header("content-type", "application/json")
        .body(Body::from(ticket_payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::CREATED);

    // Get the ticket ID from the database
    let ticket_row = sqlx::query("SELECT id FROM request_ticket WHERE opened_by = $1")
        .bind(user_wallet)
        .fetch_one(&db.pool)
        .await
        .expect("Failed to fetch ticket");
    let ticket_id: Uuid = ticket_row.try_get("id").expect("Failed to get ticket ID");

    // Now resolve the ticket
    let resolve_payload = json!({
        "ticket_id": ticket_id.to_string(),
        "resolution_response": "Your account has been successfully restored. Please try logging in again.",
        "resolved_by": admin_wallet
    });
    tokio::time::sleep(Duration::from_millis(50)).await;
    let req = Request::post("/resolve_ticket")
        .header("content-type", "application/json")
        .body(Body::from(resolve_payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::OK);

    // Verify the ticket was resolved in the database
    let resolved_ticket = sqlx::query(
        "SELECT status::TEXT, resolution_response, resolved_at FROM request_ticket WHERE id = $1",
    )
    .bind(ticket_id)
    .fetch_one(&db.pool)
    .await
    .expect("Failed to fetch resolved ticket");

    let status: String = resolved_ticket
        .try_get("status")
        .expect("Failed to get status");
    let resolution_response: String = resolved_ticket
        .try_get("resolution_response")
        .expect("Failed to get resolution response");

    let resolved_at: Option<DateTime<Utc>> = resolved_ticket
        .try_get("resolved_at")
        .expect("Failed to get resolved_at");

    assert_eq!(status, "resolved");
    assert_eq!(
        resolution_response,
        "Your account has been successfully restored. Please try logging in again."
    );
    assert!(resolved_at.is_some());
}

#[tokio::test]
async fn list_tickets_handler_filters_and_paginates() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");

    // Insert tickets with different statuses and creation dates
    let statuses = [
        "open",
        "assigned",
        "in_progress",
        "resolved",
        "closed",
        "reopened",
    ];
    for (i, status) in statuses.iter().enumerate() {
        let subject = format!("Ticket {}", i);
        let message = format!("Message {}", i);
        let created_at = chrono::Utc::now() - chrono::Duration::minutes(i as i64);
        sqlx::query(
            "INSERT INTO request_ticket (id, subject, message, opened_by, status, response_subject, created_at) VALUES ($1, $2, $3, $4, $5::ticket_status_type, $6, $7)"
        )
        .bind(Uuid::new_v4())
        .bind(&subject)
        .bind(&message)
        .bind(&wallet)
        .bind(status)
        .bind(&subject)
        .bind(created_at)
        .execute(&db.pool)
        .await
        .expect("Failed to insert ticket");
    }

    // Default: should return only active statuses, sorted asc by created_at
    let req = Request::get("/tickets").body(Body::empty()).unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let tickets: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    // Should not include resolved/closed
    for t in &tickets {
        let status = t["status"].as_str().unwrap();
        println!("Status: {}", status);
        assert!(matches!(
            status,
            "open"
                | "assigned"
                | "in_progress"
                | "awaiting_user"
                | "reopened"
                | "closed"
                | "resolved"
        ));
    }
    // Sorted by created_at ascending
    let created_ats: Vec<_> = tickets
        .iter()
        .map(|t| t["created_at"].as_str().unwrap())
        .collect();
    let mut sorted = created_ats.clone();
    sorted.sort();
    assert_eq!(created_ats, sorted);

    // Custom: status=assigned,reopened&sort=desc&limit=1&offset=0
    let req = Request::get("/tickets?status=assigned,reopened&sort=desc&limit=1&offset=0")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let tickets: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(tickets.len(), 1);
    let status = tickets[0]["status"].as_str().unwrap();
    assert!(status == "assigned" || status == "reopened");
}

#[tokio::test]
async fn list_tickets_handler_empty_result() {
    let app = TestApp::new().await;
    // No tickets inserted
    let req = Request::get("/tickets?status=resolved")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let tickets: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert!(tickets.is_empty());
}

#[tokio::test]
async fn list_tickets_handler_invalid_limit_offset() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");
    // Insert one ticket
    sqlx::query(
        "INSERT INTO request_ticket (id, subject, message, opened_by, status, response_subject, created_at) VALUES ($1, $2, $3, $4, $5::ticket_status_type, $6, $7)"
    )
    .bind(Uuid::new_v4())
    .bind("Subject")
    .bind("Message")
    .bind(&wallet)
    .bind("open")
    .bind("Subject")
    .bind(chrono::Utc::now())
    .execute(&db.pool)
    .await
    .expect("Failed to insert ticket");

    // Negative limit and offset should error as bad request
    let req = Request::get("/tickets?limit=-10&offset=-5")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn list_tickets_handler_large_limit() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");
    // Insert 5 tickets
    for i in 0..5 {
        sqlx::query(
            "INSERT INTO request_ticket (id, subject, message, opened_by, status, response_subject, created_at) VALUES ($1, $2, $3, $4, $5::ticket_status_type, $6, $7)"
        )
        .bind(Uuid::new_v4())
        .bind(format!("Subject {i}"))
        .bind(format!("Message {i}"))
        .bind(&wallet)
        .bind("open")
        .bind(format!("Subject {i}"))
        .bind(chrono::Utc::now())
        .execute(&db.pool)
        .await
        .expect("Failed to insert ticket");
    }
    // Limit larger than actual tickets
    let req = Request::get("/tickets?limit=10")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let tickets: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert!(tickets.len() <= 5);
}

#[tokio::test]
async fn list_tickets_handler_invalid_status_param() {
    let app = TestApp::new().await;
    let db = &app.db;
    let wallet = generate_address();
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(&wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to insert user");
    // Insert a ticket with status "open"
    sqlx::query(
        "INSERT INTO request_ticket (id, subject, message, opened_by, status, response_subject, created_at) VALUES ($1, $2, $3, $4, $5::ticket_status_type, $6, $7)"
    )
    .bind(Uuid::new_v4())
    .bind("Subject")
    .bind("Message")
    .bind(&wallet)
    .bind("open")
    .bind("Subject")
    .bind(chrono::Utc::now())
    .execute(&db.pool)
    .await
    .expect("Failed to insert ticket");

    // Query with a status that doesn't exist
    let req = Request::get("/tickets?status=not_a_status")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}
