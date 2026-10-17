use anyhow::{Context, Error, Result};

mod errors;
mod handlers;
mod models;
mod router;
mod templates;

#[tokio::main]
async fn main() -> Result<(), Error> {
    const PORT: &str = "8080";
    tracing_subscriber::fmt::init(); // display in console

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{PORT}"))
        .await
        .with_context(|| format!("Failed to bind to port {PORT}"))?;

    tracing::info!("Router initialized, now listening port {PORT}");

    let app = router::create_router();

    axum::serve(listener, app)
        .await
        .context("Failed to start the server")?;

    Ok(())
}
