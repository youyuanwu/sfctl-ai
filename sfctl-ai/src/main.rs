use sfctl_ai::app_loop;
use tokio_util::sync::CancellationToken;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create a cancellation token for graceful shutdown
    let token = CancellationToken::new();

    // Start the main application loop
    app_loop(token).await;

    Ok(())
}
