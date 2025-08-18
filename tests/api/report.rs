use crate::helpers::{TestApp, generate_address};
use axum::{body::Body, extract::Request, http::StatusCode};
use serde_json::json;
use uuid::Uuid;

#[tokio::test]
async fn test_reject_report_success() {
    let app = TestApp::new().await;
    let db = &app.db;

    // Create a test project first (using correct schema fields)
    let project_id = Uuid::now_v7();
    let project_wallet = generate_address();

    sqlx::query(
        r#"
        INSERT INTO projects (
            id, name, description, contract_address, owner_address, contact_info, created_at
        ) VALUES (
            $1, 'Test Project', 'A test project for reports', $2, $2, 'test@example.com', now()
        )
        "#,
    )
    .bind(project_id)
    .bind(&project_wallet)
    .execute(&db.pool)
    .await
    .expect("Failed to insert test project");

    // Create a test report
    let report_id = Uuid::now_v7();
    let researcher_wallet = generate_address();
    let validator_wallet = generate_address();

    sqlx::query(
        r#"
        INSERT INTO research_report (
            id, title, project_id, body, reported_by, status, created_at
        ) VALUES (
            $1, 'Test Report', $2, 'This is a test report body with sufficient content to meet the minimum requirement of 50 characters.', $3, 'submitted', now()
        )
        "#,
    )
    .bind(report_id)
    .bind(project_id)
    .bind(&researcher_wallet)
    .execute(&db.pool)
    .await
    .expect("Failed to insert test report");

    // Prepare rejection request
    let payload = json!({
        "report_id": report_id,
        "reason": "incomplete_information",
        "validator_notes": "The report lacks sufficient technical details to assess the vulnerability.",
        "validated_by": validator_wallet
    });

    let req = Request::builder()
        .method("POST")
        .uri("/report/reject")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::OK);

    // Verify the report has been rejected
    let report_status =
        sqlx::query_scalar::<_, String>("SELECT status::text FROM research_report WHERE id = $1")
            .bind(report_id)
            .fetch_one(&db.pool)
            .await
            .expect("Failed to check report status");

    assert_eq!(report_status, "rejected");

    // Verify the rejection details
    let report = sqlx::query!(
        r#"
        SELECT reason::text as reason, validator_notes, validated_by 
        FROM research_report 
        WHERE id = $1
        "#,
        report_id
    )
    .fetch_one(&db.pool)
    .await
    .expect("Failed to fetch rejected report");

    assert_eq!(report.reason, Some("incomplete_information".to_string()));
    assert_eq!(
        report.validator_notes,
        Some(
            "The report lacks sufficient technical details to assess the vulnerability."
                .to_string()
        )
    );
    assert_eq!(report.validated_by, Some(validator_wallet));
}

#[tokio::test]
async fn test_reject_report_not_found() {
    let app = TestApp::new().await;
    let non_existent_report_id = Uuid::now_v7();
    let validator_wallet = generate_address();

    let payload = json!({
        "report_id": non_existent_report_id,
        "reason": "duplicate_report",
        "validator_notes": "This report duplicates an existing finding.",
        "validated_by": validator_wallet
    });

    let req = Request::builder()
        .method("POST")
        .uri("/report/reject")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_reject_report_already_rejected() {
    let app = TestApp::new().await;
    let db = &app.db;

    // Create a test project (using correct schema fields)
    let project_id = Uuid::now_v7();
    let project_wallet = generate_address();

    sqlx::query(
        r#"
        INSERT INTO projects (
            id, name, description, contract_address, owner_address, contact_info, created_at
        ) VALUES (
            $1, 'Test Project', 'A test project for reports', $2, $2, 'test@example.com', now()
        )
        "#,
    )
    .bind(project_id)
    .bind(&project_wallet)
    .execute(&db.pool)
    .await
    .expect("Failed to insert test project");

    // Create an already rejected report
    let report_id = Uuid::now_v7();
    let researcher_wallet = generate_address();
    let validator_wallet = generate_address();

    sqlx::query(
        r#"
        INSERT INTO research_report (
            id, title, project_id, body, reported_by, status, reason, 
            validator_notes, validated_by, created_at
        ) VALUES (
            $1, 'Already Rejected Report', $2, 'This report was already rejected. It contains sufficient content to meet the minimum requirement of 50 characters for the body field.', $3, 'rejected', 'duplicate_report', 'Already rejected', $4, now()
        )
        "#,
    )
    .bind(report_id)
    .bind(project_id)
    .bind(&researcher_wallet)
    .bind(&validator_wallet)
    .execute(&db.pool)
    .await
    .expect("Failed to insert already rejected report");

    // Try to reject it again
    let payload = json!({
        "report_id": report_id,
        "reason": "out_of_scope",
        "validator_notes": "Trying to reject again.",
        "validated_by": validator_wallet
    });

    let req = Request::builder()
        .method("POST")
        .uri("/report/reject")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_reject_report_unauthorized_validator() {
    let app = TestApp::new().await;
    let db = &app.db;

    // Create a test project (using correct schema fields)
    let project_id = Uuid::now_v7();
    let project_wallet = generate_address();

    sqlx::query(
        r#"
        INSERT INTO projects (
            id, name, description, contract_address, owner_address, contact_info, created_at
        ) VALUES (
            $1, 'Test Project', 'A test project for reports', $2, $2, 'test@example.com', now()
        )
        "#,
    )
    .bind(project_id)
    .bind(&project_wallet)
    .execute(&db.pool)
    .await
    .expect("Failed to insert test project");

    // Create a report assigned to a specific validator
    let report_id = Uuid::now_v7();
    let researcher_wallet = generate_address();
    let assigned_validator = generate_address();
    let unauthorized_validator = generate_address();

    sqlx::query(
        r#"
        INSERT INTO research_report (
            id, title, project_id, body, reported_by, status, validated_by, created_at
        ) VALUES (
            $1, 'Assigned Report', $2, 'This report is assigned to a specific validator. It contains sufficient content to meet the minimum requirement of 50 characters for the body field.', $3, 'assigned', $4, now()
        )
        "#,
    )
    .bind(report_id)
    .bind(project_id)
    .bind(&researcher_wallet)
    .bind(&assigned_validator)
    .execute(&db.pool)
    .await
    .expect("Failed to insert assigned report");

    // Try to reject with unauthorized validator
    let payload = json!({
        "report_id": report_id,
        "reason": "already_known",
        "validator_notes": "Unauthorized rejection attempt.",
        "validated_by": unauthorized_validator
    });

    let req = Request::builder()
        .method("POST")
        .uri("/report/reject")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_reject_report_invalid_reason() {
    let app = TestApp::new().await;
    let db = &app.db;

    // Create a test project (using correct schema fields)
    let project_id = Uuid::now_v7();
    let project_wallet = generate_address();

    sqlx::query(
        r#"
        INSERT INTO projects (
            id, name, description, contract_address, owner_address, contact_info, created_at
        ) VALUES (
            $1, 'Test Project', 'A test project for reports', $2, $2, 'test@example.com', now()
        )
        "#,
    )
    .bind(project_id)
    .bind(&project_wallet)
    .execute(&db.pool)
    .await
    .expect("Failed to insert test project");

    // Create a test report
    let report_id = Uuid::now_v7();
    let researcher_wallet = generate_address();
    let validator_wallet = generate_address();

    sqlx::query(
        r#"
        INSERT INTO research_report (
            id, title, project_id, body, reported_by, status, created_at
        ) VALUES (
            $1, 'Test Report', $2, 'This is a test report body with sufficient content.', $3, 'submitted', now()
        )
        "#,
    )
    .bind(report_id)
    .bind(project_id)
    .bind(&researcher_wallet)
    .execute(&db.pool)
    .await
    .expect("Failed to insert test report");

    // Try to reject with invalid reason
    let payload = json!({
        "report_id": report_id,
        "reason": "invalid_reason",
        "validator_notes": "Invalid reason test.",
        "validated_by": validator_wallet
    });

    let req = Request::builder()
        .method("POST")
        .uri("/report/reject")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_reject_report_invalid_validator_address() {
    let app = TestApp::new().await;
    let report_id = Uuid::now_v7();

    let payload = json!({
        "report_id": report_id,
        "reason": "duplicate_report",
        "validator_notes": "Test with invalid address.",
        "validated_by": "invalid_address"
    });

    let req = Request::builder()
        .method("POST")
        .uri("/report/reject")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_reject_report_missing_fields() {
    let app = TestApp::new().await;

    let payload = json!({
        // Missing required fields
        "validator_notes": "Test missing fields."
    });

    let req = Request::builder()
        .method("POST")
        .uri("/report/reject")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_reject_report_without_notes() {
    let app = TestApp::new().await;
    let db = &app.db;

    // Create a test project (using correct schema fields)
    let project_id = Uuid::now_v7();
    let project_wallet = generate_address();

    sqlx::query(
        r#"
        INSERT INTO projects (
            id, name, description, contract_address, owner_address, contact_info, created_at
        ) VALUES (
            $1, 'Test Project', 'A test project for reports', $2, $2, 'test@example.com', now()
        )
        "#,
    )
    .bind(project_id)
    .bind(&project_wallet)
    .execute(&db.pool)
    .await
    .expect("Failed to insert test project");

    // Create a test report
    let report_id = Uuid::now_v7();
    let researcher_wallet = generate_address();
    let validator_wallet = generate_address();

    sqlx::query(
        r#"
        INSERT INTO research_report (
            id, title, project_id, body, reported_by, status, created_at
        ) VALUES (
            $1, 'Test Report', $2, 'This is a test report body with sufficient content.', $3, 'submitted', now()
        )
        "#,
    )
    .bind(report_id)
    .bind(project_id)
    .bind(&researcher_wallet)
    .execute(&db.pool)
    .await
    .expect("Failed to insert test report");

    // Reject without validator notes
    let payload = json!({
        "report_id": report_id,
        "reason": "out_of_scope",
        "validated_by": validator_wallet
    });

    let req = Request::builder()
        .method("POST")
        .uri("/report/reject")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.request(req).await;
    let status = res.status();

    assert_eq!(status, StatusCode::OK);

    // Verify the report was rejected without notes
    let report = sqlx::query!(
        r#"
        SELECT status::text as status, reason::text as reason, validator_notes, validated_by 
        FROM research_report 
        WHERE id = $1
        "#,
        report_id
    )
    .fetch_one(&db.pool)
    .await
    .expect("Failed to fetch rejected report");

    assert_eq!(report.status, Some("rejected".to_string()));
    assert_eq!(report.reason, Some("out_of_scope".to_string()));
    assert_eq!(report.validator_notes, None);
    assert_eq!(report.validated_by, Some(validator_wallet));
}
