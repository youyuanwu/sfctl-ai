use sfctl_ai::app_loop;
use tokio::signal;
use tracing_appender::rolling;
use tracing_subscriber::fmt;

fn main() {
    // Set up file appender (logs/ directory, file per day)
    let file_appender = rolling::daily("logs", "sfctl-ai.log");
    fmt()
        .with_writer(file_appender)
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .init();

    let h = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    h.block_on(async {
        let token = tokio_util::sync::CancellationToken::new();
        let app_handle = tokio::spawn({
            let token = token.clone();
            async move {
                app_loop(token).await;
            }
        });

        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl-C handler");
        tracing::info!("Ctrl-C received, shutting down.");
        token.cancel();
        app_handle.await.expect("App loop failed");
        tracing::info!("Application block_on done.");
    });
    // shutdown manually due to windows io.
    h.shutdown_background();
    tracing::info!("Application has exited.");
}
