use axum::Router;
use fortichain_server::{
    api_router,
    db::Db,
    Config,
    AppState
};
use serde_json::json;
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_create_report() {
    let db = Db::new("postgres://YOUR_USER:YOUR_PASS@localhost/YOUR_DB", 5)
        .await
        .expect("Failed to connect DB");

    let config = Config {
        listen_address: "127.0.0.1:0".parse().unwrap(),
    };

    let app_state = AppState { db, configuration: config };
    let app = api_router(app_state);

    let body = json!({
        "title": "Critical Reentrancy Vulnerability",
        "project_id": "PROJECT-UUID-HERE",
        "body": "A reentrancy was found in the withdraw method due to missing reentrancy guard.",
        "reported_by": "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    });

    let response = app
        .oneshot(
            axum::http::Request::post("/reports")
                .header("content-type", "application/json")
                .body(axum::body::Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 201);
}
