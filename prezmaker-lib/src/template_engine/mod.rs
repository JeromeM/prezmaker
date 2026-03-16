mod blocks;
mod data;
mod preview;
mod render;
mod storage;
mod tags;

use serde::{Deserialize, Serialize};

use crate::formatters::OutputFormat;
use crate::models::Rating;

/// Extra context for composite blocks that need model-level data
#[derive(Default)]
pub struct RenderContext {
    pub output_format: OutputFormat,
    pub ratings: Vec<Rating>,
    pub poster_url: Option<String>,
    pub cover_url: Option<String>,
    pub logo_url: Option<String>,
    pub screenshots: Vec<String>,
    pub tech: Option<crate::models::MediaTechInfo>,
    pub game_tech: Option<crate::models::TechInfo>,
    pub min_reqs: Option<crate::models::SystemReqs>,
    pub rec_reqs: Option<crate::models::SystemReqs>,
    pub media_analysis: Option<crate::models::MediaAnalysis>,
    pub info_bbcode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentTemplate {
    pub name: String,
    pub content_type: String,
    pub body: String,
    pub is_default: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateTag {
    pub name: String,
    pub description: String,
    pub category: String,
    pub example: Option<String>,
}

// --- Helpers ---

pub(crate) fn format_date_fr(date_str: &str) -> String {
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

pub(crate) fn translate_status(status: &str) -> String {
    match status {
        "Returning Series" => "En cours".to_string(),
        "Ended" => "Terminee".to_string(),
        "Canceled" => "Annulee".to_string(),
        "In Production" => "En production".to_string(),
        "Planned" => "Planifiee".to_string(),
        _ => status.to_string(),
    }
}

pub fn format_date_fr_pub(date_str: &str) -> String {
    format_date_fr(date_str)
}

pub fn translate_status_pub(status: &str) -> String {
    translate_status(status)
}

/// Extract an optional hex color from the end of a tag argument.
/// Returns (text, color). If no valid 6-char hex found at end, uses default.
pub(crate) fn extract_optional_color<'a>(arg: &'a str, default: &'a str) -> (&'a str, &'a str) {
    if let Some(rpos) = arg.rfind(':') {
        let candidate = &arg[rpos + 1..];
        if candidate.len() == 6 && candidate.chars().all(|c| c.is_ascii_hexdigit()) {
            return (&arg[..rpos], candidate);
        }
    }
    (arg, default)
}

// --- Re-exports ---

pub use blocks::{
    render_ratings_block, render_movie_tech_block, render_game_tech_block,
    render_game_reqs_block, render_screenshots_block, render_poster_info_block,
    render_cover_info_block, render_mediainfo_block,
};
pub use data::{
    build_movie_data, build_series_data, build_game_data, build_app_data,
    build_media_analysis_data,
};
pub use preview::{preview_template, preview_template_with_format};
pub use render::render;
pub use storage::{
    save_template_meta, reorder_templates, list_templates, get_template,
    save_template, delete_template, duplicate_template,
};
pub use tags::get_available_tags;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use render::{process_conditionals, replace_data_tags, render_layout_tags, evaluate_condition};

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
        assert_eq!(result, "A\n\nB");
    }

    #[test]
    fn test_url_tag_with_https() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{url:https://example.com:Mon lien}}", &ctx, "c0392b", "");
        assert_eq!(result, "[url=https://example.com]Mon lien[/url]");
    }

    #[test]
    fn test_url_tag_no_label() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{url:https://example.com}}", &ctx, "c0392b", "");
        assert_eq!(result, "[url=https://example.com]https://example.com[/url]");
    }

    #[test]
    fn test_url_tag_with_port() {
        let ctx = RenderContext::default();
        let result = render_layout_tags("{{url:https://example.com:8080/path:Lien}}", &ctx, "c0392b", "");
        assert_eq!(result, "[url=https://example.com:8080/path]Lien[/url]");
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
        data::build_ratings_data(&mut data, &ratings);
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
