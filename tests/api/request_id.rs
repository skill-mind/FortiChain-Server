use axum::{
    body::Body,
    http::{HeaderValue, Request, StatusCode},
};
use uuid::Uuid;

use crate::helpers::TestApp;

#[tokio::test]
async fn test_request_id_generated_when_not_provided() {
    let app = TestApp::new().await;

    let req = Request::get("/health_check").body(Body::empty()).unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    // Check that a request ID was added to the response
    let request_id_header = res.headers().get("x-request-id");
    assert!(
        request_id_header.is_some(),
        "x-request-id header should be present"
    );

    // Verify it's a valid UUID
    let request_id = request_id_header.unwrap().to_str().unwrap();
    assert!(
        Uuid::parse_str(request_id).is_ok(),
        "Request ID should be a valid UUID"
    );
}

#[tokio::test]
async fn test_request_id_preserved_when_provided() {
    let app = TestApp::new().await;
    let provided_request_id = "custom-request-id-12345";

    let req = Request::get("/health_check")
        .header("x-request-id", provided_request_id)
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    // Check that the provided request ID is returned
    let request_id_header = res.headers().get("x-request-id");
    assert!(
        request_id_header.is_some(),
        "x-request-id header should be present"
    );

    let returned_request_id = request_id_header.unwrap().to_str().unwrap();
    assert_eq!(
        returned_request_id, provided_request_id,
        "Request ID should match the provided one"
    );
}

#[tokio::test]
async fn test_request_id_different_for_each_request() {
    let app = TestApp::new().await;

    // Make first request
    let req1 = Request::get("/health_check").body(Body::empty()).unwrap();
    let res1 = app.request(req1).await;
    let request_id1 = res1
        .headers()
        .get("x-request-id")
        .unwrap()
        .to_str()
        .unwrap();

    // Make second request
    let req2 = Request::get("/health_check").body(Body::empty()).unwrap();
    let res2 = app.request(req2).await;
    let request_id2 = res2
        .headers()
        .get("x-request-id")
        .unwrap()
        .to_str()
        .unwrap();

    // Verify they are different
    assert_ne!(
        request_id1, request_id2,
        "Each request should have a unique request ID"
    );
}

#[tokio::test]
async fn test_request_id_case_insensitive_header() {
    let app = TestApp::new().await;
    let provided_request_id = "case-test-12345";

    // Test with uppercase header
    let req = Request::get("/health_check")
        .header("X-Request-ID", provided_request_id)
        .body(Body::empty())
        .unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let returned_request_id = res.headers().get("x-request-id").unwrap().to_str().unwrap();
    assert_eq!(
        returned_request_id, provided_request_id,
        "Request ID should be preserved regardless of header case"
    );
}
