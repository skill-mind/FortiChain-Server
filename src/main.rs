use fortichain_server::{Configuration, db::Db, http, telemetry};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    telemetry::setup_tracing();

    tracing::info!("Initializing configuration");
    let config = Configuration::new();

    tracing::info!("Initializing DB pool");
    let db = Db::new(&config.database_url, config.max_db_connections, config.database_type.clone())
        .await
        .expect("Failed to initialize DB");

    tracing::info!("Running Migrations");
    db.migrate().await.expect("Failed to run migrations");

    tracing::info!("Starting server");
    http::serve(config, db)
        .await
        .expect("Failed to start server.");
}
