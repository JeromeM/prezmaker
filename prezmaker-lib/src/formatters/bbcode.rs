use crate::models::Tracker;

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

pub fn hr_for(tracker: Tracker) -> String {
    match tracker {
        Tracker::C411 => String::new(),
        Tracker::TorrXyz => "[hr]".to_string(),
    }
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

/// Image with exact dimensions [img=WxH]
pub fn img_dim(url: &str, width: u32, height: u32) -> String {
    format!("[img={}x{}]{}[/img]", width, height, url)
}

/// Tracker-aware image: C411 uses [img width=W], TorrXyz uses [img=WxH]
pub fn img_sized_for(tracker: Tracker, url: &str, width: u32, height: u32) -> String {
    match tracker {
        Tracker::C411 => img_width(url, width),
        Tracker::TorrXyz => img_dim(url, width, height),
    }
}

/// Tracker-aware field: TorrXyz uses colored label (#ff857a) and value (#aaaaaa)
pub fn field_for(tracker: Tracker, label: &str, value: &str) -> String {
    match tracker {
        Tracker::C411 => field(label, value),
        Tracker::TorrXyz => format!(
            "{} {}",
            bold(&color("ff857a", &format!("{} :", label))),
            color("aaaaaa", value)
        ),
    }
}

const ASSETS_BASE_URL: &str = "https://www.arobase62.fr/files";

/// Map section title to asset image filename (for C411)
fn section_image(title: &str) -> Option<&'static str> {
    match title {
        "Informations" => Some("Informations.png"),
        "Description" | "Synopsis" => Some("Description.png"),
        "Notes" => Some("Notes.png"),
        "Screenshots" => Some("Screenshots.png"),
        "Informations techniques" => Some("InfosTech.png"),
        "Installation" => Some("Installation.png"),
        _ => None,
    }
}

/// Main title heading (h1 on C411, [b][color][size=6] on TorrXyz)
pub fn heading_title_for(tracker: Tracker, text: &str, color_hex: &str) -> String {
    match tracker {
        Tracker::C411 => center(&h1(&color(color_hex, text))),
        Tracker::TorrXyz => center(&bold(&color(color_hex, &size(6, text)))),
    }
}

/// Major section heading: C411 uses image if available, TorrXyz uses [b][color][size=6]
pub fn section_heading_for(tracker: Tracker, title: &str, color_hex: &str) -> String {
    match tracker {
        Tracker::C411 => {
            if let Some(filename) = section_image(title) {
                center(&img(&format!("{}/{}", ASSETS_BASE_URL, filename)))
            } else {
                center(&h2(&color(color_hex, title)))
            }
        }
        Tracker::TorrXyz => center(&bold(&color(color_hex, &size(6, title)))),
    }
}

/// Minor section heading: same logic as section_heading
pub fn sub_heading_for(tracker: Tracker, title: &str, color_hex: &str) -> String {
    section_heading_for(tracker, title, color_hex)
}

/// Inline heading inside a quote block (h2 on C411, [b][color] on TorrXyz)
pub fn inline_heading_for(tracker: Tracker, text: &str, color_hex: &str) -> String {
    match tracker {
        Tracker::C411 => h2(&color(color_hex, text)),
        Tracker::TorrXyz => bold(&color(color_hex, text)),
    }
}

/// Tracker-aware quote: TorrXyz wraps content in gray color
pub fn quote_for(tracker: Tracker, content: &str) -> String {
    match tracker {
        Tracker::C411 => quote(content),
        Tracker::TorrXyz => quote(&color("aaaaaa", content)),
    }
}

/// Rating color (brighter for TorrXyz dark background)
pub fn rating_color_for(tracker: Tracker, value: f64, max: f64) -> &'static str {
    let normalized = value / max * 10.0;
    match tracker {
        Tracker::C411 => {
            if normalized >= 7.0 { "27ae60" }
            else if normalized >= 5.0 { "f39c12" }
            else { "c0392b" }
        }
        Tracker::TorrXyz => {
            if normalized >= 7.0 { "55efc4" }
            else if normalized >= 5.0 { "f9ca24" }
            else { "e74c3c" }
        }
    }
}

/// Colored rating value: C411 uses [size=24], TorrXyz uses [size=5] with gray max
pub fn colored_rating_for(tracker: Tracker, value: f64, max: f64) -> String {
    let color_hex = rating_color_for(tracker, value, max);
    match tracker {
        Tracker::C411 => format!(
            "{} / {}",
            size(24, &bold(&color(color_hex, &format!("{:.1}", value)))),
            max as u32
        ),
        Tracker::TorrXyz => format!(
            "{}{}",
            bold(&color(color_hex, &size(5, &format!("{:.1}", value)))),
            color("aaaaaa", &format!(" / {}", max as u32))
        ),
    }
}

/// Ratings table header cell for TorrXyz (gray, bold, centered)
pub fn rating_header_torrxyz(source: &str) -> String {
    format!("[th]{}[/th]\n", center(&bold(&color("aaaaaa", source))))
}

/// Footer signature
pub fn footer_for(tracker: Tracker) -> String {
    let content = format!(
        "{} {} {}",
        color("e74c3c", "Upload"),
        color("3498db", "by"),
        color("e74c3c", "Grommey")
    );
    match tracker {
        Tracker::C411 => center(&small(&content)),
        Tracker::TorrXyz => center(&content),
    }
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
    fn test_field_for_torrxyz() {
        assert_eq!(
            field_for(Tracker::TorrXyz, "Genre", "Action"),
            "[b][color=#ff857a]Genre :[/color][/b] [color=#aaaaaa]Action[/color]"
        );
    }

    #[test]
    fn test_heading_title_torrxyz() {
        let result = heading_title_for(Tracker::TorrXyz, "TEST", "c0392b");
        assert_eq!(result, "[center][b][color=#c0392b][size=6]TEST[/size][/color][/b][/center]");
    }

    #[test]
    fn test_section_heading_torrxyz() {
        let result = section_heading_for(Tracker::TorrXyz, "Notes", "c0392b");
        assert_eq!(result, "[center][b][color=#c0392b][size=6]Notes[/size][/color][/b][/center]");
    }

    #[test]
    fn test_sub_heading_torrxyz() {
        let result = sub_heading_for(Tracker::TorrXyz, "Installation", "c0392b");
        assert_eq!(result, "[center][b][color=#c0392b][size=6]Installation[/size][/color][/b][/center]");
    }

    #[test]
    fn test_colored_rating_torrxyz() {
        let result = colored_rating_for(Tracker::TorrXyz, 80.0, 100.0);
        assert!(result.contains("[size=5]80.0[/size]"));
        assert!(result.contains("[color=#55efc4]"));
        assert!(result.contains("[color=#aaaaaa] / 100[/color]"));
    }

    #[test]
    fn test_footer_torrxyz_no_small() {
        let result = footer_for(Tracker::TorrXyz);
        assert!(!result.contains("[size=12]"));
        assert!(result.contains("[center]"));
        assert!(result.contains("Upload"));
    }
}
