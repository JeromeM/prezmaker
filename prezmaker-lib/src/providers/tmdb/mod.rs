pub mod models;
pub mod movie;
pub mod series;

use reqwest::Client;

pub struct TmdbClient {
    pub client: Client,
    pub api_key: String,
    pub base_url: String,
    pub language: String,
}

impl TmdbClient {
    pub fn new(api_key: String, language: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.themoviedb.org/3".to_string(),
            language,
        }
    }

    pub fn poster_url(path: &str) -> String {
        format!("https://image.tmdb.org/t/p/w500{}", path)
    }

    pub fn backdrop_url(path: &str) -> String {
        format!("https://image.tmdb.org/t/p/w1280{}", path)
    }
}
