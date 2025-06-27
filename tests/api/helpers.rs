use axum::{Router, body::Body, extract::Request, response::Response};
use fortichain_server::{AppState, Configuration, api_router, db::Db};
use sqlx::postgres::PgPool;
use tower::ServiceExt;
pub struct TestApp {
    pub router: Router,
    pub pool: PgPool,
}

impl TestApp {
    pub async fn new() -> Self {
        dotenvy::dotenv().ok();
        unsafe { std::env::set_var("PORT", "0") };

        let cfg = Configuration::new();
        let db = Db::new(&cfg).await.expect("Failed to initialize DB");
        let pool = db.pool.clone();
        let router = api_router(AppState {
            configuration: cfg,
            db,
        });

        Self { router, pool }
    }

    pub async fn request(&self, req: Request<Body>) -> Response<Body> {
        self.router.clone().oneshot(req).await.unwrap()
    }
}
