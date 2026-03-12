use regex::Regex;

pub fn convert_bbcode_to_html(bbcode: &str) -> String {
    let mut html = escape_html(bbcode);

    // [center]...[/center]
    html = replace_tag(&html, r"\[center\]([\s\S]*?)\[/center\]", "<div style=\"text-align:center\">$1</div>");

    // [b]...[/b]
    html = replace_tag(&html, r"\[b\]([\s\S]*?)\[/b\]", "<strong>$1</strong>");

    // [i]...[/i]
    html = replace_tag(&html, r"\[i\]([\s\S]*?)\[/i\]", "<em>$1</em>");

    // [u]...[/u]
    html = replace_tag(&html, r"\[u\]([\s\S]*?)\[/u\]", "<span style=\"text-decoration:underline\">$1</span>");

    // [color=#hex]...[/color]
    html = replace_tag_fn(&html, r"\[color=(#?[0-9a-fA-F]{3,8})\]([\s\S]*?)\[/color\]", |caps| {
        let mut color = caps[1].to_string();
        if !color.starts_with('#') {
            color = format!("#{}", color);
        }
        format!("<span style=\"color:{}\">{}</span>", color, &caps[2])
    });

    // [size=N]...[/size]
    html = replace_tag_fn(&html, r"\[size=(\d+)\]([\s\S]*?)\[/size\]", |caps| {
        let size: u32 = caps[1].parse().unwrap_or(14);
        let px = bbcode_size_to_px(size);
        format!("<span style=\"font-size:{}px\">{}</span>", px, &caps[2])
    });

    // [h1]...[/h1] through [h6]...[/h6]
    for level in 1..=6 {
        let pattern = format!(r"\[h{}\]([\s\S]*?)\[/h{}\]", level, level);
        let replacement = format!("<h{} style=\"margin:0.5em 0\">$1</h{}>", level, level);
        html = replace_tag(&html, &pattern, &replacement);
    }

    // [hr]
    html = html.replace("[hr]", "<hr style=\"border:1px solid #555;margin:1em 0\">");

    // [quote]...[/quote]
    html = replace_tag(&html, r"\[quote\]([\s\S]*?)\[/quote\]",
        "<blockquote style=\"border-left:3px solid #555;padding:8px 16px;margin:8px 0;background:#2a2a2a\">$1</blockquote>");

    // [alert]...[/alert]
    html = replace_tag(&html, r"\[alert\]([\s\S]*?)\[/alert\]",
        "<div style=\"border:1px solid #e74c3c;padding:8px 16px;margin:8px 0;background:#3a1a1a;color:#e74c3c\">$1</div>");

    // [spoiler=label]...[/spoiler]
    html = replace_tag(&html, r"\[spoiler=([^\]]*)\]([\s\S]*?)\[/spoiler\]",
        "<details style=\"margin:8px 0\"><summary style=\"cursor:pointer;color:#aaa\">$1</summary><div style=\"padding:8px\">$2</div></details>");

    // [url=...]...[/url]
    html = replace_tag(&html, r"\[url=([^\]]*)\]([\s\S]*?)\[/url\]",
        "<a href=\"$1\" style=\"color:#3498db\" target=\"_blank\">$2</a>");

    // [img width=N]...[/img]
    html = replace_tag_fn(&html, r"\[img width=(\d+)\]([\s\S]*?)\[/img\]", |caps| {
        format!("<img src=\"{}\" style=\"width:{}px;max-width:100%\" loading=\"lazy\">", &caps[2], &caps[1])
    });

    // [img=WxH]...[/img]
    html = replace_tag_fn(&html, r"\[img=(\d+)x(\d+)\]([\s\S]*?)\[/img\]", |caps| {
        format!("<img src=\"{}\" style=\"width:{}px;height:{}px;max-width:100%\" loading=\"lazy\">", &caps[3], &caps[1], &caps[2])
    });

    // [img]...[/img]
    html = replace_tag(&html, r"\[img\]([\s\S]*?)\[/img\]",
        "<img src=\"$1\" style=\"max-width:100%\" loading=\"lazy\">");

    // Tables: [table], [tr], [td], [th]
    html = replace_tag(&html, r"\[table\]([\s\S]*?)\[/table\]",
        "<table style=\"border-collapse:collapse;margin:8px auto\">$1</table>");
    html = replace_tag(&html, r"\[tr\]([\s\S]*?)\[/tr\]",
        "<tr>$1</tr>");
    html = replace_tag(&html, r"\[th\]([\s\S]*?)\[/th\]",
        "<th style=\"padding:4px 12px;border:1px solid #555;background:#333\">$1</th>");
    html = replace_tag(&html, r"\[td\]([\s\S]*?)\[/td\]",
        "<td style=\"padding:4px 12px;border:1px solid #555;vertical-align:top\">$1</td>");

    // [left]...[/left]
    html = replace_tag(&html, r"\[left\]([\s\S]*?)\[/left\]",
        "<div style=\"text-align:left\">$1</div>");

    // Collapse 3+ consecutive newlines into 2
    while html.contains("\n\n\n") {
        html = html.replace("\n\n\n", "\n\n");
    }

    // Newlines to <br> (after all tag replacements)
    html = html.replace('\n', "<br>");

    // Remove <br> adjacent to block-level elements to prevent excessive spacing.
    // Use regex for comprehensive matching of all block elements.
    let block_els = r"div|table|tr|td|th|blockquote|details|h[1-6]|hr";
    // <br> before any block tag (open or close): <br><div..., <br></div>, <br><table..., etc.
    let re_br_before = Regex::new(&format!(r"<br>(</?(?:{})[\s>/])", block_els)).unwrap();
    // <br> after a closing block tag: </div><br>, </table><br>, </td><br>, etc.
    let re_br_after_close = Regex::new(&format!(r"(</(?:{})>)<br>", block_els)).unwrap();
    // <br> after an opening block tag: <table style="..."><br>, <tr><br>, etc.
    let re_br_after_open = Regex::new(&format!(r"(<(?:{})\b[^>]*>)<br>", block_els)).unwrap();

    loop {
        let prev = html.clone();
        html = re_br_before.replace_all(&html, "$1").to_string();
        html = re_br_after_close.replace_all(&html, "$1").to_string();
        html = re_br_after_open.replace_all(&html, "$1").to_string();
        if html == prev {
            break;
        }
    }

    wrap_in_document(&html)
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn replace_tag(input: &str, pattern: &str, replacement: &str) -> String {
    let re = Regex::new(pattern).unwrap();
    let mut result = input.to_string();
    // Apply repeatedly to handle nested tags
    loop {
        let new = re.replace_all(&result, replacement).to_string();
        if new == result {
            break;
        }
        result = new;
    }
    result
}

fn replace_tag_fn<F>(input: &str, pattern: &str, f: F) -> String
where
    F: Fn(&regex::Captures) -> String,
{
    let re = Regex::new(pattern).unwrap();
    let mut result = input.to_string();
    loop {
        let new = re.replace_all(&result, &f).to_string();
        if new == result {
            break;
        }
        result = new;
    }
    result
}

fn bbcode_size_to_px(size: u32) -> u32 {
    match size {
        1..=8 => 10,
        9..=11 => 12,
        12 => 12,
        13..=14 => 14,
        15..=18 => 16,
        19..=22 => 20,
        23..=26 => 24,
        _ => size,
    }
}

fn wrap_in_document(body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<style>
  body {{
    background: #1a1a2e;
    color: #e0e0e0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    font-size: 14px;
    line-height: 1.6;
    padding: 16px;
    margin: 0;
    word-wrap: break-word;
  }}
  img {{ max-width: 100%; height: auto; }}
  a {{ color: #3498db; }}
  table {{ border-collapse: collapse; }}
  th, td {{ padding: 4px 12px; border: 1px solid #555; }}
  th {{ background: #333; }}
  blockquote {{ border-left: 3px solid #555; padding: 8px 16px; margin: 8px 0; background: #2a2a2a; }}
  hr {{ border: 1px solid #555; margin: 1em 0; }}
</style>
</head>
<body>
{}
</body>
</html>"#,
        body
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_bold() {
        let result = convert_bbcode_to_html("[b]test[/b]");
        assert!(result.contains("<strong>test</strong>"));
    }

    #[test]
    fn test_color() {
        let result = convert_bbcode_to_html("[color=#ff0000]red[/color]");
        assert!(result.contains("color:#ff0000"));
        assert!(result.contains("red"));
    }

    #[test]
    fn test_color_without_hash() {
        let result = convert_bbcode_to_html("[color=c0392b]test[/color]");
        assert!(result.contains("color:#c0392b"));
    }

    #[test]
    fn test_center() {
        let result = convert_bbcode_to_html("[center]centered[/center]");
        assert!(result.contains("text-align:center"));
        assert!(result.contains("centered"));
    }

    #[test]
    fn test_img_width() {
        let result = convert_bbcode_to_html("[img width=300]https://example.com/img.jpg[/img]");
        assert!(result.contains("width:300px"));
        assert!(result.contains("src=\"https://example.com/img.jpg\""));
    }

    #[test]
    fn test_img_dim() {
        let result = convert_bbcode_to_html("[img=264x352]https://example.com/img.jpg[/img]");
        assert!(result.contains("width:264px"));
        assert!(result.contains("height:352px"));
    }

    #[test]
    fn test_table() {
        let result = convert_bbcode_to_html("[table][tr][th]Header[/th][/tr][tr][td]Cell[/td][/tr][/table]");
        assert!(result.contains("<table"));
        assert!(result.contains("<th"));
        assert!(result.contains("Header"));
        assert!(result.contains("<td"));
        assert!(result.contains("Cell"));
    }

    #[test]
    fn test_url() {
        let result = convert_bbcode_to_html("[url=https://example.com]Link[/url]");
        assert!(result.contains("href=\"https://example.com\""));
        assert!(result.contains("Link"));
    }

    #[test]
    fn test_heading() {
        let result = convert_bbcode_to_html("[h1]Title[/h1]");
        assert!(result.contains("<h1"));
        assert!(result.contains("Title"));
    }

    #[test]
    fn test_hr() {
        let result = convert_bbcode_to_html("[hr]");
        assert!(result.contains("<hr"));
    }

    #[test]
    fn test_quote() {
        let result = convert_bbcode_to_html("[quote]quoted text[/quote]");
        assert!(result.contains("<blockquote"));
        assert!(result.contains("quoted text"));
    }

    #[test]
    fn test_newlines_become_br() {
        let result = convert_bbcode_to_html("line1\nline2");
        assert!(result.contains("line1<br>line2"));
    }

    #[test]
    fn test_html_escaping() {
        let result = convert_bbcode_to_html("<script>alert('xss')</script>");
        assert!(!result.contains("<script>"));
        assert!(result.contains("&lt;script&gt;"));
    }

    #[test]
    fn test_size() {
        let result = convert_bbcode_to_html("[size=6]big text[/size]");
        assert!(result.contains("font-size:"));
        assert!(result.contains("big text"));
    }

    #[test]
    fn test_nested_tags() {
        let result = convert_bbcode_to_html("[center][b][color=#c0392b]Title[/color][/b][/center]");
        assert!(result.contains("text-align:center"));
        assert!(result.contains("<strong>"));
        assert!(result.contains("color:#c0392b"));
        assert!(result.contains("Title"));
    }

    #[test]
    fn test_no_br_between_block_elements() {
        // Simulates composite blocks: heading followed by table with newlines between
        let bbcode = "[center][h2][color=#c0392b]Notes[/color][/h2][/center]\n\n[table][tr][td]Content[/td][/tr][/table]\n\n[center][h2][color=#c0392b]Config[/color][/h2][/center]";
        let html = convert_bbcode_to_html(bbcode);
        assert!(!html.contains("</div><br>"), "Found <br> after </div>");
        assert!(!html.contains("<br><table"), "Found <br> before <table>");
        assert!(!html.contains("</table><br>"), "Found <br> after </table>");
        assert!(!html.contains("<br><div"), "Found <br> before <div>");
        assert!(!html.contains("</h2><br>"), "Found <br> after </h2>");
    }

    #[test]
    fn test_no_br_inside_table() {
        // Tables with \n (as produced by bbcode::table/tr/td/th helpers)
        let bbcode = "[table]\n[tr]\n[th]Header[/th]\n[/tr]\n[tr]\n[td]Value[/td]\n[/tr]\n[/table]";
        let html = convert_bbcode_to_html(bbcode);
        // No <br> should survive inside the table structure
        assert!(!html.contains("<br><tr>"), "Found <br> before <tr>");
        assert!(!html.contains("</tr><br>"), "Found <br> after </tr>");
        assert!(!html.contains("</th><br>"), "Found <br> after </th>");
        assert!(!html.contains("</td><br>"), "Found <br> after </td>");
        assert!(!html.contains("<br></tr>"), "Found <br> before </tr>");
        assert!(!html.contains("<br></table>"), "Found <br> before </table>");
        // Verify the table itself renders
        assert!(html.contains("<table"));
        assert!(html.contains("Header"));
        assert!(html.contains("Value"));
    }

    #[test]
    fn test_composite_block_no_gaps() {
        // Simulates the exact pattern: heading + \n + table with \n inside
        let bbcode = "[center][h2][color=#c0392b]Notes[/color][/h2][/center]\n[table]\n[tr]\n[th]NOM[/th]\n[/tr]\n[tr]\n[td]76[/td]\n[/tr]\n[/table]\n\n[center][h2][color=#c0392b]Config[/color][/h2][/center]\n[table]\n[tr]\n[th]Min[/th]\n[th]Rec[/th]\n[/tr]\n[tr]\n[td]i5[/td]\n[td]i7[/td]\n[/tr]\n[/table]";
        let html = convert_bbcode_to_html(bbcode);
        // Count remaining <br> tags — should be zero between/inside block elements
        let br_count = html.matches("<br>").count();
        assert_eq!(br_count, 0, "Expected 0 <br> tags but found {}. HTML:\n{}", br_count, html);
    }

    #[test]
    fn test_movie_like_output() {
        let bbcode = r#"[center][h1][color=#c0392b]🎬 INTOUCHABLES 🎬[/color][/h1][/center]

[center][img]https://www.arobase62.fr/files/Informations.png[/img][/center]

[quote][table]
[tr]
[td][center][img width=300]https://image.tmdb.org/t/p/w500/poster.jpg[/img][/center][/td]
[td][b]Origine :[/b] France
[b]Sortie :[/b] 2 novembre 2011
[b]Duree :[/b] 1h et 52min[/td]
[/tr]
[/table][/quote]"#;
        let html = convert_bbcode_to_html(bbcode);
        assert!(html.contains("INTOUCHABLES"));
        assert!(html.contains("<h1"));
        assert!(html.contains("<table"));
        assert!(html.contains("<blockquote"));
        assert!(html.contains("width:300px"));
    }
}
