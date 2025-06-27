use axum::{
    extract::Request,
    http::{HeaderMap, HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};
use std::str::FromStr;
use tracing::Instrument;
use uuid::Uuid;

const REQUEST_ID_HEADER: &str = "x-request-id";

/// Middleware that handles request ID generation and propagation
pub async fn request_id_middleware(request: Request, next: Next) -> Response {
    // Try to extract existing request ID from headers
    let request_id = extract_or_generate_request_id(request.headers());

    // Add request ID to tracing span
    let span = tracing::info_span!(
        "http_request",
        request_id = %request_id,
        method = %request.method(),
        uri = %request.uri(),
    );

    // Process the request within the span context
    let response = async move {
        tracing::info!("Processing request");
        next.run(request).await
    }
    .instrument(span)
    .await;

    // Add request ID to response headers
    add_request_id_to_response(response, &request_id)
}

/// Extract request ID from headers or generate a new one
fn extract_or_generate_request_id(headers: &HeaderMap) -> String {
    headers
        .get(REQUEST_ID_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string())
}

/// Add request ID to response headers
fn add_request_id_to_response(mut response: Response, request_id: &str) -> Response {
    if let (Ok(header_name), Ok(header_value)) = (
        HeaderName::from_str(REQUEST_ID_HEADER),
        HeaderValue::from_str(request_id),
    ) {
        response.headers_mut().insert(header_name, header_value);
    }
    response
}
