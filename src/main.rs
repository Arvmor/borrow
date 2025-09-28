// Local modules
mod alert;
mod api;

// Imports
use std::{collections::HashMap, time::Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init tracing
    tracing_subscriber::fmt::init();
    tracing::info!("Starting the program");

    // Init Discord Alert
    let webhook = std::env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL must be set");
    let pingers = std::env::var("DISCORD_PINGERS").expect("DISCORD_PINGERS must be set");
    let discord = alert::Discord::new(webhook, pingers);

    // Cache
    let mut cache = HashMap::new();
    let vault_id = 9745;
    let id = 14;

    let mut interval = tokio::time::interval(Duration::from_secs(60));
    loop {
        // Wait for the interval
        interval.tick().await;

        // Get the vault
        let vault = match api::get_vault(vault_id, id) {
            Ok(v) => {
                tracing::info!("Got vault: {v:#?}");
                v.total_borrow
            }
            Err(e) => {
                tracing::error!("Error getting vault: {e:?}");
                continue;
            }
        };

        // Check if the vault has changed
        let last_vault = cache.entry((vault_id, id)).or_insert(vault.clone());

        // Check if the vault has changed
        if *last_vault != vault {
            // Notify
            let message =
                format!("Vault {vault_id} id {id} has changed from {last_vault} to {vault}");
            if let Err(e) = discord.notify(message) {
                tracing::error!("Error notifying Discord: {e:?}");
            }

            // Update the cache
            *last_vault = vault;
        }
    }
}
