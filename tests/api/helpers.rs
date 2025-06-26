use axum::{Router, body::Body, extract::Request, response::Response};
use fortichain_server::{AppState, Configuration, api_router};
use std::sync::Arc;
use tower::ServiceExt;

pub struct TestApp {
    pub router: Router,
}

impl TestApp {
    pub async fn new() -> Self {
        dotenvy::dotenv().ok();
        unsafe { std::env::set_var("PORT", "0") };

        let cfg = Configuration::new();
        let router = api_router(AppState {
            configuration: Arc::new(cfg),
        });

        Self { router }
    }

    pub async fn request(&self, req: Request<Body>) -> Response<Body> {
        self.router.clone().oneshot(req).await.unwrap()
    }
}
