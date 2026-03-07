pub mod game;
pub mod models;

use reqwest::Client;
use serde::Deserialize;
use std::sync::Mutex;
use tracing::debug;

pub struct IgdbClient {
    pub client: Client,
    pub client_id: String,
    pub client_secret: String,
    token: Mutex<Option<TokenData>>,
}

#[derive(Clone)]
struct TokenData {
    access_token: String,
    expires_at: std::time::Instant,
}

#[derive(Debug, Deserialize)]
struct TwitchTokenResponse {
    access_token: String,
    expires_in: u64,
}

impl IgdbClient {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client: Client::new(),
            client_id,
            client_secret,
            token: Mutex::new(None),
        }
    }

    pub async fn get_access_token(&self) -> anyhow::Result<String> {
        // Check cached token
        {
            let guard = self.token.lock().unwrap();
            if let Some(ref data) = *guard {
                if data.expires_at > std::time::Instant::now() {
                    return Ok(data.access_token.clone());
                }
            }
        }

        debug!("Refreshing Twitch OAuth token");
        let resp: TwitchTokenResponse = self
            .client
            .post("https://id.twitch.tv/oauth2/token")
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("grant_type", "client_credentials"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let data = TokenData {
            access_token: resp.access_token.clone(),
            expires_at: std::time::Instant::now()
                + std::time::Duration::from_secs(resp.expires_in.saturating_sub(60)),
        };

        {
            let mut guard = self.token.lock().unwrap();
            *guard = Some(data);
        }

        Ok(resp.access_token)
    }
}
