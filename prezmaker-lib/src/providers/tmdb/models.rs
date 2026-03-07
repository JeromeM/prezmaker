use serde::Deserialize;

// Search results
#[derive(Debug, Deserialize)]
pub struct TmdbSearchResponse<T> {
    pub results: Vec<T>,
    pub total_results: u32,
}

// Movie search result
#[derive(Debug, Deserialize)]
pub struct TmdbMovieSearchResult {
    pub id: u64,
    pub title: String,
    pub original_title: Option<String>,
    pub release_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub vote_average: Option<f64>,
}

// Movie details
#[derive(Debug, Deserialize)]
pub struct TmdbMovieDetail {
    pub id: u64,
    pub title: String,
    pub original_title: Option<String>,
    pub release_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub runtime: Option<u32>,
    pub vote_average: Option<f64>,
    pub imdb_id: Option<String>,
    pub genres: Vec<TmdbGenre>,
    pub production_countries: Vec<TmdbCountry>,
    pub credits: Option<TmdbCredits>,
}

// TV search result
#[derive(Debug, Deserialize)]
pub struct TmdbTvSearchResult {
    pub id: u64,
    pub name: String,
    pub original_name: Option<String>,
    pub first_air_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub vote_average: Option<f64>,
}

// TV details
#[derive(Debug, Deserialize)]
pub struct TmdbTvDetail {
    pub id: u64,
    pub name: String,
    pub original_name: Option<String>,
    pub first_air_date: Option<String>,
    pub last_air_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub vote_average: Option<f64>,
    pub number_of_seasons: Option<u32>,
    pub number_of_episodes: Option<u32>,
    pub episode_run_time: Option<Vec<u32>>,
    pub status: Option<String>,
    pub genres: Vec<TmdbGenre>,
    pub production_countries: Vec<TmdbCountry>,
    pub networks: Vec<TmdbNetwork>,
    pub created_by: Vec<TmdbCreator>,
    pub credits: Option<TmdbCredits>,
    pub external_ids: Option<TmdbExternalIds>,
}

#[derive(Debug, Deserialize)]
pub struct TmdbGenre {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TmdbCountry {
    pub iso_3166_1: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TmdbNetwork {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TmdbCreator {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TmdbCredits {
    pub cast: Vec<TmdbCast>,
    pub crew: Vec<TmdbCrew>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TmdbCast {
    pub name: String,
    pub character: Option<String>,
    pub order: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct TmdbCrew {
    pub name: String,
    pub job: String,
}

#[derive(Debug, Deserialize)]
pub struct TmdbExternalIds {
    pub imdb_id: Option<String>,
}
