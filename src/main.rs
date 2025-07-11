use fortichain_server::{Configuration, db::Db, http, telemetry};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Initialize tracing
    telemetry::setup_tracing();

    tracing::info!("Initializing configuration");
    let config = Configuration::new();

    tracing::info!("Starting server on {}", config.listen_address);

    tracing::info!("Initializing DB pool");
    let db = Db::new(&config.database_url, config.max_db_connections)
        .await
        .expect("Failed to initialize DB");

    tracing::info!("Running Migrations");
    db.migrate().await.expect("Failed to run migrations");

    tracing::info!("Starting server");
    http::serve(config, db)
        .await
        .expect("Failed to start server.");
}
