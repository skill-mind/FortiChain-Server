use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

// Setup Logger
pub fn setup_tracing() {
    // Default to debug if no parameter is specified in the environment.
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "fortichain_server=debug,tower_http=debug".into());

    // Format logs in JSON without targets.
    let formatting_layer = fmt::layer().json().with_target(false);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer)
        .init();
}

// Logging Middleware
pub fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}
