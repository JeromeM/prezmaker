pub fn get_default(content_type: &str) -> String {
    match content_type {
        "film" => DEFAULT_FILM.to_string(),
        "serie" => DEFAULT_SERIE.to_string(),
        "jeu" => DEFAULT_JEU.to_string(),
        "app" => DEFAULT_APP.to_string(),
        _ => String::new(),
    }
}

const DEFAULT_FILM: &str = r#"{{heading:🎬 {{titre_maj}} 🎬}}
{{hr}}

{{poster_info}}

{{hr}}

{{ratings_table}}

{{#if synopsis}}{{section:Synopsis}}

{{quote:{{synopsis}}}}

{{hr}}

{{/if}}{{tech_table}}

{{footer}}"#;

const DEFAULT_SERIE: &str = r#"{{heading:📺 {{titre_maj}} 📺}}
{{hr}}

{{poster_info}}

{{hr}}

{{ratings_table}}

{{#if synopsis}}{{section:Synopsis}}

{{quote:{{synopsis}}}}

{{hr}}

{{/if}}{{tech_table}}

{{footer}}"#;

const DEFAULT_JEU: &str = r#"{{heading:🎮 {{titre_maj}} 🎮}}
{{hr}}

{{cover_info}}

{{hr}}

{{#if synopsis}}{{section:Description}}

{{quote:{{synopsis}}}}

{{hr}}

{{/if}}{{screenshots_grid}}

{{ratings_table}}

{{game_tech_table}}

{{#if installation}}{{sub_section:Installation}}

{{quote:{{installation}}}}

{{hr}}

{{/if}}{{footer}}"#;

const DEFAULT_APP: &str = r#"{{heading:💻 {{nom_maj}} 💻}}
{{hr}}

{{logo_info}}

{{hr}}

{{#if description}}{{section:Description}}

{{quote:{{description}}}}

{{hr}}

{{/if}}{{app_tech_table}}

{{footer}}"#;
