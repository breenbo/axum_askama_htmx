#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Hello, world!");

    const PORT: &str = "8080";

    tracing::info!("Router initialized, now listening port {}", PORT);

    Ok(())
}
