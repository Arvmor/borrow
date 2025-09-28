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

    let mut interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        // Wait for the interval
        interval.tick().await;

        // Get the vault
        let vault = match api::get_vault(vault_id, id) {
            Ok(v) => {
                tracing::info!("Got vault: {v:#?}");
                v
            }
            Err(e) => {
                tracing::error!("Error getting vault: {e:?}");
                continue;
            }
        };

        // Check if the vault has changed
        let last_vault = cache.entry((vault_id, id)).or_insert(vault.clone());

        // Parse Values
        let Ok(old_total_borrow) = last_vault.total_borrow.parse::<f64>() else {
            tracing::error!("Error parsing total borrow: {}", last_vault.total_borrow);
            continue;
        };
        let Ok(old_total_borrow_liquidity) = last_vault.total_borrow_liquidity.parse::<f64>()
        else {
            tracing::error!(
                "Error parsing total borrow liquidity: {}",
                last_vault.total_borrow_liquidity
            );
            continue;
        };

        let Ok(new_total_borrow) = vault.total_borrow.parse::<f64>() else {
            tracing::error!("Error parsing vault total borrow: {}", vault.total_borrow);
            continue;
        };
        let Ok(new_total_borrow_liquidity) = vault.total_borrow_liquidity.parse::<f64>() else {
            tracing::error!(
                "Error parsing vault total borrow liquidity: {}",
                vault.total_borrow_liquidity
            );
            continue;
        };

        let Ok(new_borrowable) = vault.borrowable.parse::<f64>() else {
            tracing::error!("Error parsing vault borrowable: {}", vault.borrowable);
            continue;
        };

        // Check if the vault has changed
        let percent_change = 0.001;
        let borrowable = new_borrowable >= 20_000_000_000.;
        let change = (new_total_borrow - old_total_borrow) / old_total_borrow;
        let change_liquidity =
            (new_total_borrow_liquidity - old_total_borrow_liquidity) / old_total_borrow_liquidity;
        tracing::info!("Has changed: {change} - Has changed liquidity: {change_liquidity}");

        if change >= percent_change || change_liquidity >= percent_change {
            // Notify
            let message = format!(
                r"Vault {vault_id} id {id}
                Has changed by {change} and {change_liquidity}.
                Total Borrow: {old_total_borrow} -> {new_total_borrow}
                Total Borrow Liquidity: {old_total_borrow_liquidity} -> {new_total_borrow_liquidity}"
            );
            if let Err(e) = discord.notify(message) {
                tracing::error!("Error notifying Discord: {e:?}");
            }
        }

        if borrowable {
            // Notify
            let message = format!(
                r"Vault {vault_id} id {id}
                **Borrowable** ALERT! +20k
                Borrowable: {new_borrowable}"
            );
            if let Err(e) = discord.notify(message) {
                tracing::error!("Error notifying Discord: {e:?}");
            }
        }

        // Update the cache
        *last_vault = vault;
    }
}
