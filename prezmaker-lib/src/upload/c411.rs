use crate::error::PrezError;
use crate::torrent::ReleaseParsed;
use serde::{Deserialize, Serialize};
use std::path::Path;

const BASE_URL: &str = "https://www.c411.me/api";

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
            client: reqwest::Client::new(),
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

        let categories: Vec<C411Category> = resp.json().await?;
        Ok(categories)
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

        let options: Vec<C411OptionType> = resp.json().await?;
        Ok(options)
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
    ) -> Result<C411UploadResult, PrezError> {
        let torrent_bytes = std::fs::read(torrent_path)
            .map_err(|e| PrezError::Upload(format!("Cannot read torrent file: {}", e)))?;

        let torrent_filename = torrent_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("release.torrent")
            .to_string();

        let form = reqwest::multipart::Form::new()
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

        let form = if let Some(note) = uploader_note {
            form.text("uploaderNote", note.to_string())
        } else {
            form
        };

        let resp = self
            .client
            .post(format!("{}/torrents", BASE_URL))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;

        if resp.status().is_success() {
            match resp.json::<C411UploadResult>().await {
                Ok(result) => Ok(result),
                Err(_) => Ok(C411UploadResult {
                    success: true,
                    message: Some("Upload réussi".to_string()),
                }),
            }
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
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
        "MULTI" => Some(4),
        "FRENCH" | "VFF" => Some(2),
        "VOSTFR" => Some(8),
        "VFQ" => Some(6),
        "ENGLISH" | "ENG" => Some(1),
        _ => None,
    }
}

fn map_quality(quality: &str) -> Option<u32> {
    let q = quality.to_uppercase();
    if q.contains("2160") && q.contains("BLURAY") {
        Some(10)
    } else if q.contains("1080") && (q.contains("WEB-DL") || q.contains("WEBDL") || q.contains("WEB")) {
        Some(25)
    } else if q.contains("1080") && q.contains("BLURAY") {
        Some(413)
    } else if q.contains("2160") && (q.contains("WEB-DL") || q.contains("WEBDL") || q.contains("WEB")) {
        Some(10) // fallback 4K WEB
    } else if q.contains("720") {
        Some(24) // 720p
    } else {
        None
    }
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
        assert_eq!(map_language("FRENCH"), Some(2));
        assert_eq!(map_language("VFF"), Some(2));
        assert_eq!(map_language("VOSTFR"), Some(8));
        assert_eq!(map_language("VFQ"), Some(6));
        assert_eq!(map_language("ENGLISH"), Some(1));
        assert_eq!(map_language("unknown"), None);
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
