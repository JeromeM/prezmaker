use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedPresentation {
    pub id: String,
    pub collection_id: String,
    pub title: String,
    pub content_type: String,
    pub bbcode: String,
    pub poster_url: Option<String>,
    pub saved_at: String,
}

fn collections_base_dir() -> Result<PathBuf, String> {
    let dir = dirs::config_dir()
        .ok_or_else(|| "Cannot find config directory".to_string())?
        .join("prezmaker")
        .join("collections");
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Cannot create collections dir: {}", e))?;
    Ok(dir)
}

fn collection_dir(collection_id: &str) -> Result<PathBuf, String> {
    let dir = collections_base_dir()?.join(collection_id);
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Cannot create collection dir: {}", e))?;
    Ok(dir)
}

// --- Migration from flat structure ---

pub fn migrate_if_needed() -> Result<(), String> {
    let base = collections_base_dir()?;
    let old_files: Vec<_> = std::fs::read_dir(&base)
        .map_err(|e| format!("Cannot read collections dir: {}", e))?
        .flatten()
        .filter(|e| {
            let p = e.path();
            p.is_file() && p.extension().and_then(|x| x.to_str()) == Some("json")
        })
        .collect();

    if old_files.is_empty() {
        return Ok(());
    }

    // Create a default collection for old entries
    let col = create_collection("Défaut")?;
    let col_dir = collection_dir(&col.id)?;

    for entry in old_files {
        let path = entry.path();
        if let Ok(json) = std::fs::read_to_string(&path) {
            // Try to parse old format (no collection_id field)
            #[derive(Deserialize)]
            struct OldEntry {
                id: String,
                title: String,
                content_type: String,
                bbcode: String,
                poster_url: Option<String>,
                saved_at: String,
            }
            if let Ok(old) = serde_json::from_str::<OldEntry>(&json) {
                let new_entry = SavedPresentation {
                    id: old.id.clone(),
                    collection_id: col.id.clone(),
                    title: old.title,
                    content_type: old.content_type,
                    bbcode: old.bbcode,
                    poster_url: old.poster_url,
                    saved_at: old.saved_at,
                };
                let new_json = serde_json::to_string_pretty(&new_entry)
                    .map_err(|e| format!("JSON error: {}", e))?;
                std::fs::write(col_dir.join(format!("{}.json", old.id)), new_json)
                    .map_err(|e| format!("Cannot write migrated entry: {}", e))?;
                let _ = std::fs::remove_file(&path);
            }
        }
    }
    Ok(())
}

// --- Collection CRUD ---

pub fn create_collection(name: &str) -> Result<Collection, String> {
    let id = Uuid::new_v4().to_string();
    let col = Collection {
        id: id.clone(),
        name: name.to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    let dir = collection_dir(&id)?;
    let json = serde_json::to_string_pretty(&col)
        .map_err(|e| format!("JSON error: {}", e))?;
    std::fs::write(dir.join("meta.json"), json)
        .map_err(|e| format!("Cannot write collection meta: {}", e))?;
    Ok(col)
}

pub fn list_collections() -> Result<Vec<Collection>, String> {
    migrate_if_needed()?;
    let base = collections_base_dir()?;
    let mut collections = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&base) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let meta_path = path.join("meta.json");
                if let Ok(json) = std::fs::read_to_string(&meta_path) {
                    if let Ok(col) = serde_json::from_str::<Collection>(&json) {
                        collections.push(col);
                    }
                }
            }
        }
    }
    collections.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    Ok(collections)
}

pub fn rename_collection(id: &str, new_name: &str) -> Result<(), String> {
    let dir = collection_dir(id)?;
    let meta_path = dir.join("meta.json");
    let json = std::fs::read_to_string(&meta_path)
        .map_err(|e| format!("Cannot read collection meta: {}", e))?;
    let mut col: Collection = serde_json::from_str(&json)
        .map_err(|e| format!("Invalid JSON: {}", e))?;
    col.name = new_name.to_string();
    let json = serde_json::to_string_pretty(&col)
        .map_err(|e| format!("JSON error: {}", e))?;
    std::fs::write(&meta_path, json)
        .map_err(|e| format!("Cannot write collection meta: {}", e))?;
    Ok(())
}

pub fn delete_collection(id: &str) -> Result<(), String> {
    let dir = collections_base_dir()?.join(id);
    if dir.exists() {
        std::fs::remove_dir_all(&dir)
            .map_err(|e| format!("Cannot delete collection: {}", e))?;
    }
    Ok(())
}

// --- Entry CRUD (with upsert) ---

pub fn save_presentation(
    collection_id: &str,
    entry_id: Option<&str>,
    title: &str,
    content_type: &str,
    bbcode: &str,
    poster_url: Option<&str>,
) -> Result<SavedPresentation, String> {
    let id = entry_id
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    let saved_at = chrono::Utc::now().to_rfc3339();
    let entry = SavedPresentation {
        id: id.clone(),
        collection_id: collection_id.to_string(),
        title: title.to_string(),
        content_type: content_type.to_string(),
        bbcode: bbcode.to_string(),
        poster_url: poster_url.map(|s| s.to_string()),
        saved_at,
    };
    let dir = collection_dir(collection_id)?;
    let json = serde_json::to_string_pretty(&entry)
        .map_err(|e| format!("JSON error: {}", e))?;
    std::fs::write(dir.join(format!("{}.json", id)), json)
        .map_err(|e| format!("Cannot write collection entry: {}", e))?;
    Ok(entry)
}

pub fn list_presentations(collection_id: &str) -> Result<Vec<SavedPresentation>, String> {
    let dir = collection_dir(collection_id)?;
    let mut entries = Vec::new();
    if let Ok(read) = std::fs::read_dir(&dir) {
        for entry in read.flatten() {
            let path = entry.path();
            if path.file_name().and_then(|f| f.to_str()) == Some("meta.json") {
                continue;
            }
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

pub fn get_presentation(collection_id: &str, id: &str) -> Result<SavedPresentation, String> {
    let path = collection_dir(collection_id)?.join(format!("{}.json", id));
    let json = std::fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read collection entry: {}", e))?;
    serde_json::from_str(&json).map_err(|e| format!("Invalid JSON: {}", e))
}

pub fn delete_presentation(collection_id: &str, id: &str) -> Result<(), String> {
    let path = collection_dir(collection_id)?.join(format!("{}.json", id));
    std::fs::remove_file(&path)
        .map_err(|e| format!("Cannot delete collection entry: {}", e))
}

pub fn move_presentation(from_collection: &str, to_collection: &str, id: &str) -> Result<(), String> {
    let src = collection_dir(from_collection)?.join(format!("{}.json", id));
    let json = std::fs::read_to_string(&src)
        .map_err(|e| format!("Cannot read entry: {}", e))?;
    let mut entry: SavedPresentation = serde_json::from_str(&json)
        .map_err(|e| format!("Invalid JSON: {}", e))?;
    entry.collection_id = to_collection.to_string();
    let new_json = serde_json::to_string_pretty(&entry)
        .map_err(|e| format!("JSON error: {}", e))?;
    let dest = collection_dir(to_collection)?.join(format!("{}.json", id));
    std::fs::write(&dest, new_json)
        .map_err(|e| format!("Cannot write entry: {}", e))?;
    std::fs::remove_file(&src)
        .map_err(|e| format!("Cannot remove old entry: {}", e))?;
    Ok(())
}
