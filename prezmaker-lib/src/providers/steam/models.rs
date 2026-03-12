use serde::Deserialize;

// --- Search ---

#[derive(Debug, Deserialize)]
pub struct SteamSearchResponse {
    pub total: u32,
    #[serde(default)]
    pub items: Vec<SteamSearchItem>,
}

#[derive(Debug, Deserialize)]
pub struct SteamSearchItem {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub tiny_image: Option<String>,
    #[serde(default)]
    pub metascore: Option<String>,
}

// --- App Details ---

#[derive(Debug, Deserialize)]
pub struct SteamAppDetailsWrapper {
    pub success: bool,
    pub data: Option<SteamAppData>,
}

#[derive(Debug, Deserialize)]
pub struct SteamAppData {
    pub name: String,
    pub steam_appid: u64,
    #[serde(default)]
    pub short_description: Option<String>,
    #[serde(default)]
    pub detailed_description: Option<String>,
    #[serde(default)]
    pub header_image: Option<String>,
    #[serde(default)]
    pub developers: Vec<String>,
    #[serde(default)]
    pub publishers: Vec<String>,
    #[serde(default)]
    pub genres: Vec<SteamGenre>,
    #[serde(default)]
    pub screenshots: Vec<SteamScreenshot>,
    #[serde(default)]
    pub release_date: Option<SteamReleaseDate>,
    #[serde(default)]
    pub metacritic: Option<SteamMetacritic>,
    #[serde(default)]
    pub platforms: Option<SteamPlatforms>,
    #[serde(default)]
    pub supported_languages: Option<String>,
    #[serde(default)]
    pub categories: Vec<SteamCategory>,
    #[serde(default)]
    pub pc_requirements: Option<SteamRequirements>,
}

#[derive(Debug, Deserialize)]
pub struct SteamRequirements {
    #[serde(default)]
    pub minimum: Option<String>,
    #[serde(default)]
    pub recommended: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SteamGenre {
    pub id: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct SteamScreenshot {
    pub id: u64,
    pub path_thumbnail: String,
    pub path_full: String,
}

#[derive(Debug, Deserialize)]
pub struct SteamReleaseDate {
    pub coming_soon: bool,
    pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct SteamMetacritic {
    pub score: u32,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SteamPlatforms {
    #[serde(default)]
    pub windows: bool,
    #[serde(default)]
    pub mac: bool,
    #[serde(default)]
    pub linux: bool,
}

#[derive(Debug, Deserialize)]
pub struct SteamCategory {
    pub id: u32,
    pub description: String,
}
