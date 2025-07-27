use crate::helpers::{TestApp, generate_address};
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use bigdecimal::BigDecimal;
use uuid::Uuid;

async fn create_test_project(
    app: &TestApp,
    name: &str,
    owner_address: &str,
    bounty_amount: Option<BigDecimal>,
    closed_at: Option<chrono::DateTime<chrono::Utc>>,
    tags: Vec<&str>,
) -> Uuid {
    sqlx::query!(
        "INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT (wallet_address) DO NOTHING",
        owner_address
    )
    .execute(&app.db.pool)
    .await
    .unwrap();

    let project_id = sqlx::query_scalar!(
        r#"
        INSERT INTO projects (
            owner_address, contract_address, name, description, contact_info,
            bounty_amount, bounty_currency, bounty_expiry_date, closed_at
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9
        ) RETURNING id
        "#,
        owner_address,
        &generate_address(),
        name,
        "A test project description that meets the minimum length requirement.",
        "contact@example.com",
        bounty_amount,
        if bounty_amount.is_some() { Some("STRK") } else { None },
        if bounty_amount.is_some() { Some(chrono::Utc::now() + chrono::Duration::days(30)) } else { None },
        closed_at
    )
    .fetch_one(&app.db.pool)
    .await
    .unwrap();

    // Add tags if provided
    if !tags.is_empty() {
        for tag in tags {
            let tag_id = sqlx::query_scalar!(
                "INSERT INTO tags (name) VALUES ($1) ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name RETURNING id",
                tag
            )
            .fetch_one(&app.db.pool)
            .await
            .unwrap();

            sqlx::query!(
                "INSERT INTO project_tags (project_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                project_id,
                tag_id
            )
            .execute(&app.db.pool)
            .await
            .unwrap();
        }
    }

    project_id
}

#[tokio::test]
async fn test_list_projects_success() {
    let app = TestApp::new().await;
    let owner1 = generate_address();
    let owner2 = generate_address();

    // Create test projects
    create_test_project(&app, "Project Alpha", &owner1, Some(BigDecimal::from(1000)), None, vec!["DeFi", "Audit"]).await;
    create_test_project(&app, "Project Beta", &owner2, None, None, vec!["NFT"]).await;
    create_test_project(&app, "Project Gamma", &owner1, Some(BigDecimal::from(500)), Some(chrono::Utc::now()), vec![]).await;

    let req = Request::get("/projects")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response["projects"].as_array().unwrap().len(), 3);
    assert_eq!(response["total_count"], 3);
    assert_eq!(response["has_next"], false);

    // Verify projects are sorted by created_at desc by default
    let projects = response["projects"].as_array().unwrap();
    let project_names: Vec<&str> = projects
        .iter()
        .map(|p| p["name"].as_str().unwrap())
        .collect();
    
    // The most recently created should be first
    assert_eq!(project_names[0], "Project Gamma");
}

#[tokio::test]
async fn test_list_projects_filter_by_owner() {
    let app = TestApp::new().await;
    let owner1 = generate_address();
    let owner2 = generate_address();

    create_test_project(&app, "Project Alpha", &owner1, None, None, vec![]).await;
    create_test_project(&app, "Project Beta", &owner2, None, None, vec![]).await;
    create_test_project(&app, "Project Gamma", &owner1, None, None, vec![]).await;

    let req = Request::get(&format!("/projects?owner_address={}", owner1))
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response["projects"].as_array().unwrap().len(), 2);
    assert_eq!(response["total_count"], 2);

    // Verify all projects belong to owner1
    for project in response["projects"].as_array().unwrap() {
        assert_eq!(project["owner_address"], owner1);
    }
}

#[tokio::test]
async fn test_list_projects_filter_active_only() {
    let app = TestApp::new().await;
    let owner = generate_address();

    create_test_project(&app, "Active Project", &owner, None, None, vec![]).await;
    create_test_project(&app, "Closed Project", &owner, None, Some(chrono::Utc::now()), vec![]).await;

    let req = Request::get("/projects?active_only=true")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response["projects"].as_array().unwrap().len(), 1);
    assert_eq!(response["total_count"], 1);
    assert_eq!(response["projects"][0]["name"], "Active Project");
    assert!(response["projects"][0]["closed_at"].is_null());
}

#[tokio::test]
async fn test_list_projects_filter_has_bounty() {
    let app = TestApp::new().await;
    let owner = generate_address();

    create_test_project(&app, "Project with Bounty", &owner, Some(BigDecimal::from(1000)), None, vec![]).await;
    create_test_project(&app, "Project without Bounty", &owner, None, None, vec![]).await;
    create_test_project(&app, "Closed Project with Bounty", &owner, Some(BigDecimal::from(500)), Some(chrono::Utc::now()), vec![]).await;

    let req = Request::get("/projects?has_bounty=true")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Should only return active projects with bounties
    assert_eq!(response["projects"].as_array().unwrap().len(), 1);
    assert_eq!(response["total_count"], 1);
    assert_eq!(response["projects"][0]["name"], "Project with Bounty");
    assert!(!response["projects"][0]["bounty_amount"].is_null());
    assert!(response["projects"][0]["closed_at"].is_null());
}

#[tokio::test]
async fn test_list_projects_sort_by_name() {
    let app = TestApp::new().await;
    let owner = generate_address();

    create_test_project(&app, "Zebra Project", &owner, None, None, vec![]).await;
    create_test_project(&app, "Alpha Project", &owner, None, None, vec![]).await;
    create_test_project(&app, "Beta Project", &owner, None, None, vec![]).await;

    let req = Request::get("/projects?sort_by=name&sort_order=asc")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let project_names: Vec<&str> = response["projects"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["name"].as_str().unwrap())
        .collect();

    assert_eq!(project_names, vec!["Alpha Project", "Beta Project", "Zebra Project"]);
}

#[tokio::test]
async fn test_list_projects_sort_by_bounty_amount() {
    let app = TestApp::new().await;
    let owner = generate_address();

    create_test_project(&app, "High Bounty", &owner, Some(BigDecimal::from(2000)), None, vec![]).await;
    create_test_project(&app, "Low Bounty", &owner, Some(BigDecimal::from(500)), None, vec![]).await;
    create_test_project(&app, "No Bounty", &owner, None, None, vec![]).await;

    let req = Request::get("/projects?sort_by=bounty_amount&sort_order=desc")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let project_names: Vec<&str> = response["projects"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["name"].as_str().unwrap())
        .collect();

    // Projects with bounties should come first, sorted by amount desc, then null bounties
    assert_eq!(project_names[0], "High Bounty");
    assert_eq!(project_names[1], "Low Bounty");
    assert_eq!(project_names[2], "No Bounty");
}

#[tokio::test]
async fn test_list_projects_pagination() {
    let app = TestApp::new().await;
    let owner = generate_address();

    // Create 5 projects
    for i in 1..=5 {
        create_test_project(&app, &format!("Project {}", i), &owner, None, None, vec![]).await;
    }

    // Test first page
    let req = Request::get("/projects?limit=2&offset=0")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response["projects"].as_array().unwrap().len(), 2);
    assert_eq!(response["total_count"], 5);
    assert_eq!(response["has_next"], true);

    // Test second page
    let req = Request::get("/projects?limit=2&offset=2")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response["projects"].as_array().unwrap().len(), 2);
    assert_eq!(response["total_count"], 5);
    assert_eq!(response["has_next"], true);

    // Test last page
    let req = Request::get("/projects?limit=2&offset=4")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response["projects"].as_array().unwrap().len(), 1);
    assert_eq!(response["total_count"], 5);
    assert_eq!(response["has_next"], false);
}

#[tokio::test]
async fn test_list_projects_validation_errors() {
    let app = TestApp::new().await;

    // Test invalid owner address
    let req = Request::get("/projects?owner_address=invalid")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    // Test invalid sort_by
    let req = Request::get("/projects?sort_by=invalid_field")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    // Test invalid sort_order
    let req = Request::get("/projects?sort_order=invalid")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    // Test limit too high
    let req = Request::get("/projects?limit=25")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    // Test limit too low
    let req = Request::get("/projects?limit=0")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_list_projects_includes_tags() {
    let app = TestApp::new().await;
    let owner = generate_address();

    create_test_project(&app, "Tagged Project", &owner, None, None, vec!["DeFi", "Security", "StarkNet"]).await;
    create_test_project(&app, "Untagged Project", &owner, None, None, vec![]).await;

    let req = Request::get("/projects")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let projects = response["projects"].as_array().unwrap();
    
    // Find the tagged project
    let tagged_project = projects
        .iter()
        .find(|p| p["name"] == "Tagged Project")
        .unwrap();
    
    let tags = tagged_project["tags"].as_array().unwrap();
    assert_eq!(tags.len(), 3);
    
    let tag_names: Vec<&str> = tags
        .iter()
        .map(|t| t.as_str().unwrap())
        .collect();
    
    // Tags should be sorted alphabetically
    assert_eq!(tag_names, vec!["DeFi", "Security", "StarkNet"]);

    // Find the untagged project
    let untagged_project = projects
        .iter()
        .find(|p| p["name"] == "Untagged Project")
        .unwrap();
    
    let empty_tags = untagged_project["tags"].as_array().unwrap();
    assert_eq!(empty_tags.len(), 0);
}

#[tokio::test]
async fn test_list_projects_empty_result() {
    let app = TestApp::new().await;

    let req = Request::get("/projects")
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response["projects"].as_array().unwrap().len(), 0);
    assert_eq!(response["total_count"], 0);
    assert_eq!(response["has_next"], false);
}

#[tokio::test]
async fn test_list_projects_combined_filters() {
    let app = TestApp::new().await;
    let owner1 = generate_address();
    let owner2 = generate_address();

    // Create test scenarios
    create_test_project(&app, "Owner1 Active Bounty", &owner1, Some(BigDecimal::from(1000)), None, vec![]).await;
    create_test_project(&app, "Owner1 Active No Bounty", &owner1, None, None, vec![]).await;
    create_test_project(&app, "Owner1 Closed Bounty", &owner1, Some(BigDecimal::from(500)), Some(chrono::Utc::now()), vec![]).await;
    create_test_project(&app, "Owner2 Active Bounty", &owner2, Some(BigDecimal::from(2000)), None, vec![]).await;

    // Test owner + has_bounty filters
    let req = Request::get(&format!("/projects?owner_address={}&has_bounty=true", owner1))
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response["projects"].as_array().unwrap().len(), 1);
    assert_eq!(response["total_count"], 1);
    assert_eq!(response["projects"][0]["name"], "Owner1 Active Bounty");
    assert_eq!(response["projects"][0]["owner_address"], owner1);
    assert!(!response["projects"][0]["bounty_amount"].is_null());
    assert!(response["projects"][0]["closed_at"].is_null());
}
