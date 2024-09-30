use std::sync::Arc;

use ruline_console::{api, App, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(e) = load_envs() {
        eprintln!("Failed to load .env file: {}", e);
    }

    let config = Config::new()?;

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
