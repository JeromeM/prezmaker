use super::common::{Country, Genre, Person, Rating};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub original_title: Option<String>,
    pub year: Option<u16>,
    pub release_date: Option<String>,
    pub duration_minutes: Option<u32>,
    pub synopsis: Option<String>,
    pub poster_url: Option<String>,
    pub backdrop_url: Option<String>,
    pub genres: Vec<Genre>,
    pub countries: Vec<Country>,
    pub directors: Vec<Person>,
    pub cast: Vec<Person>,
    pub ratings: Vec<Rating>,
    pub tmdb_id: Option<u64>,
    pub imdb_id: Option<String>,
    pub allocine_url: Option<String>,
    pub trailer_url: Option<String>,
}

impl Movie {
    pub fn duration_formatted(&self) -> Option<String> {
        self.duration_minutes.map(|m| {
            let hours = m / 60;
            let mins = m % 60;
            if hours > 0 {
                format!("{}h et {}min", hours, mins)
            } else {
                format!("{}min", mins)
            }
        })
    }

    pub fn directors_display(&self) -> String {
        self.directors
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

    pub fn rating_by_source(&self, source: &str) -> Option<&Rating> {
        self.ratings.iter().find(|r| r.source == source)
    }
}
