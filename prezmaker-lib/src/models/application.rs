use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub name: String,
    pub version: Option<String>,
    pub developer: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
    pub license: Option<String>,
    pub platforms: Vec<String>,
    pub logo_url: Option<String>,
}

impl Application {
    pub fn platforms_display(&self) -> String {
        self.platforms.join(", ")
    }
}
