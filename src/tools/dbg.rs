use opentelemetry_gcloud_trace::GcpCloudTraceExporterBuilder;
use opentelemetry_sdk::{
    trace::{SdkTracerProvider, TracerProviderBuilder},
    Resource,
};
use tracing_stackdriver::CloudTraceConfiguration;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

pub fn init_logger(tracer: opentelemetry_sdk::trace::Tracer) {
    let envs = crate::envs::get();
    let filter = EnvFilter::from_env("DEBUG_LEVEL");
    let stackdriver = tracing_stackdriver::layer().with_cloud_trace(CloudTraceConfiguration {
        project_id: envs.project_id.clone(),
    });

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(telemetry)
        .with(stackdriver);

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

pub async fn init_tracer(
    service_name: String,
) -> Result<opentelemetry_sdk::trace::Tracer, Box<dyn std::error::Error>> {
    let gcp_trace_exporter = GcpCloudTraceExporterBuilder::for_default_project_id()
        .await?
        .with_resource(
            opentelemetry_sdk::Resource::builder()
                .with_attributes(vec![opentelemetry::KeyValue::new(
                    opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                    service_name.to_string(),
                )])
                .build(),
        );

    let tracer_provider = gcp_trace_exporter.create_provider().await?;

    let tracer: opentelemetry_sdk::trace::Tracer =
        gcp_trace_exporter.install(&tracer_provider).await?;

    opentelemetry::global::set_tracer_provider(tracer_provider.clone());
    Ok(tracer)
}
