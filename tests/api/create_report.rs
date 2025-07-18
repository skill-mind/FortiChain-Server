use axum::body::Body;
use axum::http::Request;
use fortichain_server::{http::api_router, AppState};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

#[tokio::test]
async fn test_create_report() {
    // Initialize test DB from env
    let test_db_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let db = fortichain_server::db::Db::new(&test_db_url, 5)
        .await
        .expect("Failed to init test DB");
    let state = AppState {
        db,
        configuration: fortichain_server::Configuration::test(),
    };
    let app = api_router(state);

    // Seed a researcher & project
    let researcher_id = Uuid::new_v4();
    let project_id = Uuid::new_v4();
    fortichain_server::tests::api::helpers::create_researcher(&app, researcher_id).await;
    fortichain_server::tests::api::helpers::create_project(&app, project_id).await;

    // Build request payload
    let body = json!({
        "title": "Critical Reentrancy Vulnerability",
        "body": "Reentrancy found in withdraw method.",
        "project_id": project_id,
        "reported_by": researcher_id
    });

    let req = Request::post("/reports")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), 201);
}
