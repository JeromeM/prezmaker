use super::models::{TmdbMovieDetail, TmdbMovieSearchResult, TmdbSearchResponse};
use super::TmdbClient;
use crate::models::{Country, Genre, Movie, Person, Rating};
use crate::providers::MovieProvider;
use async_trait::async_trait;
use tracing::debug;

#[async_trait]
impl MovieProvider for TmdbClient {
    async fn search_movies(&self, query: &str) -> anyhow::Result<Vec<Movie>> {
        let url = format!("{}/search/movie", self.base_url);
        debug!("TMDB search: {}", query);

        let resp: TmdbSearchResponse<TmdbMovieSearchResult> = self
            .client
            .get(&url)
            .query(&[
                ("api_key", self.api_key.as_str()),
                ("query", query),
                ("language", self.language.as_str()),
            ])
            .send()
            .await?
            .json()
            .await?;

        let movies = resp
            .results
            .into_iter()
            .map(|r| {
                let year = r
                    .release_date
                    .as_ref()
                    .and_then(|d| d.split('-').next())
                    .and_then(|y| y.parse().ok());

                Movie {
                    title: r.title,
                    original_title: r.original_title,
                    year,
                    release_date: r.release_date,
                    duration_minutes: None,
                    synopsis: r.overview,
                    poster_url: r.poster_path.map(|p| TmdbClient::poster_url(&p)),
                    backdrop_url: None,
                    genres: vec![],
                    countries: vec![],
                    directors: vec![],
                    cast: vec![],
                    ratings: r
                        .vote_average
                        .filter(|&v| v > 0.0)
                        .map(|v| {
                            vec![Rating {
                                source: "TMDB".to_string(),
                                value: v,
                                max: 10.0,
                            }]
                        })
                        .unwrap_or_default(),
                    tmdb_id: Some(r.id),
                    imdb_id: None,
                    allocine_url: None,
                }
            })
            .collect();

        Ok(movies)
    }

    async fn get_movie_details(&self, id: u64) -> anyhow::Result<Movie> {
        let url = format!("{}/movie/{}", self.base_url, id);
        debug!("TMDB movie details: {}", id);

        let detail: TmdbMovieDetail = self
            .client
            .get(&url)
            .query(&[
                ("api_key", self.api_key.as_str()),
                ("language", self.language.as_str()),
                ("append_to_response", "credits"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let year = detail
            .release_date
            .as_ref()
            .and_then(|d| d.split('-').next())
            .and_then(|y| y.parse().ok());

        let directors = detail
            .credits
            .as_ref()
            .map(|c| {
                c.crew
                    .iter()
                    .filter(|m| m.job == "Director")
                    .map(|m| Person {
                        name: m.name.clone(),
                        role: Some("Director".to_string()),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut cast_members: Vec<_> = detail
            .credits
            .as_ref()
            .map(|c| c.cast.clone())
            .unwrap_or_default();
        cast_members.sort_by_key(|c| c.order.unwrap_or(999));

        let cast = cast_members
            .into_iter()
            .map(|c| Person {
                name: c.name,
                role: c.character,
            })
            .collect();

        let genres = detail
            .genres
            .into_iter()
            .map(|g| Genre { name: g.name })
            .collect();

        let countries = detail
            .production_countries
            .into_iter()
            .map(|c| Country {
                name: c.name,
                iso_code: Some(c.iso_3166_1),
            })
            .collect();

        let mut ratings = Vec::new();
        if let Some(vote) = detail.vote_average {
            if vote > 0.0 {
                ratings.push(Rating {
                    source: "TMDB".to_string(),
                    value: vote,
                    max: 10.0,
                });
            }
        }

        Ok(Movie {
            title: detail.title,
            original_title: detail.original_title,
            year,
            release_date: detail.release_date,
            duration_minutes: detail.runtime,
            synopsis: detail.overview,
            poster_url: detail.poster_path.map(|p| TmdbClient::poster_url(&p)),
            backdrop_url: detail.backdrop_path.map(|p| TmdbClient::backdrop_url(&p)),
            genres,
            countries,
            directors,
            cast,
            ratings,
            tmdb_id: Some(detail.id),
            imdb_id: detail.imdb_id,
            allocine_url: None,
        })
    }
}
