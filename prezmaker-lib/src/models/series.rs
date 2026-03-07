use super::common::{Country, Genre, Person, Rating};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub title: String,
    pub original_title: Option<String>,
    pub year: Option<u16>,
    pub end_year: Option<u16>,
    pub first_air_date: Option<String>,
    pub synopsis: Option<String>,
    pub poster_url: Option<String>,
    pub backdrop_url: Option<String>,
    pub genres: Vec<Genre>,
    pub countries: Vec<Country>,
    pub creators: Vec<Person>,
    pub cast: Vec<Person>,
    pub ratings: Vec<Rating>,
    pub seasons_count: Option<u32>,
    pub episodes_count: Option<u32>,
    pub episode_runtime: Option<u32>,
    pub status: Option<String>,
    pub networks: Vec<String>,
    pub tmdb_id: Option<u64>,
    pub imdb_id: Option<String>,
    pub allocine_url: Option<String>,
}

impl Series {
    pub fn creators_display(&self) -> String {
        self.creators
            .iter()
            .map(|p| p.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn cast_display(&self, max: usize) -> String {
        self.cast
            .iter()
            .take(max)
            .map(|p| p.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn genres_display(&self) -> String {
        self.genres
            .iter()
            .map(|g| g.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn countries_display(&self) -> String {
        self.countries
            .iter()
            .map(|c| c.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn networks_display(&self) -> String {
        self.networks.join(", ")
    }

    pub fn runtime_formatted(&self) -> Option<String> {
        self.episode_runtime.map(|m| format!("{}min", m))
    }

    pub fn rating_by_source(&self, source: &str) -> Option<&Rating> {
        self.ratings.iter().find(|r| r.source == source)
    }

    pub fn year_display(&self) -> String {
        match (self.year, self.end_year) {
            (Some(start), Some(end)) => format!("{}-{}", start, end),
            (Some(start), None) => format!("{}-", start),
            _ => String::new(),
        }
    }
}
