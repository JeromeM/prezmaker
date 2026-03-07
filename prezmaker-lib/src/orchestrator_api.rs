use crate::config::Config;
use crate::error::PrezError;
use crate::formatters::{app_fmt, game_fmt, movie_fmt, series_fmt};
use crate::models::{Application, Game, Movie, Series, TechInfo, Tracker};
use crate::providers::allocine::AllocineClient;
use crate::providers::igdb::IgdbClient;
use crate::providers::tmdb::TmdbClient;
use crate::providers::translator::ClaudeClient;
use crate::providers::{GameProvider, MovieProvider, SeriesProvider};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: u64,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDetailsResponse {
    pub game: Game,
    pub claude_description: Option<String>,
}

pub struct OrchestratorApi {
    config: Config,
    language: String,
    title_color: String,
    tracker: Tracker,
}

impl OrchestratorApi {
    pub fn new(
        config: Config,
        language: Option<String>,
        title_color: Option<String>,
        tracker: Tracker,
    ) -> Self {
        let lang = language.unwrap_or_else(|| config.preferences.language.clone());
        let color = title_color.unwrap_or_else(|| config.preferences.title_color.clone());
        Self {
            config,
            language: lang,
            title_color: color,
            tracker,
        }
    }

    pub fn set_tracker(&mut self, tracker: Tracker) {
        self.tracker = tracker;
    }

    pub fn set_title_color(&mut self, color: String) {
        self.title_color = color;
    }

    pub async fn search_film(&self, query: &str) -> Result<Vec<SearchResult>, PrezError> {
        let api_key = self.config.tmdb_api_key()?;
        let tmdb = TmdbClient::new(api_key.to_string(), self.language.clone());

        info!("Recherche film : {}", query);
        let results = tmdb
            .search_movies(query)
            .await
            .map_err(|e| PrezError::Other(format!("Erreur recherche TMDB : {}", e)))?;

        Ok(results
            .into_iter()
            .filter_map(|m| {
                m.tmdb_id.map(|id| SearchResult {
                    id,
                    label: format!(
                        "{} ({})",
                        m.title,
                        m.year.map(|y| y.to_string()).unwrap_or_default()
                    ),
                })
            })
            .collect())
    }

    pub async fn search_serie(&self, query: &str) -> Result<Vec<SearchResult>, PrezError> {
        let api_key = self.config.tmdb_api_key()?;
        let tmdb = TmdbClient::new(api_key.to_string(), self.language.clone());

        info!("Recherche serie : {}", query);
        let results = tmdb
            .search_series(query)
            .await
            .map_err(|e| PrezError::Other(format!("Erreur recherche TMDB : {}", e)))?;

        Ok(results
            .into_iter()
            .filter_map(|s| {
                s.tmdb_id.map(|id| SearchResult {
                    id,
                    label: format!(
                        "{} ({})",
                        s.title,
                        s.year.map(|y| y.to_string()).unwrap_or_default()
                    ),
                })
            })
            .collect())
    }

    pub async fn search_jeu(&self, query: &str) -> Result<Vec<SearchResult>, PrezError> {
        let (client_id, client_secret) = self.config.igdb_credentials()?;
        let igdb = IgdbClient::new(client_id.to_string(), client_secret.to_string());

        info!("Recherche jeu : {}", query);
        let results = igdb
            .search_games(query)
            .await
            .map_err(|e| PrezError::Other(format!("Erreur recherche IGDB : {}", e)))?;

        Ok(results
            .into_iter()
            .filter_map(|g| {
                g.igdb_id.map(|id| SearchResult {
                    id,
                    label: format!(
                        "{} ({})",
                        g.title,
                        g.year.map(|y| y.to_string()).unwrap_or_default()
                    ),
                })
            })
            .collect())
    }

    pub async fn generate_film(
        &self,
        tmdb_id: u64,
        no_allocine: bool,
    ) -> Result<String, PrezError> {
        let api_key = self.config.tmdb_api_key()?;
        let tmdb = TmdbClient::new(api_key.to_string(), self.language.clone());

        let mut movie = tmdb
            .get_movie_details(tmdb_id)
            .await
            .map_err(|e| PrezError::Other(format!("Erreur details TMDB : {}", e)))?;

        if !no_allocine {
            match Self::enrich_movie_allocine(&mut movie).await {
                Ok(_) => info!("Notes Allocine recuperees"),
                Err(e) => warn!("Allocine indisponible : {}", e),
            }
        }

        Ok(movie_fmt::format_movie(
            &movie,
            &self.title_color,
            self.tracker,
        ))
    }

    pub async fn generate_serie(
        &self,
        tmdb_id: u64,
        no_allocine: bool,
    ) -> Result<String, PrezError> {
        let api_key = self.config.tmdb_api_key()?;
        let tmdb = TmdbClient::new(api_key.to_string(), self.language.clone());

        let mut series = tmdb
            .get_series_details(tmdb_id)
            .await
            .map_err(|e| PrezError::Other(format!("Erreur details TMDB : {}", e)))?;

        if !no_allocine {
            match Self::enrich_series_allocine(&mut series).await {
                Ok(_) => info!("Notes Allocine recuperees"),
                Err(e) => warn!("Allocine indisponible : {}", e),
            }
        }

        Ok(series_fmt::format_series(
            &series,
            &self.title_color,
            self.tracker,
        ))
    }

    pub async fn fetch_game_details(
        &self,
        igdb_id: u64,
    ) -> Result<GameDetailsResponse, PrezError> {
        let (client_id, client_secret) = self.config.igdb_credentials()?;
        let igdb = IgdbClient::new(client_id.to_string(), client_secret.to_string());

        let game = igdb
            .get_game_details(igdb_id)
            .await
            .map_err(|e| PrezError::Other(format!("Erreur details IGDB : {}", e)))?;

        let claude_description = Self::try_claude_description(&game.title, game.synopsis.as_deref());

        Ok(GameDetailsResponse {
            game,
            claude_description,
        })
    }

    pub fn generate_jeu(
        &self,
        mut game: Game,
        description: Option<String>,
        installation: Option<String>,
        tech_info: TechInfo,
    ) -> Result<String, PrezError> {
        if let Some(desc) = description {
            game.synopsis = Some(desc);
        }
        game.installation = installation;
        game.tech_info = Some(tech_info);

        Ok(game_fmt::format_game(
            &game,
            &self.title_color,
            self.tracker,
        ))
    }

    pub fn generate_app(&self, app: Application) -> Result<String, PrezError> {
        Ok(app_fmt::format_application(
            &app,
            &self.title_color,
            self.tracker,
        ))
    }

    async fn enrich_movie_allocine(movie: &mut Movie) -> anyhow::Result<()> {
        let title = movie.title.clone();
        let year = movie.year;
        let extra_ratings = tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Handle::current();
            rt.block_on(async {
                let allocine = AllocineClient::new();
                let ratings = allocine.search_movie_ratings(&title, year).await?;
                Ok::<_, anyhow::Error>(AllocineClient::ratings_to_vec(&ratings))
            })
        })
        .await??;
        movie.ratings.extend(extra_ratings);
        Ok(())
    }

    async fn enrich_series_allocine(series: &mut Series) -> anyhow::Result<()> {
        let title = series.title.clone();
        let year = series.year;
        let extra_ratings = tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Handle::current();
            rt.block_on(async {
                let allocine = AllocineClient::new();
                let ratings = allocine.search_series_ratings(&title, year).await?;
                Ok::<_, anyhow::Error>(AllocineClient::ratings_to_vec(&ratings))
            })
        })
        .await??;
        series.ratings.extend(extra_ratings);
        Ok(())
    }

    fn try_claude_description(game_title: &str, english: Option<&str>) -> Option<String> {
        let claude = ClaudeClient::new();
        if claude.is_available() {
            info!("Generation description via claude CLI...");
            match claude.write_game_description(game_title, english) {
                Ok(description) => {
                    info!("Description Claude generee !");
                    return Some(description);
                }
                Err(e) => warn!("Erreur claude CLI : {}", e),
            }
        }
        None
    }
}
