use std::{fmt::format, sync::Once};

use axum::{Router, body::Body, extract::Request, response::Response};
use fortichain_server::{AppState, Configuration, api_router, db::Db, telemetry};
use sqlx::{Connection, Executor, PgConnection};
use tower::ServiceExt;
use uuid::Uuid;
use rand::Rng;

static TRACING: Once = Once::new();

pub struct TestApp {
    pub router: Router,
    pub db: Db,
}

impl TestApp {
    pub async fn new() -> Self {
        dotenvy::dotenv().ok();
        unsafe { std::env::set_var("PORT", "0") };

        TRACING.call_once(telemetry::setup_tracing);
        let cfg = Configuration::new();

        let db_str = create_test_db(&cfg.database_url).await;
        let db = Db::new(&db_str, cfg.max_db_connections)
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

pub async fn create_test_db(db_str: &str) -> String {
    let (db_str, uuid_db) = db_str_and_uuid(db_str);

    let mut connection = PgConnection::connect(&db_str)
        .await
        .expect("Failed to connect to postgres.");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, uuid_db).as_str())
        .await
        .expect("Failed to create test database.");

    db_str.to_owned()
}

pub fn db_str_and_uuid(db_str: &str) -> (String, String) {
    let db_name =
        std::env::var("DATABASE_NAME").expect("DATABASE_NAME environment variable not specified.");
    let db_str = db_str
        .strip_suffix(&db_name)
        .expect("Failed to strip DB name from connection string");

    let uuid_db = Uuid::now_v7().to_string();

    (db_str.to_owned(), uuid_db)
}

fn random_hex_string() -> String {
    let charset = b"abcdefABCDEF0123456789";
    let mut rng = rand::rng();
    (0..8)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

pub fn generate_address() -> String {
    let last_eigth = random_hex_string();
    format!("0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefab{}", last_eigth)

}