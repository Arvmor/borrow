use std::fmt::{Debug, Display};

use serde::Serialize;
use serde_json::json;

/// Notification for Discord
///
/// Uses Webhook URL under the hood
pub struct Discord(String);

impl Discord {
    /// Notify Discord
    ///
    /// `POST /webhooks/{webhook_id}/{webhook_token}`
    pub fn notify<M, P>(&self, message: M, ping: P) -> anyhow::Result<()>
    where
        M: Serialize + Debug,
        P: Display + Debug,
    {
        tracing::info!("Alerting Discord - {ping:?} - {message:?}");

        // Struct a message to Discord
        let value = json!({
            "content": format!("<@&{ping}>"),
            "embeds": [{
                "color": 5814783,
                "fields": [{
                    "name": "-- Alert --",
                    "value": message
                }]
                }
            ],
            "username": "Alertooor",
        });

        // Send a message to Discord
        ureq::post(&self.0).send_json(&value)?;

        Ok(())
    }
}
