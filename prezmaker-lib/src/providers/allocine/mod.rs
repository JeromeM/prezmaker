pub mod movie;
pub mod selectors;
pub mod series;

use reqwest::Client;

pub struct AllocineClient {
    pub client: Client,
}

impl AllocineClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
                .build()
                .expect("Failed to build HTTP client"),
        }
    }
}
