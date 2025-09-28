use serde::Serialize;
use serde_json::json;
use std::fmt::Debug;

/// Notification for Discord
///
/// Uses Webhook URL under the hood
pub struct Discord {
    agent: ureq::Agent,
    webhook: String,
    pingers: String,
}

impl Discord {
    /// Create a new Discord instance
    pub fn new(webhook: String, pingers: String) -> Self {
        let config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .build();
        let agent = ureq::Agent::new_with_config(config);

        Self {
            agent,
            webhook,
            pingers,
        }
    }

    /// Notify Discord
    ///
    /// `POST /webhooks/{webhook_id}/{webhook_token}`
    pub fn notify<M: Serialize + Debug>(&self, message: M) -> anyhow::Result<()> {
        let ping = &self.pingers;
        let message = serde_json::to_string(&message)?;
        tracing::info!("Alerting Discord - Pinging: {ping:?} - {message:?}");

        // Struct a message to Discord
        let value = json!({
            "content": format!("<@&{ping}>"),
            "username": "Alertooor",
            "embeds": [{
                "color": 5814783,
                "fields": [{
                    "name": "-- Alert --",
                    "value": message
                }],
            }],
        });

        // Send a message to Discord
        let result = self.agent.post(&self.webhook).send_json(&value)?;
        let response = result.into_body().read_to_string()?;
        tracing::debug!("Alerted Discord Result: {response:?}");

        Ok(())
    }
}
