use fortichain_server::{Configuration, http, telemetry};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    telemetry::setup_tracing();

    tracing::debug!("Initializing configuration");

    let config = Configuration::new();

    tracing::debug!("Initializing DB pool");

    tracing::debug!("Running Migrations");

    tracing::info!("Starting server on {}", config.listen_address);

    let configuration = Configuration::new();
    http::serve(configuration)
        .await
        .expect("Failed to start server.");
}
