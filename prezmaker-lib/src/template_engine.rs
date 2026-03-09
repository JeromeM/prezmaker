use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::formatters::bbcode;
use crate::models::{Rating, Tracker};

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
    tracker: Tracker,
    title_color: &str,
) -> String {
    let mut output = template_body.to_string();

    // Pass 1: Process conditionals {{#if tag}}...{{/if}}
    output = process_conditionals(&output, data);

    // Pass 2: Replace data tags {{tag}}
    output = replace_data_tags(&output, data);

    // Pass 3: Render layout tags {{layout:args}}
    output = render_layout_tags(&output, ctx, tracker, title_color);

    // Pass 4: Always append "Prez by Grommey" footer
    let prez_footer = prez_footer(tracker);
    output.push('\n');
    output.push_str(&prez_footer);
    output.push('\n');

    output
}

fn process_conditionals(template: &str, data: &HashMap<String, String>) -> String {
    let mut result = template.to_string();

    // Simple regex-free parser for {{#if tag}}...{{/if}}
    loop {
        let start_marker = "{{#if ";
        let Some(start_pos) = result.find(start_marker) else {
            break;
        };
        let after_start = start_pos + start_marker.len();
        let Some(tag_end) = result[after_start..].find("}}") else {
            break;
        };
        let tag = result[after_start..after_start + tag_end].trim().to_lowercase();
        let block_start = after_start + tag_end + 2;

        let end_marker = format!("{{{{/if}}}}");
        let Some(end_pos) = result[block_start..].find(&end_marker) else {
            break;
        };
        let block_end = block_start + end_pos;
        let full_end = block_end + end_marker.len();

        let has_value = data
            .get(&tag)
            .map(|v| !v.is_empty())
            .unwrap_or(false);

        if has_value {
            let block_content = result[block_start..block_end].to_string();
            result = format!("{}{}{}", &result[..start_pos], block_content, &result[full_end..]);
        } else {
            result = format!("{}{}", &result[..start_pos], &result[full_end..]);
        }
    }

    result
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

fn render_layout_tags(template: &str, ctx: &RenderContext, tracker: Tracker, title_color: &str) -> String {
    let mut result = String::new();
    let mut pos = 0;

    while pos < template.len() {
        if template[pos..].starts_with("{{") {
            if let Some(end) = template[pos + 2..].find("}}") {
                let tag_content = &template[pos + 2..pos + 2 + end];
                let after = pos + 2 + end + 2;

                if let Some(rendered) = render_single_layout_tag(tag_content, ctx, tracker, title_color)
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

fn render_single_layout_tag(
    tag_content: &str,
    ctx: &RenderContext,
    tracker: Tracker,
    title_color: &str,
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
        "heading" => {
            let text = arg.unwrap_or("");
            Some(bbcode::heading_title_for(tracker, text, title_color))
        }
        "section" => {
            let text = arg.unwrap_or("");
            Some(bbcode::section_heading_for(tracker, text, title_color))
        }
        "sub_section" => {
            let text = arg.unwrap_or("");
            Some(bbcode::sub_heading_for(tracker, text, title_color))
        }
        "inline_heading" => {
            let text = arg.unwrap_or("");
            Some(bbcode::inline_heading_for(tracker, text, title_color))
        }
        "field" => {
            // {{field:label:value}}
            let text = arg.unwrap_or("");
            let (label, value) = if let Some(sep) = text.find(':') {
                (&text[..sep], &text[sep + 1..])
            } else {
                (text, "")
            };
            Some(bbcode::field_for(tracker, label, value))
        }
        "hr" => Some(bbcode::hr_for(tracker)),
        "quote" => {
            let text = arg.unwrap_or("");
            Some(bbcode::quote_for(tracker, text))
        }
        "center" => {
            let text = arg.unwrap_or("");
            Some(bbcode::center(text))
        }
        "bold" => {
            let text = arg.unwrap_or("");
            Some(bbcode::bold(text))
        }
        "img" => {
            let url = arg.unwrap_or("");
            Some(bbcode::img_width(url, 300))
        }
        "img_cover" => {
            let url = arg.unwrap_or("");
            Some(bbcode::img_sized_for(tracker, url, 264, 352))
        }
        "img_poster" => {
            let url = arg.unwrap_or("");
            Some(bbcode::img_sized_for(tracker, url, 300, 450))
        }
        "img_logo" => {
            let url = arg.unwrap_or("");
            Some(bbcode::img_sized_for(tracker, url, 200, 200))
        }
        "footer" => Some(bbcode::footer_for(tracker)),
        // Composite blocks
        "ratings_table" => {
            Some(render_ratings_block(&ctx.ratings, tracker, title_color))
        }
        "tech_table" => {
            Some(render_movie_tech_block(ctx.tech.as_ref(), tracker, title_color))
        }
        "game_tech_table" => {
            Some(render_game_tech_block(ctx.game_tech.as_ref(), tracker, title_color))
        }
        "app_tech_table" => {
            // App uses same structure as game tech but always empty
            Some(render_game_tech_block(None, tracker, title_color))
        }
        "screenshots_grid" => {
            Some(render_screenshots_block(&ctx.screenshots, tracker, title_color))
        }
        "poster_info" => {
            let info = ctx.info_bbcode.as_deref().unwrap_or("");
            Some(render_poster_info_block(ctx.poster_url.as_deref(), info, tracker))
        }
        "cover_info" => {
            let info = ctx.info_bbcode.as_deref().unwrap_or("");
            Some(render_cover_info_block(ctx.cover_url.as_deref(), info, tracker))
        }
        "logo_info" => {
            let info = ctx.info_bbcode.as_deref().unwrap_or("");
            Some(render_cover_info_block(ctx.logo_url.as_deref(), info, tracker))
        }
        _ => None, // Unknown tag → leave as-is
    }
}

fn prez_footer(tracker: Tracker) -> String {
    let content = format!(
        "{} {} {}",
        bbcode::color("e74c3c", "Prez"),
        bbcode::color("3498db", "by"),
        bbcode::color("e74c3c", "Grommey")
    );
    match tracker {
        Tracker::C411 => bbcode::center(&bbcode::small(&content)),
        Tracker::TorrXyz => bbcode::center(&content),
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
    tracker: Tracker,
    title_color: &str,
) -> String {
    if ratings.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    out.push_str(&bbcode::section_heading_for(tracker, "Notes", title_color));
    out.push_str("\n\n");

    match tracker {
        Tracker::C411 => {
            let mut ratings_table = String::new();
            let mut header_row = String::new();
            for rating in ratings {
                header_row.push_str(&bbcode::th(&rating.source));
            }
            ratings_table.push_str(&bbcode::tr(&header_row));
            let mut values_row = String::new();
            for rating in ratings {
                values_row.push_str(&bbcode::td(&bbcode::center(
                    &bbcode::colored_rating_for(tracker, rating.value, rating.max),
                )));
            }
            ratings_table.push_str(&bbcode::tr(&values_row));
            out.push_str(&bbcode::table(&ratings_table));
        }
        Tracker::TorrXyz => {
            let mut header_row = String::new();
            let mut values_row = String::new();
            for (i, rating) in ratings.iter().enumerate() {
                if i > 0 {
                    header_row.push_str(&bbcode::th("        "));
                    values_row.push_str(&bbcode::td(""));
                }
                header_row.push_str(&bbcode::rating_header_torrxyz(&rating.source));
                values_row.push_str(&bbcode::td(&bbcode::colored_rating_for(
                    tracker,
                    rating.value,
                    rating.max,
                )));
            }
            let ratings_table = format!("{}{}", bbcode::tr(&header_row), bbcode::tr(&values_row));
            out.push_str(&bbcode::center(&bbcode::table(&ratings_table)));
        }
    }
    out.push('\n');
    out.push_str(&bbcode::hr_for(tracker));
    out.push_str("\n\n");

    out
}

pub fn render_movie_tech_block(
    tech: Option<&crate::models::MediaTechInfo>,
    tracker: Tracker,
    title_color: &str,
) -> String {
    let mut out = String::new();
    out.push_str(&bbcode::sub_heading_for(
        tracker,
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

    match tracker {
        Tracker::C411 => {
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
        }
        Tracker::TorrXyz => {
            for (h, v) in headers.iter().zip(values.iter()) {
                out.push_str(&bbcode::center(&bbcode::field_for(tracker, h, v)));
                out.push('\n');
            }
        }
    }
    out.push('\n');
    out.push_str(&bbcode::hr_for(tracker));
    out.push_str("\n\n");

    out
}

pub fn render_game_tech_block(
    tech: Option<&crate::models::TechInfo>,
    tracker: Tracker,
    title_color: &str,
) -> String {
    let mut out = String::new();
    out.push_str(&bbcode::sub_heading_for(
        tracker,
        "Informations techniques",
        title_color,
    ));
    out.push_str("\n\n");

    let mut tech_headers: Vec<&str> = vec!["Plateforme", "Langue(s)", "Taille"];
    let has_install_size = tech.map_or(false, |t| !t.install_size.is_empty());
    if has_install_size {
        tech_headers.push("Taille installee");
    }

    match tracker {
        Tracker::C411 => {
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
        }
        Tracker::TorrXyz => {
            if let Some(t) = tech {
                let mut values: Vec<&str> = vec![&t.platform, &t.languages, &t.size];
                if has_install_size {
                    values.push(&t.install_size);
                }
                for (h, val) in tech_headers.iter().zip(values.iter()) {
                    let display = if val.is_empty() { " " } else { val };
                    out.push_str(&bbcode::center(&bbcode::field_for(tracker, h, display)));
                    out.push('\n');
                }
            } else {
                for h in &tech_headers {
                    out.push_str(&bbcode::center(&bbcode::field_for(tracker, h, " ")));
                    out.push('\n');
                }
            }
        }
    }
    out.push('\n');
    out.push_str(&bbcode::hr_for(tracker));
    out.push_str("\n\n");

    out
}

pub fn render_screenshots_block(
    screenshots: &[String],
    tracker: Tracker,
    title_color: &str,
) -> String {
    if screenshots.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    out.push_str(&bbcode::section_heading_for(
        tracker,
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
    out.push_str("\n\n");
    out.push_str(&bbcode::hr_for(tracker));
    out.push_str("\n\n");

    out
}

pub fn render_poster_info_block(
    poster_url: Option<&str>,
    info_bbcode: &str,
    tracker: Tracker,
) -> String {
    let mut out = String::new();
    match tracker {
        Tracker::C411 => {
            let mut table_content = String::new();
            let mut row_content = String::new();
            if let Some(poster) = poster_url {
                row_content.push_str(&bbcode::td(&bbcode::center(&bbcode::img_width(poster, 300))));
            }
            row_content.push_str(&bbcode::td(info_bbcode));
            table_content.push_str(&bbcode::tr(&row_content));
            out.push_str(&bbcode::quote(&bbcode::table(&table_content)));
        }
        Tracker::TorrXyz => {
            let mut row_content = String::new();
            if let Some(poster) = poster_url {
                row_content.push_str(&bbcode::td(&format!(
                    "\n{}\n",
                    bbcode::center(&bbcode::img_sized_for(tracker, poster, 300, 450))
                )));
                row_content.push_str(&bbcode::td(""));
            }
            row_content.push_str(&bbcode::td(&format!(
                "\n{}\n",
                bbcode::quote(info_bbcode)
            )));
            let table_content = bbcode::tr(&row_content);
            out.push_str(&bbcode::center(&bbcode::table(&table_content)));
        }
    }
    out
}

pub fn render_cover_info_block(
    cover_url: Option<&str>,
    info_bbcode: &str,
    tracker: Tracker,
) -> String {
    let mut out = String::new();
    match tracker {
        Tracker::C411 => {
            let mut table_content = String::new();
            let mut row_content = String::new();
            if let Some(cover) = cover_url {
                row_content
                    .push_str(&bbcode::td(&bbcode::center(&bbcode::img_width(cover, 264))));
            }
            row_content.push_str(&bbcode::td(info_bbcode));
            table_content.push_str(&bbcode::tr(&row_content));
            out.push_str(&bbcode::quote(&bbcode::table(&table_content)));
        }
        Tracker::TorrXyz => {
            let mut row_content = String::new();
            if let Some(cover) = cover_url {
                row_content.push_str(&bbcode::td(&format!(
                    "\n{}\n",
                    bbcode::center(&bbcode::img_sized_for(tracker, cover, 264, 352))
                )));
                row_content.push_str(&bbcode::td(""));
            }
            row_content.push_str(&bbcode::td(&format!(
                "\n{}\n",
                bbcode::quote(info_bbcode)
            )));
            let table_content = bbcode::tr(&row_content);
            out.push_str(&bbcode::center(&bbcode::table(&table_content)));
        }
    }
    out
}

// --- Preview with sample data ---

pub fn preview_template(
    template_body: &str,
    content_type: &str,
    tracker: Tracker,
    title_color: &str,
) -> String {
    let (data, ctx) = build_sample_data(content_type, tracker, title_color);
    render(template_body, &data, &ctx, tracker, title_color)
}

fn build_sample_data(
    content_type: &str,
    tracker: Tracker,
    title_color: &str,
) -> (HashMap<String, String>, RenderContext) {
    match content_type {
        "film" => build_sample_movie(tracker, title_color),
        "serie" => build_sample_series(tracker, title_color),
        "jeu" => build_sample_game(tracker, title_color),
        "app" => build_sample_app(tracker, title_color),
        _ => (HashMap::new(), RenderContext::default()),
    }
}

fn build_sample_movie(tracker: Tracker, title_color: &str) -> (HashMap<String, String>, RenderContext) {
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
    let info_bbcode = build_sample_movie_info(&movie, tracker, title_color);
    let ctx = RenderContext {
        ratings: movie.ratings.clone(),
        poster_url: movie.poster_url.clone(),
        tech: Some(tech),
        info_bbcode: Some(info_bbcode),
        ..Default::default()
    };
    (data, ctx)
}

fn build_sample_movie_info(movie: &crate::models::Movie, t: Tracker, title_color: &str) -> String {
    let mut info = String::new();
    info.push_str(&bbcode::field_for(t, "Origine", &movie.countries_display()));
    info.push('\n');
    if let Some(ref d) = movie.release_date {
        info.push_str(&bbcode::field_for(t, "Sortie", &format_date_fr(d)));
        info.push('\n');
    }
    if let Some(ref dur) = movie.duration_formatted() {
        info.push_str(&bbcode::field_for(t, "Duree", dur));
        info.push('\n');
    }
    info.push_str(&bbcode::field_for(t, "Realisateur", &movie.directors_display()));
    info.push('\n');
    info.push_str(&bbcode::field_for(t, "Genres", &movie.genres_display()));
    info.push('\n');
    info.push('\n');
    info.push_str(&bbcode::inline_heading_for(t, "Casting", title_color));
    info.push_str("\n\n");
    info.push_str(&bbcode::field_for(t, "Acteurs", &movie.cast_display(6)));
    info.push('\n');
    info
}

fn build_sample_series(tracker: Tracker, title_color: &str) -> (HashMap<String, String>, RenderContext) {
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
    let info_bbcode = build_sample_series_info(&series, tracker, title_color);
    let ctx = RenderContext {
        ratings: series.ratings.clone(),
        poster_url: series.poster_url.clone(),
        tech: Some(tech),
        info_bbcode: Some(info_bbcode),
        ..Default::default()
    };
    (data, ctx)
}

fn build_sample_series_info(series: &crate::models::Series, t: Tracker, title_color: &str) -> String {
    let mut info = String::new();
    info.push_str(&bbcode::field_for(t, "Origine", &series.countries_display()));
    info.push('\n');
    if let Some(ref d) = series.first_air_date {
        info.push_str(&bbcode::field_for(t, "Premiere diffusion", &format_date_fr(d)));
        info.push('\n');
    }
    if let Some(ref s) = series.status {
        info.push_str(&bbcode::field_for(t, "Statut", &translate_status(s)));
        info.push('\n');
    }
    if let Some(s) = series.seasons_count {
        info.push_str(&bbcode::field_for(t, "Saisons", &s.to_string()));
        info.push('\n');
    }
    if let Some(e) = series.episodes_count {
        info.push_str(&bbcode::field_for(t, "Episodes", &e.to_string()));
        info.push('\n');
    }
    if let Some(ref rt) = series.runtime_formatted() {
        info.push_str(&bbcode::field_for(t, "Duree par episode", rt));
        info.push('\n');
    }
    info.push_str(&bbcode::field_for(t, "Createur(s)", &series.creators_display()));
    info.push('\n');
    info.push_str(&bbcode::field_for(t, "Chaine / Plateforme", &series.networks_display()));
    info.push('\n');
    info.push_str(&bbcode::field_for(t, "Genres", &series.genres_display()));
    info.push('\n');
    info.push('\n');
    info.push_str(&bbcode::inline_heading_for(t, "Casting", title_color));
    info.push_str("\n\n");
    info.push_str(&bbcode::field_for(t, "Acteurs", &series.cast_display(8)));
    info.push('\n');
    info
}

fn build_sample_game(tracker: Tracker, title_color: &str) -> (HashMap<String, String>, RenderContext) {
    use crate::models::{Game, TechInfo, Rating, Genre};

    let tech_info = TechInfo {
        platform: "PC (Windows)".into(),
        languages: "FR, EN, DE, ES".into(),
        size: "85.3 Go".into(),
        install_size: "120 Go".into(),
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
        tech_info: Some(tech_info.clone()),
        installation: Some("1. Extraire l'archive\n2. Lancer le setup\n3. Jouer".into()),
    };

    let data = build_game_data(&game);
    let info_bbcode = build_sample_game_info(&game, tracker, title_color);
    let ctx = RenderContext {
        ratings: game.ratings.clone(),
        cover_url: game.cover_url.clone(),
        screenshots: game.screenshots.clone(),
        game_tech: Some(tech_info),
        info_bbcode: Some(info_bbcode),
        ..Default::default()
    };
    (data, ctx)
}

fn build_sample_game_info(game: &crate::models::Game, t: Tracker, _title_color: &str) -> String {
    let mut info = String::new();
    if let Some(ref d) = game.release_date {
        info.push_str(&bbcode::field_for(t, "Date de sortie", d));
        info.push('\n');
    }
    info.push_str(&bbcode::field_for(t, "Developpeur(s)", &game.developers_display()));
    info.push('\n');
    info.push_str(&bbcode::field_for(t, "Editeur(s)", &game.publishers_display()));
    info.push('\n');
    info.push_str(&bbcode::field_for(t, "Genres", &game.genres_display()));
    info.push('\n');
    info.push_str(&bbcode::field_for(t, "Plateformes", &game.platforms_display()));
    info.push('\n');
    info
}

fn build_sample_app(_tracker: Tracker, _title_color: &str) -> (HashMap<String, String>, RenderContext) {
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
    let info_bbcode = build_sample_app_info(&app, _tracker);
    let ctx = RenderContext {
        logo_url: app.logo_url.clone(),
        info_bbcode: Some(info_bbcode),
        ..Default::default()
    };
    (data, ctx)
}

fn build_sample_app_info(app: &crate::models::Application, t: Tracker) -> String {
    let mut info = String::new();
    info.push_str(&bbcode::field_for(t, "Nom", &app.name));
    info.push('\n');
    if let Some(ref v) = app.version {
        info.push_str(&bbcode::field_for(t, "Version", v));
        info.push('\n');
    }
    if let Some(ref d) = app.developer {
        info.push_str(&bbcode::field_for(t, "Developpeur", d));
        info.push('\n');
    }
    if let Some(ref l) = app.license {
        info.push_str(&bbcode::field_for(t, "Licence", l));
        info.push('\n');
    }
    if let Some(ref w) = app.website {
        info.push_str(&bbcode::field_for(t, "Site web", &bbcode::url(w, w)));
        info.push('\n');
    }
    info.push_str(&bbcode::field_for(t, "Plateformes", &app.platforms_display()));
    info.push('\n');
    info
}

// --- Tag reference ---

pub fn get_available_tags(content_type: &str) -> Vec<TemplateTag> {
    let mut tags = vec![
        // Layout tags (all types)
        tag("heading:texte", "Titre principal (adapté au tracker)"),
        tag("section:texte", "Titre de section"),
        tag("sub_section:texte", "Sous-titre de section"),
        tag("inline_heading:texte", "Titre inline (dans un bloc)"),
        tag("field:label:valeur", "Champ label : valeur"),
        tag("hr", "Ligne horizontale"),
        tag("quote:texte", "Citation/bloc quote"),
        tag("center:texte", "Centrer le texte"),
        tag("bold:texte", "Texte en gras"),
        tag("footer", "Signature Upload by Grommey"),
        tag("#if tag", "Bloc conditionnel (affiche si tag a une valeur)"),
        tag("/if", "Fin du bloc conditionnel"),
        tag("ratings_table", "Tableau des notes (bloc composite)"),
        tag("tech_table", "Tableau infos techniques (bloc composite)"),
        tag("poster_info", "Bloc poster + infos (bloc composite)"),
    ];

    match content_type {
        "film" => {
            tags.extend(vec![
                tag("titre", "Titre du film"),
                tag("titre_maj", "Titre en MAJUSCULES"),
                tag("titre_original", "Titre original"),
                tag("annee", "Année de sortie"),
                tag("date_sortie", "Date de sortie formatée"),
                tag("duree", "Durée formatée"),
                tag("realisateurs", "Réalisateur(s)"),
                tag("genres", "Genres"),
                tag("pays", "Pays d'origine"),
                tag("casting", "Acteurs principaux"),
                tag("synopsis", "Synopsis"),
                tag("poster_url", "URL de l'affiche"),
                tag("tech_qualite", "Qualité (ex: 1080p)"),
                tag("tech_codec", "Codec vidéo"),
                tag("tech_audio", "Audio"),
                tag("tech_langue", "Langue(s)"),
                tag("tech_soustitres", "Sous-titres"),
                tag("tech_taille", "Taille du fichier"),
            ]);
        }
        "serie" => {
            tags.extend(vec![
                tag("titre", "Titre de la série"),
                tag("titre_maj", "Titre en MAJUSCULES"),
                tag("titre_original", "Titre original"),
                tag("annee", "Année de début"),
                tag("premiere_diffusion", "Date première diffusion"),
                tag("statut", "Statut (En cours, Terminée...)"),
                tag("saisons", "Nombre de saisons"),
                tag("episodes", "Nombre d'épisodes"),
                tag("duree_episode", "Durée par épisode"),
                tag("createurs", "Créateur(s)"),
                tag("chaines", "Chaîne / Plateforme"),
                tag("genres", "Genres"),
                tag("pays", "Pays d'origine"),
                tag("casting", "Acteurs principaux"),
                tag("synopsis", "Synopsis"),
                tag("poster_url", "URL de l'affiche"),
                tag("tech_qualite", "Qualité"),
                tag("tech_codec", "Codec vidéo"),
                tag("tech_audio", "Audio"),
                tag("tech_langue", "Langue(s)"),
                tag("tech_soustitres", "Sous-titres"),
                tag("tech_taille", "Taille du fichier"),
            ]);
        }
        "jeu" => {
            tags.extend(vec![
                tag("titre", "Titre du jeu"),
                tag("titre_maj", "Titre en MAJUSCULES"),
                tag("annee", "Année de sortie"),
                tag("date_sortie", "Date de sortie"),
                tag("synopsis", "Description du jeu"),
                tag("cover_url", "URL de la jaquette"),
                tag("genres", "Genres"),
                tag("plateformes", "Plateformes"),
                tag("developpeurs", "Développeur(s)"),
                tag("editeurs", "Éditeur(s)"),
                tag("installation", "Instructions d'installation"),
                tag("tech_plateforme", "Plateforme technique"),
                tag("tech_langues", "Langue(s)"),
                tag("tech_taille", "Taille"),
                tag("tech_taille_installee", "Taille d'installation"),
                tag("screenshots", "Active la section screenshots"),
                tag("screenshots_grid", "Grille de screenshots (bloc composite)"),
            ]);
        }
        "app" => {
            tags.extend(vec![
                tag("nom", "Nom de l'application"),
                tag("nom_maj", "Nom en MAJUSCULES"),
                tag("version", "Version"),
                tag("developpeur", "Développeur"),
                tag("description", "Description"),
                tag("site_web", "URL du site web"),
                tag("licence", "Licence"),
                tag("plateformes", "Plateformes"),
                tag("logo_url", "URL du logo"),
            ]);
        }
        _ => {}
    }

    tags
}

fn tag(name: &str, desc: &str) -> TemplateTag {
    TemplateTag {
        name: name.to_string(),
        description: desc.to_string(),
    }
}

// --- Helpers ---

fn build_ratings_data(data: &mut HashMap<String, String>, ratings: &[crate::models::Rating]) {
    if !ratings.is_empty() {
        data.insert("has_ratings".into(), "true".into());
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
            Tracker::C411,
            "c0392b",
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
            Tracker::C411,
            "c0392b",
        );
        assert!(result.contains("[b]Genre :[/b] Action"));
    }

    #[test]
    fn test_prez_footer_always_present() {
        let mut data = HashMap::new();
        data.insert("titre".into(), "Test".into());
        let ctx = RenderContext::default();
        let result = render("{{titre}}", &data, &ctx, Tracker::C411, "c0392b");
        assert!(result.contains("Prez"));
        assert!(result.contains("Grommey"));
    }
}
