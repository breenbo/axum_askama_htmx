use anyhow::{Context, Error, Result};

mod auth;
mod base;
mod database;
mod errors;
mod ideas;
mod root;
mod router;

#[tokio::main]
async fn main() -> Result<(), Error> {
    const PORT: &str = "8080";
    tracing_subscriber::fmt::init(); // display in console
    //
    let app_state = database::connect_sqlite().await?;

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{PORT}"))
        .await
        .with_context(|| format!("Failed to bind to port {PORT}"))?;
    tracing::info!("Router initialized, now listening port {PORT}");

    let app = router::create_router(app_state);

    axum::serve(listener, app)
        .await
        .context("Failed to start the server")?;

    Ok(())
}
