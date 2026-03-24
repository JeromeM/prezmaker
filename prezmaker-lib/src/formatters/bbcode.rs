/// BBCode helper functions for UNIT3D compatible output
pub fn center(content: &str) -> String {
    format!("[center]{}[/center]", content)
}

pub fn bold(content: &str) -> String {
    format!("[b]{}[/b]", content)
}

pub fn italic(content: &str) -> String {
    format!("[i]{}[/i]", content)
}

pub fn underline(content: &str) -> String {
    format!("[u]{}[/u]", content)
}

pub fn small(content: &str) -> String {
    size(12, content)
}

pub fn color(hex: &str, content: &str) -> String {
    format!("[color=#{}]{}[/color]", hex, content)
}

pub fn size(px: u32, content: &str) -> String {
    format!("[size={}]{}[/size]", px, content)
}

pub fn img(url: &str) -> String {
    format!("[img]{}[/img]", url)
}

pub fn img_width(url: &str, width: u32) -> String {
    format!("[img width={}]{}[/img]", width, url)
}

pub fn url(href: &str, label: &str) -> String {
    format!("[url={}]{}[/url]", href, label)
}

pub fn h1(content: &str) -> String {
    format!("[h1]{}[/h1]", content)
}

pub fn h2(content: &str) -> String {
    format!("[h2]{}[/h2]", content)
}

pub fn h3(content: &str) -> String {
    format!("[h3]{}[/h3]", content)
}

pub fn hr() -> String {
    "[hr]".to_string()
}

pub fn quote(content: &str) -> String {
    format!("[quote]{}[/quote]", content)
}

pub fn alert(content: &str) -> String {
    format!("[alert]{}[/alert]", content)
}

pub fn spoiler(label: &str, content: &str) -> String {
    format!("[spoiler={}]{}[/spoiler]", label, content)
}

pub fn table(content: &str) -> String {
    format!("[table]\n{}[/table]", content)
}

pub fn tr(content: &str) -> String {
    format!("[tr]\n{}[/tr]\n", content)
}

pub fn td(content: &str) -> String {
    format!("[td]{}[/td]\n", content)
}

pub fn th(content: &str) -> String {
    format!("[th]{}[/th]\n", content)
}

/// Format a field label + value on one line
pub fn field(label: &str, value: &str) -> String {
    format!("{} {}", bold(&format!("{} :", label)), value)
}

/// Image with exact dimensions [img=WxH]
pub fn img_dim(url: &str, width: u32, height: u32) -> String {
    format!("[img={}x{}]{}[/img]", width, height, url)
}

/// Color for a rating value based on thresholds (normalized to 10)
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

/// Format a rating value with color
pub fn colored_rating(value: f64, max: f64) -> String {
    let color_hex = rating_color(value, max);
    format!(
        "{} / {}",
        size(24, &bold(&color(color_hex, &format!("{:.1}", value)))),
        max as u32
    )
}

/// Main title heading
pub fn heading_title(text: &str, color_hex: &str) -> String {
    center(&h1(&color(color_hex, text)))
}

/// Major section heading
pub fn section_heading(title: &str, color_hex: &str) -> String {
    center(&h2(&color(color_hex, title)))
}

/// Minor section heading (same as section heading)
pub fn sub_heading(title: &str, color_hex: &str) -> String {
    section_heading(title, color_hex)
}

/// Inline heading inside a quote block
pub fn inline_heading(text: &str, color_hex: &str) -> String {
    h2(&color(color_hex, text))
}

/// Footer signature
pub fn footer(pseudo: &str) -> String {
    if pseudo.is_empty() {
        return String::new();
    }
    let content = format!(
        "{} {} {}",
        color("e74c3c", "Upload"),
        color("3498db", "by"),
        color("e74c3c", pseudo)
    );
    center(&small(&content))
}

pub fn youtube(url: &str) -> String {
    format!("[youtube]{}[/youtube]", url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold() {
        assert_eq!(bold("test"), "[b]test[/b]");
    }

    #[test]
    fn test_color() {
        assert_eq!(color("ff0000", "red"), "[color=#ff0000]red[/color]");
    }

    #[test]
    fn test_img_width() {
        assert_eq!(
            img_width("http://example.com/img.jpg", 300),
            "[img width=300]http://example.com/img.jpg[/img]"
        );
    }

    #[test]
    fn test_img_dim() {
        assert_eq!(
            img_dim("http://example.com/img.jpg", 264, 352),
            "[img=264x352]http://example.com/img.jpg[/img]"
        );
    }

    #[test]
    fn test_rating_color() {
        assert_eq!(rating_color(8.0, 10.0), "27ae60");
        assert_eq!(rating_color(6.0, 10.0), "f39c12");
        assert_eq!(rating_color(3.0, 10.0), "c0392b");
        assert_eq!(rating_color(4.0, 5.0), "27ae60");
    }

    #[test]
    fn test_field() {
        assert_eq!(field("Genre", "Action"), "[b]Genre :[/b] Action");
    }

    #[test]
    fn test_heading_title() {
        let result = heading_title("TEST", "c0392b");
        assert_eq!(result, "[center][h1][color=#c0392b]TEST[/color][/h1][/center]");
    }

    #[test]
    fn test_section_heading() {
        let result = section_heading("Notes", "c0392b");
        assert_eq!(result, "[center][h2][color=#c0392b]Notes[/color][/h2][/center]");
    }

    #[test]
    fn test_footer_empty_pseudo() {
        let result = footer("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_footer_with_pseudo() {
        let result = footer("TestUser");
        assert!(result.contains("[center]"));
        assert!(result.contains("Upload"));
        assert!(result.contains("TestUser"));
        assert!(result.contains("[size=12]"));
    }
}
