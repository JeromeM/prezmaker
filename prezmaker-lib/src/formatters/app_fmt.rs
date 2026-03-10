use crate::formatters::bbcode::*;
use crate::models::Application;

pub fn format_application(app: &Application, title_color: &str, pseudo: &str) -> String {
    let mut out = String::new();

    // Header
    let title_upper = format!("\u{1F4BB} {} \u{1F4BB}", app.name.to_uppercase());
    out.push_str(&heading_title(&title_upper, title_color));
    out.push('\n');
    out.push('\n');

    // Section Informations
    out.push_str(&section_heading("Informations", title_color));
    out.push('\n');
    out.push('\n');

    let mut info = String::new();
    info.push_str(&field("Nom", &app.name));
    info.push('\n');
    if let Some(ref version) = app.version {
        info.push_str(&field("Version", version));
        info.push('\n');
    }
    if let Some(ref dev) = app.developer {
        info.push_str(&field("Developpeur", dev));
        info.push('\n');
    }
    if let Some(ref license) = app.license {
        info.push_str(&field("Licence", license));
        info.push('\n');
    }
    if let Some(ref website) = app.website {
        info.push_str(&field("Site web", &url(website, website)));
        info.push('\n');
    }
    if !app.platforms.is_empty() {
        info.push_str(&field("Plateformes", &app.platforms_display()));
        info.push('\n');
    }

    let mut table_content = String::new();
    let mut row_content = String::new();
    if let Some(ref logo) = app.logo_url {
        row_content.push_str(&td(&center(&img_width(logo, 200))));
    }
    row_content.push_str(&td(&info));
    table_content.push_str(&tr(&row_content));
    out.push_str(&quote(&table(&table_content)));

    out.push('\n');
    out.push('\n');

    // Description
    if let Some(ref desc) = app.description {
        if !desc.is_empty() {
            out.push_str(&section_heading("Description", title_color));
            out.push('\n');
            out.push('\n');
            out.push_str(&quote(desc));
            out.push('\n');
            out.push('\n');
        }
    }

    // Technical info
    out.push_str(&sub_heading("Informations techniques", title_color));
    out.push('\n');
    out.push('\n');

    let tech_headers = ["Plateforme", "Langue(s)", "Taille"];
    let mut tech_table = String::new();
    let mut header_row = String::new();
    for h in &tech_headers {
        header_row.push_str(&th(h));
    }
    tech_table.push_str(&tr(&header_row));
    let mut empty_row = String::new();
    for _ in &tech_headers {
        empty_row.push_str(&td(&center(" ")));
    }
    tech_table.push_str(&tr(&empty_row));
    out.push_str(&table(&tech_table));
    out.push('\n');

    let footer = footer(pseudo);
    if !footer.is_empty() {
        out.push_str(&footer);
        out.push('\n');
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_application() {
        let app = Application {
            name: "VLC".to_string(),
            version: Some("3.0.20".to_string()),
            developer: Some("VideoLAN".to_string()),
            description: Some("Lecteur multimedia libre".to_string()),
            website: Some("https://www.videolan.org".to_string()),
            license: Some("GPLv2".to_string()),
            platforms: vec!["Windows".to_string(), "macOS".to_string(), "Linux".to_string()],
            logo_url: None,
        };

        let bbcode = format_application(&app, "c0392b", "TestUser");
        assert!(bbcode.contains("VLC"));
        assert!(bbcode.contains("3.0.20"));
        assert!(bbcode.contains("VideoLAN"));
        assert!(bbcode.contains("GPLv2"));
        assert!(bbcode.contains("videolan.org"));
        assert!(bbcode.contains("Windows, macOS, Linux"));
        assert!(bbcode.contains("Lecteur multimedia libre"));
        assert!(bbcode.contains("Upload"));
        assert!(bbcode.contains("TestUser"));
    }
}
