use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde_json::json;
use sqlx::Row;
use uuid::Uuid;

use crate::helpers::{TestApp, generate_address};

#[tokio::test]
async fn test_get_project_success() {
    let app = TestApp::new().await;

    let project_id = Uuid::now_v7();
    let name = "Test Project";
    let owner_address = generate_address();
    let contract_address = generate_address();
    let description = "A test project for verification";
    let contact_info = "test@example.com";

    sqlx::query(
        r#"
        INSERT INTO projects (id, name, owner_address, contract_address, description, contact_info)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(project_id)
    .bind(name)
    .bind(&owner_address)
    .bind(&contract_address)
    .bind(description)
    .bind(contact_info)
    .execute(&app.db.pool)
    .await
    .expect("Failed to insert test project");

    let req = Request::get(&format!("/projects/{}", project_id))
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let project: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(project["id"], project_id.to_string());
    assert_eq!(project["name"], name);
    assert_eq!(project["owner_address"], owner_address);
    assert_eq!(project["contract_address"], contract_address);
    assert_eq!(project["description"], description);
}

#[tokio::test]
async fn test_get_project_not_found() {
    // TestApp sets up a clean, migrated database.
    let app = TestApp::new().await;

    // Test getting a non-existent project
    let non_existent_id = Uuid::now_v7();
    let req = Request::get(&format!("/projects/{}", non_existent_id))
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_verify_project_success() {
    let app = TestApp::new().await;

    let project_id = Uuid::now_v7();
    let owner_address = generate_address();
    let contract_address = generate_address();
    let contact_info = "test@example.com";

    sqlx::query(
        r#"
        INSERT INTO projects (id, name, owner_address, contract_address, description, contact_info)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(project_id)
    .bind("Test Project")
    .bind(&owner_address)
    .bind(&contract_address)
    .bind("A test project for verification")
    .bind(contact_info)
    .execute(&app.db.pool)
    .await
    .expect("Failed to insert test project");

    let verify_request = json!({
        "repository_url": "https://github.com/test/repo",
        "owner_address": owner_address
    });

    let req = Request::post(&format!("/projects/{}/verify", project_id))
        .header("content-type", "application/json")
        .body(Body::from(verify_request.to_string()))
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let updated_project =
        sqlx::query("SELECT is_verified, repository_url FROM projects WHERE id = $1")
            .bind(project_id)
            .fetch_one(&app.db.pool)
            .await
            .expect("Failed to fetch updated project");

    let is_verified: bool = updated_project.get("is_verified");
    let repository_url: Option<String> = updated_project.get("repository_url");

    assert!(is_verified);
    assert_eq!(
        repository_url,
        Some("https://github.com/test/repo".to_string())
    );
}

#[tokio::test]
async fn test_verify_project_invalid_repository_url() {
    let app = TestApp::new().await;
    let project_id = Uuid::now_v7();
    let owner_address = generate_address();

    let verify_request = json!({
        "repository_url": "invalid-url",
        "owner_address": owner_address
    });

    let req = Request::post(&format!("/projects/{}/verify", project_id))
        .header("content-type", "application/json")
        .body(Body::from(verify_request.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_verify_project_invalid_owner_address() {
    let app = TestApp::new().await;
    let project_id = Uuid::now_v7();

    // Test with invalid owner address
    let verify_request = json!({
        "repository_url": "https://github.com/test/repo",
        "owner_address": "invalid-address"
    });

    let req = Request::post(&format!("/projects/{}/verify", project_id))
        .header("content-type", "application/json")
        .body(Body::from(verify_request.to_string()))
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_verify_project_not_owner() {
    let app = TestApp::new().await;

    let project_id = Uuid::now_v7();
    let owner_address = generate_address();
    let different_address = generate_address();
    let contract_address = generate_address();
    let contact_info = "test@example.com";

    sqlx::query(
        r#"
        INSERT INTO projects (id, name, owner_address, contract_address, description, contact_info)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(project_id)
    .bind("Test Project")
    .bind(&owner_address)
    .bind(&contract_address)
    .bind("A test project for verification")
    .bind(contact_info)
    .execute(&app.db.pool)
    .await
    .expect("Failed to insert test project");

    let verify_request = json!({
        "repository_url": "https://github.com/test/repo",
        "owner_address": different_address
    });

    let req = Request::post(&format!("/projects/{}/verify", project_id))
        .header("content-type", "application/json")
        .body(Body::from(verify_request.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_verify_project_already_verified() {
    let app = TestApp::new().await;

    let project_id = Uuid::now_v7();
    let owner_address = generate_address();
    let contract_address = generate_address();
    let contact_info = "test@example.com";

    sqlx::query(
        r#"
        INSERT INTO projects (id, name, owner_address, contract_address, description, contact_info, is_verified, verification_date, repository_url)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(project_id)
    .bind("Test Project")
    .bind(&owner_address)
    .bind(&contract_address)
    .bind("A test project for verification")
    .bind(contact_info)
    .bind(true)
    .bind(chrono::Utc::now())
    .bind("https://github.com/test/already-verified")
    .execute(&app.db.pool)
    .await
    .expect("Failed to insert test project");

    let verify_request = json!({
        "repository_url": "https://github.com/test/repo",
        "owner_address": owner_address
    });

    let req = Request::post(&format!("/projects/{}/verify", project_id))
        .header("content-type", "application/json")
        .body(Body::from(verify_request.to_string()))
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
