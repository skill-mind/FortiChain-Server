use crate::helpers::{TestApp, generate_address};
use axum::{body::Body, extract::Request, http::StatusCode};
use serde_json::json;

#[tokio::test]
async fn test_view_validator_profile_success() {
    let app = TestApp::new().await;
    let db = &app.db;

    // Create a test validator profile
    let wallet_address = generate_address();
    let validator_id = uuid::Uuid::now_v7();

    sqlx::query(
        r#"
        INSERT INTO validator_profiles (
            id, wallet_address, government_name, date_of_birth, nationality,
            email_address, mobile_number, years_of_experience, resume_path,
            country, document, document_front_path, document_back_path, verification
        ) VALUES (
            $1, $2, 'Jane Doe', '1990-05-15', 'Nigerian',
            'jane.doe@example.com', '+2348123456789', 7, '/resumes/jane_doe.pdf',
            'Nigeria', 'passport', '/kyc/jane_doe_passport.jpg', '/kyc/jane_doe_passport_back.jpg', 'verified'
        )
        "#,
    )
    .bind(validator_id)
    .bind(&wallet_address)
    .execute(&db.pool)
    .await
    .expect("Failed to insert test validator profile");

    let language_id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO programming_languages (name) VALUES ('Cairo') RETURNING id",
    )
    .fetch_one(&db.pool)
    .await
    .expect("Failed to insert programming language");

    sqlx::query(
        "INSERT INTO validator_programming_languages (validator_id, language_id) VALUES ($1, $2)",
    )
    .bind(validator_id)
    .bind(language_id)
    .execute(&db.pool)
    .await
    .expect("Failed to insert validator programming language");

    let expertise_id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO expertise (name) VALUES ('Smart Contract Auditing') RETURNING id",
    )
    .fetch_one(&db.pool)
    .await
    .expect("Failed to insert expertise");

    sqlx::query("INSERT INTO validator_expertise (validator_id, expertise_id) VALUES ($1, $2)")
        .bind(validator_id)
        .bind(expertise_id)
        .execute(&db.pool)
        .await
        .expect("Failed to insert validator expertise");

    let payload = json!({
        "wallet_address": wallet_address
    });

    let req = Request::builder()
        .method("POST")
        .uri("/validator/profile/view")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();
    assert_eq!(status, StatusCode::OK);

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["government_name"], "Jane Doe");
    assert_eq!(json["email_address"], "jane.doe@example.com");
    assert_eq!(json["years_of_experience"], 7);
    assert_eq!(json["verification"], "verified");
    assert!(json["programming_languages"].as_array().unwrap().contains(&json!("Cairo")));
    assert!(json["expertise"].as_array().unwrap().contains(&json!("Smart Contract Auditing")));
}

#[tokio::test]
async fn test_view_validator_profile_not_found() {
    let app = TestApp::new().await;
    let non_existent_address = generate_address();

    let payload = json!({
        "wallet_address": non_existent_address
    });

    let req = Request::builder()
        .method("POST")
        .uri("/validator/profile/view")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_view_validator_profile_invalid_address() {
    let app = TestApp::new().await;

    let payload = json!({
        "wallet_address": "invalid_address"
    });

    let req = Request::builder()
        .method("POST")
        .uri("/validator/profile/view")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_view_validator_profile_missing_wallet_address() {
    let app = TestApp::new().await;

    let payload = json!({});

    let req = Request::builder()
        .method("POST")
        .uri("/validator/profile/view")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}
