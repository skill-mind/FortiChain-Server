use crate::helpers::TestApp;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use fortichain_server::http::newsletter::domain::{NewsletterSubscriber, SubscriberStatus};
use serde_json::json;
use sqlx::Row;
use uuid::Uuid;

#[tokio::test]
async fn test_verify_subscriber_success() {
    let app = TestApp::new().await;
    let db = &app.db;

    // Create a test subscriber
    let subscriber_id = Uuid::now_v7();
    let email = "test@example.com";
    let name = "Test User";
    let token = "test-verification-token-123";

    // Insert subscriber
    sqlx::query(
        r#"
        INSERT INTO newsletter_subscribers (id, email, name, status)
        VALUES ($1, $2, $3, 'pending')
        "#,
    )
    .bind(subscriber_id)
    .bind(email)
    .bind(name)
    .execute(&db.pool)
    .await
    .expect("Failed to insert test subscriber");

    // Insert verification token
    sqlx::query(
        r#"
        INSERT INTO subscription_token (subscription_token, subscriber_id)
        VALUES ($1, $2)
        "#,
    )
    .bind(token)
    .bind(subscriber_id)
    .execute(&db.pool)
    .await
    .expect("Failed to insert verification token");

    let verify_request = json!({
        "token": token
    });

    let req = Request::post("/newsletter/verify")
        .header("content-type", "application/json")
        .body(Body::from(verify_request.to_string()))
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    // Verify the subscriber status was updated
    let updated_subscriber = sqlx::query(
        "SELECT status::text as status, subscribed_at FROM newsletter_subscribers WHERE id = $1",
    )
    .bind(subscriber_id)
    .fetch_one(&db.pool)
    .await
    .expect("Failed to fetch updated subscriber");

    let status: String = updated_subscriber.get("status");
    let subscribed_at: Option<chrono::DateTime<chrono::Utc>> =
        updated_subscriber.get("subscribed_at");

    assert_eq!(status, "active");
    assert!(subscribed_at.is_some());
}

#[tokio::test]
async fn test_verify_subscriber_invalid_token() {
    let app = TestApp::new().await;

    let verify_request = json!({
        "token": "invalid-token"
    });

    let req = Request::post("/newsletter/verify")
        .header("content-type", "application/json")
        .body(Body::from(verify_request.to_string()))
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_verify_subscriber_already_verified() {
    let app = TestApp::new().await;
    let db = &app.db;

    // Create a test subscriber that's already active
    let subscriber_id = Uuid::now_v7();
    let email = "test@example.com";
    let name = "Test User";
    let token = "test-verification-token-456";

    // Insert subscriber with active status
    sqlx::query(
        r#"
        INSERT INTO newsletter_subscribers (id, email, name, status, subscribed_at)
        VALUES ($1, $2, $3, 'active', NOW())
        "#,
    )
    .bind(subscriber_id)
    .bind(email)
    .bind(name)
    .execute(&db.pool)
    .await
    .expect("Failed to insert test subscriber");

    // Insert verification token
    sqlx::query(
        r#"
        INSERT INTO subscription_token (subscription_token, subscriber_id)
        VALUES ($1, $2)
        "#,
    )
    .bind(token)
    .bind(subscriber_id)
    .execute(&db.pool)
    .await
    .expect("Failed to insert verification token");

    let verify_request = json!({
        "token": token
    });

    let req = Request::post("/newsletter/verify")
        .header("content-type", "application/json")
        .body(Body::from(verify_request.to_string()))
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_verify_subscriber_empty_token() {
    let app = TestApp::new().await;

    let verify_request = json!({
        "token": ""
    });

    let req = Request::post("/newsletter/verify")
        .header("content-type", "application/json")
        .body(Body::from(verify_request.to_string()))
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = TestApp::new().await;

    let body = json!({
        "email": "test@example.com",
        "name": "Test"
    });
    // Act
    let req = Request::post("/newsletter/subscribe")
        .header("Content-Type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();

    let response = app.request(req).await;

    // Assert
    assert_eq!(response.status(), StatusCode::CREATED);

    let saved = sqlx::query!(
        "SELECT id, email, name, status as \"status!: SubscriberStatus\", subscribed_at, created_at, updated_at FROM newsletter_subscribers",
    )
    .fetch_one(&app.db.pool)
    .await
    .expect("Failed to fetch saved subscriber.");

    let subscriber = NewsletterSubscriber {
        id: saved.id,
        email: saved.email,
        name: saved.name,
        status: saved.status,
        subscribed_at: saved.subscribed_at,
        created_at: saved.created_at,
        updated_at: saved.updated_at,
    };

    assert_eq!(subscriber.email, "test@example.com");
    assert_eq!(subscriber.name, "Test");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = TestApp::new().await;
    let test_cases = vec![
        (json!({"name": "Test"}), "missing the email"),
        (json!({"email": "test@example.com"}), "missing the name"),
        (json!({}), "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let req = Request::post("/newsletter/subscribe")
            .header("Content-Type", "application/json")
            .body(Body::from(invalid_body.to_string()))
            .unwrap();
        let response = app.request(req).await;

        // Assert
        assert_eq!(
            response.status(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "The API did not fail with 422 Unprocessable Entity when the payload was {}.",
            error_message
        );
    }
}
