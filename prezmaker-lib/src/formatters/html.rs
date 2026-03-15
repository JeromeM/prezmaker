/// HTML helper functions mirroring the BBCode formatter but producing
/// HTML output.  Every wrapping element accepts an optional `style`
/// parameter — when `Some(css)`, a `style="css"` attribute is added;
/// when `None`, the attribute is omitted entirely.

// ── internal helpers ────────────────────────────────────────────────

/// Build a ` style="…"` attribute string.  Returns an empty string
/// when there is nothing to emit.
fn style_attr(style: Option<&str>) -> String {
    match style {
        Some(s) if !s.is_empty() => format!(" style=\"{}\"", s),
        _ => String::new(),
    }
}

/// Merge a base CSS string with an optional user-supplied one.
/// Returns `None` only when both are empty.
fn merge_style(base: &str, user: Option<&str>) -> Option<String> {
    match user {
        Some(s) if !s.is_empty() => Some(format!("{};{}", base, s)),
        _ => {
            if base.is_empty() {
                None
            } else {
                Some(base.to_string())
            }
        }
    }
}

// ── inline / block primitives ───────────────────────────────────────

pub fn center(content: &str, style: Option<&str>) -> String {
    let merged = merge_style("text-align:center", style);
    format!(
        "<div{}>{}</div>",
        style_attr(merged.as_deref()),
        content
    )
}

pub fn bold(content: &str, style: Option<&str>) -> String {
    format!("<strong{}>{}</strong>", style_attr(style), content)
}

pub fn italic(content: &str, style: Option<&str>) -> String {
    format!("<em{}>{}</em>", style_attr(style), content)
}

pub fn underline(content: &str, style: Option<&str>) -> String {
    let merged = merge_style("text-decoration:underline", style);
    format!(
        "<span{}>{}</span>",
        style_attr(merged.as_deref()),
        content
    )
}

pub fn small(content: &str, style: Option<&str>) -> String {
    size(12, content, style)
}

pub fn color(hex: &str, content: &str, style: Option<&str>) -> String {
    let merged = merge_style(&format!("color:#{}", hex), style);
    format!(
        "<span{}>{}</span>",
        style_attr(merged.as_deref()),
        content
    )
}

pub fn size(px: u32, content: &str, style: Option<&str>) -> String {
    let merged = merge_style(&format!("font-size:{}px", px), style);
    format!(
        "<span{}>{}</span>",
        style_attr(merged.as_deref()),
        content
    )
}

pub fn img(url: &str, style: Option<&str>) -> String {
    let merged = merge_style("max-width:100%", style);
    format!(
        "<img src=\"{}\"{}>",
        url,
        style_attr(merged.as_deref())
    )
}

pub fn img_width(url: &str, width: u32, style: Option<&str>) -> String {
    let merged = merge_style(&format!("max-width:100%;width:{}px", width), style);
    format!(
        "<img src=\"{}\"{}>",
        url,
        style_attr(merged.as_deref())
    )
}

pub fn img_dim(url: &str, width: u32, height: u32, style: Option<&str>) -> String {
    let merged = merge_style(
        &format!("width:{}px;height:{}px", width, height),
        style,
    );
    format!(
        "<img src=\"{}\"{}>",
        url,
        style_attr(merged.as_deref())
    )
}

pub fn url(href: &str, label: &str, style: Option<&str>) -> String {
    format!(
        "<a href=\"{}\"{}>{}</a>",
        href,
        style_attr(style),
        label
    )
}

pub fn h1(content: &str, style: Option<&str>) -> String {
    format!("<h1{}>{}</h1>", style_attr(style), content)
}

pub fn h2(content: &str, style: Option<&str>) -> String {
    format!("<h2{}>{}</h2>", style_attr(style), content)
}

pub fn h3(content: &str, style: Option<&str>) -> String {
    format!("<h3{}>{}</h3>", style_attr(style), content)
}

pub fn hr(style: Option<&str>) -> String {
    format!("<hr{}>", style_attr(style))
}

pub fn quote(content: &str, style: Option<&str>) -> String {
    format!("<blockquote{}>{}</blockquote>", style_attr(style), content)
}

pub fn alert(content: &str, style: Option<&str>) -> String {
    // No dedicated alert element in HTML — fall back to blockquote.
    quote(content, style)
}

pub fn spoiler(label: &str, content: &str, style: Option<&str>) -> String {
    format!(
        "<details{}><summary>{}</summary>{}</details>",
        style_attr(style),
        label,
        content
    )
}

// ── table helpers ───────────────────────────────────────────────────

pub fn table(content: &str, style: Option<&str>) -> String {
    let merged = merge_style("width:100%;border-collapse:collapse", style);
    format!(
        "<table{}>\n{}</table>",
        style_attr(merged.as_deref()),
        content
    )
}

pub fn tr(content: &str, style: Option<&str>) -> String {
    format!("<tr{}>\n{}</tr>\n", style_attr(style), content)
}

pub fn td(content: &str, style: Option<&str>) -> String {
    format!("<td{}>{}</td>\n", style_attr(style), content)
}

pub fn th(content: &str, style: Option<&str>) -> String {
    format!("<th{}>{}</th>\n", style_attr(style), content)
}

// ── composite helpers ───────────────────────────────────────────────

/// Format a field label + value on one line.
pub fn field(label: &str, value: &str) -> String {
    format!("{} {}", bold(&format!("{} :", label), None), value)
}

/// Color for a rating value based on thresholds (normalized to 10).
pub fn rating_color(value: f64, max: f64) -> &'static str {
    let normalized = value / max * 10.0;
    if normalized >= 7.0 {
        "27ae60" // green
    } else if normalized >= 5.0 {
        "f39c12" // orange
    } else {
        "c0392b" // red
    }
}

/// Format a rating value with color.
pub fn colored_rating(value: f64, max: f64) -> String {
    let color_hex = rating_color(value, max);
    format!(
        "{} / {}",
        size(24, &bold(&color(color_hex, &format!("{:.1}", value), None), None), None),
        max as u32
    )
}

/// Main title heading.
pub fn heading_title(text: &str, color_hex: &str) -> String {
    center(&h1(&color(color_hex, text, None), None), None)
}

/// Major section heading.
pub fn section_heading(title: &str, color_hex: &str) -> String {
    center(&h2(&color(color_hex, title, None), None), None)
}

/// Minor section heading (same as section heading).
pub fn sub_heading(title: &str, color_hex: &str) -> String {
    section_heading(title, color_hex)
}

/// Inline heading inside a quote block.
pub fn inline_heading(text: &str, color_hex: &str) -> String {
    h2(&color(color_hex, text, None), None)
}

/// Footer signature.
pub fn footer(pseudo: &str) -> String {
    if pseudo.is_empty() {
        return String::new();
    }
    let content = format!(
        "{} {} {}",
        color("e74c3c", "Upload", None),
        color("3498db", "by", None),
        color("e74c3c", pseudo, None)
    );
    center(&small(&content, None), None)
}

// ── tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // -- style helpers --

    #[test]
    fn test_style_attr_none() {
        assert_eq!(style_attr(None), "");
    }

    #[test]
    fn test_style_attr_empty() {
        assert_eq!(style_attr(Some("")), "");
    }

    #[test]
    fn test_style_attr_some() {
        assert_eq!(style_attr(Some("color:red")), " style=\"color:red\"");
    }

    #[test]
    fn test_merge_style_both_empty() {
        assert_eq!(merge_style("", None), None);
    }

    #[test]
    fn test_merge_style_base_only() {
        assert_eq!(merge_style("color:red", None), Some("color:red".to_string()));
    }

    #[test]
    fn test_merge_style_both() {
        assert_eq!(
            merge_style("color:red", Some("font-size:12px")),
            Some("color:red;font-size:12px".to_string())
        );
    }

    // -- primitives --

    #[test]
    fn test_bold() {
        assert_eq!(bold("test", None), "<strong>test</strong>");
    }

    #[test]
    fn test_bold_with_style() {
        assert_eq!(
            bold("test", Some("color:red")),
            "<strong style=\"color:red\">test</strong>"
        );
    }

    #[test]
    fn test_italic() {
        assert_eq!(italic("test", None), "<em>test</em>");
    }

    #[test]
    fn test_underline() {
        assert_eq!(
            underline("test", None),
            "<span style=\"text-decoration:underline\">test</span>"
        );
    }

    #[test]
    fn test_color() {
        assert_eq!(
            color("ff0000", "red", None),
            "<span style=\"color:#ff0000\">red</span>"
        );
    }

    #[test]
    fn test_color_with_style() {
        assert_eq!(
            color("ff0000", "red", Some("font-weight:bold")),
            "<span style=\"color:#ff0000;font-weight:bold\">red</span>"
        );
    }

    #[test]
    fn test_size() {
        assert_eq!(
            size(24, "big", None),
            "<span style=\"font-size:24px\">big</span>"
        );
    }

    #[test]
    fn test_small() {
        assert_eq!(
            small("tiny", None),
            "<span style=\"font-size:12px\">tiny</span>"
        );
    }

    #[test]
    fn test_img() {
        assert_eq!(
            img("http://example.com/img.jpg", None),
            "<img src=\"http://example.com/img.jpg\" style=\"max-width:100%\">"
        );
    }

    #[test]
    fn test_img_with_style() {
        assert_eq!(
            img("http://example.com/img.jpg", Some("border:1px solid")),
            "<img src=\"http://example.com/img.jpg\" style=\"max-width:100%;border:1px solid\">"
        );
    }

    #[test]
    fn test_img_width() {
        assert_eq!(
            img_width("http://example.com/img.jpg", 300, None),
            "<img src=\"http://example.com/img.jpg\" style=\"max-width:100%;width:300px\">"
        );
    }

    #[test]
    fn test_img_dim() {
        assert_eq!(
            img_dim("http://example.com/img.jpg", 264, 352, None),
            "<img src=\"http://example.com/img.jpg\" style=\"width:264px;height:352px\">"
        );
    }

    #[test]
    fn test_url() {
        assert_eq!(
            url("http://example.com", "click", None),
            "<a href=\"http://example.com\">click</a>"
        );
    }

    #[test]
    fn test_h1() {
        assert_eq!(h1("title", None), "<h1>title</h1>");
    }

    #[test]
    fn test_h2() {
        assert_eq!(h2("title", None), "<h2>title</h2>");
    }

    #[test]
    fn test_h3() {
        assert_eq!(h3("title", None), "<h3>title</h3>");
    }

    #[test]
    fn test_hr() {
        assert_eq!(hr(None), "<hr>");
    }

    #[test]
    fn test_hr_with_style() {
        assert_eq!(hr(Some("border:none")), "<hr style=\"border:none\">");
    }

    #[test]
    fn test_quote() {
        assert_eq!(
            quote("content", None),
            "<blockquote>content</blockquote>"
        );
    }

    #[test]
    fn test_alert() {
        assert_eq!(
            alert("warning", None),
            "<blockquote>warning</blockquote>"
        );
    }

    #[test]
    fn test_spoiler() {
        assert_eq!(
            spoiler("Reveal", "hidden", None),
            "<details><summary>Reveal</summary>hidden</details>"
        );
    }

    #[test]
    fn test_spoiler_with_style() {
        assert_eq!(
            spoiler("Reveal", "hidden", Some("border:1px solid")),
            "<details style=\"border:1px solid\"><summary>Reveal</summary>hidden</details>"
        );
    }

    #[test]
    fn test_center() {
        assert_eq!(
            center("middle", None),
            "<div style=\"text-align:center\">middle</div>"
        );
    }

    #[test]
    fn test_center_with_style() {
        assert_eq!(
            center("middle", Some("padding:8px")),
            "<div style=\"text-align:center;padding:8px\">middle</div>"
        );
    }

    // -- table --

    #[test]
    fn test_table() {
        assert_eq!(
            table("rows", None),
            "<table style=\"width:100%;border-collapse:collapse\">\nrows</table>"
        );
    }

    #[test]
    fn test_tr() {
        assert_eq!(tr("cells", None), "<tr>\ncells</tr>\n");
    }

    #[test]
    fn test_td() {
        assert_eq!(td("val", None), "<td>val</td>\n");
    }

    #[test]
    fn test_th() {
        assert_eq!(th("hdr", None), "<th>hdr</th>\n");
    }

    // -- composite --

    #[test]
    fn test_field() {
        assert_eq!(field("Genre", "Action"), "<strong>Genre :</strong> Action");
    }

    #[test]
    fn test_rating_color() {
        assert_eq!(rating_color(8.0, 10.0), "27ae60");
        assert_eq!(rating_color(6.0, 10.0), "f39c12");
        assert_eq!(rating_color(3.0, 10.0), "c0392b");
        assert_eq!(rating_color(4.0, 5.0), "27ae60");
    }

    #[test]
    fn test_heading_title() {
        let result = heading_title("TEST", "c0392b");
        assert_eq!(
            result,
            "<div style=\"text-align:center\"><h1><span style=\"color:#c0392b\">TEST</span></h1></div>"
        );
    }

    #[test]
    fn test_section_heading() {
        let result = section_heading("Notes", "c0392b");
        assert_eq!(
            result,
            "<div style=\"text-align:center\"><h2><span style=\"color:#c0392b\">Notes</span></h2></div>"
        );
    }

    #[test]
    fn test_footer_empty_pseudo() {
        let result = footer("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_footer_with_pseudo() {
        let result = footer("TestUser");
        assert!(result.contains("text-align:center"));
        assert!(result.contains("Upload"));
        assert!(result.contains("TestUser"));
        assert!(result.contains("font-size:12px"));
    }
}
