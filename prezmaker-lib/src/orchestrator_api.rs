use crate::config::Config;
use crate::error::PrezError;
use crate::formatters::{app_fmt, game_fmt, movie_fmt, series_fmt};
use crate::formatters::bbcode;
use crate::models::{Application, Game, MediaTechInfo, Movie, Series, TechInfo};
use crate::template_engine::{self, RenderContext};
use crate::providers::allocine::AllocineClient;
use crate::providers::igdb::IgdbClient;
use crate::providers::steam::SteamClient;
use crate::providers::tmdb::TmdbClient;
use crate::providers::llm::LlmClient;
use crate::providers::translator::ClaudeClient;
use crate::providers::wikipedia::WikipediaClient;
use crate::providers::{GameProvider, MovieProvider, SeriesProvider};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: u64,
    pub label: String,
    #[serde(default)]
    pub source: Option<String>,
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
    pseudo: String,
}

impl OrchestratorApi {
    pub fn new(
        config: Config,
        language: Option<String>,
        title_color: Option<String>,
    ) -> Self {
        let lang = language.unwrap_or_else(|| config.preferences.language.clone());
        let color = title_color.unwrap_or_else(|| config.preferences.title_color.clone());
        let pseudo = config.preferences.pseudo.clone();
        Self {
            config,
            language: lang,
            title_color: color,
            pseudo,
        }
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
                    source: None,
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
                    source: None,
                })
            })
            .collect())
    }

    pub async fn search_jeu(&self, query: &str) -> Result<Vec<SearchResult>, PrezError> {
        info!("Recherche jeu : {}", query);

        // Try IGDB first if configured
        if let Ok((client_id, client_secret)) = self.config.igdb_credentials() {
            match IgdbClient::new(client_id.to_string(), client_secret.to_string())
                .search_games(query)
                .await
            {
                Ok(results) if !results.is_empty() => {
                    info!("IGDB : {} resultats", results.len());
                    return Ok(results
                        .into_iter()
                        .filter_map(|g| {
                            g.igdb_id.map(|id| SearchResult {
                                id,
                                label: format!(
                                    "{} ({})",
                                    g.title,
                                    g.year.map(|y| y.to_string()).unwrap_or_default()
                                ),
                                source: Some("igdb".to_string()),
                            })
                        })
                        .collect());
                }
                Ok(_) => info!("IGDB : aucun resultat, fallback Steam"),
                Err(e) => warn!("IGDB indisponible ({}), fallback Steam", e),
            }
        } else {
            info!("IGDB non configure, recherche Steam directe");
        }

        // Fallback: Steam (no API key needed)
        let steam = SteamClient::new(self.language.clone());
        let results = steam
            .search_games(query)
            .await
            .map_err(|e| PrezError::Other(format!("Erreur recherche Steam : {}", e)))?;

        info!("Steam : {} resultats", results.len());
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
                    source: Some("steam".to_string()),
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
            &self.pseudo,
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
            &self.pseudo,
        ))
    }

    pub async fn generate_film_with_tech(
        &self,
        tmdb_id: u64,
        no_allocine: bool,
        tech: MediaTechInfo,
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

        Ok(movie_fmt::format_movie_with_tech(
            &movie,
            &self.title_color,
            Some(&tech),
            &self.pseudo,
        ))
    }

    pub async fn generate_serie_with_tech(
        &self,
        tmdb_id: u64,
        no_allocine: bool,
        tech: MediaTechInfo,
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

        Ok(series_fmt::format_series_with_tech(
            &series,
            &self.title_color,
            Some(&tech),
            &self.pseudo,
        ))
    }

    pub async fn fetch_game_details(
        &self,
        game_id: u64,
        source: Option<&str>,
    ) -> Result<GameDetailsResponse, PrezError> {
        let game = match source.unwrap_or("igdb") {
            "steam" => {
                info!("Recuperation details Steam : {}", game_id);
                let steam = SteamClient::new(self.language.clone());
                steam
                    .get_game_details(game_id)
                    .await
                    .map_err(|e| PrezError::Other(format!("Erreur details Steam : {}", e)))?
            }
            _ => {
                info!("Recuperation details IGDB : {}", game_id);
                let (client_id, client_secret) = self.config.igdb_credentials()?;
                let igdb = IgdbClient::new(client_id.to_string(), client_secret.to_string());
                igdb.get_game_details(game_id)
                    .await
                    .map_err(|e| PrezError::Other(format!("Erreur details IGDB : {}", e)))?
            }
        };

        let claude_description = self
            .resolve_description(&game.title, game.synopsis.as_deref())
            .await;

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
            &self.pseudo,
        ))
    }

    pub fn generate_app(&self, app: Application) -> Result<String, PrezError> {
        Ok(app_fmt::format_application(
            &app,
            &self.title_color,
            &self.pseudo,
        ))
    }

    // --- Template-based generation ---

    pub async fn generate_film_from_template(
        &self,
        tmdb_id: u64,
        no_allocine: bool,
        tech: Option<MediaTechInfo>,
        template_name: &str,
    ) -> Result<String, PrezError> {
        let api_key = self.config.tmdb_api_key()?;
        let tmdb = TmdbClient::new(api_key.to_string(), self.language.clone());
        let mut movie = tmdb.get_movie_details(tmdb_id).await
            .map_err(|e| PrezError::Other(format!("Erreur details TMDB : {}", e)))?;
        if !no_allocine {
            match Self::enrich_movie_allocine(&mut movie).await {
                Ok(_) => info!("Notes Allocine recuperees"),
                Err(e) => warn!("Allocine indisponible : {}", e),
            }
        }

        let tpl = template_engine::get_template("film", template_name)
            .map_err(|e| PrezError::Other(e))?;
        let data = template_engine::build_movie_data(&movie, tech.as_ref());

        // Build info BBCode for poster_info composite
        let info_bbcode = self.build_movie_info_bbcode(&movie);
        let ctx = RenderContext {
            ratings: movie.ratings.clone(),
            poster_url: movie.poster_url.clone(),
            tech,
            info_bbcode: Some(info_bbcode),
            ..Default::default()
        };

        Ok(template_engine::render(&tpl.body, &data, &ctx, &self.title_color, &self.pseudo))
    }

    pub async fn generate_serie_from_template(
        &self,
        tmdb_id: u64,
        no_allocine: bool,
        tech: Option<MediaTechInfo>,
        template_name: &str,
    ) -> Result<String, PrezError> {
        let api_key = self.config.tmdb_api_key()?;
        let tmdb = TmdbClient::new(api_key.to_string(), self.language.clone());
        let mut series = tmdb.get_series_details(tmdb_id).await
            .map_err(|e| PrezError::Other(format!("Erreur details TMDB : {}", e)))?;
        if !no_allocine {
            match Self::enrich_series_allocine(&mut series).await {
                Ok(_) => info!("Notes Allocine recuperees"),
                Err(e) => warn!("Allocine indisponible : {}", e),
            }
        }

        let tpl = template_engine::get_template("serie", template_name)
            .map_err(|e| PrezError::Other(e))?;
        let data = template_engine::build_series_data(&series, tech.as_ref());

        let info_bbcode = self.build_series_info_bbcode(&series);
        let ctx = RenderContext {
            ratings: series.ratings.clone(),
            poster_url: series.poster_url.clone(),
            tech,
            info_bbcode: Some(info_bbcode),
            ..Default::default()
        };

        Ok(template_engine::render(&tpl.body, &data, &ctx, &self.title_color, &self.pseudo))
    }

    pub fn generate_jeu_from_template(
        &self,
        mut game: Game,
        description: Option<String>,
        installation: Option<String>,
        tech_info: TechInfo,
        template_name: &str,
    ) -> Result<String, PrezError> {
        if let Some(desc) = description {
            game.synopsis = Some(desc);
        }
        game.installation = installation;
        game.tech_info = Some(tech_info);

        let tpl = template_engine::get_template("jeu", template_name)
            .map_err(|e| PrezError::Other(e))?;
        let data = template_engine::build_game_data(&game);

        let info_bbcode = self.build_game_info_bbcode(&game);
        let ctx = RenderContext {
            ratings: game.ratings.clone(),
            cover_url: game.cover_url.clone(),
            screenshots: game.screenshots.clone(),
            game_tech: game.tech_info.clone(),
            min_reqs: game.min_reqs.clone(),
            rec_reqs: game.rec_reqs.clone(),
            info_bbcode: Some(info_bbcode),
            ..Default::default()
        };

        Ok(template_engine::render(&tpl.body, &data, &ctx, &self.title_color, &self.pseudo))
    }

    pub fn generate_app_from_template(
        &self,
        app: Application,
        template_name: &str,
    ) -> Result<String, PrezError> {
        let tpl = template_engine::get_template("app", template_name)
            .map_err(|e| PrezError::Other(e))?;
        let data = template_engine::build_app_data(&app);

        let info_bbcode = self.build_app_info_bbcode(&app);
        let ctx = RenderContext {
            logo_url: app.logo_url.clone(),
            info_bbcode: Some(info_bbcode),
            ..Default::default()
        };

        Ok(template_engine::render(&tpl.body, &data, &ctx, &self.title_color, &self.pseudo))
    }

    // --- Info BBCode builders (for poster_info/cover_info composites) ---

    fn build_movie_info_bbcode(&self, movie: &Movie) -> String {
        let mut info = String::new();
        if !movie.countries.is_empty() {
            info.push_str(&bbcode::field("Origine", &movie.countries_display()));
            info.push('\n');
        }
        if let Some(ref date) = movie.release_date {
            info.push_str(&bbcode::field("Sortie", &template_engine::format_date_fr_pub(date)));
            info.push('\n');
        }
        if let Some(ref dur) = movie.duration_formatted() {
            info.push_str(&bbcode::field("Duree", dur));
            info.push('\n');
        }
        if !movie.directors.is_empty() {
            info.push_str(&bbcode::field("Realisateur", &movie.directors_display()));
            info.push('\n');
        }
        if !movie.genres.is_empty() {
            info.push_str(&bbcode::field("Genres", &movie.genres_display()));
            info.push('\n');
        }
        if !movie.cast.is_empty() {
            info.push('\n');
            info.push_str(&bbcode::inline_heading("Casting", &self.title_color));
            info.push_str("\n\n");
            info.push_str(&bbcode::field("Acteurs", &movie.cast_display(6)));
            info.push('\n');
        }
        info
    }

    fn build_series_info_bbcode(&self, series: &Series) -> String {
        let mut info = String::new();
        if !series.countries.is_empty() {
            info.push_str(&bbcode::field("Origine", &series.countries_display()));
            info.push('\n');
        }
        if let Some(ref date) = series.first_air_date {
            info.push_str(&bbcode::field("Premiere diffusion", &template_engine::format_date_fr_pub(date)));
            info.push('\n');
        }
        if let Some(ref status) = series.status {
            info.push_str(&bbcode::field("Statut", &template_engine::translate_status_pub(status)));
            info.push('\n');
        }
        if let Some(seasons) = series.seasons_count {
            info.push_str(&bbcode::field("Saisons", &seasons.to_string()));
            info.push('\n');
        }
        if let Some(episodes) = series.episodes_count {
            info.push_str(&bbcode::field("Episodes", &episodes.to_string()));
            info.push('\n');
        }
        if let Some(ref runtime) = series.runtime_formatted() {
            info.push_str(&bbcode::field("Duree par episode", runtime));
            info.push('\n');
        }
        if !series.creators.is_empty() {
            info.push_str(&bbcode::field("Createur(s)", &series.creators_display()));
            info.push('\n');
        }
        if !series.networks.is_empty() {
            info.push_str(&bbcode::field("Chaine / Plateforme", &series.networks_display()));
            info.push('\n');
        }
        if !series.genres.is_empty() {
            info.push_str(&bbcode::field("Genres", &series.genres_display()));
            info.push('\n');
        }
        if !series.cast.is_empty() {
            info.push('\n');
            info.push_str(&bbcode::inline_heading("Casting", &self.title_color));
            info.push_str("\n\n");
            info.push_str(&bbcode::field("Acteurs", &series.cast_display(8)));
            info.push('\n');
        }
        info
    }

    fn build_game_info_bbcode(&self, game: &Game) -> String {
        let mut info = String::new();
        if let Some(ref date) = game.release_date {
            info.push_str(&bbcode::field("Date de sortie", date));
            info.push('\n');
        }
        if !game.developers.is_empty() {
            info.push_str(&bbcode::field("Developpeur(s)", &game.developers_display()));
            info.push('\n');
        }
        if !game.publishers.is_empty() {
            info.push_str(&bbcode::field("Editeur(s)", &game.publishers_display()));
            info.push('\n');
        }
        if !game.genres.is_empty() {
            info.push_str(&bbcode::field("Genres", &game.genres_display()));
            info.push('\n');
        }
        info
    }

    fn build_app_info_bbcode(&self, app: &Application) -> String {
        let mut info = String::new();
        info.push_str(&bbcode::field("Nom", &app.name));
        info.push('\n');
        if let Some(ref version) = app.version {
            info.push_str(&bbcode::field("Version", version));
            info.push('\n');
        }
        if let Some(ref dev) = app.developer {
            info.push_str(&bbcode::field("Developpeur", dev));
            info.push('\n');
        }
        if let Some(ref license) = app.license {
            info.push_str(&bbcode::field("Licence", license));
            info.push('\n');
        }
        if let Some(ref website) = app.website {
            info.push_str(&bbcode::field("Site web", &bbcode::url(website, website)));
            info.push('\n');
        }
        if !app.platforms.is_empty() {
            info.push_str(&bbcode::field("Plateformes", &app.platforms_display()));
            info.push('\n');
        }
        info
    }

    async fn enrich_movie_allocine(movie: &mut Movie) -> anyhow::Result<()> {
        let title = movie.title.clone();
        let year = movie.year;
        let (extra_ratings, page_url) = tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Handle::current();
            rt.block_on(async {
                let allocine = AllocineClient::new();
                let ratings = allocine.search_movie_ratings(&title, year).await?;
                let url = ratings.page_url.clone();
                Ok::<_, anyhow::Error>((AllocineClient::ratings_to_vec(&ratings), url))
            })
        })
        .await??;
        movie.ratings.extend(extra_ratings);
        if movie.allocine_url.is_none() {
            movie.allocine_url = page_url;
        }
        Ok(())
    }

    async fn enrich_series_allocine(series: &mut Series) -> anyhow::Result<()> {
        let title = series.title.clone();
        let year = series.year;
        let (extra_ratings, page_url) = tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Handle::current();
            rt.block_on(async {
                let allocine = AllocineClient::new();
                let ratings = allocine.search_series_ratings(&title, year).await?;
                let url = ratings.page_url.clone();
                Ok::<_, anyhow::Error>((AllocineClient::ratings_to_vec(&ratings), url))
            })
        })
        .await??;
        series.ratings.extend(extra_ratings);
        if series.allocine_url.is_none() {
            series.allocine_url = page_url;
        }
        Ok(())
    }

    async fn resolve_description(&self, game_title: &str, english: Option<&str>) -> Option<String> {
        // 1. LLM API (si configuré)
        if let (Some(provider), Some(api_key)) = (
            self.config.llm.provider.as_deref(),
            self.config.llm.api_key.as_deref(),
        ) {
            if !provider.is_empty() && !api_key.is_empty() {
                info!("Generation description via LLM ({})...", provider);
                let client = LlmClient::new(provider, api_key);
                match client.generate_game_description(game_title, english).await {
                    Ok(desc) if !desc.is_empty() => {
                        info!("Description LLM generee !");
                        return Some(desc);
                    }
                    Ok(_) => warn!("LLM a retourne une description vide"),
                    Err(e) => warn!("Erreur LLM : {}", e),
                }
            }
        }

        // 2. Claude CLI (si disponible)
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

        // 3. Wikipedia FR
        info!("Recherche description sur Wikipedia FR...");
        let wiki = WikipediaClient::new();
        match wiki.search_game_description(game_title).await {
            Ok(Some(desc)) => {
                info!("Description Wikipedia trouvee !");
                return Some(desc);
            }
            Ok(None) => warn!("Aucune description Wikipedia pour {}", game_title),
            Err(e) => warn!("Erreur Wikipedia : {}", e),
        }

        // 4. Fallback → None (synopsis EN sera utilisé)
        None
    }
}
