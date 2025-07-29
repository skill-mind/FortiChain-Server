use crate::helpers::TestApp;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use fortichain_server::http::newsletter::domain::NewsletterSubscriber;
use serde_json::json;

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
    assert_eq!(response.status(), StatusCode::OK);

    let saved = sqlx::query_as!(
        NewsletterSubscriber,
        "SELECT email, name FROM newsletter_subscribers",
    )
    .fetch_one(&app.db.pool)
    .await
    .expect("Failed to fetch saved subscriber.");

    assert_eq!(saved.email, "test@example.com");
    assert_eq!(saved.name, "Test");
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
