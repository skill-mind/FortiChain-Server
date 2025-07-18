use crate::helpers::TestApp;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use sqlx::Row;

#[tokio::test]
async fn test_create_project_ok() {
    let app = TestApp::new().await;
    let db = &app.db;

    // Ensure the test user exists in escrow_users
    let owner_address = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(owner_address)
        .execute(&db.pool)
        .await
        .expect("Failed to insert test escrow user");

    // Prepare a valid project creation payload
    let payload = json!({
        "owner_address": owner_address,
        "contract_address": "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd",
        "name": "Test Project",
        "description": "A valid project description for testing.",
        "contact_info": "contact@example.com",
        "supporting_document_path": null,
        "project_logo_path": null,
        "repository_url": null,
        "tags": ["DeFi", "Audit"],
        "bounty_amount": null,
        "bounty_currency": null,
        "bounty_expiry_date": null
    });

    let req = Request::post("/create_project")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::CREATED);

    let row = sqlx::query("SELECT name FROM projects WHERE contract_address = $1")
        .bind("0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd")
        .fetch_one(&db.pool)
        .await
        .expect("Project not found in DB");
    let name: String = row.get("name");
    assert_eq!(name, "Test Project");

    sqlx::query("DELETE FROM projects WHERE contract_address = $1")
        .bind("0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd")
        .execute(&db.pool)
        .await
        .expect("Failed to clean up project");
}

#[tokio::test]
async fn test_create_project_missing_required_field() {
    let app = TestApp::new().await;
    let payload = serde_json::json!({
        // Missing owner_address
        "contract_address": "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd",
        "name": "Test Project",
        "description": "A valid project description for testing.",
        "contact_info": "contact@example.com",
        "tags": ["DeFi"]
    });
    let req = Request::post("/create_project")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_create_project_invalid_address() {
    let app = TestApp::new().await;
    let payload = serde_json::json!({
        "owner_address": "invalid_address",
        "contract_address": "invalid_address",
        "name": "Test Project",
        "description": "A valid project description for testing.",
        "contact_info": "contact@example.com",
        "tags": ["DeFi"]
    });
    let req = Request::post("/create_project")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_project_partial_bounty_fields() {
    let app = TestApp::new().await;
    let owner_address = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(owner_address)
        .execute(&app.db.pool)
        .await
        .expect("Failed to insert test escrow user");
    let payload = serde_json::json!({
        "owner_address": owner_address,
        "contract_address": "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabce",
        "name": "Test Project",
        "description": "A valid project description for testing.",
        "contact_info": "contact@example.com",
        "tags": ["DeFi"],
        "bounty_amount": "1000.00"
        // Missing bounty_currency and bounty_expiry_date
    });
    let req = Request::post("/create_project")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_project_duplicate_contract_address() {
    let app = TestApp::new().await;
    let db = &app.db;
    let owner_address = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(owner_address)
        .execute(&db.pool)
        .await
        .expect("Failed to insert test escrow user");
    let contract_address = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcf";
    let payload = serde_json::json!({
        "owner_address": owner_address,
        "contract_address": contract_address,
        "name": "Test Project",
        "description": "A valid project description for testing.",
        "contact_info": "contact@example.com",
        "tags": ["DeFi"]
    });

    let req1 = Request::post("/create_project")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res1 = app.request(req1).await;
    let status1 = res1.status();

    assert_eq!(status1, StatusCode::CREATED);

    let req2 = Request::post("/create_project")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res2 = app.request(req2).await;
    let status2 = res2.status();

    assert_eq!(status2, StatusCode::CONFLICT);

    sqlx::query("DELETE FROM projects WHERE contract_address = $1")
        .bind(contract_address)
        .execute(&db.pool)
        .await
        .expect("Failed to clean up project");
}

#[tokio::test]
async fn test_create_project_invalid_contact_info() {
    let app = TestApp::new().await;
    let owner_address = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(owner_address)
        .execute(&app.db.pool)
        .await
        .expect("Failed to insert test escrow user");
    let payload = serde_json::json!({
        "owner_address": owner_address,
        "contract_address": "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabdd",
        "name": "Test Project",
        "description": "A valid project description for testing.",
        "contact_info": "not-an-email-or-url",
        "tags": ["DeFi"]
    });
    let req = Request::post("/create_project")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::BAD_REQUEST);
}
