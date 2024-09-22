use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let static_dir = tower_http::services::ServeDir::new("ui/dist/static");
    let main_page = tower_http::services::ServeFile::new("ui/dist/index.html");

    let app = axum::Router::new()
        .nest_service("/ui", main_page)
        .nest_service("/static", static_dir)
        .layer(tower_http::compression::CompressionLayer::new());

    let addr = "0.0.0.0:8000";
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
