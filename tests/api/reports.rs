use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde_json::json;
use uuid::Uuid;

use crate::helpers::TestApp;
use fortichain_server::db::DbPool;

const MAX_BODY_SIZE: usize = 1024 * 1024; // 1MB

#[tokio::test]
async fn test_list_reports() {
    let app = TestApp::new().await;
    let db = &app.db;

    // Create test data
    let (project_id, report_ids) = match db.pool() {
        DbPool::Sqlite(pool) => {
            // Create a test project
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS projects (
                    id TEXT PRIMARY KEY,
                    owner_address TEXT NOT NULL,
                    contract_address TEXT NOT NULL UNIQUE,
                    name TEXT NOT NULL,
                    description TEXT NOT NULL,
                    contact_info TEXT NOT NULL
                )"
            )
            .execute(pool)
            .await
            .expect("Failed to create projects table");

            sqlx::query(
                "CREATE TABLE IF NOT EXISTS research_report (
                    id TEXT PRIMARY KEY,
                    title TEXT NOT NULL,
                    project_id TEXT NOT NULL,
                    body TEXT NOT NULL,
                    reported_by TEXT NOT NULL,
                    validated_by TEXT,
                    status TEXT NOT NULL,
                    severity TEXT,
                    allocated_reward REAL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME,
                    FOREIGN KEY (project_id) REFERENCES projects (id)
                )"
            )
            .execute(pool)
            .await
            .expect("Failed to create research_report table");

            let project_id = Uuid::new_v4();
            let report_ids = vec![
                Uuid::new_v4(), // submitted report
                Uuid::new_v4(), // validated report
                Uuid::new_v4(), // accepted report with reward
            ];

            // Create a test project
            sqlx::query(
                "INSERT INTO projects (id, owner_address, contract_address, name, description, contact_info) 
                 VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind(project_id.to_string())
            .bind("0x1234567890123456789012345678901234567890123456789012345678901234")
            .bind("0x2234567890123456789012345678901234567890123456789012345678901234")
            .bind("Test Project")
            .bind("Test Description")
            .bind("test@example.com")
            .execute(pool)
            .await
            .expect("Failed to create test project");

            // Create test reports with different statuses and metadata
            // 1. Submitted report
            sqlx::query(
                "INSERT INTO research_report (id, title, project_id, body, reported_by, status, severity) 
                 VALUES (?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(report_ids[0].to_string())
            .bind("Test Report 1")
            .bind(project_id.to_string())
            .bind("Test report body 1")
            .bind("0x3234567890123456789012345678901234567890123456789012345678901234")
            .bind("submitted")
            .bind("low")
            .execute(pool)
            .await
            .expect("Failed to create test report 1");

            // 2. Validated report
            sqlx::query(
                "INSERT INTO research_report (id, title, project_id, body, reported_by, validated_by, status, severity) 
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(report_ids[1].to_string())
            .bind("Test Report 2")
            .bind(project_id.to_string())
            .bind("Test report body 2")
            .bind("0x4234567890123456789012345678901234567890123456789012345678901234")
            .bind("0x5234567890123456789012345678901234567890123456789012345678901234")
            .bind("validated")
            .bind("medium")
            .execute(pool)
            .await
            .expect("Failed to create test report 2");

            // 3. Accepted report with reward
            sqlx::query(
                "INSERT INTO research_report (id, title, project_id, body, reported_by, validated_by, status, severity, allocated_reward) 
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(report_ids[2].to_string())
            .bind("Test Report 3")
            .bind(project_id.to_string())
            .bind("Test report body 3")
            .bind("0x6234567890123456789012345678901234567890123456789012345678901234")
            .bind("0x7234567890123456789012345678901234567890123456789012345678901234")
            .bind("accepted")
            .bind("high")
            .bind(1000.0)
            .execute(pool)
            .await
            .expect("Failed to create test report 3");

            (project_id, report_ids)
        }
        DbPool::Postgres(pool) => {
            let project_id = Uuid::new_v4();
            let report_ids = vec![
                Uuid::new_v4(), // submitted report
                Uuid::new_v4(), // validated report
                Uuid::new_v4(), // accepted report with reward
            ];

            // Create a test project
            sqlx::query(
                "INSERT INTO projects (id, owner_address, contract_address, name, description, contact_info) 
                 VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(project_id)
            .bind("0x1234567890123456789012345678901234567890123456789012345678901234")
            .bind("0x2234567890123456789012345678901234567890123456789012345678901234")
            .bind("Test Project")
            .bind("Test Description")
            .bind("test@example.com")
            .execute(pool)
            .await
            .expect("Failed to create test project");

            // Create test reports with different statuses and metadata
            // 1. Submitted report
            sqlx::query(
                "INSERT INTO research_report (id, title, project_id, body, reported_by, status, severity) 
                 VALUES ($1, $2, $3, $4, $5, $6, $7)"
            )
            .bind(report_ids[0])
            .bind("Test Report 1")
            .bind(project_id)
            .bind("Test report body 1")
            .bind("0x3234567890123456789012345678901234567890123456789012345678901234")
            .bind("submitted")
            .bind("low")
            .execute(pool)
            .await
            .expect("Failed to create test report 1");

            // 2. Validated report
            sqlx::query(
                "INSERT INTO research_report (id, title, project_id, body, reported_by, validated_by, status, severity) 
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
            )
            .bind(report_ids[1])
            .bind("Test Report 2")
            .bind(project_id)
            .bind("Test report body 2")
            .bind("0x4234567890123456789012345678901234567890123456789012345678901234")
            .bind("0x5234567890123456789012345678901234567890123456789012345678901234")
            .bind("validated")
            .bind("medium")
            .execute(pool)
            .await
            .expect("Failed to create test report 2");

            // 3. Accepted report with reward
            sqlx::query(
                "INSERT INTO research_report (id, title, project_id, body, reported_by, validated_by, status, severity, allocated_reward) 
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
            )
            .bind(report_ids[2])
            .bind("Test Report 3")
            .bind(project_id)
            .bind("Test report body 3")
            .bind("0x6234567890123456789012345678901234567890123456789012345678901234")
            .bind("0x7234567890123456789012345678901234567890123456789012345678901234")
            .bind("accepted")
            .bind("high")
            .bind(1000.0)
            .execute(pool)
            .await
            .expect("Failed to create test report 3");

            (project_id, report_ids)
        }
    };

    // Test 1: List all reports for existing project
    let req = Request::get(&format!("/api/projects/{}/reports", project_id))
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), MAX_BODY_SIZE).await.unwrap();
    let reports: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(reports.len(), 3);
    // Check all reports are returned with correct data
    let mut found_statuses = vec![false, false, false];
    for report in reports {
        match report["status"].as_str().unwrap() {
            "submitted" => {
                found_statuses[0] = true;
                assert_eq!(report["id"], json!(report_ids[0].to_string()));
                assert_eq!(report["severity"], "low");
                assert_eq!(report["validated_by"], json!(null));
                assert_eq!(report["allocated_reward"], json!(null));
            }
            "validated" => {
                found_statuses[1] = true;
                assert_eq!(report["id"], json!(report_ids[1].to_string()));
                assert_eq!(report["severity"], "medium");
                assert_eq!(report["validated_by"], "0x5234567890123456789012345678901234567890123456789012345678901234");
                assert_eq!(report["allocated_reward"], json!(null));
            }
            "accepted" => {
                found_statuses[2] = true;
                assert_eq!(report["id"], json!(report_ids[2].to_string()));
                assert_eq!(report["severity"], "high");
                assert_eq!(report["validated_by"], "0x7234567890123456789012345678901234567890123456789012345678901234");
                assert_eq!(report["allocated_reward"], 1000.0);
            }
            _ => panic!("Unexpected report status"),
        }
    }
    assert!(found_statuses.iter().all(|&x| x), "Not all report statuses were found");

    // Test 2: Try to list reports for non-existent project
    let non_existent_id = Uuid::new_v4();
    let req = Request::get(&format!("/api/projects/{}/reports", non_existent_id))
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    // Test 3: Filter reports by status - submitted
    let req = Request::get(&format!("/api/projects/{}/reports?status=submitted", project_id))
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), MAX_BODY_SIZE).await.unwrap();
    let reports: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0]["status"], "submitted");
    assert_eq!(reports[0]["id"], json!(report_ids[0].to_string()));

    // Test 4: Filter reports by status - validated
    let req = Request::get(&format!("/api/projects/{}/reports?status=validated", project_id))
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), MAX_BODY_SIZE).await.unwrap();
    let reports: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0]["status"], "validated");
    assert_eq!(reports[0]["id"], json!(report_ids[1].to_string()));

    // Test 5: Filter reports by status - accepted
    let req = Request::get(&format!("/api/projects/{}/reports?status=accepted", project_id))
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), MAX_BODY_SIZE).await.unwrap();
    let reports: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0]["status"], "accepted");
    assert_eq!(reports[0]["id"], json!(report_ids[2].to_string()));

    // Test 6: Filter reports by non-existent status
    let req = Request::get(&format!("/api/projects/{}/reports?status=invalid_status", project_id))
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), MAX_BODY_SIZE).await.unwrap();
    let reports: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(reports.len(), 0);

    // Test 7: Pagination - limit 1, offset 0
    let req = Request::get(&format!("/api/projects/{}/reports?limit=1&offset=0", project_id))
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), MAX_BODY_SIZE).await.unwrap();
    let reports: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0]["id"], json!(report_ids[0].to_string()));

    // Test 8: Pagination - limit 1, offset 1
    let req = Request::get(&format!("/api/projects/{}/reports?limit=1&offset=1", project_id))
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = to_bytes(res.into_body(), MAX_BODY_SIZE).await.unwrap();
    let reports: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0]["id"], json!(report_ids[1].to_string()));

    // Clean up
    match db.pool() {
        DbPool::Sqlite(pool) => {
            sqlx::query("DROP TABLE research_report")
                .execute(pool)
                .await
                .expect("Failed to drop research_report table");
            sqlx::query("DROP TABLE projects")
                .execute(pool)
                .await
                .expect("Failed to drop projects table");
        }
        DbPool::Postgres(_) => {
            // PostgreSQL tables will be cleaned up automatically between tests
        }
    }
} 