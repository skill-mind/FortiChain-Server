use crate::helpers::TestApp;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use sqlx::{PgPool, Row};
use time::{Duration, OffsetDateTime};

// Helpers
async fn create_project(db: &PgPool) -> Result<String, sqlx::Error> {
    let owner_address = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let contract_address = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let name = "Test Project";
    let description = "This is just a test project";
    let contact_info = "tester@tester.test";
    let supporting_document_path = "https://path-to-docs.document";
    let project_logo_path = "https://path-to-logo.document";
    let repository_url = "https://path-to-repo.document";
    let bounty_amount = 1000000000;
    let bounty_currency = "ETH";
    let bounty_expiry_date = OffsetDateTime::now_utc() + Duration::hours(1);

    let query = r#"
        INSERT INTO projects (
            owner_address, contract_address, name, description, contact_info,
            supporting_document_path, project_logo_path, repository_url,
            bounty_amount, bounty_currency, bounty_expiry_date
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id::text
    "#;
    let project = sqlx::query(&query)
        .bind(owner_address)
        .bind(contract_address)
        .bind(name)
        .bind(description)
        .bind(contact_info)
        .bind(supporting_document_path)
        .bind(project_logo_path)
        .bind(repository_url)
        .bind(bounty_amount)
        .bind(bounty_currency)
        .bind(bounty_expiry_date)
        .fetch_one(db)
        .await?;

    let id: String = project.try_get("id")?;
    Ok(id)
}
#[tokio::test]
async fn test_deposit_successful_with_no_escrow_users() {}

#[tokio::test]
async fn test_deposit_successful_with_escrow_users_available() {
    let app = TestApp::new().await;
    let db = &app.db;
    // Create escrow account
    let wallet = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabca";
    sqlx::query("INSERT INTO escrow_users (wallet_address) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(wallet)
        .execute(&db.pool)
        .await
        .expect("Failed to create escrow account");

    // Create the project
    let project_id = create_project(&db.pool)
        .await
        .expect("Failed to create project");
    let payload = json!({
        "wallet_address": wallet,
        "project_id": project_id,
        "amount": 10000000,
        "currency": "ETH",
        "notes": Some("Test project funding"),
        "transaction_hash": "tx_hash"
    });
    let request = Request::post("/deposit")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let res = app.request(request).await;
    assert_eq!(res.status(), StatusCode::CREATED);
}
