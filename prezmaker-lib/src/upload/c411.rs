use crate::error::PrezError;
use crate::torrent::ReleaseParsed;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use std::time::Duration;

const BASE_URL: &str = "https://c411.org/api";

/// Écrit un journal d'upload dans le dossier config de PrezMaker.
/// Fichier : `%APPDATA%/prezmaker/upload_log.txt` (Windows) ou `~/.config/prezmaker/upload_log.txt` (Linux).
fn log_upload_request(
    title: &str,
    description: &str,
    category_id: u32,
    subcategory_id: u32,
    options_json: &str,
    uploader_note: Option<&str>,
    description_format: Option<&DescriptionFormat>,
    tmdb_data: Option<&str>,
    rawg_data: Option<&str>,
    torrent_filename: &str,
    nfo_len: usize,
) {
    let log_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("prezmaker");
    let log_path = log_dir.join("upload_log.txt");

    let mut file = match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        Ok(f) => f,
        Err(_) => return,
    };

    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    let fmt_str = match description_format {
        Some(DescriptionFormat::Html) => "html",
        Some(DescriptionFormat::Standard) | None => "standard",
    };

    let _ = writeln!(file, "═══════════════════════════════════════════════════════════");
    let _ = writeln!(file, "  UPLOAD C411 — {}", now);
    let _ = writeln!(file, "═══════════════════════════════════════════════════════════");
    let _ = writeln!(file);
    let _ = writeln!(file, "POST {}/torrents", BASE_URL);
    let _ = writeln!(file, "Content-Type: multipart/form-data");
    let _ = writeln!(file, "Authorization: Bearer ***");
    let _ = writeln!(file);
    let _ = writeln!(file, "── Champs ─────────────────────────────────────────────────");
    let _ = writeln!(file, "title             = {}", title);
    let _ = writeln!(file, "categoryId        = {}", category_id);
    let _ = writeln!(file, "subcategoryId     = {}", subcategory_id);
    let _ = writeln!(file, "descriptionFormat = {}", fmt_str);
    let _ = writeln!(file, "options           = {}", options_json);
    if let Some(note) = uploader_note {
        let _ = writeln!(file, "uploaderNote      = {}", note);
    }
    let _ = writeln!(file);
    let _ = writeln!(file, "── Fichiers ───────────────────────────────────────────────");
    let _ = writeln!(file, "torrent           = {} (fichier binaire)", torrent_filename);
    let _ = writeln!(file, "nfo               = release.nfo ({} octets)", nfo_len);
    let _ = writeln!(file);
    let _ = writeln!(file, "── Description ({}) ────────────────────────────────────", fmt_str);
    let _ = writeln!(file, "{}", description);
    let _ = writeln!(file);

    if let Some(tmdb) = tmdb_data {
        let _ = writeln!(file, "── tmdbData ───────────────────────────────────────────────");
        // Pretty-print le JSON pour lisibilité
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(tmdb) {
            let _ = writeln!(file, "{}", serde_json::to_string_pretty(&parsed).unwrap_or_else(|_| tmdb.to_string()));
        } else {
            let _ = writeln!(file, "{}", tmdb);
        }
        let _ = writeln!(file);
    }

    if let Some(rawg) = rawg_data {
        let _ = writeln!(file, "── rawgData ───────────────────────────────────────────────");
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(rawg) {
            let _ = writeln!(file, "{}", serde_json::to_string_pretty(&parsed).unwrap_or_else(|_| rawg.to_string()));
        } else {
            let _ = writeln!(file, "{}", rawg);
        }
        let _ = writeln!(file);
    }

    let _ = writeln!(file);
}

fn log_upload_response(status: u16, body: &str) {
    let log_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("prezmaker");
    let log_path = log_dir.join("upload_log.txt");

    let mut file = match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        Ok(f) => f,
        Err(_) => return,
    };

    let _ = writeln!(file, "── Réponse ────────────────────────────────────────────────");
    let _ = writeln!(file, "HTTP {}", status);
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body) {
        let _ = writeln!(file, "{}", serde_json::to_string_pretty(&parsed).unwrap_or_else(|_| body.to_string()));
    } else {
        let _ = writeln!(file, "{}", body);
    }
    let _ = writeln!(file);
    let _ = writeln!(file);
}

/// Formats de description acceptés par l'API C411.
/// - `Standard` : BBCode converti automatiquement en HTML côté serveur.
/// - `Html` : HTML brut sanitisé (nécessite la permission `torrent:use_html_prez`).
///   Éléments interdits : script, style, link, iframe, svg, canvas, form, input,
///   button, select, textarea, math, dialog, template, object, embed, video, audio.
///   Attributs interdits : class, id, data-*, on*.
///   Images : src doit pointer vers un domaine de la liste blanche (TMDB, RAWG…).
///   Liens : protocoles https://, http://, mailto: uniquement.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DescriptionFormat {
    #[default]
    Standard,
    Html,
}

/// Wrapper for API responses: `{ "data": [...] }`
#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    data: T,
}

// --- Types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C411Category {
    pub id: u32,
    pub name: String,
    pub subcategories: Vec<C411Subcategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C411Subcategory {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct C411OptionType {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub allows_multiple: bool,
    pub is_required: bool,
    pub sort_order: u32,
    pub values: Vec<C411OptionValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct C411OptionValue {
    pub id: u32,
    pub value: String,
    pub slug: String,
    pub sort_order: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C411UploadResult {
    pub success: bool,
    pub message: Option<String>,
}

// --- Client ---

pub struct C411Client {
    client: reqwest::Client,
    api_key: String,
}

impl C411Client {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to build C411 HTTP client"),
            api_key,
        }
    }

    /// GET /api/categories
    pub async fn fetch_categories(&self) -> Result<Vec<C411Category>, PrezError> {
        let resp = self
            .client
            .get(format!("{}/categories", BASE_URL))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .error_for_status()
            .map_err(|e| PrezError::Upload(format!("Fetch categories failed: {}", e)))?;

        let wrapper: ApiResponse<Vec<C411Category>> = resp.json().await?;
        Ok(wrapper.data)
    }

    /// GET /api/categories/{subcategoryId}/options
    pub async fn fetch_options(
        &self,
        subcategory_id: u32,
    ) -> Result<Vec<C411OptionType>, PrezError> {
        let resp = self
            .client
            .get(format!("{}/categories/{}/options", BASE_URL, subcategory_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .error_for_status()
            .map_err(|e| PrezError::Upload(format!("Fetch options failed: {}", e)))?;

        let wrapper: ApiResponse<Vec<C411OptionType>> = resp.json().await?;
        Ok(wrapper.data)
    }

    /// POST /api/torrents (multipart/form-data)
    pub async fn upload(
        &self,
        torrent_path: &Path,
        nfo_content: &str,
        title: &str,
        description: &str,
        category_id: u32,
        subcategory_id: u32,
        options_json: &str,
        uploader_note: Option<&str>,
        description_format: Option<&DescriptionFormat>,
        tmdb_data: Option<&str>,
        rawg_data: Option<&str>,
    ) -> Result<C411UploadResult, PrezError> {
        let torrent_bytes = std::fs::read(torrent_path)
            .map_err(|e| PrezError::Upload(format!("Cannot read torrent file: {}", e)))?;

        let torrent_filename = torrent_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("release.torrent")
            .to_string();

        // Journal d'upload
        log_upload_request(
            title,
            description,
            category_id,
            subcategory_id,
            options_json,
            uploader_note,
            description_format,
            tmdb_data,
            rawg_data,
            &torrent_filename,
            nfo_content.len(),
        );

        let mut form = reqwest::multipart::Form::new()
            .part(
                "torrent",
                reqwest::multipart::Part::bytes(torrent_bytes)
                    .file_name(torrent_filename)
                    .mime_str("application/x-bittorrent")
                    .map_err(|e| PrezError::Upload(e.to_string()))?,
            )
            .part(
                "nfo",
                reqwest::multipart::Part::bytes(nfo_content.as_bytes().to_vec())
                    .file_name("release.nfo")
                    .mime_str("text/plain")
                    .map_err(|e| PrezError::Upload(e.to_string()))?,
            )
            .text("title", title.to_string())
            .text("description", description.to_string())
            .text("categoryId", category_id.to_string())
            .text("subcategoryId", subcategory_id.to_string())
            .text("options", options_json.to_string());

        if let Some(note) = uploader_note {
            form = form.text("uploaderNote", note.to_string());
        }

        if let Some(fmt) = description_format {
            let value = match fmt {
                DescriptionFormat::Standard => "standard",
                DescriptionFormat::Html => "html",
            };
            form = form.text("descriptionFormat", value.to_string());
        }

        if let Some(tmdb) = tmdb_data {
            form = form.text("tmdbData", tmdb.to_string());
        }

        if let Some(rawg) = rawg_data {
            form = form.text("rawgData", rawg.to_string());
        }

        let resp = self
            .client
            .post(format!("{}/torrents", BASE_URL))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;

        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();

        // Log de la réponse
        log_upload_response(status.as_u16(), &body);

        if status.is_success() {
            match serde_json::from_str::<C411UploadResult>(&body) {
                Ok(result) => Ok(result),
                Err(_) => Ok(C411UploadResult {
                    success: true,
                    message: Some("Upload réussi".to_string()),
                }),
            }
        } else {
            Err(PrezError::Upload(format!(
                "Upload failed (HTTP {}): {}",
                status, body
            )))
        }
    }
}

// --- Auto-mapping ---

/// Maps PrezMaker contentType to C411 (categoryId, subcategoryId)
pub fn auto_map_category(content_type: &str) -> (u32, u32) {
    match content_type {
        "film" => (1, 6),
        "serie" => (1, 7),
        "jeu" => (5, 36),
        "app" => (4, 27),
        _ => (1, 6), // default to film
    }
}

/// Builds a pre-filled options JSON from ReleaseParsed + available options
pub fn auto_map_options(
    parsed: &ReleaseParsed,
    available_options: &[C411OptionType],
) -> serde_json::Value {
    let mut result = serde_json::Map::new();

    for opt_type in available_options {
        match opt_type.id {
            // Langue (type 1)
            1 => {
                if let Some(lang) = &parsed.language {
                    let value_id = map_language(lang);
                    if let Some(id) = value_id {
                        if opt_type.allows_multiple {
                            result.insert(
                                opt_type.id.to_string(),
                                serde_json::json!([id]),
                            );
                        } else {
                            result.insert(
                                opt_type.id.to_string(),
                                serde_json::json!(id),
                            );
                        }
                    }
                }
            }
            // Qualité (type 2)
            2 => {
                if let Some(quality) = &parsed.quality {
                    let value_id = map_quality(quality);
                    if let Some(id) = value_id {
                        result.insert(
                            opt_type.id.to_string(),
                            serde_json::json!(id),
                        );
                    }
                }
            }
            // Épisode (type 6)
            6 => {
                let value_id = match parsed.episode {
                    None => Some(96),                    // saison complète
                    Some(n) if n > 0 => Some(96 + n),   // épisode N
                    _ => None,
                };
                if let Some(id) = value_id {
                    result.insert(
                        opt_type.id.to_string(),
                        serde_json::json!(id),
                    );
                }
            }
            // Saison (type 7)
            7 => {
                if let Some(season) = parsed.season {
                    let value_id = 120 + season;
                    result.insert(
                        opt_type.id.to_string(),
                        serde_json::json!(value_id),
                    );
                }
            }
            // Genre Jeux (type 23) - multi-sélection, pas de mapping auto
            23 => {}
            _ => {}
        }
    }

    serde_json::Value::Object(result)
}

fn map_language(lang: &str) -> Option<u32> {
    let upper = lang.to_uppercase();
    match upper.as_str() {
        "MULTI" => Some(4),       // Multi (FR inclus)
        "MULTI VF2" => Some(422), // Multi VF2 (FR+QC)
        "FRENCH" | "VFF" => Some(2), // Français (VFF)
        "VOSTFR" => Some(8),
        "VFQ" => Some(6),         // Québécois (VFQ)
        "ENGLISH" | "ENG" => Some(1),
        _ => None,
    }
}

fn map_quality(quality: &str) -> Option<u32> {
    let q = quality.to_uppercase();
    // Ordre : du plus spécifique au plus général
    if q.contains("REMUX") && q.contains("2160") {
        Some(10)  // BluRay 4K (remux 4K)
    } else if q.contains("REMUX") {
        Some(12)  // BluRay Remux
    } else if q.contains("2160") && (q.contains("WEB-DL") || q.contains("WEBDL") || q.contains("WEB")) {
        Some(26)  // WEB-DL 4K
    } else if q.contains("2160") && q.contains("BLURAY") {
        Some(10)  // BluRay 4K
    } else if q.contains("1080") && (q.contains("WEB-DL") || q.contains("WEBDL") || q.contains("WEB")) {
        Some(25)  // WEB-DL 1080
    } else if q.contains("1080") && q.contains("BLURAY") {
        Some(413) // BluRay 1080 (existant)
    } else if q.contains("1080") && (q.contains("HDRIP") || q.contains("BDRIP") || q.contains("BRRIP")) {
        Some(16)  // HDRip 1080
    } else if q.contains("720") {
        Some(24)
    } else if q.contains("480") || q.contains("DVDRIP") {
        Some(23)  // SD / DVDRip
    } else {
        None
    }
}

// --- TMDB metadata for C411 ---

/// Cherche un film/série sur TMDB par titre et retourne les métadonnées au format C411.
pub async fn fetch_tmdb_metadata_for_c411(
    api_key: &str,
    title: &str,
    content_type: &str,
    language: &str,
) -> Result<serde_json::Value, PrezError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| PrezError::Upload(e.to_string()))?;

    let base = "https://api.themoviedb.org/3";

    let (search_type, detail_type) = match content_type {
        "serie" => ("tv", "tv"),
        _ => ("movie", "movie"),
    };

    // 1. Recherche
    let search: serde_json::Value = client
        .get(format!("{}/search/{}", base, search_type))
        .query(&[("api_key", api_key), ("query", title), ("language", language)])
        .send()
        .await?
        .json()
        .await
        .map_err(|e| PrezError::Upload(format!("TMDB search failed: {}", e)))?;

    let first = search["results"]
        .as_array()
        .and_then(|r| r.first())
        .ok_or_else(|| PrezError::Upload(format!("Aucun résultat TMDB pour \"{}\"", title)))?;

    let tmdb_id = first["id"]
        .as_u64()
        .ok_or_else(|| PrezError::Upload("TMDB id manquant".to_string()))?;

    // 2. Détails avec credits
    let detail: serde_json::Value = client
        .get(format!("{}/{}/{}", base, detail_type, tmdb_id))
        .query(&[("api_key", api_key), ("language", language), ("append_to_response", "credits")])
        .send()
        .await?
        .json()
        .await
        .map_err(|e| PrezError::Upload(format!("TMDB detail failed: {}", e)))?;

    Ok(detail)
}

// --- RAWG metadata for C411 ---

/// Cherche un jeu sur RAWG par titre et retourne les métadonnées au format C411.
pub async fn fetch_rawg_metadata_for_c411(
    api_key: &str,
    title: &str,
) -> Result<serde_json::Value, PrezError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| PrezError::Upload(e.to_string()))?;

    // 1. Recherche
    let search: serde_json::Value = client
        .get("https://api.rawg.io/api/games")
        .query(&[("key", api_key), ("search", title), ("page_size", "1")])
        .send()
        .await?
        .json()
        .await
        .map_err(|e| PrezError::Upload(format!("RAWG search failed: {}", e)))?;

    let first = search["results"]
        .as_array()
        .and_then(|r| r.first())
        .ok_or_else(|| PrezError::Upload(format!("Aucun résultat RAWG pour \"{}\"", title)))?;

    let slug = first["slug"]
        .as_str()
        .ok_or_else(|| PrezError::Upload("RAWG slug manquant".to_string()))?;

    // 2. Détails complets
    let detail: serde_json::Value = client
        .get(format!("https://api.rawg.io/api/games/{}", slug))
        .query(&[("key", api_key)])
        .send()
        .await?
        .json()
        .await
        .map_err(|e| PrezError::Upload(format!("RAWG detail failed: {}", e)))?;

    Ok(detail)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::torrent::DetectedContentType;

    #[test]
    fn test_auto_map_category() {
        assert_eq!(auto_map_category("film"), (1, 6));
        assert_eq!(auto_map_category("serie"), (1, 7));
        assert_eq!(auto_map_category("jeu"), (5, 36));
        assert_eq!(auto_map_category("app"), (4, 27));
    }

    #[test]
    fn test_map_language() {
        assert_eq!(map_language("MULTI"), Some(4));
        assert_eq!(map_language("MULTI VF2"), Some(422));
        assert_eq!(map_language("FRENCH"), Some(2));
        assert_eq!(map_language("VFF"), Some(2));
        assert_eq!(map_language("VOSTFR"), Some(8));
        assert_eq!(map_language("VFQ"), Some(6));
        assert_eq!(map_language("ENGLISH"), Some(1));
        assert_eq!(map_language("unknown"), None);
    }

    #[test]
    fn test_map_quality_extended() {
        // 4K
        assert_eq!(map_quality("BluRay 2160p REMUX"), Some(10));
        assert_eq!(map_quality("WEB-DL 2160p"), Some(26));
        assert_eq!(map_quality("BluRay 2160p"), Some(10));
        // 1080p
        assert_eq!(map_quality("WEB-DL 1080p"), Some(25));
        assert_eq!(map_quality("BluRay 1080p"), Some(413));
        assert_eq!(map_quality("HDRip 1080p"), Some(16));
        // Remux sans résolution
        assert_eq!(map_quality("REMUX"), Some(12));
        // SD
        assert_eq!(map_quality("DVDRip"), Some(23));
        assert_eq!(map_quality("480p"), Some(23));
    }

    #[test]
    fn test_auto_map_options_film() {
        let parsed = ReleaseParsed {
            content_type: DetectedContentType::Film,
            title: "Test".to_string(),
            year: None,
            quality: Some("WEB-DL 1080".to_string()),
            video_codec: None,
            audio: None,
            language: Some("MULTI".to_string()),
            group: None,
            season: None,
            episode: None,
        };
        let options = vec![
            C411OptionType {
                id: 1,
                name: "Langue".to_string(),
                slug: "langue".to_string(),
                allows_multiple: true,
                is_required: true,
                sort_order: 1,
                values: vec![],
            },
            C411OptionType {
                id: 2,
                name: "Qualité".to_string(),
                slug: "qualite".to_string(),
                allows_multiple: false,
                is_required: true,
                sort_order: 2,
                values: vec![],
            },
        ];
        let result = auto_map_options(&parsed, &options);
        assert_eq!(result["1"], serde_json::json!([4])); // MULTI → [4] (allows_multiple)
        assert_eq!(result["2"], serde_json::json!(25));   // WEB-DL 1080 → 25
    }

    #[test]
    fn test_auto_map_options_serie() {
        let parsed = ReleaseParsed {
            content_type: DetectedContentType::Serie,
            title: "Test".to_string(),
            year: None,
            quality: Some("BluRay 1080".to_string()),
            video_codec: None,
            audio: None,
            language: Some("FRENCH".to_string()),
            group: None,
            season: Some(1),
            episode: None,
        };
        let options = vec![
            C411OptionType {
                id: 1,
                name: "Langue".to_string(),
                slug: "langue".to_string(),
                allows_multiple: true,
                is_required: true,
                sort_order: 1,
                values: vec![],
            },
            C411OptionType {
                id: 2,
                name: "Qualité".to_string(),
                slug: "qualite".to_string(),
                allows_multiple: false,
                is_required: true,
                sort_order: 2,
                values: vec![],
            },
            C411OptionType {
                id: 7,
                name: "Saison".to_string(),
                slug: "saison".to_string(),
                allows_multiple: false,
                is_required: true,
                sort_order: 3,
                values: vec![],
            },
            C411OptionType {
                id: 6,
                name: "Épisode".to_string(),
                slug: "episode".to_string(),
                allows_multiple: false,
                is_required: true,
                sort_order: 4,
                values: vec![],
            },
        ];
        let result = auto_map_options(&parsed, &options);
        assert_eq!(result["1"], serde_json::json!([2]));  // FRENCH → [2]
        assert_eq!(result["2"], serde_json::json!(413));   // BluRay 1080 → 413
        assert_eq!(result["7"], serde_json::json!(121));   // Saison 1 → 120+1
        assert_eq!(result["6"], serde_json::json!(96));    // Pas d'épisode → saison complète
    }
}
