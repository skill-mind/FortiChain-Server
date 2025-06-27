use opentelemetry::{global, trace::TracerProvider};
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_stdout as stdout;
use tracing_subscriber::{prelude::*, EnvFilter};
use fortichain_server::{Configuration, http};
use opentelemetry_appender_tracing::layer;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let exporter = opentelemetry_stdout::LogExporter::default();

    
    let exporter = stdout::SpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter)
        .build();
        
    let configuration = Configuration::new();

    let filter_otel = EnvFilter::new("info")
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());
    let otel_layer = layer::OpenTelemetryTracingBridge::new(&provider).with_filter(filter_otel);
    
    let server_result = http::serve(configuration).await;
    
    provider.shutdown().expect("TracerProvider should shutdown successfully");
    server_result.expect("Failed to start server.");
}
