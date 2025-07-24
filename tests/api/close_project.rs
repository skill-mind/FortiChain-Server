use crate::helpers::{TestApp, generate_address};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use bigdecimal::BigDecimal;
use serde_json::json;
use uuid::Uuid;

#[tokio::test]
async fn test_close_project_success() {
    let app = TestApp::new().await;
    let owner_address = generate_address();
    sqlx::query!(
        "INSERT INTO escrow_users (wallet_address) VALUES ($1)",
        owner_address
    )
    .execute(&app.db.pool)
    .await
    .unwrap();

    let project_id = sqlx::query_scalar!(
        r#"
        INSERT INTO projects (
            owner_address, contract_address, name, description, contact_info,
            supporting_document_path, project_logo_path, repository_url,
            bounty_amount, bounty_currency, bounty_expiry_date
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
        ) RETURNING id
        "#,
        owner_address,
        &generate_address(),
        "StarkNet Yield Aggregator",
        "A decentralized protocol for yield farming on the StarkNet ecosystem.",
        "contact@starkyield.com",
        "https://github.com/starkyield/doc.pdf",
        "https://github.com/starkyield/logo.png",
        "https://github.com/starkyield",
        BigDecimal::from(1000),
        "USD",
        chrono::Utc::now() + chrono::Duration::days(30)
    )
    .fetch_one(&app.db.pool)
    .await
    .unwrap();

    let req = Request::post("/closed_project")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "project_id": project_id,
                "owner_address": owner_address
            })
            .to_string(),
        ))
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let project = sqlx::query!("SELECT closed_at FROM projects WHERE id = $1", project_id)
        .fetch_one(&app.db.pool)
        .await
        .unwrap();
    assert!(project.closed_at.is_some());

    let user = sqlx::query!(
        "SELECT balance FROM escrow_users WHERE wallet_address = $1",
        owner_address
    )
    .fetch_one(&app.db.pool)
    .await
    .unwrap();
    assert_eq!(user.balance, BigDecimal::from(1000));
}

#[tokio::test]
async fn test_close_project_with_partial_refund() {
    let app = TestApp::new().await;
    let owner_address = generate_address();

    sqlx::query!(
        "INSERT INTO escrow_users (wallet_address, balance) VALUES ($1, 0)",
        owner_address
    )
    .execute(&app.db.pool)
    .await
    .unwrap();

    let project_id = sqlx::query_scalar!(
        r#"
        INSERT INTO projects (
            owner_address, contract_address, name, description, contact_info,
            supporting_document_path, project_logo_path, repository_url,
            bounty_amount, bounty_currency, bounty_expiry_date
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
        ) RETURNING id
        "#,
        owner_address,
        &generate_address(),
        "StarkNet Yield Aggregator",
        "A decentralized protocol for yield farming on the StarkNet ecosystem.",
        "contact@starkyield.com",
        "https://github.com/starkyield/doc.pdf",
        "https://github.com/starkyield/logo.png",
        "https://github.com/starkyield",
        BigDecimal::from(1000),
        "USD",
        chrono::Utc::now() + chrono::Duration::days(30)
    )
    .fetch_one(&app.db.pool)
    .await
    .unwrap();

    let disbursed_amount = BigDecimal::from(100);
    let disbursed_wallet = &generate_address();

    sqlx::query!(
        "INSERT INTO escrow_users (wallet_address, balance) VALUES ($1, 0)",
        disbursed_wallet
    )
    .execute(&app.db.pool)
    .await
    .unwrap();

    sqlx::query!(
        r#"
        INSERT INTO escrow_transactions (
            wallet_address, project_id, type, amount, currency, status, notes, transaction_hash
        ) VALUES ($1, $2, 'bounty_disbursement', $3, $4, 'completed', $5, $6)
        "#,
        disbursed_wallet,
        project_id,
        disbursed_amount,
        "STRK",
        "Bounty disbursement for completing task X in Project Y",
        &generate_address()
    )
    .execute(&app.db.pool)
    .await
    .unwrap();

    sqlx::query!(
        r#"
        UPDATE escrow_users
        SET balance = balance + $1
        WHERE wallet_address = $2
        "#,
        disbursed_amount,
        disbursed_wallet
    )
    .execute(&app.db.pool)
    .await
    .unwrap();

    sqlx::query!(
        r#"
        UPDATE projects
        SET bounty_amount = bounty_amount - $1
        WHERE id = $2
        "#,
        disbursed_amount,
        project_id
    )
    .execute(&app.db.pool)
    .await
    .unwrap();

    let req = Request::post("/closed_project")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "project_id": project_id,
                "owner_address": owner_address
            })
            .to_string(),
        ))
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let user = sqlx::query!(
        "SELECT balance FROM escrow_users WHERE wallet_address = $1",
        owner_address
    )
    .fetch_one(&app.db.pool)
    .await
    .unwrap();
    assert_eq!(user.balance, BigDecimal::from(900));
}

#[tokio::test]
async fn test_close_project_not_found() {
    let app = TestApp::new().await;
    let owner_address = generate_address();
    let invalid_project_id = Uuid::now_v7();

    let req = Request::post("/closed_project")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "project_id": invalid_project_id,
                "owner_address": owner_address
            })
            .to_string(),
        ))
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_close_project_unauthorized() {
    let app = TestApp::new().await;
    let owner_address = generate_address();
    let attacker_address = generate_address();

    sqlx::query!(
        "INSERT INTO escrow_users (wallet_address) VALUES ($1)",
        owner_address
    )
    .execute(&app.db.pool)
    .await
    .unwrap();

    let project_id = sqlx::query_scalar!(
        r#"
        INSERT INTO projects (
            owner_address, contract_address, name, description, contact_info,
            supporting_document_path, project_logo_path, repository_url,
            bounty_amount, bounty_currency, bounty_expiry_date
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
        ) RETURNING id
        "#,
        owner_address,
        &generate_address(),
        "StarkNet Yield Aggregator",
        "A decentralized protocol for yield farming on the StarkNet ecosystem.",
        "contact@starkyield.com",
        "https://github.com/starkyield/doc.pdf",
        "https://github.com/starkyield/logo.png",
        "https://github.com/starkyield",
        BigDecimal::from(1000),
        "USD",
        chrono::Utc::now() + chrono::Duration::days(30)
    )
    .fetch_one(&app.db.pool)
    .await
    .unwrap();

    let req = Request::post("/closed_project")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "project_id": project_id,
                "owner_address": attacker_address
            })
            .to_string(),
        ))
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_close_project_already_closed() {
    let app = TestApp::new().await;
    let owner_address = generate_address();

    sqlx::query!(
        "INSERT INTO escrow_users (wallet_address) VALUES ($1)",
        owner_address
    )
    .execute(&app.db.pool)
    .await
    .unwrap();

    let project_id = sqlx::query_scalar!(
        r#"
        INSERT INTO projects (
            owner_address, contract_address, name, description, contact_info,
            supporting_document_path, project_logo_path, repository_url,
            bounty_amount, bounty_currency, bounty_expiry_date, closed_at
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
        ) RETURNING id
        "#,
        owner_address,
        &generate_address(),
        "StarkNet Yield Aggregator",
        "A decentralized protocol for yield farming on the StarkNet ecosystem.",
        "contact@starkyield.com",
        "https://github.com/starkyield/doc.pdf",
        "https://github.com/starkyield/logo.png",
        "https://github.com/starkyield",
        BigDecimal::from(1000),
        "USD",
        chrono::Utc::now() + chrono::Duration::days(30),
        chrono::Utc::now()
    )
    .fetch_one(&app.db.pool)
    .await
    .unwrap();

    let req = Request::post("/closed_project")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "project_id": project_id,
                "owner_address": owner_address
            })
            .to_string(),
        ))
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_close_project_user_not_exists() {
    let app = TestApp::new().await;
    let owner_address = generate_address();
    let non_existent_user = generate_address();

    let project_id = sqlx::query_scalar!(
        r#"
            INSERT INTO projects (
                owner_address, contract_address, name, description, contact_info,
                supporting_document_path, project_logo_path, repository_url,
                bounty_amount, bounty_currency, bounty_expiry_date
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            ) RETURNING id
            "#,
        owner_address,
        &generate_address(),
        "StarkNet Yield Aggregator",
        "A decentralized protocol for yield farming on the StarkNet ecosystem.",
        "contact@starkyield.com",
        "https://github.com/starkyield/doc.pdf",
        "https://github.com/starkyield/logo.png",
        "https://github.com/starkyield",
        BigDecimal::from(1000),
        "USD",
        chrono::Utc::now() + chrono::Duration::days(30)
    )
    .fetch_one(&app.db.pool)
    .await
    .unwrap();

    let req = Request::post("/closed_project")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "project_id": project_id,
                "owner_address": non_existent_user
            })
            .to_string(),
        ))
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_close_project_no_refund_needed() {
    let app = TestApp::new().await;
    let owner_address = generate_address();

    sqlx::query!(
        "INSERT INTO escrow_users (wallet_address, balance) VALUES ($1, 0)",
        owner_address
    )
    .execute(&app.db.pool)
    .await
    .unwrap();

    let project_id = sqlx::query_scalar!(
        r#"
            INSERT INTO projects (
                owner_address, contract_address, name, description, contact_info,
                supporting_document_path, project_logo_path, repository_url,
                bounty_amount, bounty_currency, bounty_expiry_date
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            ) RETURNING id
            "#,
        owner_address,
        &generate_address(),
        "StarkNet Yield Aggregator",
        "A decentralized protocol for yield farming on the StarkNet ecosystem.",
        "contact@starkyield.com",
        "https://github.com/starkyield/doc.pdf",
        "https://github.com/starkyield/logo.png",
        "https://github.com/starkyield",
        BigDecimal::from(1000),
        "USD",
        chrono::Utc::now() + chrono::Duration::days(30)
    )
    .fetch_one(&app.db.pool)
    .await
    .unwrap();

    let disbursed_amount = BigDecimal::from(1000);
    let disbursed_wallet = &generate_address();

    sqlx::query!(
        "INSERT INTO escrow_users (wallet_address, balance) VALUES ($1, 0)",
        disbursed_wallet
    )
    .execute(&app.db.pool)
    .await
    .unwrap();

    sqlx::query!(
        r#"
        INSERT INTO escrow_transactions (
            wallet_address, project_id, type, amount, currency, status, notes, transaction_hash
        ) VALUES ($1, $2, 'bounty_disbursement', $3, $4, 'completed', $5, $6)
        "#,
        disbursed_wallet,
        project_id,
        disbursed_amount,
        "STRK",
        "Bounty disbursement for completing task X in Project Y",
        &generate_address()
    )
    .execute(&app.db.pool)
    .await
    .unwrap();

    sqlx::query!(
        r#"
        UPDATE escrow_users
        SET balance = balance + $1
        WHERE wallet_address = $2
        "#,
        disbursed_amount,
        disbursed_wallet
    )
    .execute(&app.db.pool)
    .await
    .unwrap();

    sqlx::query!(
        r#"
        UPDATE projects
        SET bounty_amount = bounty_amount - $1
        WHERE id = $2
        "#,
        disbursed_amount,
        project_id
    )
    .execute(&app.db.pool)
    .await
    .unwrap();

    let req = Request::post("/closed_project")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "project_id": project_id,
                "owner_address": owner_address
            })
            .to_string(),
        ))
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let user = sqlx::query!(
        "SELECT balance FROM escrow_users WHERE wallet_address = $1",
        owner_address
    )
    .fetch_one(&app.db.pool)
    .await
    .unwrap();
    assert_eq!(user.balance, BigDecimal::from(0));
}
