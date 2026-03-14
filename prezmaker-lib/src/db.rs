use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

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
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbTemplate {
    pub name: String,
    pub content_type: String,
    pub body: String,
    pub title_color: Option<String>,
    #[serde(default)]
    pub display_order: i32,
}

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

fn db_path() -> Result<PathBuf, String> {
    let dir = dirs::config_dir()
        .ok_or_else(|| "Cannot find config directory".to_string())?
        .join("prezmaker");
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Cannot create config dir: {}", e))?;
    Ok(dir.join("prezmaker.db"))
}

impl Database {
    pub fn open() -> Result<Self, String> {
        let path = db_path()?;
        let conn = Connection::open(&path)
            .map_err(|e| format!("Cannot open database: {}", e))?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS collections (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS saved_presentations (
                id TEXT PRIMARY KEY,
                collection_id TEXT NOT NULL,
                title TEXT NOT NULL,
                content_type TEXT NOT NULL,
                bbcode TEXT NOT NULL,
                poster_url TEXT,
                saved_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS content_templates (
                name TEXT NOT NULL,
                content_type TEXT NOT NULL,
                body TEXT NOT NULL,
                title_color TEXT,
                display_order INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (content_type, name)
            );

            CREATE INDEX IF NOT EXISTS idx_presentations_collection
                ON saved_presentations(collection_id);
            CREATE INDEX IF NOT EXISTS idx_presentations_content_type
                ON saved_presentations(content_type);
            CREATE INDEX IF NOT EXISTS idx_templates_type
                ON content_templates(content_type);
            ",
        )
        .map_err(|e| format!("Cannot initialize database schema: {}", e))?;

        // Enable WAL mode for better concurrent performance
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
            .map_err(|e| format!("Cannot set PRAGMA: {}", e))?;

        Ok(())
    }

    // ========================
    // Collections
    // ========================

    pub fn create_collection(&self, name: &str) -> Result<Collection, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO collections (id, name, created_at) VALUES (?1, ?2, ?3)",
            params![id, name, created_at],
        )
        .map_err(|e| format!("Cannot create collection: {}", e))?;
        Ok(Collection {
            id,
            name: name.to_string(),
            created_at,
        })
    }

    pub fn list_collections(&self) -> Result<Vec<Collection>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, name, created_at FROM collections ORDER BY created_at")
            .map_err(|e| format!("SQL error: {}", e))?;
        let rows = stmt
            .query_map([], |row| {
                Ok(Collection {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })
            .map_err(|e| format!("Query error: {}", e))?;
        let mut collections = Vec::new();
        for row in rows {
            collections.push(row.map_err(|e| format!("Row error: {}", e))?);
        }
        Ok(collections)
    }

    pub fn rename_collection(&self, id: &str, new_name: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE collections SET name = ?1 WHERE id = ?2",
            params![new_name, id],
        )
        .map_err(|e| format!("Cannot rename collection: {}", e))?;
        Ok(())
    }

    pub fn delete_collection(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        // CASCADE will delete presentations too
        conn.execute("DELETE FROM collections WHERE id = ?1", params![id])
            .map_err(|e| format!("Cannot delete collection: {}", e))?;
        Ok(())
    }

    // ========================
    // Presentations
    // ========================

    pub fn save_presentation(
        &self,
        collection_id: &str,
        entry_id: Option<&str>,
        title: &str,
        content_type: &str,
        bbcode: &str,
        poster_url: Option<&str>,
    ) -> Result<SavedPresentation, String> {
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().unwrap();

        if let Some(eid) = entry_id {
            // Upsert: try update first
            let updated = conn
                .execute(
                    "UPDATE saved_presentations SET title=?1, content_type=?2, bbcode=?3, poster_url=?4, updated_at=?5, collection_id=?6 WHERE id=?7",
                    params![title, content_type, bbcode, poster_url, now, collection_id, eid],
                )
                .map_err(|e| format!("Cannot update presentation: {}", e))?;

            if updated > 0 {
                // Fetch the existing saved_at
                let saved_at: String = conn
                    .query_row(
                        "SELECT saved_at FROM saved_presentations WHERE id=?1",
                        params![eid],
                        |row| row.get(0),
                    )
                    .map_err(|e| format!("Cannot read presentation: {}", e))?;

                return Ok(SavedPresentation {
                    id: eid.to_string(),
                    collection_id: collection_id.to_string(),
                    title: title.to_string(),
                    content_type: content_type.to_string(),
                    bbcode: bbcode.to_string(),
                    poster_url: poster_url.map(|s| s.to_string()),
                    saved_at,
                    updated_at: now,
                });
            }
            // If not found, fall through to insert with this ID
            let id = eid.to_string();
            conn.execute(
                "INSERT INTO saved_presentations (id, collection_id, title, content_type, bbcode, poster_url, saved_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![id, collection_id, title, content_type, bbcode, poster_url, now, now],
            )
            .map_err(|e| format!("Cannot insert presentation: {}", e))?;
            return Ok(SavedPresentation {
                id,
                collection_id: collection_id.to_string(),
                title: title.to_string(),
                content_type: content_type.to_string(),
                bbcode: bbcode.to_string(),
                poster_url: poster_url.map(|s| s.to_string()),
                saved_at: now.clone(),
                updated_at: now,
            });
        }

        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO saved_presentations (id, collection_id, title, content_type, bbcode, poster_url, saved_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![id, collection_id, title, content_type, bbcode, poster_url, now, now],
        )
        .map_err(|e| format!("Cannot insert presentation: {}", e))?;

        Ok(SavedPresentation {
            id,
            collection_id: collection_id.to_string(),
            title: title.to_string(),
            content_type: content_type.to_string(),
            bbcode: bbcode.to_string(),
            poster_url: poster_url.map(|s| s.to_string()),
            saved_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn list_presentations(
        &self,
        collection_id: &str,
        sort_by: Option<&str>,
        sort_order: Option<&str>,
        filter_type: Option<&str>,
        filter_search: Option<&str>,
    ) -> Result<Vec<SavedPresentation>, String> {
        let conn = self.conn.lock().unwrap();

        let mut sql = String::from(
            "SELECT id, collection_id, title, content_type, bbcode, poster_url, saved_at, updated_at FROM saved_presentations WHERE collection_id = ?1",
        );
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = vec![Box::new(collection_id.to_string())];

        if let Some(ct) = filter_type {
            if !ct.is_empty() {
                sql.push_str(" AND content_type = ?");
                param_values.push(Box::new(ct.to_string()));
            }
        }
        if let Some(search) = filter_search {
            if !search.is_empty() {
                sql.push_str(" AND title LIKE ?");
                param_values.push(Box::new(format!("%{}%", search)));
            }
        }

        let order_col = match sort_by.unwrap_or("date") {
            "title" => "title",
            "type" => "content_type",
            _ => "saved_at",
        };
        let order_dir = match sort_order.unwrap_or("desc") {
            "asc" => "ASC",
            _ => "DESC",
        };
        sql.push_str(&format!(" ORDER BY {} {}", order_col, order_dir));

        let mut stmt = conn.prepare(&sql).map_err(|e| format!("SQL error: {}", e))?;

        let param_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|b| b.as_ref()).collect();
        let rows = stmt
            .query_map(param_refs.as_slice(), |row| {
                Ok(SavedPresentation {
                    id: row.get(0)?,
                    collection_id: row.get(1)?,
                    title: row.get(2)?,
                    content_type: row.get(3)?,
                    bbcode: row.get(4)?,
                    poster_url: row.get(5)?,
                    saved_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })
            .map_err(|e| format!("Query error: {}", e))?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row.map_err(|e| format!("Row error: {}", e))?);
        }
        Ok(entries)
    }

    pub fn get_presentation(&self, collection_id: &str, id: &str) -> Result<SavedPresentation, String> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, collection_id, title, content_type, bbcode, poster_url, saved_at, updated_at FROM saved_presentations WHERE id = ?1 AND collection_id = ?2",
            params![id, collection_id],
            |row| {
                Ok(SavedPresentation {
                    id: row.get(0)?,
                    collection_id: row.get(1)?,
                    title: row.get(2)?,
                    content_type: row.get(3)?,
                    bbcode: row.get(4)?,
                    poster_url: row.get(5)?,
                    saved_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            },
        )
        .map_err(|e| format!("Cannot get presentation: {}", e))
    }

    pub fn delete_presentation(&self, collection_id: &str, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM saved_presentations WHERE id = ?1 AND collection_id = ?2",
            params![id, collection_id],
        )
        .map_err(|e| format!("Cannot delete presentation: {}", e))?;
        Ok(())
    }

    pub fn move_presentation(&self, from_collection: &str, to_collection: &str, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE saved_presentations SET collection_id = ?1, updated_at = ?2 WHERE id = ?3 AND collection_id = ?4",
            params![to_collection, now, id, from_collection],
        )
        .map_err(|e| format!("Cannot move presentation: {}", e))?;
        Ok(())
    }

    // ========================
    // Templates
    // ========================

    pub fn list_templates(&self, content_type: &str) -> Result<Vec<DbTemplate>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT name, content_type, body, title_color, display_order FROM content_templates WHERE content_type = ?1 ORDER BY display_order, name")
            .map_err(|e| format!("SQL error: {}", e))?;
        let rows = stmt
            .query_map(params![content_type], |row| {
                Ok(DbTemplate {
                    name: row.get(0)?,
                    content_type: row.get(1)?,
                    body: row.get(2)?,
                    title_color: row.get(3)?,
                    display_order: row.get(4)?,
                })
            })
            .map_err(|e| format!("Query error: {}", e))?;
        let mut templates = Vec::new();
        for row in rows {
            templates.push(row.map_err(|e| format!("Row error: {}", e))?);
        }
        Ok(templates)
    }

    pub fn get_template(&self, content_type: &str, name: &str) -> Result<Option<DbTemplate>, String> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT name, content_type, body, title_color, display_order FROM content_templates WHERE content_type = ?1 AND name = ?2",
            params![content_type, name],
            |row| {
                Ok(DbTemplate {
                    name: row.get(0)?,
                    content_type: row.get(1)?,
                    body: row.get(2)?,
                    title_color: row.get(3)?,
                    display_order: row.get(4)?,
                })
            },
        )
        .optional()
        .map_err(|e| format!("Query error: {}", e))
    }

    pub fn save_template(&self, content_type: &str, name: &str, body: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO content_templates (name, content_type, body, display_order) VALUES (?1, ?2, ?3, 0)
             ON CONFLICT(content_type, name) DO UPDATE SET body = excluded.body",
            params![name, content_type, body],
        )
        .map_err(|e| format!("Cannot save template: {}", e))?;
        Ok(())
    }

    pub fn save_template_meta(&self, content_type: &str, name: &str, title_color: Option<&str>) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE content_templates SET title_color = ?1 WHERE content_type = ?2 AND name = ?3",
            params![title_color, content_type, name],
        )
        .map_err(|e| format!("Cannot update template meta: {}", e))?;
        Ok(())
    }

    pub fn delete_template(&self, content_type: &str, name: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM content_templates WHERE content_type = ?1 AND name = ?2",
            params![content_type, name],
        )
        .map_err(|e| format!("Cannot delete template: {}", e))?;
        Ok(())
    }

    pub fn duplicate_template(&self, content_type: &str, name: &str, new_name: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO content_templates (name, content_type, body, title_color, display_order)
             SELECT ?1, content_type, body, title_color, display_order FROM content_templates
             WHERE content_type = ?2 AND name = ?3",
            params![new_name, content_type, name],
        )
        .map_err(|e| format!("Cannot duplicate template: {}", e))?;
        Ok(())
    }

    pub fn reorder_templates(&self, content_type: &str, names: &[String]) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        for (i, name) in names.iter().enumerate() {
            conn.execute(
                "UPDATE content_templates SET display_order = ?1 WHERE content_type = ?2 AND name = ?3",
                params![i as i32, content_type, name],
            )
            .map_err(|e| format!("Cannot reorder template: {}", e))?;
        }
        Ok(())
    }

    // ========================
    // Migration from JSON files
    // ========================

    pub fn migrate_from_json(&self) -> Result<bool, String> {
        let base_dir = dirs::config_dir()
            .ok_or_else(|| "Cannot find config directory".to_string())?
            .join("prezmaker");

        let mut migrated = false;

        // Migrate collections
        let collections_dir = base_dir.join("collections");
        if collections_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&collections_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let meta_path = path.join("meta.json");
                        if let Ok(json) = std::fs::read_to_string(&meta_path) {
                            #[derive(Deserialize)]
                            struct OldCollection {
                                id: String,
                                name: String,
                                created_at: String,
                            }
                            if let Ok(col) = serde_json::from_str::<OldCollection>(&json) {
                                let conn = self.conn.lock().unwrap();
                                // Skip if already exists
                                let exists: bool = conn
                                    .query_row(
                                        "SELECT COUNT(*) > 0 FROM collections WHERE id = ?1",
                                        params![col.id],
                                        |row| row.get(0),
                                    )
                                    .unwrap_or(false);
                                if exists {
                                    continue;
                                }
                                let _ = conn.execute(
                                    "INSERT OR IGNORE INTO collections (id, name, created_at) VALUES (?1, ?2, ?3)",
                                    params![col.id, col.name, col.created_at],
                                );
                                drop(conn);

                                // Migrate presentations in this collection
                                if let Ok(files) = std::fs::read_dir(&path) {
                                    for file in files.flatten() {
                                        let fp = file.path();
                                        if fp.file_name().and_then(|f| f.to_str()) == Some("meta.json") {
                                            continue;
                                        }
                                        if fp.extension().and_then(|e| e.to_str()) == Some("json") {
                                            if let Ok(pjson) = std::fs::read_to_string(&fp) {
                                                #[derive(Deserialize)]
                                                struct OldPres {
                                                    id: String,
                                                    collection_id: String,
                                                    title: String,
                                                    content_type: String,
                                                    bbcode: String,
                                                    poster_url: Option<String>,
                                                    saved_at: String,
                                                }
                                                if let Ok(p) = serde_json::from_str::<OldPres>(&pjson) {
                                                    let conn = self.conn.lock().unwrap();
                                                    let _ = conn.execute(
                                                        "INSERT OR IGNORE INTO saved_presentations (id, collection_id, title, content_type, bbcode, poster_url, saved_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                                                        params![p.id, p.collection_id, p.title, p.content_type, p.bbcode, p.poster_url, p.saved_at, p.saved_at],
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                                migrated = true;
                            }
                        }
                    }
                }
            }
        }

        // Migrate content templates
        let tpl_dir = base_dir.join("content_templates");
        if tpl_dir.exists() {
            if let Ok(type_dirs) = std::fs::read_dir(&tpl_dir) {
                for type_entry in type_dirs.flatten() {
                    let type_path = type_entry.path();
                    if type_path.is_dir() {
                        let content_type = type_path.file_name().and_then(|f| f.to_str()).unwrap_or("").to_string();
                        if content_type.is_empty() {
                            continue;
                        }
                        if let Ok(files) = std::fs::read_dir(&type_path) {
                            for file in files.flatten() {
                                let fp = file.path();
                                if fp.extension().and_then(|e| e.to_str()) == Some("tpl") {
                                    let name = fp.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
                                    if name.is_empty() {
                                        continue;
                                    }
                                    if let Ok(body) = std::fs::read_to_string(&fp) {
                                        let mut title_color: Option<String> = None;
                                        let mut display_order: i32 = 0;
                                        let meta_path = type_path.join(format!("{}.meta.json", name));
                                        if let Ok(meta_json) = std::fs::read_to_string(&meta_path) {
                                            #[derive(Deserialize)]
                                            struct Meta {
                                                title_color: Option<String>,
                                                order: Option<i32>,
                                            }
                                            if let Ok(m) = serde_json::from_str::<Meta>(&meta_json) {
                                                title_color = m.title_color;
                                                display_order = m.order.unwrap_or(0);
                                            }
                                        }
                                        let conn = self.conn.lock().unwrap();
                                        let _ = conn.execute(
                                            "INSERT OR IGNORE INTO content_templates (name, content_type, body, title_color, display_order) VALUES (?1, ?2, ?3, ?4, ?5)",
                                            params![name, content_type, body, title_color, display_order],
                                        );
                                    }
                                }
                            }
                        }
                        migrated = true;
                    }
                }
            }
        }

        Ok(migrated)
    }
}
