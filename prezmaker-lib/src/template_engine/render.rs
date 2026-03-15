use std::collections::HashMap;

use crate::formatters::dispatch;
use crate::formatters::OutputFormat;

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

    loop {
        let start_marker = "{{#if ";
        let end_marker = "{{/if}}";

        let Some(first_end) = result.find(end_marker) else {
            break;
        };

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

        let condition_met = evaluate_condition(condition_str, data);

        let mut content_start = block_start;
        let content_end = block_end;
        if result.as_bytes().get(content_start) == Some(&b'\n') {
            content_start += 1;
        }

        let mut consume_end = full_end;
        if result.as_bytes().get(consume_end) == Some(&b'\n') {
            consume_end += 1;
        }

        let mut consume_start = start_pos;
        if consume_start > 0 && result.as_bytes().get(consume_start - 1) == Some(&b'\n') {
            // Already at line start
        } else {
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

pub(crate) fn evaluate_condition(condition: &str, data: &HashMap<String, String>) -> bool {
    let operators = [">=", "<=", "!=", "==", ">", "<"];
    for op in &operators {
        if let Some(pos) = condition.find(op) {
            let key = condition[..pos].trim().to_lowercase();
            let compare_val = condition[pos + op.len()..].trim();

            let data_val = match data.get(&key) {
                Some(v) => v.as_str(),
                None => return false,
            };

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

            return match *op {
                "==" => data_val == compare_val,
                "!=" => data_val != compare_val,
                _ => false,
            };
        }
    }

    let tag = condition.to_lowercase();
    data.get(&tag).map(|v| !v.is_empty()).unwrap_or(false)
}

pub(crate) fn replace_data_tags(template: &str, data: &HashMap<String, String>) -> String {
    let mut result = template.to_string();

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

/// Parse optional style from tag arg: `content | css-style`
/// Returns (content, Option<style>)
fn parse_style(arg: &str) -> (&str, Option<&str>) {
    // Find pipe separator that's not inside a URL (not preceded by :/)
    // We search from the end to find the LAST pipe
    if let Some(pipe_pos) = arg.rfind(" | ") {
        let content = arg[..pipe_pos].trim();
        let style = arg[pipe_pos + 3..].trim();
        if !style.is_empty() {
            return (content, Some(style));
        }
    }
    (arg, None)
}

/// Parse style from a tag that has no colon arg (e.g. {{hr | border-color:red}})
fn parse_style_no_arg(tag_content: &str) -> (Option<&str>, Option<&str>) {
    if let Some(pipe_pos) = tag_content.find(" | ") {
        let tag = tag_content[..pipe_pos].trim();
        let style = tag_content[pipe_pos + 3..].trim();
        if !style.is_empty() {
            return (Some(tag), Some(style));
        }
        return (Some(tag), None);
    }
    (None, None)
}

fn render_single_layout_tag(
    tag_content: &str,
    ctx: &RenderContext,
    title_color: &str,
    pseudo: &str,
) -> Option<String> {
    let fmt = ctx.output_format;

    // Parse tag:arg format, then parse style from arg
    let (tag_name, raw_arg) = if let Some(colon_pos) = tag_content.find(':') {
        (
            tag_content[..colon_pos].trim(),
            Some(tag_content[colon_pos + 1..].trim()),
        )
    } else {
        (tag_content.trim(), None)
    };

    // Extract style from arg (for tags with args)
    let (arg, style) = match raw_arg {
        Some(a) => {
            let (content, s) = parse_style(a);
            (Some(content), s)
        }
        None => {
            // For tags without arg, check if tag_content itself has a style
            let (_, s) = parse_style_no_arg(tag_content);
            (None, s)
        }
    };

    match tag_name.to_lowercase().as_str() {
        // --- Line break ---
        "br" => Some("\n\n".to_string()),

        // --- Headings with optional color ---
        "heading" => {
            let text = arg.unwrap_or("");
            let (label, col) = extract_optional_color(text, title_color);
            Some(dispatch::heading_title(fmt, label, col, style))
        }
        "section" => {
            let text = arg.unwrap_or("");
            let (label, col) = extract_optional_color(text, title_color);
            Some(dispatch::section_heading(fmt, label, col, style))
        }
        "sub_section" => {
            let text = arg.unwrap_or("");
            let (label, col) = extract_optional_color(text, title_color);
            Some(dispatch::sub_heading(fmt, label, col, style))
        }
        "inline_heading" => {
            let text = arg.unwrap_or("");
            let (label, col) = extract_optional_color(text, title_color);
            Some(dispatch::inline_heading(fmt, label, col))
        }

        // --- Field ---
        "field" => {
            let text = arg.unwrap_or("");
            let (label, value) = if let Some(sep) = text.find(':') {
                (&text[..sep], &text[sep + 1..])
            } else {
                (text, "")
            };
            Some(dispatch::field(fmt, label, value))
        }

        // --- Separator ---
        "hr" => Some(dispatch::hr(fmt, style)),

        // --- Block pairs: closing tags ---
        "/center" => Some(dispatch::close_center(fmt)),
        "/quote" => Some(dispatch::close_quote(fmt)),
        "/bold" => Some(dispatch::close_bold(fmt)),
        "/italic" => Some(dispatch::close_italic(fmt)),
        "/underline" => Some(dispatch::close_underline(fmt)),
        "/table" => Some(dispatch::close_table(fmt)),
        "/tr" => Some(dispatch::close_tr(fmt)),
        "/spoiler" => Some(dispatch::close_spoiler(fmt)),
        "/details" => Some(dispatch::close_details(fmt)),

        // --- Block pairs: opening tags (with arg → inline, without → opening only) ---
        "quote" => {
            match arg {
                Some(text) if !text.is_empty() => Some(dispatch::quote(fmt, text, style)),
                _ => Some(dispatch::open_quote(fmt, style)),
            }
        }
        "center" => {
            match arg {
                Some(text) if !text.is_empty() => Some(dispatch::center(fmt, text, style)),
                _ => Some(dispatch::open_center(fmt, style)),
            }
        }
        "bold" => {
            match arg {
                Some(text) if !text.is_empty() => Some(dispatch::bold(fmt, text, style)),
                _ => Some(dispatch::open_bold(fmt, style)),
            }
        }
        "italic" => {
            match arg {
                Some(text) if !text.is_empty() => Some(dispatch::italic(fmt, text, style)),
                _ => Some(dispatch::open_italic(fmt, style)),
            }
        }
        "underline" => {
            match arg {
                Some(text) if !text.is_empty() => Some(dispatch::underline(fmt, text, style)),
                _ => Some(dispatch::open_underline(fmt, style)),
            }
        }

        // --- Table tags ---
        "table" => Some(dispatch::open_table(fmt, style)),
        "tr" => Some(dispatch::open_tr(fmt, style)),
        "td" => {
            let text = arg.unwrap_or("");
            Some(dispatch::td(fmt, text, style))
        }
        "th" => {
            let text = arg.unwrap_or("");
            Some(dispatch::th(fmt, text, style))
        }

        // --- Color & Size ---
        "color" => {
            let text = arg.unwrap_or("");
            if let Some(sep) = text.find(':') {
                let hex = &text[..sep];
                let content = &text[sep + 1..];
                Some(dispatch::color(fmt, hex, content))
            } else {
                Some(text.to_string())
            }
        }
        "size" => {
            let text = arg.unwrap_or("");
            if let Some(sep) = text.find(':') {
                let size_str = &text[..sep];
                let content = &text[sep + 1..];
                if let Ok(px) = size_str.parse::<u32>() {
                    Some(dispatch::size(fmt, px, content))
                } else {
                    Some(format!("[size={}]{}[/size]", size_str, content))
                }
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
                Some(dispatch::spoiler(fmt, label, content, style))
            } else {
                Some(dispatch::open_spoiler(fmt, text, style))
            }
        }

        // --- HTML-exclusive: details/summary ---
        "details" => {
            let label = arg.unwrap_or("");
            let mut result = dispatch::open_details(fmt, style);
            if !label.is_empty() {
                result.push_str(&dispatch::summary(fmt, label, None));
            }
            Some(result)
        }
        "summary" => {
            let text = arg.unwrap_or("");
            Some(dispatch::summary(fmt, text, style))
        }

        // --- Paragraph (HTML-exclusive, pass-through in BBCode) ---
        "p" => {
            match arg {
                Some(text) if !text.is_empty() => Some(dispatch::p(fmt, text, style)),
                _ => match fmt {
                    OutputFormat::Html => Some(match style {
                        Some(s) if !s.is_empty() => format!("<p style=\"{}\">", s),
                        _ => "<p>".to_string(),
                    }),
                    OutputFormat::Bbcode => Some(String::new()),
                },
            }
        }
        "/p" => match fmt {
            OutputFormat::Html => Some("</p>".to_string()),
            OutputFormat::Bbcode => Some("\n".to_string()),
        },

        // --- Pre (code block) ---
        "pre" => {
            match arg {
                Some(text) if !text.is_empty() => Some(dispatch::pre(fmt, text, style)),
                _ => match fmt {
                    OutputFormat::Html => Some(match style {
                        Some(s) if !s.is_empty() => format!("<pre style=\"{}\">", s),
                        _ => "<pre>".to_string(),
                    }),
                    OutputFormat::Bbcode => Some("[code]".to_string()),
                },
            }
        }
        "/pre" => match fmt {
            OutputFormat::Html => Some("</pre>".to_string()),
            OutputFormat::Bbcode => Some("[/code]".to_string()),
        },

        // --- URL ---
        "url" => {
            let text = arg.unwrap_or("");
            let search_start = text.find("://").map(|p| p + 3).unwrap_or(0);
            if let Some(sep) = text[search_start..].rfind(':') {
                let actual_sep = search_start + sep;
                let href = &text[..actual_sep];
                let label = &text[actual_sep + 1..];
                Some(dispatch::url(fmt, href, label, style))
            } else if !text.is_empty() {
                Some(dispatch::url(fmt, text, text, style))
            } else {
                Some(String::new())
            }
        }

        // --- Images ---
        "img" => {
            let url_full = arg.unwrap_or("");
            if let Some(rpos) = url_full.rfind(':') {
                let candidate = &url_full[rpos + 1..];
                if let Ok(width) = candidate.parse::<u32>() {
                    let url = &url_full[..rpos];
                    return Some(dispatch::img_width(fmt, url, width, style));
                }
            }
            Some(dispatch::img(fmt, url_full, style))
        }
        "img_cover" => {
            let url = arg.unwrap_or("");
            Some(dispatch::img_width(fmt, url, 264, style))
        }
        "img_poster" => {
            let url = arg.unwrap_or("");
            Some(dispatch::img_width(fmt, url, 300, style))
        }
        "img_logo" => {
            let url = arg.unwrap_or("");
            Some(dispatch::img_width(fmt, url, 200, style))
        }

        // --- Footer ---
        "footer" => Some(dispatch::footer(fmt, pseudo)),

        // --- Composite blocks ---
        "ratings_table" => {
            Some(render_ratings_block(&ctx.ratings, title_color, fmt))
        }
        "tech_table" => {
            Some(render_movie_tech_block(ctx.tech.as_ref(), title_color, fmt))
        }
        "game_tech_table" => {
            Some(render_game_tech_block(ctx.game_tech.as_ref(), title_color, fmt))
        }
        "game_reqs_table" => {
            Some(render_game_reqs_block(ctx.min_reqs.as_ref(), ctx.rec_reqs.as_ref(), title_color, fmt))
        }
        "app_tech_table" => {
            Some(render_game_tech_block(None, title_color, fmt))
        }
        "mediainfo_table" => {
            if let Some(ref ma) = ctx.media_analysis {
                Some(render_mediainfo_block(ma, title_color, fmt))
            } else {
                Some(String::new())
            }
        }
        "screenshots_grid" => {
            Some(render_screenshots_block(&ctx.screenshots, title_color, fmt))
        }
        "poster_info" => {
            let info = ctx.info_bbcode.as_deref().unwrap_or("");
            Some(render_poster_info_block(ctx.poster_url.as_deref(), info, fmt))
        }
        "cover_info" => {
            let info = ctx.info_bbcode.as_deref().unwrap_or("");
            Some(render_cover_info_block(ctx.cover_url.as_deref(), info, fmt))
        }
        "logo_info" => {
            let info = ctx.info_bbcode.as_deref().unwrap_or("");
            Some(render_cover_info_block(ctx.logo_url.as_deref(), info, fmt))
        }
        _ => None,
    }
}
