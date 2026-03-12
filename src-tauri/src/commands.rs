use crate::bbcode_to_html;
use prezmaker_lib::config::Config;
use prezmaker_lib::models::{Application, Game, MediaTechInfo, SystemReqs, TechInfo};
use prezmaker_lib::providers::llm::LlmClient;
use prezmaker_lib::orchestrator_api::{GameDetailsResponse, OrchestratorApi, SearchResult};
use prezmaker_lib::torrent::{self, TorrentInfo};
use prezmaker_lib::template_engine::{self, ContentTemplate, TemplateTag};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub config: Arc<Mutex<Config>>,
}

fn make_api(config: &Config, title_color: Option<&str>) -> OrchestratorApi {
    let mut api = OrchestratorApi::new(config.clone(), None, None);
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
    title_color: Option<String>,
) -> Result<Vec<SearchResult>, String> {
    let config = state.config.lock().unwrap().clone();
    let api = make_api(&config, title_color.as_deref());

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
    title_color: Option<String>,
) -> Result<String, String> {
    let config = state.config.lock().unwrap().clone();
    let api = make_api(&config, title_color.as_deref());
    api.generate_film(tmdb_id, false)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_serie(
    state: tauri::State<'_, AppState>,
    tmdb_id: u64,
    title_color: Option<String>,
) -> Result<String, String> {
    let config = state.config.lock().unwrap().clone();
    let api = make_api(&config, title_color.as_deref());
    api.generate_serie(tmdb_id, false)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_game_details(
    state: tauri::State<'_, AppState>,
    game_id: u64,
    source: Option<String>,
    title_color: Option<String>,
) -> Result<GameDetailsResponse, String> {
    let config = state.config.lock().unwrap().clone();
    let api = make_api(&config, title_color.as_deref());
    api.fetch_game_details(game_id, source.as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[derive(Serialize)]
pub struct SteamReqsResponse {
    pub min_reqs: Option<SystemReqs>,
    pub rec_reqs: Option<SystemReqs>,
}

#[tauri::command]
pub async fn fetch_steam_requirements(
    game_title: String,
) -> Result<SteamReqsResponse, String> {
    use prezmaker_lib::providers::steam::SteamClient;
    use prezmaker_lib::providers::GameProvider;

    let steam = SteamClient::new("fr".to_string());
    let results = steam
        .search_games(&game_title)
        .await
        .map_err(|e| format!("Recherche Steam échouée: {}", e))?;

    let first = results
        .into_iter()
        .next()
        .ok_or_else(|| format!("Aucun jeu trouvé sur Steam pour: {}", game_title))?;

    let appid = first
        .steam_appid
        .ok_or("Pas d'App ID Steam")?;

    let details = steam
        .get_game_details(appid)
        .await
        .map_err(|e| format!("Détails Steam échoués: {}", e))?;

    Ok(SteamReqsResponse {
        min_reqs: details.min_reqs,
        rec_reqs: details.rec_reqs,
    })
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
    title_color: Option<String>,
) -> Result<String, String> {
    let config = state.config.lock().unwrap().clone();
    let api = make_api(&config, title_color.as_deref());
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
    title_color: Option<String>,
) -> Result<String, String> {
    let config = state.config.lock().unwrap().clone();
    let api = make_api(&config, title_color.as_deref());
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
pub fn parse_torrent(path: String) -> Result<TorrentInfo, String> {
    torrent::analyze_torrent(Path::new(&path))
}

#[tauri::command]
pub async fn generate_film_with_tech(
    state: tauri::State<'_, AppState>,
    tmdb_id: u64,
    title_color: Option<String>,
    tech: MediaTechInfo,
) -> Result<String, String> {
    let config = state.config.lock().unwrap().clone();
    let api = make_api(&config, title_color.as_deref());
    api.generate_film_with_tech(tmdb_id, false, tech)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_serie_with_tech(
    state: tauri::State<'_, AppState>,
    tmdb_id: u64,
    title_color: Option<String>,
    tech: MediaTechInfo,
) -> Result<String, String> {
    let config = state.config.lock().unwrap().clone();
    let api = make_api(&config, title_color.as_deref());
    api.generate_serie_with_tech(tmdb_id, false, tech)
        .await
        .map_err(|e| e.to_string())
}

// --- Content Templates ---

#[tauri::command]
pub fn preview_template(
    state: tauri::State<'_, AppState>,
    body: String,
    content_type: String,
    title_color: Option<String>,
) -> String {
    let config = state.config.lock().unwrap();
    let color = title_color.as_deref().unwrap_or("c0392b");
    let pseudo = &config.preferences.pseudo;
    template_engine::preview_template(&body, &content_type, color, pseudo)
}

#[tauri::command]
pub fn list_content_templates(content_type: String) -> Result<Vec<ContentTemplate>, String> {
    template_engine::list_templates(&content_type)
}

#[tauri::command]
pub fn get_content_template(content_type: String, name: String) -> Result<ContentTemplate, String> {
    template_engine::get_template(&content_type, &name)
}

#[tauri::command]
pub fn save_content_template(content_type: String, name: String, body: String, title_color: Option<String>) -> Result<(), String> {
    template_engine::save_template(&content_type, &name, &body)?;
    template_engine::save_template_meta(&content_type, &name, title_color)
}

#[tauri::command]
pub fn delete_content_template(content_type: String, name: String) -> Result<(), String> {
    template_engine::delete_template(&content_type, &name)
}

#[tauri::command]
pub fn duplicate_content_template(content_type: String, name: String, new_name: String) -> Result<(), String> {
    template_engine::duplicate_template(&content_type, &name, &new_name)
}

#[tauri::command]
pub fn get_template_tags(content_type: String) -> Vec<TemplateTag> {
    template_engine::get_available_tags(&content_type)
}

// --- Template-based generation ---

#[tauri::command]
pub async fn generate_from_template(
    state: tauri::State<'_, AppState>,
    content_type: String,
    tmdb_id: Option<u64>,
    title_color: Option<String>,
    template_name: String,
    tech: Option<MediaTechInfo>,
    game_payload: Option<GenerateJeuPayload>,
    app_payload: Option<GenerateAppPayload>,
) -> Result<String, String> {
    let config = state.config.lock().unwrap().clone();
    // Per-template title_color overrides global
    let tpl_meta = template_engine::get_template(&content_type, &template_name).ok();
    let effective_color = tpl_meta.and_then(|t| t.title_color).or(title_color);
    let api = make_api(&config, effective_color.as_deref());

    match content_type.as_str() {
        "film" => {
            let id = tmdb_id.ok_or("tmdb_id required for film")?;
            api.generate_film_from_template(id, false, tech, &template_name)
                .await
                .map_err(|e| e.to_string())
        }
        "serie" => {
            let id = tmdb_id.ok_or("tmdb_id required for serie")?;
            api.generate_serie_from_template(id, false, tech, &template_name)
                .await
                .map_err(|e| e.to_string())
        }
        "jeu" => {
            let p = game_payload.ok_or("game_payload required for jeu")?;
            api.generate_jeu_from_template(p.game, p.description, p.installation, p.tech_info, &template_name)
                .map_err(|e| e.to_string())
        }
        "app" => {
            let p = app_payload.ok_or("app_payload required for app")?;
            let app = Application {
                name: p.name,
                version: p.version,
                developer: p.developer,
                description: p.description,
                website: p.website,
                license: p.license,
                platforms: p.platforms,
                logo_url: p.logo_url,
            };
            api.generate_app_from_template(app, &template_name)
                .map_err(|e| e.to_string())
        }
        _ => Err(format!("Unknown content type: {}", content_type)),
    }
}

#[tauri::command]
pub fn convert_bbcode(bbcode: String) -> String {
    bbcode_to_html::convert_bbcode_to_html(&bbcode)
}

#[tauri::command]
pub async fn generate_nfo(
    state: tauri::State<'_, AppState>,
    bbcode: String,
) -> Result<String, String> {
    let config = state.config.lock().unwrap().clone();
    let provider = config.llm.provider.as_deref().unwrap_or("");
    let api_key = config.llm.api_key.as_deref().unwrap_or("");

    if provider.is_empty() || api_key.is_empty() {
        return Err("LLM non configuré. Allez dans les paramètres pour configurer un provider LLM.".to_string());
    }

    let client = LlmClient::new(provider, api_key);
    client
        .generate_nfo(&bbcode)
        .await
        .map_err(|e| e.to_string())
}

// --- Settings ---

#[derive(Serialize, Deserialize, Clone)]
pub struct SettingsPayload {
    pub tmdb_api_key: Option<String>,
    pub igdb_client_id: Option<String>,
    pub igdb_client_secret: Option<String>,
    pub language: String,
    pub title_color: String,
    #[serde(default)]
    pub default_templates: HashMap<String, String>,
    pub auto_clipboard: bool,
    pub llm_provider: Option<String>,
    pub llm_api_key: Option<String>,
    pub pseudo: String,
}

#[tauri::command]
pub fn get_settings(state: tauri::State<'_, AppState>) -> SettingsPayload {
    let config = state.config.lock().unwrap();
    SettingsPayload {
        tmdb_api_key: config.tmdb.api_key.clone(),
        igdb_client_id: config.igdb.client_id.clone(),
        igdb_client_secret: config.igdb.client_secret.clone(),
        language: config.preferences.language.clone(),
        title_color: config.preferences.title_color.clone(),
        default_templates: config.preferences.default_templates.clone(),
        auto_clipboard: config.preferences.auto_clipboard,
        llm_provider: config.llm.provider.clone(),
        llm_api_key: config.llm.api_key.clone(),
        pseudo: config.preferences.pseudo.clone(),
    }
}

#[tauri::command]
pub fn save_settings(
    state: tauri::State<'_, AppState>,
    settings: SettingsPayload,
) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    config.tmdb.api_key = settings.tmdb_api_key;
    config.igdb.client_id = settings.igdb_client_id;
    config.igdb.client_secret = settings.igdb_client_secret;
    config.preferences.language = settings.language;
    config.preferences.title_color = settings.title_color;
    config.preferences.auto_clipboard = settings.auto_clipboard;
    config.llm.provider = settings.llm_provider;
    config.llm.api_key = settings.llm_api_key;
    config.preferences.pseudo = settings.pseudo;
    config.preferences.default_templates = settings.default_templates;
    config.save().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_default_template(
    state: tauri::State<'_, AppState>,
    content_type: String,
    template_name: String,
) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    if template_name.is_empty() {
        config.preferences.default_templates.remove(&content_type);
    } else {
        config.preferences.default_templates.insert(content_type, template_name);
    }
    config.save().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_default_template(
    state: tauri::State<'_, AppState>,
    content_type: String,
) -> Option<String> {
    let config = state.config.lock().unwrap();
    config.preferences.default_templates.get(&content_type).cloned()
}

// --- Templates ---

fn templates_dir() -> Result<PathBuf, String> {
    let dir = dirs::config_dir()
        .ok_or_else(|| "Cannot find config directory".to_string())?
        .join("prezmaker")
        .join("templates");
    std::fs::create_dir_all(&dir).map_err(|e| format!("Cannot create templates dir: {}", e))?;
    Ok(dir)
}

fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' || c == '.' { c } else { '_' })
        .collect::<String>()
        .trim()
        .to_string()
}

fn template_path(name: &str) -> Result<PathBuf, String> {
    let safe = sanitize_name(name);
    if safe.is_empty() {
        return Err("Template name is empty".to_string());
    }
    Ok(templates_dir()?.join(format!("{}.bbcode", safe)))
}

#[derive(Serialize)]
pub struct TemplateInfo {
    pub name: String,
    pub size: u64,
    pub modified: u64,
}

#[tauri::command]
pub fn list_templates() -> Result<Vec<TemplateInfo>, String> {
    let dir = templates_dir()?;
    let mut templates = Vec::new();
    let entries = std::fs::read_dir(&dir).map_err(|e| e.to_string())?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("bbcode") {
            if let Ok(meta) = path.metadata() {
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                let modified = meta
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                templates.push(TemplateInfo {
                    name,
                    size: meta.len(),
                    modified,
                });
            }
        }
    }
    templates.sort_by(|a, b| b.modified.cmp(&a.modified));
    Ok(templates)
}

#[tauri::command]
pub fn load_template(name: String) -> Result<String, String> {
    let path = template_path(&name)?;
    std::fs::read_to_string(&path).map_err(|e| format!("Cannot read template: {}", e))
}

#[tauri::command]
pub fn save_template(name: String, content: String) -> Result<(), String> {
    let path = template_path(&name)?;
    std::fs::write(&path, content).map_err(|e| format!("Cannot write template: {}", e))
}

#[tauri::command]
pub fn delete_template(name: String) -> Result<(), String> {
    let path = template_path(&name)?;
    std::fs::remove_file(&path).map_err(|e| format!("Cannot delete template: {}", e))
}

#[tauri::command]
pub fn rename_template(old_name: String, new_name: String) -> Result<(), String> {
    let old_path = template_path(&old_name)?;
    let new_path = template_path(&new_name)?;
    if new_path.exists() {
        return Err(format!("Template '{}' already exists", new_name));
    }
    std::fs::rename(&old_path, &new_path).map_err(|e| format!("Cannot rename template: {}", e))
}

#[tauri::command]
pub fn duplicate_template(name: String, new_name: String) -> Result<(), String> {
    let src = template_path(&name)?;
    let dst = template_path(&new_name)?;
    if dst.exists() {
        return Err(format!("Template '{}' already exists", new_name));
    }
    std::fs::copy(&src, &dst).map_err(|e| format!("Cannot duplicate template: {}", e))?;
    Ok(())
}
