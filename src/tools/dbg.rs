use tracing_stackdriver::CloudTraceConfiguration;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Layer};

pub fn init_logger(tracer: opentelemetry_sdk::trace::Tracer, project_id: String) {
    let base_level = std::env::var("DEBUG_LEVEL").unwrap_or("info".to_string());
    let filter = EnvFilter::new(format!("tower_http=warn,axum=warn,{}", base_level));

    let stackdriver = tracing_stackdriver::layer()
        .with_cloud_trace(CloudTraceConfiguration {
            project_id: project_id.clone(),
        })
        .with_filter(filter);

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::registry()
        .with(telemetry)
        .with(stackdriver);

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

pub fn init_logger_without_trace() {
    let base_level = std::env::var("DEBUG_LEVEL").unwrap_or("info".to_string());
    let filter = EnvFilter::new(format!("tower_http=warn,axum=warn,{}", base_level));
    let stackdriver = tracing_stackdriver::layer().with_filter(filter);
    let telemetry = tracing_opentelemetry::layer();
    let subscriber = tracing_subscriber::registry()
        .with(telemetry)
        .with(stackdriver);

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
