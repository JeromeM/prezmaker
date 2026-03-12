use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::formatters::bbcode;
use crate::models::Rating;

/// Extra context for composite blocks that need model-level data
#[derive(Default)]
pub struct RenderContext {
    pub ratings: Vec<Rating>,
    pub poster_url: Option<String>,
    pub cover_url: Option<String>,
    pub logo_url: Option<String>,
    pub screenshots: Vec<String>,
    pub tech: Option<crate::models::MediaTechInfo>,
    pub game_tech: Option<crate::models::TechInfo>,
    pub min_reqs: Option<crate::models::SystemReqs>,
    pub rec_reqs: Option<crate::models::SystemReqs>,
    pub info_bbcode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentTemplate {
    pub name: String,
    pub content_type: String,
    pub body: String,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateTag {
    pub name: String,
    pub description: String,
    pub category: String,
    pub example: Option<String>,
}

// --- Template storage ---

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
                templates.push(ContentTemplate {
                    name: name.clone(),
                    content_type: content_type.to_string(),
                    body,
                    is_default: name == "default",
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
        });
    }

    templates.sort_by(|a, b| {
        // Default first, then alphabetical
        b.is_default.cmp(&a.is_default).then(a.name.cmp(&b.name))
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
        });
    }

    let body = std::fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read template '{}': {}", name, e))?;
    Ok(ContentTemplate {
        name: safe.clone(),
        content_type: content_type.to_string(),
        body,
        is_default: safe == "default",
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
    std::fs::remove_file(&path).map_err(|e| format!("Cannot delete template: {}", e))
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
    std::fs::write(&dst, &src.body).map_err(|e| format!("Cannot write template: {}", e))
}

// --- Template rendering ---

pub fn render(
    template_body: &str,
    data: &HashMap<String, String>,
    ctx: &RenderContext,
    title_color: &str,
    pseudo: &str,
) -> String {
    // Pass 0: Strip template indentation (leading whitespace per line)
    // This allows users to indent inside {{#if}} blocks without affecting output
    let mut output = template_body
        .lines()
        .map(|line| line.trim_start())
        .collect::<Vec<_>>()
        .join("\n");

    // Inject info_bbcode into data if available
    let mut augmented_data;
    let effective_data = if ctx.info_bbcode.is_some() && !data.contains_key("info_bbcode") {
        augmented_data = data.clone();
        if let Some(ref info) = ctx.info_bbcode {
            augmented_data.insert("info_bbcode".into(), info.clone());
        }
        &augmented_data
    } else {
        data
    };

    // Pass 1: Process conditionals {{#if tag}}...{{/if}}
    output = process_conditionals(&output, effective_data);

    // Pass 2: Replace data tags {{tag}}
    output = replace_data_tags(&output, effective_data);

    // Pass 3: Render layout tags {{layout:args}}
    output = render_layout_tags(&output, ctx, title_color, pseudo);

    // Pass 4: Collapse excessive blank lines (3+ consecutive newlines → 2)
    while output.contains("\n\n\n") {
        output = output.replace("\n\n\n", "\n\n");
    }

    // Trim trailing whitespace
    output = output.trim_end().to_string();
    output.push('\n');

    output
}

fn process_conditionals(template: &str, data: &HashMap<String, String>) -> String {
    let mut result = template.to_string();

    // Process from innermost to outermost: find the first {{#if ...}} whose matching
    // {{/if}} contains no nested {{#if}}.
    loop {
        let start_marker = "{{#if ";
        let end_marker = "{{/if}}";

        // Find the innermost {{#if ...}} by finding the last one before any {{/if}}
        let Some(first_end) = result.find(end_marker) else {
            break;
        };

        // Search backwards from first_end to find the nearest {{#if ...}} before it
        let search_region = &result[..first_end];
        let Some(start_pos) = search_region.rfind(start_marker) else {
            break;
        };

        let after_start = start_pos + start_marker.len();
        let Some(tag_end) = result[after_start..].find("}}") else {
            break;
        };
        let condition_str = result[after_start..after_start + tag_end].trim();
        let block_start = after_start + tag_end + 2;

        let block_end = first_end;
        let full_end = block_end + end_marker.len();

        // Evaluate condition
        let condition_met = evaluate_condition(condition_str, data);

        // Extract block content: skip the newline right after {{#if ...}} opening
        let mut content_start = block_start;
        let content_end = block_end;
        if result.as_bytes().get(content_start) == Some(&b'\n') {
            content_start += 1;
        }

        // Consume the newline after {{/if}} if present
        let mut consume_end = full_end;
        if result.as_bytes().get(consume_end) == Some(&b'\n') {
            consume_end += 1;
        }

        // If {{#if ...}} is at the start of a line, also consume leading whitespace on that line
        let mut consume_start = start_pos;
        if consume_start > 0 && result.as_bytes().get(consume_start - 1) == Some(&b'\n') {
            // Already at line start, nothing extra to consume
        } else {
            // Check if everything before on this line is whitespace
            let line_start = result[..consume_start].rfind('\n').map_or(0, |p| p + 1);
            if result[line_start..consume_start].chars().all(|c| c == ' ' || c == '\t') {
                consume_start = line_start;
            }
        }

        if condition_met {
            let block_content = result[content_start..content_end].to_string();
            result = format!("{}{}{}", &result[..consume_start], block_content, &result[consume_end..]);
        } else {
            result = format!("{}{}", &result[..consume_start], &result[consume_end..]);
        }
    }

    result
}

/// Evaluate a conditional expression.
/// Supports:
///   - `tag` → true if tag exists and is non-empty
///   - `tag > value`, `tag >= value`, `tag < value`, `tag <= value` → numeric comparison
///   - `tag == value`, `tag != value` → string or numeric comparison
fn evaluate_condition(condition: &str, data: &HashMap<String, String>) -> bool {
    // Try to parse as comparison
    let operators = [">=", "<=", "!=", "==", ">", "<"];
    for op in &operators {
        if let Some(pos) = condition.find(op) {
            let key = condition[..pos].trim().to_lowercase();
            let compare_val = condition[pos + op.len()..].trim();

            let data_val = match data.get(&key) {
                Some(v) => v.as_str(),
                None => return false,
            };

            // Try numeric comparison first
            if let (Ok(lhs), Ok(rhs)) = (data_val.parse::<f64>(), compare_val.parse::<f64>()) {
                return match *op {
                    ">=" => lhs >= rhs,
                    "<=" => lhs <= rhs,
                    "!=" => (lhs - rhs).abs() > f64::EPSILON,
                    "==" => (lhs - rhs).abs() < f64::EPSILON,
                    ">" => lhs > rhs,
                    "<" => lhs < rhs,
                    _ => false,
                };
            }

            // Fall back to string comparison for == and !=
            return match *op {
                "==" => data_val == compare_val,
                "!=" => data_val != compare_val,
                _ => false, // Can't do > < on non-numeric
            };
        }
    }

    // Simple existence check
    let tag = condition.to_lowercase();
    data.get(&tag).map(|v| !v.is_empty()).unwrap_or(false)
}

fn replace_data_tags(template: &str, data: &HashMap<String, String>) -> String {
    let mut result = template.to_string();

    // Sort keys by length descending to avoid partial replacements
    let mut keys: Vec<&String> = data.keys().collect();
    keys.sort_by(|a, b| b.len().cmp(&a.len()));

    for key in keys {
        let tag = format!("{{{{{}}}}}", key);
        if let Some(value) = data.get(key) {
            result = result.replace(&tag, value);
        }
    }

    result
}

fn render_layout_tags(template: &str, ctx: &RenderContext, title_color: &str, pseudo: &str) -> String {
    let mut result = String::new();
    let mut pos = 0;

    while pos < template.len() {
        if template[pos..].starts_with("{{") {
            if let Some(end) = template[pos + 2..].find("}}") {
                let tag_content = &template[pos + 2..pos + 2 + end];
                let after = pos + 2 + end + 2;

                if let Some(rendered) = render_single_layout_tag(tag_content, ctx, title_color, pseudo)
                {
                    result.push_str(&rendered);
                    pos = after;
                    continue;
                }
            }
        }
        if pos < template.len() {
            let ch = &template[pos..pos + 1];
            result.push_str(ch);
            pos += 1;
        }
    }

    result
}

/// Extract an optional hex color from the end of a tag argument.
/// Returns (text, color). If no valid 6-char hex found at end, uses default.
fn extract_optional_color<'a>(arg: &'a str, default: &'a str) -> (&'a str, &'a str) {
    if let Some(rpos) = arg.rfind(':') {
        let candidate = &arg[rpos + 1..];
        if candidate.len() == 6 && candidate.chars().all(|c| c.is_ascii_hexdigit()) {
            return (&arg[..rpos], candidate);
        }
    }
    (arg, default)
}

fn render_single_layout_tag(
    tag_content: &str,
    ctx: &RenderContext,
    title_color: &str,
    pseudo: &str,
) -> Option<String> {
    // Parse tag:arg format
    let (tag_name, arg) = if let Some(colon_pos) = tag_content.find(':') {
        (
            tag_content[..colon_pos].trim(),
            Some(tag_content[colon_pos + 1..].trim()),
        )
    } else {
        (tag_content.trim(), None)
    };

    match tag_name.to_lowercase().as_str() {
        // --- Line break ---
        "br" => Some("\n".to_string()),

        // --- Headings with optional color ---
        "heading" => {
            let text = arg.unwrap_or("");
            let (label, col) = extract_optional_color(text, title_color);
            Some(bbcode::heading_title(label, col))
        }
        "section" => {
            let text = arg.unwrap_or("");
            let (label, col) = extract_optional_color(text, title_color);
            Some(bbcode::section_heading(label, col))
        }
        "sub_section" => {
            let text = arg.unwrap_or("");
            let (label, col) = extract_optional_color(text, title_color);
            Some(bbcode::sub_heading(label, col))
        }
        "inline_heading" => {
            let text = arg.unwrap_or("");
            let (label, col) = extract_optional_color(text, title_color);
            Some(bbcode::inline_heading(label, col))
        }

        // --- Field ---
        "field" => {
            let text = arg.unwrap_or("");
            let (label, value) = if let Some(sep) = text.find(':') {
                (&text[..sep], &text[sep + 1..])
            } else {
                (text, "")
            };
            Some(bbcode::field(label, value))
        }

        // --- Separator ---
        "hr" => Some(bbcode::hr()),

        // --- Block pairs: opening/closing tags ---
        // Closing tags
        "/center" => Some("[/center]".to_string()),
        "/quote" => Some("[/quote]".to_string()),
        "/bold" => Some("[/b]".to_string()),
        "/italic" => Some("[/i]".to_string()),
        "/underline" => Some("[/u]".to_string()),
        "/table" => Some("[/table]".to_string()),
        "/tr" => Some("[/tr]\n".to_string()),
        "/spoiler" => Some("[/spoiler]".to_string()),

        // Opening tags: with arg → inline, without arg → opening only
        "quote" => {
            match arg {
                Some(text) if !text.is_empty() => Some(bbcode::quote(text)),
                _ => Some("[quote]".to_string()),
            }
        }
        "center" => {
            match arg {
                Some(text) if !text.is_empty() => Some(bbcode::center(text)),
                _ => Some("[center]".to_string()),
            }
        }
        "bold" => {
            match arg {
                Some(text) if !text.is_empty() => Some(bbcode::bold(text)),
                _ => Some("[b]".to_string()),
            }
        }
        "italic" => {
            match arg {
                Some(text) if !text.is_empty() => Some(bbcode::italic(text)),
                _ => Some("[i]".to_string()),
            }
        }
        "underline" => {
            match arg {
                Some(text) if !text.is_empty() => Some(bbcode::underline(text)),
                _ => Some("[u]".to_string()),
            }
        }

        // --- Table tags ---
        "table" => Some("[table]\n".to_string()),
        "tr" => Some("[tr]\n".to_string()),
        "td" => {
            let text = arg.unwrap_or("");
            Some(bbcode::td(text))
        }
        "th" => {
            let text = arg.unwrap_or("");
            Some(bbcode::th(text))
        }

        // --- Color & Size ---
        "color" => {
            let text = arg.unwrap_or("");
            if let Some(sep) = text.find(':') {
                let hex = &text[..sep];
                let content = &text[sep + 1..];
                Some(bbcode::color(hex, content))
            } else {
                Some(text.to_string())
            }
        }
        "size" => {
            let text = arg.unwrap_or("");
            if let Some(sep) = text.find(':') {
                let size_str = &text[..sep];
                let content = &text[sep + 1..];
                Some(format!("[size={}]{}[/size]", size_str, content))
            } else {
                Some(text.to_string())
            }
        }

        // --- Spoiler ---
        "spoiler" => {
            let text = arg.unwrap_or("");
            if let Some(sep) = text.find(':') {
                let label = &text[..sep];
                let content = &text[sep + 1..];
                Some(bbcode::spoiler(label, content))
            } else {
                // No content → opening tag only
                if text.is_empty() {
                    Some("[spoiler]".to_string())
                } else {
                    Some(format!("[spoiler={}]", text))
                }
            }
        }

        // --- Images ---
        "img" => {
            let url_full = arg.unwrap_or("");
            // Check if last segment after ':' is a number (width)
            if let Some(rpos) = url_full.rfind(':') {
                let candidate = &url_full[rpos + 1..];
                if let Ok(width) = candidate.parse::<u32>() {
                    let url = &url_full[..rpos];
                    return Some(bbcode::img_width(url, width));
                }
            }
            // No width → original size
            Some(bbcode::img(url_full))
        }
        "img_cover" => {
            let url = arg.unwrap_or("");
            Some(bbcode::img_width(url, 264))
        }
        "img_poster" => {
            let url = arg.unwrap_or("");
            Some(bbcode::img_width(url, 300))
        }
        "img_logo" => {
            let url = arg.unwrap_or("");
            Some(bbcode::img_width(url, 200))
        }

        // --- Footer ---
        "footer" => Some(bbcode::footer(pseudo)),

        // --- Composite blocks ---
        "ratings_table" => {
            Some(render_ratings_block(&ctx.ratings, title_color))
        }
        "tech_table" => {
            Some(render_movie_tech_block(ctx.tech.as_ref(), title_color))
        }
        "game_tech_table" => {
            Some(render_game_tech_block(ctx.game_tech.as_ref(), title_color))
        }
        "game_reqs_table" => {
            Some(render_game_reqs_block(ctx.min_reqs.as_ref(), ctx.rec_reqs.as_ref(), title_color))
        }
        "app_tech_table" => {
            Some(render_game_tech_block(None, title_color))
        }
        "screenshots_grid" => {
            Some(render_screenshots_block(&ctx.screenshots, title_color))
        }
        "poster_info" => {
            let info = ctx.info_bbcode.as_deref().unwrap_or("");
            Some(render_poster_info_block(ctx.poster_url.as_deref(), info))
        }
        "cover_info" => {
            let info = ctx.info_bbcode.as_deref().unwrap_or("");
            Some(render_cover_info_block(ctx.cover_url.as_deref(), info))
        }
        "logo_info" => {
            let info = ctx.info_bbcode.as_deref().unwrap_or("");
            Some(render_cover_info_block(ctx.logo_url.as_deref(), info))
        }
        _ => None, // Unknown tag → leave as-is
    }
}

// --- Build data maps from models ---

pub fn build_movie_data(
    movie: &crate::models::Movie,
    tech: Option<&crate::models::MediaTechInfo>,
) -> HashMap<String, String> {
    let mut data = HashMap::new();

    data.insert("titre".into(), movie.title.clone());
    data.insert("titre_maj".into(), movie.title.to_uppercase());
    if let Some(ref ot) = movie.original_title {
        data.insert("titre_original".into(), ot.clone());
    }
    if let Some(y) = movie.year {
        data.insert("annee".into(), y.to_string());
    }
    if let Some(ref d) = movie.release_date {
        data.insert("date_sortie".into(), format_date_fr(d));
    }
    if let Some(ref dur) = movie.duration_formatted() {
        data.insert("duree".into(), dur.clone());
    }
    if !movie.directors.is_empty() {
        data.insert("realisateurs".into(), movie.directors_display());
    }
    if !movie.genres.is_empty() {
        data.insert("genres".into(), movie.genres_display());
    }
    if !movie.countries.is_empty() {
        data.insert("pays".into(), movie.countries_display());
    }
    if !movie.cast.is_empty() {
        data.insert("casting".into(), movie.cast_display(6));
    }
    if let Some(ref s) = movie.synopsis {
        if !s.is_empty() {
            data.insert("synopsis".into(), s.clone());
        }
    }
    if let Some(ref url) = movie.poster_url {
        data.insert("poster_url".into(), url.clone());
    }

    // Links
    if let Some(id) = movie.tmdb_id {
        let tmdb_link = format!("https://www.themoviedb.org/movie/{}", id);
        data.insert("tmdb_link".into(), tmdb_link.clone());
        data.insert("link".into(), tmdb_link);
    }
    if let Some(ref id) = movie.imdb_id {
        data.insert("imdb_link".into(), format!("https://www.imdb.com/title/{}/", id));
    }
    if let Some(ref url) = movie.allocine_url {
        data.insert("allocine_link".into(), url.clone());
    }

    // Ratings as individual tags
    build_ratings_data(&mut data, &movie.ratings);

    // Tech info
    if let Some(t) = tech {
        build_media_tech_data(&mut data, t);
    }

    data
}

pub fn build_series_data(
    series: &crate::models::Series,
    tech: Option<&crate::models::MediaTechInfo>,
) -> HashMap<String, String> {
    let mut data = HashMap::new();

    data.insert("titre".into(), series.title.clone());
    data.insert("titre_maj".into(), series.title.to_uppercase());
    if let Some(ref ot) = series.original_title {
        data.insert("titre_original".into(), ot.clone());
    }
    if let Some(y) = series.year {
        data.insert("annee".into(), y.to_string());
    }
    if let Some(ref d) = series.first_air_date {
        data.insert("premiere_diffusion".into(), format_date_fr(d));
    }
    if let Some(ref s) = series.status {
        data.insert("statut".into(), translate_status(s));
    }
    if let Some(s) = series.seasons_count {
        data.insert("saisons".into(), s.to_string());
    }
    if let Some(e) = series.episodes_count {
        data.insert("episodes".into(), e.to_string());
    }
    if let Some(ref rt) = series.runtime_formatted() {
        data.insert("duree_episode".into(), rt.clone());
    }
    if !series.creators.is_empty() {
        data.insert("createurs".into(), series.creators_display());
    }
    if !series.networks.is_empty() {
        data.insert("chaines".into(), series.networks_display());
    }
    if !series.genres.is_empty() {
        data.insert("genres".into(), series.genres_display());
    }
    if !series.countries.is_empty() {
        data.insert("pays".into(), series.countries_display());
    }
    if !series.cast.is_empty() {
        data.insert("casting".into(), series.cast_display(8));
    }
    if let Some(ref s) = series.synopsis {
        if !s.is_empty() {
            data.insert("synopsis".into(), s.clone());
        }
    }
    if let Some(ref url) = series.poster_url {
        data.insert("poster_url".into(), url.clone());
    }

    // Links
    if let Some(id) = series.tmdb_id {
        let tmdb_link = format!("https://www.themoviedb.org/tv/{}", id);
        data.insert("tmdb_link".into(), tmdb_link.clone());
        data.insert("link".into(), tmdb_link);
    }
    if let Some(ref id) = series.imdb_id {
        data.insert("imdb_link".into(), format!("https://www.imdb.com/title/{}/", id));
    }
    if let Some(ref url) = series.allocine_url {
        data.insert("allocine_link".into(), url.clone());
    }

    build_ratings_data(&mut data, &series.ratings);

    if let Some(t) = tech {
        build_media_tech_data(&mut data, t);
    }

    data
}

pub fn build_game_data(game: &crate::models::Game) -> HashMap<String, String> {
    let mut data = HashMap::new();

    data.insert("titre".into(), game.title.clone());
    data.insert("titre_maj".into(), game.title.to_uppercase());
    if let Some(ref d) = game.release_date {
        data.insert("date_sortie".into(), d.clone());
    }
    if let Some(y) = game.year {
        data.insert("annee".into(), y.to_string());
    }
    if let Some(ref s) = game.synopsis {
        if !s.is_empty() {
            data.insert("synopsis".into(), s.clone());
        }
    }
    if let Some(ref url) = game.cover_url {
        data.insert("cover_url".into(), url.clone());
    }
    if !game.genres.is_empty() {
        data.insert("genres".into(), game.genres_display());
    }
    if !game.platforms.is_empty() {
        data.insert("plateformes".into(), game.platforms_display());
    }
    if !game.developers.is_empty() {
        data.insert("developpeurs".into(), game.developers_display());
    }
    if !game.publishers.is_empty() {
        data.insert("editeurs".into(), game.publishers_display());
    }
    if !game.screenshots.is_empty() {
        // Store individual screenshot URLs
        for (i, ss) in game.screenshots.iter().take(4).enumerate() {
            data.insert(format!("screenshot_{}", i + 1), ss.clone());
        }
        data.insert("screenshots".into(), "true".into());
    }
    if let Some(ref install) = game.installation {
        data.insert("installation".into(), install.clone());
    }
    if let Some(ref tech) = game.tech_info {
        data.insert("tech_plateforme".into(), tech.platform.clone());
        data.insert("tech_langues".into(), tech.languages.clone());
        data.insert("tech_taille".into(), tech.size.clone());
        if !tech.install_size.is_empty() {
            data.insert("tech_taille_installee".into(), tech.install_size.clone());
        }
    }

    // System requirements
    if let Some(ref reqs) = game.min_reqs {
        if !reqs.is_empty() {
            data.insert("config_mini".into(), format_system_reqs(reqs));
        }
    }
    if let Some(ref reqs) = game.rec_reqs {
        if !reqs.is_empty() {
            data.insert("config_reco".into(), format_system_reqs(reqs));
        }
    }

    // Links
    if let Some(ref slug) = game.igdb_slug {
        let igdb_link = format!("https://www.igdb.com/games/{}", slug);
        data.insert("igdb_link".into(), igdb_link.clone());
        data.insert("link".into(), igdb_link);
    }
    if let Some(appid) = game.steam_appid {
        let steam_link = format!("https://store.steampowered.com/app/{}/", appid);
        data.insert("steam_link".into(), steam_link.clone());
        // If no IGDB link, use Steam as primary
        if !data.contains_key("link") {
            data.insert("link".into(), steam_link);
        }
    }

    build_ratings_data(&mut data, &game.ratings);

    data
}

pub fn build_app_data(app: &crate::models::Application) -> HashMap<String, String> {
    let mut data = HashMap::new();

    data.insert("nom".into(), app.name.clone());
    data.insert("nom_maj".into(), app.name.to_uppercase());
    if let Some(ref v) = app.version {
        data.insert("version".into(), v.clone());
    }
    if let Some(ref d) = app.developer {
        data.insert("developpeur".into(), d.clone());
    }
    if let Some(ref d) = app.description {
        if !d.is_empty() {
            data.insert("description".into(), d.clone());
        }
    }
    if let Some(ref w) = app.website {
        data.insert("site_web".into(), w.clone());
        data.insert("link".into(), w.clone());
    }
    if let Some(ref l) = app.license {
        data.insert("licence".into(), l.clone());
    }
    if !app.platforms.is_empty() {
        data.insert("plateformes".into(), app.platforms_display());
    }
    if let Some(ref url) = app.logo_url {
        data.insert("logo_url".into(), url.clone());
    }

    data
}

// --- Composite block renderers ---

pub fn render_ratings_block(
    ratings: &[crate::models::Rating],
    title_color: &str,
) -> String {
    if ratings.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    out.push_str(&bbcode::section_heading("Notes", title_color));
    out.push_str("\n\n");

    let mut ratings_table = String::new();
    let mut header_row = String::new();
    for rating in ratings {
        header_row.push_str(&bbcode::th(&rating.source));
    }
    ratings_table.push_str(&bbcode::tr(&header_row));
    let mut values_row = String::new();
    for rating in ratings {
        values_row.push_str(&bbcode::td(&bbcode::center(
            &bbcode::colored_rating(rating.value, rating.max),
        )));
    }
    ratings_table.push_str(&bbcode::tr(&values_row));
    out.push_str(&bbcode::table(&ratings_table));

    out
}

pub fn render_movie_tech_block(
    tech: Option<&crate::models::MediaTechInfo>,
    title_color: &str,
) -> String {
    let mut out = String::new();
    out.push_str(&bbcode::sub_heading(
        "Informations techniques",
        title_color,
    ));
    out.push_str("\n\n");

    let quality_val = tech.and_then(|t| t.quality.as_deref()).unwrap_or(" ");
    let codec_val = tech.and_then(|t| t.video_codec.as_deref()).unwrap_or(" ");
    let lang_val = tech.and_then(|t| t.language.as_deref()).unwrap_or(" ");
    let sub_val = tech.and_then(|t| t.subtitles.as_deref()).unwrap_or(" ");
    let audio_val = tech.and_then(|t| t.audio.as_deref());
    let size_val = tech.and_then(|t| t.size.as_deref());

    let mut headers: Vec<&str> = vec!["Qualite", "Codec Video", "Langue(s)", "Sous-titres"];
    let mut values: Vec<&str> = vec![quality_val, codec_val, lang_val, sub_val];

    if let Some(a) = audio_val {
        headers.push("Audio");
        values.push(a);
    }
    if let Some(s) = size_val {
        headers.push("Taille");
        values.push(s);
    }

    let mut tech_table = String::new();
    let mut header_row = String::new();
    for h in &headers {
        header_row.push_str(&bbcode::th(h));
    }
    tech_table.push_str(&bbcode::tr(&header_row));
    let mut val_row = String::new();
    for v in &values {
        val_row.push_str(&bbcode::td(&bbcode::center(v)));
    }
    tech_table.push_str(&bbcode::tr(&val_row));
    out.push_str(&bbcode::table(&tech_table));

    out
}

pub fn render_game_tech_block(
    tech: Option<&crate::models::TechInfo>,
    title_color: &str,
) -> String {
    let mut out = String::new();
    out.push_str(&bbcode::sub_heading(
        "Informations techniques",
        title_color,
    ));
    out.push_str("\n\n");

    let mut tech_headers: Vec<&str> = vec!["Plateforme", "Langue(s)", "Taille"];
    let has_install_size = tech.map_or(false, |t| !t.install_size.is_empty());
    if has_install_size {
        tech_headers.push("Taille installee");
    }

    let mut tech_table = String::new();
    let mut header_row = String::new();
    for h in &tech_headers {
        header_row.push_str(&bbcode::th(h));
    }
    tech_table.push_str(&bbcode::tr(&header_row));
    let mut values_row = String::new();
    if let Some(t) = tech {
        let mut values: Vec<&str> = vec![&t.platform, &t.languages, &t.size];
        if has_install_size {
            values.push(&t.install_size);
        }
        for val in &values {
            let display = if val.is_empty() { " " } else { val };
            values_row.push_str(&bbcode::td(&bbcode::center(display)));
        }
    } else {
        for _ in &tech_headers {
            values_row.push_str(&bbcode::td(&bbcode::center(" ")));
        }
    }
    tech_table.push_str(&bbcode::tr(&values_row));
    out.push_str(&bbcode::table(&tech_table));

    out
}

fn format_system_reqs(reqs: &crate::models::SystemReqs) -> String {
    let fields: [(&str, &str); 5] = [
        ("OS", &reqs.os),
        ("Processeur", &reqs.cpu),
        ("RAM", &reqs.ram),
        ("Carte graphique", &reqs.gpu),
        ("Stockage", &reqs.storage),
    ];
    fields
        .iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(label, value)| bbcode::field(label, value))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn render_game_reqs_block(
    min_reqs: Option<&crate::models::SystemReqs>,
    rec_reqs: Option<&crate::models::SystemReqs>,
    title_color: &str,
) -> String {
    let has_min = min_reqs.map_or(false, |r| !r.is_empty());
    let has_rec = rec_reqs.map_or(false, |r| !r.is_empty());
    if !has_min && !has_rec {
        return String::new();
    }

    let mut out = String::new();
    out.push_str(&bbcode::sub_heading("Configuration requise", title_color));
    out.push_str("\n\n");

    let mut tech_table = String::new();

    // Header row
    let mut header_row = String::new();
    if has_min {
        header_row.push_str(&bbcode::th("Configuration minimale"));
    }
    if has_rec {
        header_row.push_str(&bbcode::th("Configuration recommandee"));
    }
    tech_table.push_str(&bbcode::tr(&header_row));

    // Content row
    let mut content_row = String::new();
    if has_min {
        content_row.push_str(&bbcode::td(&format_system_reqs(min_reqs.unwrap())));
    }
    if has_rec {
        content_row.push_str(&bbcode::td(&format_system_reqs(rec_reqs.unwrap())));
    }
    tech_table.push_str(&bbcode::tr(&content_row));

    out.push_str(&bbcode::table(&tech_table));
    out
}

pub fn render_screenshots_block(
    screenshots: &[String],
    title_color: &str,
) -> String {
    if screenshots.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    out.push_str(&bbcode::section_heading(
        "Screenshots",
        title_color,
    ));
    out.push_str("\n\n");

    let taken: Vec<_> = screenshots.iter().take(4).collect();
    let mut inner = String::new();
    for pair in taken.chunks(2) {
        let line = pair
            .iter()
            .map(|ss| bbcode::img_width(ss, 400))
            .collect::<Vec<_>>()
            .join(" ");
        if !inner.is_empty() {
            inner.push('\n');
        }
        inner.push_str(&line);
    }
    out.push_str(&bbcode::center(&inner));

    out
}

pub fn render_poster_info_block(
    poster_url: Option<&str>,
    info_bbcode: &str,
) -> String {
    let mut out = String::new();
    let mut table_content = String::new();
    let mut row_content = String::new();
    if let Some(poster) = poster_url {
        row_content.push_str(&bbcode::td(&bbcode::center(&bbcode::img_width(poster, 300))));
    }
    row_content.push_str(&bbcode::td(info_bbcode));
    table_content.push_str(&bbcode::tr(&row_content));
    out.push_str(&bbcode::quote(&bbcode::table(&table_content)));
    out
}

pub fn render_cover_info_block(
    cover_url: Option<&str>,
    info_bbcode: &str,
) -> String {
    let mut out = String::new();
    let mut table_content = String::new();
    let mut row_content = String::new();
    if let Some(cover) = cover_url {
        row_content
            .push_str(&bbcode::td(&bbcode::center(&bbcode::img_width(cover, 264))));
    }
    row_content.push_str(&bbcode::td(info_bbcode));
    table_content.push_str(&bbcode::tr(&row_content));
    out.push_str(&bbcode::quote(&bbcode::table(&table_content)));
    out
}

// --- Preview with sample data ---

pub fn preview_template(
    template_body: &str,
    content_type: &str,
    title_color: &str,
    pseudo: &str,
) -> String {
    let (data, ctx) = build_sample_data(content_type, title_color);
    render(template_body, &data, &ctx, title_color, pseudo)
}

fn build_sample_data(
    content_type: &str,
    title_color: &str,
) -> (HashMap<String, String>, RenderContext) {
    match content_type {
        "film" => build_sample_movie(title_color),
        "serie" => build_sample_series(title_color),
        "jeu" => build_sample_game(title_color),
        "app" => build_sample_app(),
        _ => (HashMap::new(), RenderContext::default()),
    }
}

fn build_sample_movie(title_color: &str) -> (HashMap<String, String>, RenderContext) {
    use crate::models::{Movie, MediaTechInfo, Rating, Genre, Country, Person};

    let movie = Movie {
        title: "Interstellar".into(),
        original_title: Some("Interstellar".into()),
        year: Some(2014),
        release_date: Some("2014-11-05".into()),
        duration_minutes: Some(169),
        synopsis: Some("Les aventures d'un groupe d'explorateurs qui utilisent une faille dans l'espace-temps pour repousser les limites de l'exploration spatiale et conquerir les distances astronomiques.".into()),
        poster_url: Some("https://image.tmdb.org/t/p/w500/gEU2QniE6E77NI6lCU6MxlNBvIx.jpg".into()),
        backdrop_url: None,
        genres: vec![
            Genre { name: "Science-Fiction".into() },
            Genre { name: "Drame".into() },
            Genre { name: "Aventure".into() },
        ],
        countries: vec![Country { name: "Etats-Unis".into(), iso_code: Some("US".into()) }],
        directors: vec![Person { name: "Christopher Nolan".into(), role: None }],
        cast: vec![
            Person { name: "Matthew McConaughey".into(), role: Some("Cooper".into()) },
            Person { name: "Anne Hathaway".into(), role: Some("Brand".into()) },
            Person { name: "Jessica Chastain".into(), role: Some("Murph".into()) },
            Person { name: "Michael Caine".into(), role: Some("Professeur Brand".into()) },
        ],
        ratings: vec![
            Rating { source: "TMDB".into(), value: 8.4, max: 10.0 },
            Rating { source: "Allocine".into(), value: 4.2, max: 5.0 },
        ],
        tmdb_id: Some(157336),
        imdb_id: Some("tt0816692".into()),
        allocine_url: None,
    };

    let tech = MediaTechInfo {
        quality: Some("1080p".into()),
        video_codec: Some("x264".into()),
        audio: Some("DTS-HD MA 5.1".into()),
        language: Some("Multi (FR, EN)".into()),
        subtitles: Some("FR, EN".into()),
        size: Some("12.5 Go".into()),
    };

    let data = build_movie_data(&movie, Some(&tech));
    let info_bbcode = build_sample_movie_info(&movie, title_color);
    let ctx = RenderContext {
        ratings: movie.ratings.clone(),
        poster_url: movie.poster_url.clone(),
        tech: Some(tech),
        info_bbcode: Some(info_bbcode),
        ..Default::default()
    };
    (data, ctx)
}

fn build_sample_movie_info(movie: &crate::models::Movie, title_color: &str) -> String {
    let mut info = String::new();
    info.push_str(&bbcode::field("Origine", &movie.countries_display()));
    info.push('\n');
    if let Some(ref d) = movie.release_date {
        info.push_str(&bbcode::field("Sortie", &format_date_fr(d)));
        info.push('\n');
    }
    if let Some(ref dur) = movie.duration_formatted() {
        info.push_str(&bbcode::field("Duree", dur));
        info.push('\n');
    }
    info.push_str(&bbcode::field("Realisateur", &movie.directors_display()));
    info.push('\n');
    info.push_str(&bbcode::field("Genres", &movie.genres_display()));
    info.push('\n');
    info.push('\n');
    info.push_str(&bbcode::inline_heading("Casting", title_color));
    info.push_str("\n\n");
    info.push_str(&bbcode::field("Acteurs", &movie.cast_display(6)));
    info.push('\n');
    info
}

fn build_sample_series(title_color: &str) -> (HashMap<String, String>, RenderContext) {
    use crate::models::{Series, MediaTechInfo, Rating, Genre, Country, Person};

    let series = Series {
        title: "Breaking Bad".into(),
        original_title: Some("Breaking Bad".into()),
        year: Some(2008),
        end_year: Some(2013),
        first_air_date: Some("2008-01-20".into()),
        synopsis: Some("Un professeur de chimie atteint d'un cancer du poumon s'associe a un ancien eleve pour fabriquer et vendre de la methamphétamine.".into()),
        poster_url: Some("https://image.tmdb.org/t/p/w500/ggFHVNu6YYI5L9pCfOacjizRGt.jpg".into()),
        backdrop_url: None,
        genres: vec![
            Genre { name: "Drame".into() },
            Genre { name: "Crime".into() },
        ],
        countries: vec![Country { name: "Etats-Unis".into(), iso_code: Some("US".into()) }],
        creators: vec![Person { name: "Vince Gilligan".into(), role: None }],
        cast: vec![
            Person { name: "Bryan Cranston".into(), role: Some("Walter White".into()) },
            Person { name: "Aaron Paul".into(), role: Some("Jesse Pinkman".into()) },
            Person { name: "Anna Gunn".into(), role: Some("Skyler White".into()) },
        ],
        ratings: vec![
            Rating { source: "TMDB".into(), value: 8.9, max: 10.0 },
            Rating { source: "Allocine".into(), value: 4.6, max: 5.0 },
        ],
        seasons_count: Some(5),
        episodes_count: Some(62),
        episode_runtime: Some(47),
        status: Some("Ended".into()),
        networks: vec!["AMC".into()],
        tmdb_id: Some(1396),
        imdb_id: Some("tt0903747".into()),
        allocine_url: None,
    };

    let tech = MediaTechInfo {
        quality: Some("1080p".into()),
        video_codec: Some("x265".into()),
        audio: Some("AAC 5.1".into()),
        language: Some("Multi (FR, EN)".into()),
        subtitles: Some("FR".into()),
        size: Some("45.2 Go".into()),
    };

    let data = build_series_data(&series, Some(&tech));
    let info_bbcode = build_sample_series_info(&series, title_color);
    let ctx = RenderContext {
        ratings: series.ratings.clone(),
        poster_url: series.poster_url.clone(),
        tech: Some(tech),
        info_bbcode: Some(info_bbcode),
        ..Default::default()
    };
    (data, ctx)
}

fn build_sample_series_info(series: &crate::models::Series, title_color: &str) -> String {
    let mut info = String::new();
    info.push_str(&bbcode::field("Origine", &series.countries_display()));
    info.push('\n');
    if let Some(ref d) = series.first_air_date {
        info.push_str(&bbcode::field("Premiere diffusion", &format_date_fr(d)));
        info.push('\n');
    }
    if let Some(ref s) = series.status {
        info.push_str(&bbcode::field("Statut", &translate_status(s)));
        info.push('\n');
    }
    if let Some(s) = series.seasons_count {
        info.push_str(&bbcode::field("Saisons", &s.to_string()));
        info.push('\n');
    }
    if let Some(e) = series.episodes_count {
        info.push_str(&bbcode::field("Episodes", &e.to_string()));
        info.push('\n');
    }
    if let Some(ref rt) = series.runtime_formatted() {
        info.push_str(&bbcode::field("Duree par episode", rt));
        info.push('\n');
    }
    info.push_str(&bbcode::field("Createur(s)", &series.creators_display()));
    info.push('\n');
    info.push_str(&bbcode::field("Chaine / Plateforme", &series.networks_display()));
    info.push('\n');
    info.push_str(&bbcode::field("Genres", &series.genres_display()));
    info.push('\n');
    info.push('\n');
    info.push_str(&bbcode::inline_heading("Casting", title_color));
    info.push_str("\n\n");
    info.push_str(&bbcode::field("Acteurs", &series.cast_display(8)));
    info.push('\n');
    info
}

fn build_sample_game(_title_color: &str) -> (HashMap<String, String>, RenderContext) {
    use crate::models::{Game, TechInfo, SystemReqs, Rating, Genre};

    let tech_info = TechInfo {
        platform: "PC (Windows)".into(),
        languages: "FR, EN, DE, ES".into(),
        size: "85.3 Go".into(),
        install_size: "120 Go".into(),
    };

    let min_reqs = SystemReqs {
        os: "Windows 10 64-bit".into(),
        cpu: "Intel Core i5-3570K / AMD FX-8310".into(),
        ram: "8 Go".into(),
        gpu: "NVIDIA GTX 970 / AMD RX 470".into(),
        storage: "70 Go SSD".into(),
    };

    let rec_reqs = SystemReqs {
        os: "Windows 10/11 64-bit".into(),
        cpu: "Intel Core i7-4790 / AMD Ryzen 3 3200G".into(),
        ram: "12 Go".into(),
        gpu: "NVIDIA GTX 1060 6Go / AMD RX 590".into(),
        storage: "70 Go SSD".into(),
    };

    let game = Game {
        title: "Cyberpunk 2077".into(),
        release_date: Some("10 decembre 2020".into()),
        year: Some(2020),
        synopsis: Some("Cyberpunk 2077 est un RPG en monde ouvert se deroulant a Night City, une megalopole obsedee par le pouvoir, le glamour et la modification corporelle.".into()),
        cover_url: Some("https://images.igdb.com/igdb/image/upload/t_cover_big/co4hkv.png".into()),
        screenshots: vec![
            "https://images.igdb.com/igdb/image/upload/t_screenshot_big/sc7ngs.jpg".into(),
            "https://images.igdb.com/igdb/image/upload/t_screenshot_big/sc7ngt.jpg".into(),
        ],
        genres: vec![
            Genre { name: "RPG".into() },
            Genre { name: "Aventure".into() },
        ],
        platforms: vec!["PC".into(), "PlayStation 5".into(), "Xbox Series X|S".into()],
        developers: vec!["CD Projekt Red".into()],
        publishers: vec!["CD Projekt".into()],
        ratings: vec![
            Rating { source: "IGDB".into(), value: 78.0, max: 100.0 },
        ],
        igdb_id: Some(1877),
        igdb_slug: Some("cyberpunk-2077".into()),
        steam_appid: Some(1091500),
        tech_info: Some(tech_info.clone()),
        installation: Some("1. Extraire l'archive\n2. Lancer le setup\n3. Jouer".into()),
        min_reqs: Some(min_reqs.clone()),
        rec_reqs: Some(rec_reqs.clone()),
    };

    let data = build_game_data(&game);
    let info_bbcode = build_sample_game_info(&game);
    let ctx = RenderContext {
        ratings: game.ratings.clone(),
        cover_url: game.cover_url.clone(),
        screenshots: game.screenshots.clone(),
        game_tech: Some(tech_info),
        min_reqs: Some(min_reqs),
        rec_reqs: Some(rec_reqs),
        info_bbcode: Some(info_bbcode),
        ..Default::default()
    };
    (data, ctx)
}

fn build_sample_game_info(game: &crate::models::Game) -> String {
    let mut info = String::new();
    if let Some(ref d) = game.release_date {
        info.push_str(&bbcode::field("Date de sortie", d));
        info.push('\n');
    }
    info.push_str(&bbcode::field("Developpeur(s)", &game.developers_display()));
    info.push('\n');
    info.push_str(&bbcode::field("Editeur(s)", &game.publishers_display()));
    info.push('\n');
    info.push_str(&bbcode::field("Genres", &game.genres_display()));
    info.push('\n');
    info.push_str(&bbcode::field("Plateformes", &game.platforms_display()));
    info.push('\n');
    info
}

fn build_sample_app() -> (HashMap<String, String>, RenderContext) {
    use crate::models::Application;

    let app = Application {
        name: "qBittorrent".into(),
        version: Some("4.6.3".into()),
        developer: Some("qBittorrent Team".into()),
        description: Some("Client BitTorrent libre et open source avec une interface intuitive, un moteur de recherche integre et le support des flux RSS.".into()),
        website: Some("https://www.qbittorrent.org".into()),
        license: Some("GPLv2".into()),
        platforms: vec!["Windows".into(), "macOS".into(), "Linux".into()],
        logo_url: None,
    };

    let data = build_app_data(&app);
    let info_bbcode = build_sample_app_info(&app);
    let ctx = RenderContext {
        logo_url: app.logo_url.clone(),
        info_bbcode: Some(info_bbcode),
        ..Default::default()
    };
    (data, ctx)
}

fn build_sample_app_info(app: &crate::models::Application) -> String {
    let mut info = String::new();
    info.push_str(&bbcode::field("Nom", &app.name));
    info.push('\n');
    if let Some(ref v) = app.version {
        info.push_str(&bbcode::field("Version", v));
        info.push('\n');
    }
    if let Some(ref d) = app.developer {
        info.push_str(&bbcode::field("Developpeur", d));
        info.push('\n');
    }
    if let Some(ref l) = app.license {
        info.push_str(&bbcode::field("Licence", l));
        info.push('\n');
    }
    if let Some(ref w) = app.website {
        info.push_str(&bbcode::field("Site web", &bbcode::url(w, w)));
        info.push('\n');
    }
    info.push_str(&bbcode::field("Plateformes", &app.platforms_display()));
    info.push('\n');
    info
}

// --- Tag reference ---

pub fn get_available_tags(content_type: &str) -> Vec<TemplateTag> {
    let layout = "Mise en page";
    let formatting = "Formatage";
    let images = "Images";
    let tables = "Tableaux";
    let shortcuts = "Raccourcis";
    let data_cat = "Donnees";
    let tech_cat = "Donnees techniques";
    let links_cat = "Liens";
    let notes_cat = "Notes";
    let cond_cat = "Conditionnel";

    let mut tags = vec![
        // --- Mise en page ---
        tag_ex("heading:texte", "Titre principal centré", layout, "{{heading:Mon Titre}}"),
        tag_ex("section:texte", "Titre de section centré", layout, "{{section:Synopsis}}"),
        tag_ex("section:texte:couleur", "Section avec couleur personnalisée", layout, "{{section:Synopsis:3498db}}"),
        tag_ex("sub_section:texte", "Sous-titre de section", layout, "{{sub_section:Details}}"),
        tag_ex("inline_heading:texte", "Titre inline (dans un bloc)", layout, "{{inline_heading:Casting}}"),
        tag_ex("field:label:valeur", "Champ label : valeur", layout, "{{field:Genre:Action}}"),
        tag("hr", "Ligne de séparation", layout),
        tag("br", "Saut de ligne", layout),
        tag_ex("footer", "Signature 'Upload by [pseudo]'", layout, "{{footer}}"),

        // --- Formatage ---
        tag_ex("bold:texte", "Texte en gras (inline)", formatting, "{{bold:important}}"),
        tag_ex("italic:texte", "Texte en italique (inline)", formatting, "{{italic:emphase}}"),
        tag_ex("underline:texte", "Texte souligné (inline)", formatting, "{{underline:souligné}}"),
        tag_ex("center:texte", "Centrer le texte (inline)", formatting, "{{center:texte centré}}"),
        tag_ex("quote:texte", "Citation (inline)", formatting, "{{quote:contenu cité}}"),
        tag_ex("color:hex:texte", "Texte coloré", formatting, "{{color:e74c3c:rouge}}"),
        tag_ex("size:N:texte", "Taille de texte", formatting, "{{size:18:Gros titre}}"),
        tag_ex("spoiler:label:contenu", "Spoiler avec contenu", formatting, "{{spoiler:Cliquer:texte caché}}"),
        // Block pairs
        tag_ex("center}}...{{/center", "Centrer un bloc de contenu", formatting, "{{center}}...{{/center}}"),
        tag_ex("quote}}...{{/quote", "Bloc citation", formatting, "{{quote}}...{{/quote}}"),
        tag_ex("bold}}...{{/bold", "Bloc gras", formatting, "{{bold}}...{{/bold}}"),
        tag_ex("italic}}...{{/italic", "Bloc italique", formatting, "{{italic}}...{{/italic}}"),

        // --- Images ---
        tag_ex("img:URL", "Image taille originale", images, "{{img:https://...}}"),
        tag_ex("img:URL:largeur", "Image avec largeur en pixels", images, "{{img:https://...:400}}"),
        tag_ex("img_poster:URL", "Image poster (300px)", images, "{{img_poster:{{poster_url}}}}"),
        tag_ex("img_cover:URL", "Image jaquette (264px)", images, "{{img_cover:{{cover_url}}}}"),
        tag_ex("img_logo:URL", "Image logo (200px)", images, "{{img_logo:{{logo_url}}}}"),

        // --- Tableaux ---
        tag_ex("table}}...{{/table", "Bloc tableau", tables, "{{table}}...{{/table}}"),
        tag_ex("tr}}...{{/tr", "Ligne de tableau", tables, "{{tr}}...{{/tr}}"),
        tag_ex("td:contenu", "Cellule de tableau", tables, "{{td:contenu}}"),
        tag_ex("th:contenu", "En-tête de tableau", tables, "{{th:Titre}}"),

        // --- Raccourcis composites ---
        tag("poster_info", "Bloc poster + infos (film/série)", shortcuts),
        tag("cover_info", "Bloc jaquette + infos (jeu)", shortcuts),
        tag("logo_info", "Bloc logo + infos (app)", shortcuts),
        tag("ratings_table", "Tableau des notes formaté", shortcuts),
        tag("tech_table", "Tableau infos techniques (film/série)", shortcuts),
        tag("game_tech_table", "Tableau infos techniques (jeu)", shortcuts),
        tag("game_reqs_table", "Tableau configuration requise min/rec (jeu)", shortcuts),
        tag("screenshots_grid", "Grille de screenshots (jeu)", shortcuts),

        // --- Conditionnel ---
        tag_ex("#if tag", "Affiche le bloc si la balise a une valeur", cond_cat, "{{#if synopsis}}...{{/if}}"),
        tag_ex("#if tag > valeur", "Condition avec comparaison (>, >=, <, <=, ==, !=)", cond_cat, "{{#if ratings_count > 0}}...{{/if}}"),
        tag("/if", "Fin du bloc conditionnel", cond_cat),
    ];

    // --- Données spécifiques au type ---
    match content_type {
        "film" => {
            tags.extend(vec![
                tag("titre", "Titre du film", data_cat),
                tag("titre_maj", "Titre en MAJUSCULES", data_cat),
                tag("titre_original", "Titre original", data_cat),
                tag("annee", "Année de sortie", data_cat),
                tag("date_sortie", "Date de sortie formatée", data_cat),
                tag("duree", "Durée formatée", data_cat),
                tag("realisateurs", "Réalisateur(s)", data_cat),
                tag("genres", "Genres", data_cat),
                tag("pays", "Pays d'origine", data_cat),
                tag("casting", "Acteurs principaux", data_cat),
                tag("synopsis", "Synopsis", data_cat),
                tag("poster_url", "URL de l'affiche", data_cat),
                tag("info_bbcode", "Contenu info auto-généré (origine, durée, casting...)", data_cat),
                // Tech
                tag("tech_qualite", "Qualité (ex: 1080p)", tech_cat),
                tag("tech_codec", "Codec vidéo", tech_cat),
                tag("tech_audio", "Audio", tech_cat),
                tag("tech_langue", "Langue(s)", tech_cat),
                tag("tech_soustitres", "Sous-titres", tech_cat),
                tag("tech_taille", "Taille du fichier", tech_cat),
                // Liens
                tag_ex("link", "Lien principal (TMDB)", links_cat, "{{#if link}}{{field:Lien:{{link}}}}{{/if}}"),
                tag("tmdb_link", "Lien vers la page TMDB", links_cat),
                tag("imdb_link", "Lien vers la page IMDb", links_cat),
                tag("allocine_link", "Lien vers la page Allocine", links_cat),
            ]);
        }
        "serie" => {
            tags.extend(vec![
                tag("titre", "Titre de la série", data_cat),
                tag("titre_maj", "Titre en MAJUSCULES", data_cat),
                tag("titre_original", "Titre original", data_cat),
                tag("annee", "Année de début", data_cat),
                tag("premiere_diffusion", "Date première diffusion", data_cat),
                tag("statut", "Statut (En cours, Terminée...)", data_cat),
                tag("saisons", "Nombre de saisons", data_cat),
                tag("episodes", "Nombre d'épisodes", data_cat),
                tag("duree_episode", "Durée par épisode", data_cat),
                tag("createurs", "Créateur(s)", data_cat),
                tag("chaines", "Chaîne / Plateforme", data_cat),
                tag("genres", "Genres", data_cat),
                tag("pays", "Pays d'origine", data_cat),
                tag("casting", "Acteurs principaux", data_cat),
                tag("synopsis", "Synopsis", data_cat),
                tag("poster_url", "URL de l'affiche", data_cat),
                tag("info_bbcode", "Contenu info auto-généré (origine, statut, casting...)", data_cat),
                // Tech
                tag("tech_qualite", "Qualité", tech_cat),
                tag("tech_codec", "Codec vidéo", tech_cat),
                tag("tech_audio", "Audio", tech_cat),
                tag("tech_langue", "Langue(s)", tech_cat),
                tag("tech_soustitres", "Sous-titres", tech_cat),
                tag("tech_taille", "Taille du fichier", tech_cat),
                // Liens
                tag_ex("link", "Lien principal (TMDB)", links_cat, "{{#if link}}{{field:Lien:{{link}}}}{{/if}}"),
                tag("tmdb_link", "Lien vers la page TMDB", links_cat),
                tag("imdb_link", "Lien vers la page IMDb", links_cat),
                tag("allocine_link", "Lien vers la page Allocine", links_cat),
            ]);
        }
        "jeu" => {
            tags.extend(vec![
                tag("titre", "Titre du jeu", data_cat),
                tag("titre_maj", "Titre en MAJUSCULES", data_cat),
                tag("annee", "Année de sortie", data_cat),
                tag("date_sortie", "Date de sortie", data_cat),
                tag("synopsis", "Description du jeu", data_cat),
                tag("cover_url", "URL de la jaquette", data_cat),
                tag("genres", "Genres", data_cat),
                tag("plateformes", "Plateformes", data_cat),
                tag("developpeurs", "Développeur(s)", data_cat),
                tag("editeurs", "Éditeur(s)", data_cat),
                tag("installation", "Instructions d'installation", data_cat),
                tag("info_bbcode", "Contenu info auto-généré (date, dev, genres...)", data_cat),
                // Tech
                tag("tech_plateforme", "Plateforme technique", tech_cat),
                tag("tech_langues", "Langue(s)", tech_cat),
                tag("tech_taille", "Taille", tech_cat),
                tag("tech_taille_installee", "Taille d'installation", tech_cat),
                // Config requise
                tag("config_mini", "Configuration minimale formatee", tech_cat),
                tag("config_reco", "Configuration recommandee formatee", tech_cat),
                // Liens
                tag_ex("link", "Lien principal (IGDB ou Steam)", links_cat, "{{#if link}}{{field:Lien:{{link}}}}{{/if}}"),
                tag("igdb_link", "Lien vers la page IGDB", links_cat),
                tag("steam_link", "Lien vers la page Steam", links_cat),
            ]);
        }
        "app" => {
            tags.extend(vec![
                tag("nom", "Nom de l'application", data_cat),
                tag("nom_maj", "Nom en MAJUSCULES", data_cat),
                tag("version", "Version", data_cat),
                tag("developpeur", "Développeur", data_cat),
                tag("description", "Description", data_cat),
                tag("site_web", "URL du site web", data_cat),
                tag("licence", "Licence", data_cat),
                tag("plateformes", "Plateformes", data_cat),
                tag("logo_url", "URL du logo", data_cat),
                tag("info_bbcode", "Contenu info auto-généré (nom, version, licence...)", data_cat),
                // Liens
                tag_ex("link", "Lien principal (site web)", links_cat, "{{#if link}}{{field:Lien:{{link}}}}{{/if}}"),
            ]);
        }
        _ => {}
    }

    // --- Notes individuelles (si ratings possibles) ---
    if matches!(content_type, "film" | "serie" | "jeu") {
        tags.extend(vec![
            tag("ratings_count", "Nombre de notes disponibles", notes_cat),
            tag_ex("rating_1_source", "Source de la note 1", notes_cat, "TMDB, Allocine, IGDB..."),
            tag("rating_1_value", "Valeur de la note 1", notes_cat),
            tag("rating_1_max", "Maximum de la note 1", notes_cat),
            tag("rating_1_display", "Note 1 formatée avec couleur", notes_cat),
            tag_ex("rating_2_source", "Source de la note 2", notes_cat, "TMDB, Allocine, IGDB..."),
            tag("rating_2_value", "Valeur de la note 2", notes_cat),
            tag("rating_2_max", "Maximum de la note 2", notes_cat),
            tag("rating_2_display", "Note 2 formatée avec couleur", notes_cat),
        ]);
    }

    tags
}

fn tag(name: &str, desc: &str, category: &str) -> TemplateTag {
    TemplateTag {
        name: name.to_string(),
        description: desc.to_string(),
        category: category.to_string(),
        example: None,
    }
}

fn tag_ex(name: &str, desc: &str, category: &str, example: &str) -> TemplateTag {
    TemplateTag {
        name: name.to_string(),
        description: desc.to_string(),
        category: category.to_string(),
        example: Some(example.to_string()),
    }
}

// --- Helpers ---

fn build_ratings_data(data: &mut HashMap<String, String>, ratings: &[crate::models::Rating]) {
    if !ratings.is_empty() {
        data.insert("has_ratings".into(), "true".into());
        data.insert("ratings_count".into(), ratings.len().to_string());
    }
    for (i, rating) in ratings.iter().enumerate() {
        let idx = i + 1;
        data.insert(format!("rating_{}_source", idx), rating.source.clone());
        data.insert(format!("rating_{}_value", idx), format!("{:.1}", rating.value));
        data.insert(format!("rating_{}_max", idx), format!("{}", rating.max as u32));
        data.insert(
            format!("rating_{}_display", idx),
            bbcode::colored_rating(rating.value, rating.max),
        );
    }
}

fn build_media_tech_data(data: &mut HashMap<String, String>, tech: &crate::models::MediaTechInfo) {
    if let Some(ref q) = tech.quality {
        data.insert("tech_qualite".into(), q.clone());
    }
    if let Some(ref c) = tech.video_codec {
        data.insert("tech_codec".into(), c.clone());
    }
    if let Some(ref a) = tech.audio {
        data.insert("tech_audio".into(), a.clone());
    }
    if let Some(ref l) = tech.language {
        data.insert("tech_langue".into(), l.clone());
    }
    if let Some(ref s) = tech.subtitles {
        data.insert("tech_soustitres".into(), s.clone());
    }
    if let Some(ref s) = tech.size {
        data.insert("tech_taille".into(), s.clone());
    }
}

pub fn format_date_fr_pub(date_str: &str) -> String {
    format_date_fr(date_str)
}

pub fn translate_status_pub(status: &str) -> String {
    translate_status(status)
}

fn format_date_fr(date_str: &str) -> String {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() == 3 {
        let month_name = match parts[1] {
            "01" => "janvier",
            "02" => "fevrier",
            "03" => "mars",
            "04" => "avril",
            "05" => "mai",
            "06" => "juin",
            "07" => "juillet",
            "08" => "aout",
            "09" => "septembre",
            "10" => "octobre",
            "11" => "novembre",
            "12" => "decembre",
            _ => parts[1],
        };
        let day = parts[2].trim_start_matches('0');
        format!("{} {} {}", day, month_name, parts[0])
    } else {
        date_str.to_string()
    }
}

fn translate_status(status: &str) -> String {
    match status {
        "Returning Series" => "En cours".to_string(),
        "Ended" => "Terminee".to_string(),
        "Canceled" => "Annulee".to_string(),
        "In Production" => "En production".to_string(),
        "Planned" => "Planifiee".to_string(),
        _ => status.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_conditionals_present() {
        let mut data = HashMap::new();
        data.insert("titre".into(), "Matrix".into());
        let result = process_conditionals("Before{{#if titre}}SHOWN{{/if}}After", &data);
        assert_eq!(result, "BeforeSHOWNAfter");
    }

    #[test]
    fn test_process_conditionals_absent() {
        let data = HashMap::new();
        let result = process_conditionals("Before{{#if titre}}HIDDEN{{/if}}After", &data);
        assert_eq!(result, "BeforeAfter");
    }

    #[test]
    fn test_replace_data_tags() {
        let mut data = HashMap::new();
        data.insert("titre".into(), "Matrix".into());
        data.insert("annee".into(), "1999".into());
        let result = replace_data_tags("{{titre}} ({{annee}})", &data);
        assert_eq!(result, "Matrix (1999)");
    }

    #[test]
    fn test_render_layout_heading() {
        let ctx = RenderContext::default();
        let result = render_layout_tags(
            "{{heading:TEST}}",
            &ctx,
            "c0392b",
            "",
        );
        assert!(result.contains("[h1]"));
        assert!(result.contains("[color=#c0392b]TEST"));
    }

    #[test]
    fn test_render_layout_field() {
        let ctx = RenderContext::default();
        let result = render_layout_tags(
            "{{field:Genre:Action}}",
            &ctx,
            "c0392b",
            "",
        );
        assert!(result.contains("[b]Genre :[/b] Action"));
    }

    #[test]
    fn test_footer_with_pseudo() {
        let mut data = HashMap::new();
        data.insert("titre".into(), "Test".into());
        let ctx = RenderContext::default();
        let result = render("{{titre}}\n{{footer}}", &data, &ctx, "c0392b", "MonPseudo");
        assert!(result.contains("Upload"));
        assert!(result.contains("MonPseudo"));
    }

    #[test]
    fn test_footer_empty_pseudo() {
        let mut data = HashMap::new();
        data.insert("titre".into(), "Test".into());
        let ctx = RenderContext::default();
        let result = render("{{titre}}\n{{footer}}", &data, &ctx, "c0392b", "");
        assert!(!result.contains("Upload"));
    }

    #[test]
    fn test_br_tag() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("A{{br}}B", &ctx, "c0392b", "");
        assert_eq!(result, "A\nB");
    }

    #[test]
    fn test_block_center() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{center}}texte{{/center}}", &ctx, "c0392b", "");
        assert_eq!(result, "[center]texte[/center]");
    }

    #[test]
    fn test_inline_center() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{center:texte}}", &ctx, "c0392b", "");
        assert_eq!(result, "[center]texte[/center]");
    }

    #[test]
    fn test_block_quote() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{quote}}contenu{{/quote}}", &ctx, "c0392b", "");
        assert_eq!(result, "[quote]contenu[/quote]");
    }

    #[test]
    fn test_block_bold_italic() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{bold}}gras{{/bold}} {{italic}}ital{{/italic}}", &ctx, "c0392b", "");
        assert_eq!(result, "[b]gras[/b] [i]ital[/i]");
    }

    #[test]
    fn test_inline_italic() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{italic:emphase}}", &ctx, "c0392b", "");
        assert_eq!(result, "[i]emphase[/i]");
    }

    #[test]
    fn test_inline_underline() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{underline:souligne}}", &ctx, "c0392b", "");
        assert_eq!(result, "[u]souligne[/u]");
    }

    #[test]
    fn test_td_th() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{td:cellule}}{{th:entete}}", &ctx, "c0392b", "");
        assert_eq!(result, "[td]cellule[/td]\n[th]entete[/th]\n");
    }

    #[test]
    fn test_spoiler_inline() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{spoiler:Cliquer:secret}}", &ctx, "c0392b", "");
        assert_eq!(result, "[spoiler=Cliquer]secret[/spoiler]");
    }

    #[test]
    fn test_img_no_width() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{img:https://example.com/img.jpg}}", &ctx, "c0392b", "");
        assert_eq!(result, "[img]https://example.com/img.jpg[/img]");
    }

    #[test]
    fn test_img_with_width() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{img:https://example.com/img.jpg:400}}", &ctx, "c0392b", "");
        assert_eq!(result, "[img width=400]https://example.com/img.jpg[/img]");
    }

    #[test]
    fn test_section_custom_color() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{section:Synopsis:3498db}}", &ctx, "c0392b", "");
        assert!(result.contains("[color=#3498db]Synopsis"));
        assert!(!result.contains("c0392b"));
    }

    #[test]
    fn test_section_default_color() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{section:Synopsis}}", &ctx, "c0392b", "");
        assert!(result.contains("[color=#c0392b]Synopsis"));
    }

    #[test]
    fn test_extract_optional_color_with_color() {
        let (text, color) = extract_optional_color("Synopsis:3498db", "c0392b");
        assert_eq!(text, "Synopsis");
        assert_eq!(color, "3498db");
    }

    #[test]
    fn test_extract_optional_color_no_color() {
        let (text, color) = extract_optional_color("Synopsis", "c0392b");
        assert_eq!(text, "Synopsis");
        assert_eq!(color, "c0392b");
    }

    #[test]
    fn test_extract_optional_color_not_hex() {
        let (text, color) = extract_optional_color("Synopsis:notahex", "c0392b");
        assert_eq!(text, "Synopsis:notahex");
        assert_eq!(color, "c0392b");
    }

    #[test]
    fn test_ratings_individual_data() {
        use crate::models::Rating;
        let mut data = HashMap::new();
        let ratings = vec![
            Rating { source: "TMDB".into(), value: 8.4, max: 10.0 },
            Rating { source: "Allocine".into(), value: 4.2, max: 5.0 },
        ];
        build_ratings_data(&mut data, &ratings);
        assert_eq!(data.get("ratings_count").unwrap(), "2");
        assert_eq!(data.get("rating_1_source").unwrap(), "TMDB");
        assert_eq!(data.get("rating_1_value").unwrap(), "8.4");
        assert_eq!(data.get("rating_1_max").unwrap(), "10");
        assert!(data.get("rating_1_display").unwrap().contains("[color="));
        assert_eq!(data.get("rating_2_source").unwrap(), "Allocine");
    }

    #[test]
    fn test_info_bbcode_injected() {
        let data = HashMap::new();
        let ctx = RenderContext {
            info_bbcode: Some("test info content".into()),
            ..Default::default()
        };
        let result = render("{{info_bbcode}}", &data, &ctx, "c0392b", "");
        assert!(result.contains("test info content"));
    }

    #[test]
    fn test_table_block() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{table}}{{tr}}{{td:A}}{{/tr}}{{/table}}", &ctx, "c0392b", "");
        assert!(result.contains("[table]"));
        assert!(result.contains("[tr]"));
        assert!(result.contains("[td]A[/td]"));
        assert!(result.contains("[/tr]"));
        assert!(result.contains("[/table]"));
    }

    #[test]
    fn test_tags_have_categories() {
        let tags = get_available_tags("film");
        for t in &tags {
            assert!(!t.category.is_empty(), "Tag '{}' has empty category", t.name);
        }
        // Check a specific category
        let heading = tags.iter().find(|t| t.name.starts_with("heading")).unwrap();
        assert_eq!(heading.category, "Mise en page");
    }

    #[test]
    fn test_nested_conditionals() {
        let mut data = HashMap::new();
        data.insert("outer".into(), "yes".into());
        data.insert("inner".into(), "yes".into());
        let template = "{{#if outer}}A{{#if inner}}B{{/if}}C{{/if}}";
        let result = process_conditionals(template, &data);
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_nested_conditionals_inner_false() {
        let mut data = HashMap::new();
        data.insert("outer".into(), "yes".into());
        let template = "{{#if outer}}A{{#if inner}}B{{/if}}C{{/if}}";
        let result = process_conditionals(template, &data);
        assert_eq!(result, "AC");
    }

    #[test]
    fn test_nested_conditionals_outer_false() {
        let mut data = HashMap::new();
        data.insert("inner".into(), "yes".into());
        let template = "Before{{#if outer}}A{{#if inner}}B{{/if}}C{{/if}}After";
        let result = process_conditionals(template, &data);
        assert_eq!(result, "BeforeAfter");
    }

    #[test]
    fn test_conditional_comparison_greater() {
        let mut data = HashMap::new();
        data.insert("ratings_count".into(), "2".into());
        let result = process_conditionals("{{#if ratings_count > 0}}YES{{/if}}", &data);
        assert_eq!(result, "YES");
    }

    #[test]
    fn test_conditional_comparison_equal() {
        let mut data = HashMap::new();
        data.insert("ratings_count".into(), "0".into());
        let result = process_conditionals("{{#if ratings_count == 0}}ZERO{{/if}}", &data);
        assert_eq!(result, "ZERO");
    }

    #[test]
    fn test_conditional_comparison_fail() {
        let mut data = HashMap::new();
        data.insert("ratings_count".into(), "0".into());
        let result = process_conditionals("{{#if ratings_count > 0}}YES{{/if}}", &data);
        assert_eq!(result, "");
    }

    #[test]
    fn test_conditional_newline_cleanup() {
        let mut data = HashMap::new();
        data.insert("key".into(), "val".into());
        // Newlines around #if should be consumed
        let template = "Before\n{{#if key}}\nContent\n{{/if}}\nAfter";
        let result = process_conditionals(template, &data);
        assert_eq!(result, "Before\nContent\nAfter");
    }

    #[test]
    fn test_conditional_newline_cleanup_false() {
        let data = HashMap::new();
        let template = "Before\n{{#if key}}\nContent\n{{/if}}\nAfter";
        let result = process_conditionals(template, &data);
        assert_eq!(result, "Before\nAfter");
    }

    #[test]
    fn test_indentation_stripped() {
        let mut data = HashMap::new();
        data.insert("key".into(), "val".into());
        let ctx = RenderContext::default();
        let template = "  {{#if key}}\n    {{bold:test}}\n  {{/if}}";
        let result = render(template, &data, &ctx, "c0392b", "");
        assert!(result.contains("[b]test[/b]"));
        // Should not contain leading spaces
        assert!(!result.starts_with("  "));
    }

    #[test]
    fn test_evaluate_condition_string_not_equal() {
        let mut data = HashMap::new();
        data.insert("statut".into(), "Ended".into());
        assert!(evaluate_condition("statut != En cours", &data));
        assert!(!evaluate_condition("statut != Ended", &data));
    }
}
