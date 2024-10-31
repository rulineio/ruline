use std::sync::Arc;

use opentelemetry::{global, trace::TracerProvider, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    propagation::TraceContextPropagator, runtime::Tokio, trace::Sampler, Resource,
};
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use ruline_console::{api, App, Config};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let res = load_envs();

    let config = Config::new()?;

    init_tracing(&config)?;

    if let Err(e) = res {
        debug!({error = %e},"Failed to load environment variables");
    }

    let app = App::new(config).await.map(Arc::new)?;

    let router = api::router(app.to_owned());

    let addr = format!("0.0.0.0:{}", &app.config.http_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Server ready and listening on: {}", addr);

    axum::serve(listener, router).await?;

    global::shutdown_tracer_provider();

    Ok(())
}

fn load_envs() -> anyhow::Result<()> {
    dotenvy::from_path("./crates/ruline-console/.env")?;
    dotenvy::from_path("./crates/ruline-console/.env.secret")?;
    Ok(())
}

fn init_tracing(config: &Config) -> anyhow::Result<()> {
    let filter_layer = EnvFilter::from_default_env();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .compact()
        .with_target(false);

    let registry = tracing_subscriber::registry().with(fmt_layer);

    match (&config.otel_service_name, &config.otel_agent_endpoint) {
        (Some(service_name), Some(agent_endpoint)) => {
            let otlp_trace_exporter = opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(agent_endpoint);
            let trace_config = opentelemetry_sdk::trace::Config::default()
                .with_sampler(Sampler::AlwaysOn)
                .with_resource(Resource::new(vec![KeyValue::new(
                    SERVICE_NAME,
                    service_name.to_owned(),
                )]));
            let tracer = opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(otlp_trace_exporter)
                .with_trace_config(trace_config)
                .install_batch(Tokio)?;
            let otel_trace_layer =
                tracing_opentelemetry::layer().with_tracer(tracer.tracer(service_name.to_owned()));

            registry
                .with(otel_trace_layer)
                .with(filter_layer)
                .try_init()?;

            global::set_text_map_propagator(TraceContextPropagator::new());
            global::set_tracer_provider(tracer);
        }
        _ => {
            registry.with(filter_layer).try_init()?;
        }
    };

    Ok(())
}
