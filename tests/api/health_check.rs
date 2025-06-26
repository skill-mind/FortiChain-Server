use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use crate::helpers::TestApp;

#[tokio::test]
async fn test_health_check_ok() {
    let app = TestApp::new().await;

    let req = Request::get("/health_check").body(Body::empty()).unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);
}
