use std::collections::HashMap;

use crate::formatters::bbcode;

use super::{extract_optional_color, RenderContext};
use super::blocks::{
    render_ratings_block, render_movie_tech_block, render_game_tech_block,
    render_game_reqs_block, render_screenshots_block, render_poster_info_block,
    render_cover_info_block, render_mediainfo_block,
};

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

pub(crate) fn process_conditionals(template: &str, data: &HashMap<String, String>) -> String {
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
pub(crate) fn evaluate_condition(condition: &str, data: &HashMap<String, String>) -> bool {
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

pub(crate) fn replace_data_tags(template: &str, data: &HashMap<String, String>) -> String {
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

pub(crate) fn render_layout_tags(template: &str, ctx: &RenderContext, title_color: &str, pseudo: &str) -> String {
    let mut result = String::new();
    let bytes = template.as_bytes();
    let mut pos = 0;

    while pos < bytes.len() {
        if pos + 1 < bytes.len() && bytes[pos] == b'{' && bytes[pos + 1] == b'{' {
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
        // Advance by one UTF-8 character
        if pos < bytes.len() {
            let b = bytes[pos];
            let char_len = if b < 0x80 { 1 }
                else if b < 0xE0 { 2 }
                else if b < 0xF0 { 3 }
                else { 4 };
            let end = (pos + char_len).min(bytes.len());
            result.push_str(&template[pos..end]);
            pos = end;
        }
    }

    result
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
        "br" => Some("\n\n".to_string()),

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

        // --- URL ---
        "url" => {
            let text = arg.unwrap_or("");
            // Find the label separator (:) AFTER the protocol (://)
            let search_start = text.find("://").map(|p| p + 3).unwrap_or(0);
            if let Some(sep) = text[search_start..].rfind(':') {
                let actual_sep = search_start + sep;
                let href = &text[..actual_sep];
                let label = &text[actual_sep + 1..];
                Some(bbcode::url(href, label))
            } else if !text.is_empty() {
                Some(bbcode::url(text, text))
            } else {
                Some(String::new())
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
        "mediainfo_table" => {
            if let Some(ref ma) = ctx.media_analysis {
                Some(render_mediainfo_block(ma, title_color))
            } else {
                Some(String::new())
            }
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
