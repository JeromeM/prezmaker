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

{{section:Informations}}

{{#if pays}}{{field:Origine:{{pays}}}}
{{/if}}{{#if date_sortie}}{{field:Sortie:{{date_sortie}}}}
{{/if}}{{#if duree}}{{field:Duree:{{duree}}}}
{{/if}}{{#if realisateurs}}{{field:Realisateur:{{realisateurs}}}}
{{/if}}{{#if genres}}{{field:Genres:{{genres}}}}
{{/if}}
{{#if casting}}
{{inline_heading:Casting}}

{{field:Acteurs:{{casting}}}}
{{/if}}

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

{{section:Informations}}

{{#if pays}}{{field:Origine:{{pays}}}}
{{/if}}{{#if premiere_diffusion}}{{field:Premiere diffusion:{{premiere_diffusion}}}}
{{/if}}{{#if statut}}{{field:Statut:{{statut}}}}
{{/if}}{{#if saisons}}{{field:Saisons:{{saisons}}}}
{{/if}}{{#if episodes}}{{field:Episodes:{{episodes}}}}
{{/if}}{{#if duree_episode}}{{field:Duree par episode:{{duree_episode}}}}
{{/if}}{{#if createurs}}{{field:Createur(s):{{createurs}}}}
{{/if}}{{#if chaines}}{{field:Chaine / Plateforme:{{chaines}}}}
{{/if}}{{#if genres}}{{field:Genres:{{genres}}}}
{{/if}}
{{#if casting}}
{{inline_heading:Casting}}

{{field:Acteurs:{{casting}}}}
{{/if}}

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

{{section:Informations}}

{{#if date_sortie}}{{field:Date de sortie:{{date_sortie}}}}
{{/if}}{{#if developpeurs}}{{field:Developpeur(s):{{developpeurs}}}}
{{/if}}{{#if editeurs}}{{field:Editeur(s):{{editeurs}}}}
{{/if}}{{#if genres}}{{field:Genres:{{genres}}}}
{{/if}}{{#if plateformes}}{{field:Plateformes:{{plateformes}}}}
{{/if}}

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

{{section:Informations}}

{{field:Nom:{{nom}}}}
{{#if version}}{{field:Version:{{version}}}}
{{/if}}{{#if developpeur}}{{field:Developpeur:{{developpeur}}}}
{{/if}}{{#if licence}}{{field:Licence:{{licence}}}}
{{/if}}{{#if site_web}}{{field:Site web:{{site_web}}}}
{{/if}}{{#if plateformes}}{{field:Plateformes:{{plateformes}}}}
{{/if}}

{{logo_info}}

{{hr}}

{{#if description}}{{section:Description}}

{{quote:{{description}}}}

{{hr}}

{{/if}}{{app_tech_table}}

{{footer}}"#;
