// Local modules
mod api;

// Imports
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init tracing
    tracing_subscriber::fmt::init();
    tracing::info!("Starting the program");

    let mut interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        // Get the vault
        match api::get_vault("9745", "14") {
            Ok(vault) => tracing::info!("API Vault: {vault:?}"),
            Err(e) => tracing::error!("Error getting vault: {e:?}"),
        }

        // Wait for the interval
        interval.tick().await;
    }
}
