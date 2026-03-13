use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedPresentation {
    pub id: String,
    pub title: String,
    pub content_type: String,
    pub bbcode: String,
    pub poster_url: Option<String>,
    pub saved_at: String,
}

fn collections_dir() -> Result<PathBuf, String> {
    let dir = dirs::config_dir()
        .ok_or_else(|| "Cannot find config directory".to_string())?
        .join("prezmaker")
        .join("collections");
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Cannot create collections dir: {}", e))?;
    Ok(dir)
}

pub fn save_presentation(
    title: &str,
    content_type: &str,
    bbcode: &str,
    poster_url: Option<&str>,
) -> Result<SavedPresentation, String> {
    let id = Uuid::new_v4().to_string();
    let saved_at = chrono::Utc::now().to_rfc3339();
    let entry = SavedPresentation {
        id: id.clone(),
        title: title.to_string(),
        content_type: content_type.to_string(),
        bbcode: bbcode.to_string(),
        poster_url: poster_url.map(|s| s.to_string()),
        saved_at,
    };
    let path = collections_dir()?.join(format!("{}.json", id));
    let json = serde_json::to_string_pretty(&entry)
        .map_err(|e| format!("JSON error: {}", e))?;
    std::fs::write(&path, json)
        .map_err(|e| format!("Cannot write collection entry: {}", e))?;
    Ok(entry)
}

pub fn list_presentations() -> Result<Vec<SavedPresentation>, String> {
    let dir = collections_dir()?;
    let mut entries = Vec::new();
    if let Ok(read) = std::fs::read_dir(&dir) {
        for entry in read.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Ok(json) = std::fs::read_to_string(&path) {
                    if let Ok(pres) = serde_json::from_str::<SavedPresentation>(&json) {
                        entries.push(pres);
                    }
                }
            }
        }
    }
    entries.sort_by(|a, b| b.saved_at.cmp(&a.saved_at));
    Ok(entries)
}

pub fn get_presentation(id: &str) -> Result<SavedPresentation, String> {
    let path = collections_dir()?.join(format!("{}.json", id));
    let json = std::fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read collection entry: {}", e))?;
    serde_json::from_str(&json).map_err(|e| format!("Invalid JSON: {}", e))
}

pub fn delete_presentation(id: &str) -> Result<(), String> {
    let path = collections_dir()?.join(format!("{}.json", id));
    std::fs::remove_file(&path)
        .map_err(|e| format!("Cannot delete collection entry: {}", e))
}
