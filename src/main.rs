use fortichain_server::{Configuration, db::Db, http, telemetry};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    telemetry::setup_tracing();

    tracing::debug!("Initializing configuration");
    let config = Configuration::new();

    tracing::info!("Starting server on {}", config.listen_address);

    let configuration = Configuration::new();
    let db = Db::new(&configuration)
        .await
        .expect("Failed to initialize DB");
    http::serve(configuration, db)
        .await
        .expect("Failed to start server.");
}
