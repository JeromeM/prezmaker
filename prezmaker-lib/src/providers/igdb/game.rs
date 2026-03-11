use super::models::IgdbGame;
use super::IgdbClient;
use crate::models::{Game, Genre, Rating};
use crate::providers::GameProvider;
use async_trait::async_trait;
use tracing::debug;

impl IgdbClient {
    async fn igdb_query(&self, endpoint: &str, body: &str) -> anyhow::Result<Vec<IgdbGame>> {
        let token = self.get_access_token().await?;
        let url = format!("https://api.igdb.com/v4/{}", endpoint);
        debug!("IGDB query: {} -> {}", endpoint, body);

        let resp = self
            .client
            .post(&url)
            .header("Client-ID", &self.client_id)
            .header("Authorization", format!("Bearer {}", token))
            .body(body.to_string())
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }
}

#[async_trait]
impl GameProvider for IgdbClient {
    async fn search_games(&self, query: &str) -> anyhow::Result<Vec<Game>> {
        let body = format!(
            r#"search "{}"; fields name,slug,summary,first_release_date,cover.image_id,total_rating; limit 10;"#,
            query.replace('"', "\\\"")
        );

        let results = self.igdb_query("games", &body).await?;

        let games = results
            .into_iter()
            .map(|g| {
                let year = g.first_release_date.map(chrono_timestamp_to_year);

                Game {
                    title: g.name,
                    release_date: g
                        .first_release_date
                        .map(format_timestamp),
                    year,
                    synopsis: g.summary,
                    cover_url: g.cover.map(|c| c.url_big()),
                    screenshots: vec![],
                    genres: vec![],
                    platforms: vec![],
                    developers: vec![],
                    publishers: vec![],
                    ratings: g
                        .total_rating
                        .filter(|&v| v > 0.0)
                        .map(|v| {
                            vec![Rating {
                                source: "IGDB".to_string(),
                                value: (v / 10.0 * 10.0).round() / 10.0,
                                max: 100.0,
                            }]
                        })
                        .unwrap_or_default(),
                    igdb_id: Some(g.id),
                    igdb_slug: g.slug,
                    steam_appid: None,
                    tech_info: None,
                    installation: None,
                }
            })
            .collect();

        Ok(games)
    }

    async fn get_game_details(&self, id: u64) -> anyhow::Result<Game> {
        let body = format!(
            r#"where id = {}; fields name,slug,summary,first_release_date,genres.name,platforms.name,involved_companies.company.name,involved_companies.developer,involved_companies.publisher,cover.image_id,screenshots.image_id,total_rating,aggregated_rating;"#,
            id
        );

        let mut results = self.igdb_query("games", &body).await?;
        let g = results
            .pop()
            .ok_or_else(|| anyhow::anyhow!("Game not found: {}", id))?;

        let year = g.first_release_date.map(chrono_timestamp_to_year);

        let genres = g
            .genres
            .unwrap_or_default()
            .into_iter()
            .map(|g| Genre { name: g.name })
            .collect();

        let platforms = g
            .platforms
            .unwrap_or_default()
            .into_iter()
            .map(|p| p.name)
            .collect();

        let companies = g.involved_companies.unwrap_or_default();
        let developers = companies
            .iter()
            .filter(|c| c.developer)
            .map(|c| c.company.name.clone())
            .collect();
        let publishers = companies
            .iter()
            .filter(|c| c.publisher)
            .map(|c| c.company.name.clone())
            .collect();

        let screenshots = g
            .screenshots
            .unwrap_or_default()
            .into_iter()
            .map(|s| s.url_hd())
            .collect();

        let mut ratings = Vec::new();
        if let Some(total) = g.total_rating {
            if total > 0.0 {
                ratings.push(Rating {
                    source: "IGDB".to_string(),
                    value: (total * 10.0).round() / 10.0,
                    max: 100.0,
                });
            }
        }
        if let Some(agg) = g.aggregated_rating {
            if agg > 0.0 {
                ratings.push(Rating {
                    source: "Presse".to_string(),
                    value: (agg * 10.0).round() / 10.0,
                    max: 100.0,
                });
            }
        }

        Ok(Game {
            title: g.name,
            release_date: g.first_release_date.map(format_timestamp),
            year,
            synopsis: g.summary,
            cover_url: g.cover.map(|c| c.url_big()),
            screenshots,
            genres,
            platforms,
            developers,
            publishers,
            ratings,
            igdb_id: Some(id),
            igdb_slug: g.slug,
            steam_appid: None,
            tech_info: None,
            installation: None,
        })
    }
}

fn chrono_timestamp_to_year(ts: i64) -> u16 {
    // Unix timestamp to year
    let secs_per_year = 365.25 * 24.0 * 3600.0;
    (1970.0 + (ts as f64 / secs_per_year)) as u16
}

fn format_timestamp(ts: i64) -> String {
    // Simple date formatting from unix timestamp
    let days = ts / 86400;
    let mut y = 1970i32;
    let mut remaining = days;

    loop {
        let days_in_year = if is_leap(y) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        y += 1;
    }

    let months_days = if is_leap(y) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut m = 0;
    for (i, &md) in months_days.iter().enumerate() {
        if remaining < md {
            m = i + 1;
            break;
        }
        remaining -= md;
    }

    let d = remaining + 1;
    format!("{:02}/{:02}/{}", d, m, y)
}

fn is_leap(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}
