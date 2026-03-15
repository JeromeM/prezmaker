use super::TemplateTag;

pub fn get_available_tags(content_type: &str) -> Vec<TemplateTag> {
    let layout = "Mise en page";
    let formatting = "Formatage";
    let images = "Images";
    let tables = "Tableaux";
    let shortcuts = "Raccourcis";
    let data_cat = "Donnees";
    let tech_cat = "Donnees techniques";
    let links_cat = "Liens";
    let notes_cat = "Notes";
    let cond_cat = "Conditionnel";

    let mut tags = vec![
        // --- Mise en page ---
        tag_ex("heading:texte", "Titre principal centré", layout, "{{heading:Mon Titre}}"),
        tag_ex("section:texte", "Titre de section centré", layout, "{{section:Synopsis}}"),
        tag_ex("section:texte:couleur", "Section avec couleur personnalisée", layout, "{{section:Synopsis:3498db}}"),
        tag_ex("sub_section:texte", "Sous-titre de section", layout, "{{sub_section:Details}}"),
        tag_ex("inline_heading:texte", "Titre inline (dans un bloc)", layout, "{{inline_heading:Casting}}"),
        tag_ex("field:label:valeur", "Champ label : valeur", layout, "{{field:Genre:Action}}"),
        tag("hr", "Ligne de séparation", layout),
        tag("br", "Saut de ligne", layout),
        tag_ex("footer", "Signature 'Upload by [pseudo]'", layout, "{{footer}}"),

        // --- Formatage ---
        tag_ex("bold:texte", "Texte en gras (inline)", formatting, "{{bold:important}}"),
        tag_ex("italic:texte", "Texte en italique (inline)", formatting, "{{italic:emphase}}"),
        tag_ex("underline:texte", "Texte souligné (inline)", formatting, "{{underline:souligné}}"),
        tag_ex("center:texte", "Centrer le texte (inline)", formatting, "{{center:texte centré}}"),
        tag_ex("quote:texte", "Citation (inline)", formatting, "{{quote:contenu cité}}"),
        tag_ex("color:hex:texte", "Texte coloré", formatting, "{{color:e74c3c:rouge}}"),
        tag_ex("size:N:texte", "Taille de texte", formatting, "{{size:18:Gros titre}}"),
        tag_ex("spoiler:label:contenu", "Spoiler avec contenu", formatting, "{{spoiler:Cliquer:texte caché}}"),
        // Block pairs
        tag_ex("center}}...{{/center", "Centrer un bloc de contenu", formatting, "{{center}}...{{/center}}"),
        tag_ex("quote}}...{{/quote", "Bloc citation", formatting, "{{quote}}...{{/quote}}"),
        tag_ex("bold}}...{{/bold", "Bloc gras", formatting, "{{bold}}...{{/bold}}"),
        tag_ex("italic}}...{{/italic", "Bloc italique", formatting, "{{italic}}...{{/italic}}"),

        // --- Images ---
        tag_ex("img:URL", "Image taille originale", images, "{{img:https://...}}"),
        tag_ex("img:URL:largeur", "Image avec largeur en pixels", images, "{{img:https://...:400}}"),
        tag_ex("img_poster:URL", "Image poster (300px)", images, "{{img_poster:{{poster_url}}}}"),
        tag_ex("img_cover:URL", "Image jaquette (264px)", images, "{{img_cover:{{cover_url}}}}"),
        tag_ex("img_logo:URL", "Image logo (200px)", images, "{{img_logo:{{logo_url}}}}"),

        // --- URL ---
        tag_ex("url:URL:label", "Lien hypertexte", links_cat, "{{url:https://...:Cliquez ici}}"),

        // --- Tableaux ---
        tag_ex("table}}...{{/table", "Bloc tableau", tables, "{{table}}...{{/table}}"),
        tag_ex("tr}}...{{/tr", "Ligne de tableau", tables, "{{tr}}...{{/tr}}"),
        tag_ex("td:contenu", "Cellule de tableau", tables, "{{td:contenu}}"),
        tag_ex("th:contenu", "En-tête de tableau", tables, "{{th:Titre}}"),

        // --- Conditionnel ---
        tag_ex("#if tag", "Affiche le bloc si la balise a une valeur", cond_cat, "{{#if synopsis}}...{{/if}}"),
        tag_ex("#if tag > valeur", "Condition avec comparaison (>, >=, <, <=, ==, !=)", cond_cat, "{{#if ratings_count > 0}}...{{/if}}"),
        tag("/if", "Fin du bloc conditionnel", cond_cat),
    ];

    // --- Données spécifiques au type ---
    match content_type {
        "film" => {
            tags.extend(vec![
                tag("titre", "Titre du film", data_cat),
                tag("titre_maj", "Titre en MAJUSCULES", data_cat),
                tag("titre_original", "Titre original", data_cat),
                tag("annee", "Année de sortie", data_cat),
                tag("date_sortie", "Date de sortie formatée", data_cat),
                tag("duree", "Durée formatée", data_cat),
                tag("realisateurs", "Réalisateur(s)", data_cat),
                tag("genres", "Genres", data_cat),
                tag("pays", "Pays d'origine", data_cat),
                tag("casting", "Acteurs principaux", data_cat),
                tag("synopsis", "Synopsis", data_cat),
                tag("poster_url", "URL de l'affiche", data_cat),
                tag("info_bbcode", "Contenu info auto-généré (origine, durée, casting...)", data_cat),
                // Tech
                tag("tech_qualite", "Qualité (ex: 1080p)", tech_cat),
                tag("tech_codec", "Codec vidéo", tech_cat),
                tag("tech_audio", "Audio", tech_cat),
                tag("tech_langue", "Langue(s)", tech_cat),
                tag("tech_soustitres", "Sous-titres", tech_cat),
                tag("tech_taille", "Taille du fichier", tech_cat),
                // MediaInfo
                tag("mi_video_codec", "Codec vidéo (MediaInfo)", tech_cat),
                tag("mi_resolution", "Résolution vidéo (ex: 1920x1080)", tech_cat),
                tag("mi_video_fps", "Images par seconde", tech_cat),
                tag("mi_duration", "Durée (MediaInfo)", tech_cat),
                tag("mi_bitrate", "Débit global", tech_cat),
                tag("mi_file_size", "Taille du fichier (MediaInfo)", tech_cat),
                tag("mi_audio_langs", "Langues audio", tech_cat),
                tag("mi_subtitle_langs", "Langues sous-titres", tech_cat),
                tag("has_mediainfo", "Flag: fichier média analysé", tech_cat),
                // Liens
                tag_ex("link", "Lien principal (TMDB)", links_cat, "{{#if link}}{{field:Lien:{{link}}}}{{/if}}"),
                tag("tmdb_link", "Lien vers la page TMDB", links_cat),
                tag("imdb_link", "Lien vers la page IMDb", links_cat),
                tag("allocine_link", "Lien vers la page Allocine", links_cat),
                // Raccourcis
                tag("poster_info", "Bloc poster + infos", shortcuts),
                tag("ratings_table", "Tableau des notes formaté", shortcuts),
                tag("tech_table", "Tableau infos techniques", shortcuts),
                tag("mediainfo_table", "Tableau MediaInfo complet (vidéo, audio, sous-titres)", shortcuts),
            ]);
        }
        "serie" => {
            tags.extend(vec![
                tag("titre", "Titre de la série", data_cat),
                tag("titre_maj", "Titre en MAJUSCULES", data_cat),
                tag("titre_original", "Titre original", data_cat),
                tag("annee", "Année de début", data_cat),
                tag("premiere_diffusion", "Date première diffusion", data_cat),
                tag("statut", "Statut (En cours, Terminée...)", data_cat),
                tag("saisons", "Nombre de saisons", data_cat),
                tag("episodes", "Nombre d'épisodes", data_cat),
                tag("duree_episode", "Durée par épisode", data_cat),
                tag("createurs", "Créateur(s)", data_cat),
                tag("chaines", "Chaîne / Plateforme", data_cat),
                tag("genres", "Genres", data_cat),
                tag("pays", "Pays d'origine", data_cat),
                tag("casting", "Acteurs principaux", data_cat),
                tag("synopsis", "Synopsis", data_cat),
                tag("poster_url", "URL de l'affiche", data_cat),
                tag("info_bbcode", "Contenu info auto-généré (origine, statut, casting...)", data_cat),
                // Tech
                tag("tech_qualite", "Qualité", tech_cat),
                tag("tech_codec", "Codec vidéo", tech_cat),
                tag("tech_audio", "Audio", tech_cat),
                tag("tech_langue", "Langue(s)", tech_cat),
                tag("tech_soustitres", "Sous-titres", tech_cat),
                tag("tech_taille", "Taille du fichier", tech_cat),
                // MediaInfo
                tag("mi_video_codec", "Codec vidéo (MediaInfo)", tech_cat),
                tag("mi_resolution", "Résolution vidéo (ex: 1920x1080)", tech_cat),
                tag("mi_video_fps", "Images par seconde", tech_cat),
                tag("mi_duration", "Durée (MediaInfo)", tech_cat),
                tag("mi_bitrate", "Débit global", tech_cat),
                tag("mi_file_size", "Taille du fichier (MediaInfo)", tech_cat),
                tag("mi_audio_langs", "Langues audio", tech_cat),
                tag("mi_subtitle_langs", "Langues sous-titres", tech_cat),
                tag("has_mediainfo", "Flag: fichier média analysé", tech_cat),
                // Liens
                tag_ex("link", "Lien principal (TMDB)", links_cat, "{{#if link}}{{field:Lien:{{link}}}}{{/if}}"),
                tag("tmdb_link", "Lien vers la page TMDB", links_cat),
                tag("imdb_link", "Lien vers la page IMDb", links_cat),
                tag("allocine_link", "Lien vers la page Allocine", links_cat),
                // Raccourcis
                tag("poster_info", "Bloc poster + infos", shortcuts),
                tag("ratings_table", "Tableau des notes formaté", shortcuts),
                tag("tech_table", "Tableau infos techniques", shortcuts),
                tag("mediainfo_table", "Tableau MediaInfo complet (vidéo, audio, sous-titres)", shortcuts),
            ]);
        }
        "jeu" => {
            tags.extend(vec![
                tag("titre", "Titre du jeu", data_cat),
                tag("titre_maj", "Titre en MAJUSCULES", data_cat),
                tag("annee", "Année de sortie", data_cat),
                tag("date_sortie", "Date de sortie", data_cat),
                tag("synopsis", "Description du jeu", data_cat),
                tag("cover_url", "URL de la jaquette", data_cat),
                tag("genres", "Genres", data_cat),
                tag("plateformes", "Plateformes", data_cat),
                tag("developpeurs", "Développeur(s)", data_cat),
                tag("editeurs", "Éditeur(s)", data_cat),
                tag("installation", "Instructions d'installation", data_cat),
                tag("info_bbcode", "Contenu info auto-généré (date, dev, genres...)", data_cat),
                // Tech
                tag("tech_plateforme", "Plateforme technique", tech_cat),
                tag("tech_langues", "Langue(s)", tech_cat),
                tag("tech_taille", "Taille", tech_cat),
                tag("tech_taille_installee", "Taille d'installation", tech_cat),
                // Config requise
                tag("config_mini", "Configuration minimale formatee", tech_cat),
                tag("config_reco", "Configuration recommandee formatee", tech_cat),
                // Liens
                tag_ex("link", "Lien principal (IGDB ou Steam)", links_cat, "{{#if link}}{{field:Lien:{{link}}}}{{/if}}"),
                tag("igdb_link", "Lien vers la page IGDB", links_cat),
                tag("steam_link", "Lien vers la page Steam", links_cat),
                // Raccourcis
                tag("cover_info", "Bloc jaquette + infos", shortcuts),
                tag("ratings_table", "Tableau des notes formaté", shortcuts),
                tag("game_tech_table", "Tableau infos techniques", shortcuts),
                tag("game_reqs_table", "Tableau configuration requise min/rec", shortcuts),
                tag("screenshots_grid", "Grille de screenshots", shortcuts),
            ]);
        }
        "app" => {
            tags.extend(vec![
                tag("nom", "Nom de l'application", data_cat),
                tag("nom_maj", "Nom en MAJUSCULES", data_cat),
                tag("version", "Version", data_cat),
                tag("developpeur", "Développeur", data_cat),
                tag("description", "Description", data_cat),
                tag("site_web", "URL du site web", data_cat),
                tag("licence", "Licence", data_cat),
                tag("plateformes", "Plateformes", data_cat),
                tag("logo_url", "URL du logo", data_cat),
                tag("info_bbcode", "Contenu info auto-généré (nom, version, licence...)", data_cat),
                // Liens
                tag_ex("link", "Lien principal (site web)", links_cat, "{{#if link}}{{field:Lien:{{link}}}}{{/if}}"),
                // Raccourcis
                tag("logo_info", "Bloc logo + infos", shortcuts),
            ]);
        }
        _ => {}
    }

    // --- Notes individuelles (si ratings possibles) ---
    if matches!(content_type, "film" | "serie" | "jeu") {
        tags.extend(vec![
            tag("ratings_count", "Nombre de notes disponibles", notes_cat),
            tag_ex("rating_1_source", "Source de la note 1", notes_cat, "TMDB, Allocine, IGDB..."),
            tag("rating_1_value", "Valeur de la note 1", notes_cat),
            tag("rating_1_max", "Maximum de la note 1", notes_cat),
            tag("rating_1_display", "Note 1 formatée avec couleur", notes_cat),
            tag_ex("rating_2_source", "Source de la note 2", notes_cat, "TMDB, Allocine, IGDB..."),
            tag("rating_2_value", "Valeur de la note 2", notes_cat),
            tag("rating_2_max", "Maximum de la note 2", notes_cat),
            tag("rating_2_display", "Note 2 formatée avec couleur", notes_cat),
        ]);
    }

    tags
}

fn tag(name: &str, desc: &str, category: &str) -> TemplateTag {
    TemplateTag {
        name: name.to_string(),
        description: desc.to_string(),
        category: category.to_string(),
        example: None,
    }
}

fn tag_ex(name: &str, desc: &str, category: &str, example: &str) -> TemplateTag {
    TemplateTag {
        name: name.to_string(),
        description: desc.to_string(),
        category: category.to_string(),
        example: Some(example.to_string()),
    }
}
