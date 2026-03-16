use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::ContentTemplate;

/// Per-template metadata stored in companion `.meta.json` files
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct TemplateMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    title_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    order: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    output_format: Option<String>,
}

fn templates_base_dir() -> Result<PathBuf, String> {
    let dir = dirs::config_dir()
        .ok_or_else(|| "Cannot find config directory".to_string())?
        .join("prezmaker")
        .join("content_templates");
    Ok(dir)
}

fn content_type_dir(content_type: &str) -> Result<PathBuf, String> {
    let dir = templates_base_dir()?.join(content_type);
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Cannot create templates dir: {}", e))?;
    Ok(dir)
}

fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim()
        .to_string()
}

fn meta_path(content_type: &str, name: &str) -> Result<PathBuf, String> {
    let safe = sanitize_name(name);
    if safe.is_empty() {
        return Err("Template name is empty".to_string());
    }
    Ok(content_type_dir(content_type)?.join(format!("{}.meta.json", safe)))
}

fn load_meta(content_type: &str, name: &str) -> TemplateMeta {
    meta_path(content_type, name)
        .ok()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_full_meta(content_type: &str, name: &str, meta: &TemplateMeta) -> Result<(), String> {
    if meta.title_color.is_some() || meta.order.is_some() || meta.output_format.is_some() {
        let path = meta_path(content_type, name)?;
        let json = serde_json::to_string_pretty(meta)
            .map_err(|e| format!("JSON error: {}", e))?;
        std::fs::write(&path, json)
            .map_err(|e| format!("Cannot write meta: {}", e))?;
    } else {
        if let Ok(path) = meta_path(content_type, name) {
            let _ = std::fs::remove_file(path);
        }
    }
    Ok(())
}

pub fn save_template_meta(content_type: &str, name: &str, title_color: Option<String>) -> Result<(), String> {
    let mut meta = load_meta(content_type, name);
    meta.title_color = title_color;
    save_full_meta(content_type, name, &meta)
}

pub fn save_template_format(content_type: &str, name: &str, output_format: Option<String>) -> Result<(), String> {
    let mut meta = load_meta(content_type, name);
    meta.output_format = output_format;
    save_full_meta(content_type, name, &meta)
}

pub fn reorder_templates(content_type: &str, names: Vec<String>) -> Result<(), String> {
    for (i, name) in names.iter().enumerate() {
        let mut meta = load_meta(content_type, name);
        meta.order = Some(i as u32);
        save_full_meta(content_type, name, &meta)?;
    }
    Ok(())
}

pub fn list_templates(content_type: &str) -> Result<Vec<ContentTemplate>, String> {
    let dir = content_type_dir(content_type)?;
    let mut templates = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("tpl") {
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                let body =
                    std::fs::read_to_string(&path).map_err(|e| format!("Read error: {}", e))?;
                let meta = load_meta(content_type, &name);
                // Auto-detect format: "default-html" → html, or from metadata
                let fmt = meta.output_format.or_else(|| {
                    if name.contains("-html") { Some("html".to_string()) } else { None }
                });
                templates.push(ContentTemplate {
                    name: name.clone(),
                    content_type: content_type.to_string(),
                    body,
                    is_default: name == "default",
                    title_color: meta.title_color,
                    order: meta.order,
                    output_format: fmt,
                });
            }
        }
    }

    // Ensure default template exists
    if !templates.iter().any(|t| t.is_default) {
        let default_body = crate::default_templates::get_default(content_type);
        let default_path = dir.join("default.tpl");
        std::fs::write(&default_path, &default_body)
            .map_err(|e| format!("Cannot write default template: {}", e))?;
        templates.push(ContentTemplate {
            name: "default".to_string(),
            content_type: content_type.to_string(),
            body: default_body,
            is_default: true,
            title_color: None,
            order: Some(0),
            output_format: None, // BBCode (default)
        });
    }

    // Ensure default-html template exists and is up-to-date
    if let Some(html_body) = crate::default_templates_html::get_default_html(content_type) {
        let html_path = dir.join("default-html.tpl");
        let needs_update = if let Some(existing) = templates.iter().find(|t| t.name == "default-html") {
            // Re-create if the template contains old broken patterns
            existing.body.contains("{{p:{{field") || existing.body.contains("{{field:Date de sortie:") || existing.body.contains("Voir la fiche") || existing.body.contains("{{ratings_table}}")
        } else {
            true
        };
        if needs_update {
            std::fs::write(&html_path, &html_body)
                .map_err(|e| format!("Cannot write default-html template: {}", e))?;
            templates.retain(|t| t.name != "default-html");
            templates.push(ContentTemplate {
                name: "default-html".to_string(),
                content_type: content_type.to_string(),
                body: html_body,
                is_default: false,
                title_color: None,
                order: Some(1),
                output_format: Some("html".to_string()),
            });
        }
    }

    templates.sort_by(|a, b| {
        // Sort by order (None goes last), then alphabetical as tiebreaker
        a.order.unwrap_or(u32::MAX).cmp(&b.order.unwrap_or(u32::MAX))
            .then(a.name.cmp(&b.name))
    });
    Ok(templates)
}

pub fn get_template(content_type: &str, name: &str) -> Result<ContentTemplate, String> {
    let safe = sanitize_name(name);
    if safe.is_empty() {
        return Err("Template name is empty".to_string());
    }
    let dir = content_type_dir(content_type)?;
    let path = dir.join(format!("{}.tpl", safe));

    if !path.exists() && safe == "default" {
        // Auto-create default
        let body = crate::default_templates::get_default(content_type);
        std::fs::write(&path, &body)
            .map_err(|e| format!("Cannot write default template: {}", e))?;
        return Ok(ContentTemplate {
            name: safe,
            content_type: content_type.to_string(),
            body,
            is_default: true,
            title_color: None,
            order: Some(0),
            output_format: None,
        });
    }

    let body = std::fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read template '{}': {}", name, e))?;
    let meta = load_meta(content_type, &safe);
    let fmt = meta.output_format.or_else(|| {
        if safe.contains("-html") { Some("html".to_string()) } else { None }
    });
    Ok(ContentTemplate {
        name: safe.clone(),
        content_type: content_type.to_string(),
        body,
        is_default: safe == "default",
        title_color: meta.title_color,
        order: meta.order,
        output_format: fmt,
    })
}

pub fn save_template(content_type: &str, name: &str, body: &str) -> Result<(), String> {
    let safe = sanitize_name(name);
    if safe.is_empty() {
        return Err("Template name is empty".to_string());
    }
    let dir = content_type_dir(content_type)?;
    let path = dir.join(format!("{}.tpl", safe));
    std::fs::write(&path, body).map_err(|e| format!("Cannot write template: {}", e))
}

pub fn delete_template(content_type: &str, name: &str) -> Result<(), String> {
    let safe = sanitize_name(name);
    if safe == "default" {
        return Err("Cannot delete the default template".to_string());
    }
    let dir = content_type_dir(content_type)?;
    let path = dir.join(format!("{}.tpl", safe));
    std::fs::remove_file(&path).map_err(|e| format!("Cannot delete template: {}", e))?;
    // Also remove metadata file if present
    if let Ok(mp) = meta_path(content_type, &safe) {
        let _ = std::fs::remove_file(mp);
    }
    Ok(())
}

pub fn duplicate_template(
    content_type: &str,
    name: &str,
    new_name: &str,
) -> Result<(), String> {
    let src = get_template(content_type, name)?;
    let safe_new = sanitize_name(new_name);
    if safe_new.is_empty() {
        return Err("New template name is empty".to_string());
    }
    let dir = content_type_dir(content_type)?;
    let dst = dir.join(format!("{}.tpl", safe_new));
    if dst.exists() {
        return Err(format!("Template '{}' already exists", new_name));
    }
    std::fs::write(&dst, &src.body).map_err(|e| format!("Cannot write template: {}", e))?;
    // Also duplicate metadata if present
    if src.title_color.is_some() {
        save_template_meta(content_type, &safe_new, src.title_color)?;
    }
    Ok(())
}
