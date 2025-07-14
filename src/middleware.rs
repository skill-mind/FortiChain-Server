use std::time::Duration;

use axum::http::HeaderName;
use tower_http::{
    cors::{AllowHeaders, Any, CorsLayer},
    normalize_path::NormalizePathLayer,
    request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    timeout::TimeoutLayer,
};

#[derive(Clone, Default)]
pub struct Id;

impl MakeRequestId for Id {
    fn make_request_id<B>(
        &mut self,
        _: &hyper::Request<B>,
    ) -> Option<tower_http::request_id::RequestId> {
        let id = uuid::Uuid::now_v7().to_string().parse().unwrap();
        Some(RequestId::new(id))
    }
}

pub fn request_id_layer() -> SetRequestIdLayer<Id> {
    let x_request_id = HeaderName::from_static("x-request-id");
    SetRequestIdLayer::new(x_request_id.clone(), Id)
}

pub fn propagate_request_id_layer() -> PropagateRequestIdLayer {
    let x_request_id = HeaderName::from_static("x-request-id");
    PropagateRequestIdLayer::new(x_request_id)
}

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(AllowHeaders::mirror_request())
        .max_age(Duration::from_secs(600))
}

pub fn timeout_layer() -> TimeoutLayer {
    TimeoutLayer::new(Duration::from_secs(15))
}

pub fn normalize_path_layer() -> NormalizePathLayer {
    NormalizePathLayer::trim_trailing_slash()
}
