use opentelemetry_gcloud_trace::GcpCloudTraceExporterBuilder;

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
