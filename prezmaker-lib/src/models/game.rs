use super::common::{Genre, Rating};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TechInfo {
    pub platform: String,
    pub languages: String,
    pub size: String,
    #[serde(default)]
    pub install_size: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemReqs {
    #[serde(default)]
    pub os: String,
    #[serde(default)]
    pub cpu: String,
    #[serde(default)]
    pub ram: String,
    #[serde(default)]
    pub gpu: String,
    #[serde(default)]
    pub storage: String,
}

impl SystemReqs {
    pub fn is_empty(&self) -> bool {
        self.os.is_empty()
            && self.cpu.is_empty()
            && self.ram.is_empty()
            && self.gpu.is_empty()
            && self.storage.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub title: String,
    pub release_date: Option<String>,
    pub year: Option<u16>,
    pub synopsis: Option<String>,
    pub cover_url: Option<String>,
    pub screenshots: Vec<String>,
    pub genres: Vec<Genre>,
    pub platforms: Vec<String>,
    pub developers: Vec<String>,
    pub publishers: Vec<String>,
    pub ratings: Vec<Rating>,
    pub igdb_id: Option<u64>,
    pub igdb_slug: Option<String>,
    pub steam_appid: Option<u64>,
    pub tech_info: Option<TechInfo>,
    pub installation: Option<String>,
    #[serde(default)]
    pub min_reqs: Option<SystemReqs>,
    #[serde(default)]
    pub rec_reqs: Option<SystemReqs>,
}

impl Game {
    pub fn genres_display(&self) -> String {
        self.genres
            .iter()
            .map(|g| g.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn platforms_display(&self) -> String {
        self.platforms.join(", ")
    }

    pub fn developers_display(&self) -> String {
        self.developers.join(", ")
    }

    pub fn publishers_display(&self) -> String {
        self.publishers.join(", ")
    }

    pub fn rating_by_source(&self, source: &str) -> Option<&Rating> {
        self.ratings.iter().find(|r| r.source == source)
    }
}
