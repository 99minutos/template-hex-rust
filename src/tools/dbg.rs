use tracing_stackdriver::CloudTraceConfiguration;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

pub fn init_logger(tracer: opentelemetry_sdk::trace::Tracer, project_id: String) {
    let filter = EnvFilter::from_env("DEBUG_LEVEL");
    let stackdriver = tracing_stackdriver::layer().with_cloud_trace(CloudTraceConfiguration {
        project_id: project_id.clone(),
    });

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(telemetry)
        .with(stackdriver);

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

pub fn init_logger_without_trace() {
    let filter = EnvFilter::from_env("DEBUG_LEVEL");
    let stackdriver = tracing_stackdriver::layer();
    let telemetry = tracing_opentelemetry::layer();
    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(telemetry)
        .with(stackdriver);

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
