use crate::cli::Commands;
use prezmaker_lib::config::Config;
use prezmaker_lib::error::PrezError;
use prezmaker_lib::formatters::{app_fmt, game_fmt, movie_fmt, series_fmt};
use prezmaker_lib::models::Application;
use prezmaker_lib::providers::allocine::AllocineClient;
use prezmaker_lib::providers::igdb::IgdbClient;
use prezmaker_lib::providers::llm::LlmClient;
use prezmaker_lib::providers::tmdb::TmdbClient;
use prezmaker_lib::providers::translator::ClaudeClient;
use prezmaker_lib::providers::wikipedia::WikipediaClient;
use prezmaker_lib::models::Tracker;
use prezmaker_lib::providers::{GameProvider, MovieProvider, SeriesProvider};
use prezmaker_lib::models::TechInfo;
use dialoguer::{Input, Select};
use tracing::{info, warn};

pub struct Orchestrator {
    config: Config,
    language: String,
    title_color: String,
    tracker: Tracker,
}

impl Orchestrator {
    pub fn new(config: Config, language: Option<String>, title_color: Option<String>, tracker: Tracker) -> Self {
        let lang = language.unwrap_or_else(|| config.preferences.language.clone());
        let color = title_color.unwrap_or_else(|| config.preferences.title_color.clone());
        Self {
            config,
            language: lang,
            title_color: color,
            tracker,
        }
    }

    pub async fn run(&self, command: &Commands) -> Result<String, PrezError> {
        match command {
            Commands::Film { query, no_allocine } => {
                self.handle_film(query, *no_allocine).await
            }
            Commands::Serie { query, no_allocine } => {
                self.handle_serie(query, *no_allocine).await
            }
            Commands::Jeu { query } => self.handle_jeu(query).await,
            cmd @ Commands::App { .. } => self.handle_app(cmd),
        }
    }

    async fn handle_film(&self, query: &str, no_allocine: bool) -> Result<String, PrezError> {
        let api_key = self.config.tmdb_api_key()?;
        let tmdb = TmdbClient::new(api_key.to_string(), self.language.clone());

        info!("Recherche film : {}", query);
        let results = tmdb.search_movies(query).await.map_err(|e| {
            PrezError::Other(format!("Erreur recherche TMDB : {}", e))
        })?;

        if results.is_empty() {
            return Err(PrezError::NoResults(query.to_string()));
        }

        let selected_id = self.select_from_results(
            &results
                .iter()
                .map(|m| {
                    format!(
                        "{} ({})",
                        m.title,
                        m.year.map(|y| y.to_string()).unwrap_or_default()
                    )
                })
                .collect::<Vec<_>>(),
        )?;

        let tmdb_id = results[selected_id].tmdb_id.unwrap();
        let mut movie = tmdb.get_movie_details(tmdb_id).await.map_err(|e| {
            PrezError::Other(format!("Erreur details TMDB : {}", e))
        })?;

        // Allocine enrichment
        if !no_allocine {
            match self.enrich_movie_allocine(&mut movie).await {
                Ok(_) => info!("Notes Allocine recuperees"),
                Err(e) => warn!("Allocine indisponible : {}", e),
            }
        }

        let bbcode = movie_fmt::format_movie(&movie, &self.title_color, self.tracker);
        Ok(bbcode)
    }

    async fn handle_serie(&self, query: &str, no_allocine: bool) -> Result<String, PrezError> {
        let api_key = self.config.tmdb_api_key()?;
        let tmdb = TmdbClient::new(api_key.to_string(), self.language.clone());

        info!("Recherche serie : {}", query);
        let results = tmdb.search_series(query).await.map_err(|e| {
            PrezError::Other(format!("Erreur recherche TMDB : {}", e))
        })?;

        if results.is_empty() {
            return Err(PrezError::NoResults(query.to_string()));
        }

        let selected_id = self.select_from_results(
            &results
                .iter()
                .map(|s| {
                    format!(
                        "{} ({})",
                        s.title,
                        s.year.map(|y| y.to_string()).unwrap_or_default()
                    )
                })
                .collect::<Vec<_>>(),
        )?;

        let tmdb_id = results[selected_id].tmdb_id.unwrap();
        let mut series = tmdb.get_series_details(tmdb_id).await.map_err(|e| {
            PrezError::Other(format!("Erreur details TMDB : {}", e))
        })?;

        // Allocine enrichment
        if !no_allocine {
            match self.enrich_series_allocine(&mut series).await {
                Ok(_) => info!("Notes Allocine recuperees"),
                Err(e) => warn!("Allocine indisponible : {}", e),
            }
        }

        let bbcode = series_fmt::format_series(&series, &self.title_color, self.tracker);
        Ok(bbcode)
    }

    async fn handle_jeu(&self, query: &str) -> Result<String, PrezError> {
        let (client_id, client_secret) = self.config.igdb_credentials()?;
        let igdb = IgdbClient::new(client_id.to_string(), client_secret.to_string());

        info!("Recherche jeu : {}", query);
        let results = igdb.search_games(query).await.map_err(|e| {
            PrezError::Other(format!("Erreur recherche IGDB : {}", e))
        })?;

        if results.is_empty() {
            return Err(PrezError::NoResults(query.to_string()));
        }

        let selected_id = self.select_from_results(
            &results
                .iter()
                .map(|g| {
                    format!(
                        "{} ({})",
                        g.title,
                        g.year.map(|y| y.to_string()).unwrap_or_default()
                    )
                })
                .collect::<Vec<_>>(),
        )?;

        let igdb_id = results[selected_id].igdb_id.unwrap();
        let mut game = igdb.get_game_details(igdb_id).await.map_err(|e| {
            PrezError::Other(format!("Erreur details IGDB : {}", e))
        })?;

        // Description en francais : LLM > Claude CLI > Wikipedia FR > saisie manuelle
        game.synopsis = self.resolve_french_description(&game.title, game.synopsis.as_deref()).await?;

        // Saisie des etapes d'installation
        game.installation = self.prompt_installation()?;

        // Saisie interactive des informations techniques
        game.tech_info = Some(self.prompt_tech_info()?);

        let bbcode = game_fmt::format_game(&game, &self.title_color, self.tracker);
        Ok(bbcode)
    }

    fn handle_app(&self, command: &Commands) -> Result<String, PrezError> {
        let Commands::App {
            name,
            version,
            developer,
            description,
            website,
            license,
            logo,
            platforms,
        } = command
        else {
            unreachable!()
        };

        let app = Application {
            name: name.clone(),
            version: version.clone(),
            developer: developer.clone(),
            description: description.clone(),
            website: website.clone(),
            license: license.clone(),
            platforms: platforms
                .as_deref()
                .map(|p| p.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default(),
            logo_url: logo.clone(),
        };

        let bbcode = app_fmt::format_application(&app, &self.title_color, self.tracker);
        Ok(bbcode)
    }

    async fn enrich_movie_allocine(
        &self,
        movie: &mut prezmaker_lib::models::Movie,
    ) -> anyhow::Result<()> {
        let allocine = AllocineClient::new();
        let ratings = allocine
            .search_movie_ratings(&movie.title, movie.year)
            .await?;
        let extra_ratings = AllocineClient::ratings_to_vec(&ratings);
        movie.ratings.extend(extra_ratings);
        Ok(())
    }

    async fn enrich_series_allocine(
        &self,
        series: &mut prezmaker_lib::models::Series,
    ) -> anyhow::Result<()> {
        let allocine = AllocineClient::new();
        let ratings = allocine
            .search_series_ratings(&series.title, series.year)
            .await?;
        let extra_ratings = AllocineClient::ratings_to_vec(&ratings);
        series.ratings.extend(extra_ratings);
        Ok(())
    }

    async fn resolve_french_description(
        &self,
        game_title: &str,
        english: Option<&str>,
    ) -> Result<Option<String>, PrezError> {
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
                        eprintln!("\n--- Description (generee par {}) ---", provider);
                        eprintln!("{}", desc);
                        return Ok(Some(desc));
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
                    eprintln!("\n--- Description (generee par Claude) ---");
                    eprintln!("{}", description);
                    return Ok(Some(description));
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
                eprintln!("\n--- Description (Wikipedia FR) ---");
                eprintln!("{}", desc);
                return Ok(Some(desc));
            }
            Ok(None) => warn!("Aucune description Wikipedia pour {}", game_title),
            Err(e) => warn!("Erreur Wikipedia : {}", e),
        }

        // 4. Saisie manuelle
        if let Some(en) = english {
            eprintln!("\n--- Description (EN) ---");
            eprintln!("{}", en);
        }
        eprintln!();

        let input: String = Input::new()
            .with_prompt("Description en francais (laisser vide pour garder l'anglais)")
            .allow_empty(true)
            .interact_text()
            .map_err(|_| PrezError::Cancelled)?;

        if input.is_empty() {
            Ok(english.map(String::from))
        } else {
            Ok(Some(input))
        }
    }

    fn prompt_installation(&self) -> Result<Option<String>, PrezError> {
        eprintln!("\n--- Installation (une etape par ligne, ligne vide pour terminer) ---");

        let mut steps = Vec::new();
        let mut step_num = 1;

        loop {
            let input: String = Input::new()
                .with_prompt(format!("Etape {}", step_num))
                .allow_empty(true)
                .interact_text()
                .map_err(|_| PrezError::Cancelled)?;

            if input.is_empty() {
                break;
            }

            steps.push(input);
            step_num += 1;
        }

        if steps.is_empty() {
            Ok(None)
        } else {
            let formatted = steps
                .iter()
                .enumerate()
                .map(|(i, s)| format!("{}. {}", i + 1, s))
                .collect::<Vec<_>>()
                .join("\n");
            Ok(Some(formatted))
        }
    }

    fn prompt_tech_info(&self) -> Result<TechInfo, PrezError> {
        eprintln!("\n--- Informations techniques ---");

        let platform: String = Input::new()
            .with_prompt("Plateforme")
            .allow_empty(true)
            .interact_text()
            .map_err(|_| PrezError::Cancelled)?;

        let languages: String = Input::new()
            .with_prompt("Langue(s)")
            .allow_empty(true)
            .interact_text()
            .map_err(|_| PrezError::Cancelled)?;

        let size: String = Input::new()
            .with_prompt("Taille")
            .allow_empty(true)
            .interact_text()
            .map_err(|_| PrezError::Cancelled)?;

        Ok(TechInfo {
            platform,
            languages,
            size,
        })
    }

    fn select_from_results(&self, items: &[String]) -> Result<usize, PrezError> {
        if items.len() == 1 {
            return Ok(0);
        }

        Select::new()
            .with_prompt("Plusieurs resultats trouves, selectionnez")
            .items(items)
            .default(0)
            .interact()
            .map_err(|_| PrezError::Cancelled)
    }
}
