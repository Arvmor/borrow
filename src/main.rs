// Local modules
mod alert;
mod api;

// Imports
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init tracing
    tracing_subscriber::fmt::init();
    tracing::info!("Starting the program");

    // Init Discord Alert
    let webhook = std::env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL must be set");
    let pingers = std::env::var("DISCORD_PINGERS").expect("DISCORD_PINGERS must be set");
    let discord = alert::Discord::new(webhook, pingers);

    let mut interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        // Get the vault
        if let Err(e) = api::get_vault("9745", "14").and_then(|v| discord.notify(v)) {
            tracing::error!("Error getting vault: {e:?}")
        };

        // Wait for the interval
        interval.tick().await;
    }
}
