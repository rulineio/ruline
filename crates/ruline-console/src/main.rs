use std::sync::Arc;

use ruline_console::{api, App, Config};
use tracing_subscriber::{
    fmt::format::JsonFields, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(e) = load_envs() {
        eprintln!("Failed to load .env file: {}", e);
    }

    let config = Config::new()?;

    init_tracing()?;

    let app = App::new(config).await.map(Arc::new)?;

    let router = api::router(app.to_owned());

    let addr = format!("0.0.0.0:{}", &app.config.http_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, router).await?;

    Ok(())
}

fn load_envs() -> anyhow::Result<()> {
    dotenvy::from_path("./crates/ruline-console/.env")?;
    dotenvy::from_path("./crates/ruline-console/.env.secret")?;

    Ok(())
}

fn init_tracing() -> anyhow::Result<()> {
    let filter = EnvFilter::from_default_env();

    let layer = tracing_subscriber::fmt::layer()
        .fmt_fields(JsonFields::default())
        .event_format(
            tracing_subscriber::fmt::format()
                .json()
                .flatten_event(true)
                .with_span_list(false),
        )
        .with_filter(filter);

    tracing_subscriber::registry().with(layer).init();

    Ok(())
}
