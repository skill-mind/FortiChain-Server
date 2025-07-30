use crate::helpers::{TestApp, generate_address};
use axum::{body::Body, extract::Request, http::StatusCode};
use serde_json::json;
use uuid::Uuid;

#[tokio::test]
async fn test_delete_validator_profile_success() {
    let app = TestApp::new().await;
    let db = &app.db;

    // Create a test validator profile
    let wallet_address = generate_address();
    let validator_id = Uuid::now_v7();

    // Insert validator profile
    sqlx::query(
        r#"
        INSERT INTO validator_profiles (
            id, wallet_address, government_name, date_of_birth, nationality, 
            email_address, mobile_number, years_of_experience, resume_path,
            country, document, document_front_path, document_back_path
        ) VALUES (
            $1, $2, 'John Doe', '1990-01-01', 'American', 
            'john.doe@example.com', '+1234567890', 5, '/path/to/resume.pdf',
            'United States', 'passport', '/path/to/front.jpg', '/path/to/back.jpg'
        )
        "#,
    )
    .bind(validator_id)
    .bind(&wallet_address)
    .execute(&db.pool)
    .await
    .expect("Failed to insert test validator profile");

    // Insert some related data
    let language_id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO programming_languages (name) VALUES ('Rust') RETURNING id",
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

    // Add validator to escrow_users
    sqlx::query("INSERT INTO escrow_users (wallet_address, balance) VALUES ($1, 0.0)")
        .bind(&wallet_address)
        .execute(&db.pool)
        .await
        .expect("Failed to insert escrow user");

    // Prepare delete request
    let payload = json!({
        "wallet_address": wallet_address
    });

    let req = Request::builder()
        .method("DELETE")
        .uri("/validator/profile/delete")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::OK);

    // Verify the validator profile has been deleted
    let profile_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM validator_profiles WHERE wallet_address = $1",
    )
    .bind(&wallet_address)
    .fetch_one(&db.pool)
    .await
    .expect("Failed to check validator profile existence");

    assert_eq!(profile_exists, 0);

    // Verify related data has been cleaned up
    let lang_relations = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM validator_programming_languages WHERE validator_id = $1",
    )
    .bind(validator_id)
    .fetch_one(&db.pool)
    .await
    .expect("Failed to check language relations");

    assert_eq!(lang_relations, 0);

    let expertise_relations = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM validator_expertise WHERE validator_id = $1",
    )
    .bind(validator_id)
    .fetch_one(&db.pool)
    .await
    .expect("Failed to check expertise relations");

    assert_eq!(expertise_relations, 0);

    // Verify escrow user has been removed
    let escrow_exists =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM escrow_users WHERE wallet_address = $1")
            .bind(&wallet_address)
            .fetch_one(&db.pool)
            .await
            .expect("Failed to check escrow user existence");

    assert_eq!(escrow_exists, 0);
}

#[tokio::test]
async fn test_delete_validator_profile_not_found() {
    let app = TestApp::new().await;
    let non_existent_address = generate_address();

    let payload = json!({
        "wallet_address": non_existent_address
    });

    let req = Request::builder()
        .method("DELETE")
        .uri("/validator/profile/delete")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_validator_profile_invalid_address() {
    let app = TestApp::new().await;

    let payload = json!({
        "wallet_address": "invalid_address"
    });

    let req = Request::builder()
        .method("DELETE")
        .uri("/validator/profile/delete")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_delete_validator_profile_missing_wallet_address() {
    let app = TestApp::new().await;

    let payload = json!({
        // Missing wallet_address field
    });

    let req = Request::builder()
        .method("DELETE")
        .uri("/validator/profile/delete")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY); // 422 for missing required field
}
