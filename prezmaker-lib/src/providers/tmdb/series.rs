use super::models::{TmdbSearchResponse, TmdbTvDetail, TmdbTvSearchResult};
use super::TmdbClient;
use crate::models::{Country, Genre, Person, Rating, Series};
use crate::providers::SeriesProvider;
use async_trait::async_trait;
use tracing::debug;

#[async_trait]
impl SeriesProvider for TmdbClient {
    async fn search_series(&self, query: &str) -> anyhow::Result<Vec<Series>> {
        let url = format!("{}/search/tv", self.base_url);
        debug!("TMDB TV search: {}", query);

        let resp: TmdbSearchResponse<TmdbTvSearchResult> = self
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

        let series = resp
            .results
            .into_iter()
            .map(|r| {
                let year = r
                    .first_air_date
                    .as_ref()
                    .and_then(|d| d.split('-').next())
                    .and_then(|y| y.parse().ok());

                Series {
                    title: r.name,
                    original_title: r.original_name,
                    year,
                    end_year: None,
                    first_air_date: r.first_air_date,
                    synopsis: r.overview,
                    poster_url: r.poster_path.map(|p| TmdbClient::poster_url(&p)),
                    backdrop_url: None,
                    genres: vec![],
                    countries: vec![],
                    creators: vec![],
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
                    seasons_count: None,
                    episodes_count: None,
                    episode_runtime: None,
                    status: None,
                    networks: vec![],
                    tmdb_id: Some(r.id),
                    imdb_id: None,
                    allocine_url: None,
                    trailer_url: None,
                }
            })
            .collect();

        Ok(series)
    }

    async fn get_series_details(&self, id: u64) -> anyhow::Result<Series> {
        let url = format!("{}/tv/{}", self.base_url, id);
        debug!("TMDB TV details: {}", id);

        let detail: TmdbTvDetail = self
            .client
            .get(&url)
            .query(&[
                ("api_key", self.api_key.as_str()),
                ("language", self.language.as_str()),
                ("append_to_response", "credits,external_ids,videos"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let year = detail
            .first_air_date
            .as_ref()
            .and_then(|d| d.split('-').next())
            .and_then(|y| y.parse().ok());

        let end_year = detail
            .last_air_date
            .as_ref()
            .and_then(|d| d.split('-').next())
            .and_then(|y| y.parse().ok());

        let creators = detail
            .created_by
            .iter()
            .map(|c| Person {
                name: c.name.clone(),
                role: Some("Creator".to_string()),
            })
            .collect();

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

        let networks = detail.networks.into_iter().map(|n| n.name).collect();

        let episode_runtime = detail
            .episode_run_time
            .as_ref()
            .and_then(|rts| rts.first().copied());

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

        let imdb_id = detail.external_ids.and_then(|e| e.imdb_id);

        Ok(Series {
            title: detail.name,
            original_title: detail.original_name,
            year,
            end_year,
            first_air_date: detail.first_air_date,
            synopsis: detail.overview,
            poster_url: detail.poster_path.map(|p| TmdbClient::poster_url(&p)),
            backdrop_url: detail.backdrop_path.map(|p| TmdbClient::backdrop_url(&p)),
            genres,
            countries,
            creators,
            cast,
            ratings,
            seasons_count: detail.number_of_seasons,
            episodes_count: detail.number_of_episodes,
            episode_runtime,
            status: detail.status,
            networks,
            tmdb_id: Some(detail.id),
            imdb_id,
            allocine_url: None,
            trailer_url: extract_trailer_url(&detail.videos),
        })
    }
}

fn extract_trailer_url(videos: &Option<super::models::TmdbVideos>) -> Option<String> {
    let videos = videos.as_ref()?;
    // Prefer official trailer, then any trailer, then any teaser
    let best = videos.results.iter()
        .filter(|v| v.site == "YouTube")
        .max_by_key(|v| {
            let type_score = match v.video_type.as_str() {
                "Trailer" => 3,
                "Teaser" => 2,
                _ => 1,
            };
            let official_score = if v.official.unwrap_or(false) { 10 } else { 0 };
            type_score + official_score
        })?;
    Some(format!("https://www.youtube.com/watch?v={}", best.key))
}
