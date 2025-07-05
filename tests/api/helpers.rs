use std::sync::Once;

use axum::{Router, body::Body, extract::Request, response::Response};
use fortichain_server::{AppState, Configuration, api_router, db::Db, telemetry, config::DatabaseType};
use tower::ServiceExt;

static TRACING: Once = Once::new();

pub struct TestApp {
    pub router: Router,
    pub db: Db,
}

impl TestApp {
    pub async fn new() -> Self {
        dotenvy::dotenv().ok();
        unsafe { std::env::set_var("PORT", "0") };

        // Set test-specific environment variables
        unsafe {
            // SAFETY: These environment variables are only used in tests
            // and are set once at the start of each test
            std::env::set_var("DATABASE_TYPE", "sqlite");
            std::env::set_var("DATABASE_URL", "sqlite::memory:");
            std::env::set_var("DB_MAX_CONNECTIONS", "1");
            std::env::set_var("APP_ENVIRONMENT", "local");
        }

        TRACING.call_once(telemetry::setup_tracing);
        let cfg = Configuration::new();

        let db = Db::new(&cfg.database_url, cfg.max_db_connections, DatabaseType::Sqlite)
            .await
            .expect("Failed to initialize DB");

        tracing::debug!("Running migrations");
        db.migrate().await.expect("Failed to run migrations");

        let router = api_router(AppState {
            configuration: cfg,
            db: db.clone(),
        });
        Self { router, db }
    }

    pub async fn request(&self, req: Request<Body>) -> Response<Body> {
        self.router.clone().oneshot(req).await.unwrap()
    }
}
