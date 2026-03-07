use crate::bbcode_to_html;
use prezmaker_lib::config::Config;
use prezmaker_lib::models::{Application, Game, TechInfo, Tracker};
use prezmaker_lib::orchestrator_api::{GameDetailsResponse, OrchestratorApi, SearchResult};
use serde::Deserialize;
use std::sync::Arc;

pub struct AppState {
    pub config: Arc<Config>,
}

fn parse_tracker(tracker: &str) -> Tracker {
    match tracker {
        "torr.xyz" | "TorrXyz" => Tracker::TorrXyz,
        _ => Tracker::C411,
    }
}

fn make_api(config: &Config, tracker: &str, title_color: Option<&str>) -> OrchestratorApi {
    let t = parse_tracker(tracker);
    let mut api = OrchestratorApi::new(config.clone(), None, None, t);
    if let Some(color) = title_color {
        if !color.is_empty() {
            api.set_title_color(color.to_string());
        }
    }
    api
}

#[tauri::command]
pub async fn search(
    state: tauri::State<'_, AppState>,
    query: String,
    content_type: String,
    tracker: String,
    title_color: Option<String>,
) -> Result<Vec<SearchResult>, String> {
    let api = make_api(&state.config, &tracker, title_color.as_deref());

    match content_type.as_str() {
        "film" => api.search_film(&query).await.map_err(|e| e.to_string()),
        "serie" => api.search_serie(&query).await.map_err(|e| e.to_string()),
        "jeu" => api.search_jeu(&query).await.map_err(|e| e.to_string()),
        _ => Err(format!("Type inconnu: {}", content_type)),
    }
}

#[tauri::command]
pub async fn generate_film(
    state: tauri::State<'_, AppState>,
    tmdb_id: u64,
    tracker: String,
    title_color: Option<String>,
) -> Result<String, String> {
    let api = make_api(&state.config, &tracker, title_color.as_deref());
    api.generate_film(tmdb_id, false)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_serie(
    state: tauri::State<'_, AppState>,
    tmdb_id: u64,
    tracker: String,
    title_color: Option<String>,
) -> Result<String, String> {
    let api = make_api(&state.config, &tracker, title_color.as_deref());
    api.generate_serie(tmdb_id, false)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_game_details(
    state: tauri::State<'_, AppState>,
    igdb_id: u64,
    tracker: String,
    title_color: Option<String>,
) -> Result<GameDetailsResponse, String> {
    let api = make_api(&state.config, &tracker, title_color.as_deref());
    api.fetch_game_details(igdb_id)
        .await
        .map_err(|e| e.to_string())
}

#[derive(Deserialize)]
pub struct GenerateJeuPayload {
    pub game: Game,
    pub description: Option<String>,
    pub installation: Option<String>,
    pub tech_info: TechInfo,
}

#[tauri::command]
pub async fn generate_jeu(
    state: tauri::State<'_, AppState>,
    payload: GenerateJeuPayload,
    tracker: String,
    title_color: Option<String>,
) -> Result<String, String> {
    let api = make_api(&state.config, &tracker, title_color.as_deref());
    api.generate_jeu(
        payload.game,
        payload.description,
        payload.installation,
        payload.tech_info,
    )
    .map_err(|e| e.to_string())
}

#[derive(Deserialize)]
pub struct GenerateAppPayload {
    pub name: String,
    pub version: Option<String>,
    pub developer: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
    pub license: Option<String>,
    pub platforms: Vec<String>,
    pub logo_url: Option<String>,
}

#[tauri::command]
pub async fn generate_app(
    state: tauri::State<'_, AppState>,
    payload: GenerateAppPayload,
    tracker: String,
    title_color: Option<String>,
) -> Result<String, String> {
    let api = make_api(&state.config, &tracker, title_color.as_deref());
    let app = Application {
        name: payload.name,
        version: payload.version,
        developer: payload.developer,
        description: payload.description,
        website: payload.website,
        license: payload.license,
        platforms: payload.platforms,
        logo_url: payload.logo_url,
    };
    api.generate_app(app).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn convert_bbcode(bbcode: String) -> String {
    bbcode_to_html::convert_bbcode_to_html(&bbcode)
}
