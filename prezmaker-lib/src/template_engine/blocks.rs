use crate::formatters::bbcode;

pub fn render_ratings_block(
    ratings: &[crate::models::Rating],
    title_color: &str,
) -> String {
    if ratings.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    out.push_str(&bbcode::section_heading("Notes", title_color));
    out.push('\n');

    let mut ratings_table = String::new();
    let mut header_row = String::new();
    for rating in ratings {
        header_row.push_str(&bbcode::th(&rating.source));
    }
    ratings_table.push_str(&bbcode::tr(&header_row));
    let mut values_row = String::new();
    for rating in ratings {
        values_row.push_str(&bbcode::td(&bbcode::center(
            &bbcode::colored_rating(rating.value, rating.max),
        )));
    }
    ratings_table.push_str(&bbcode::tr(&values_row));
    out.push_str(&bbcode::table(&ratings_table));

    out
}

pub fn render_movie_tech_block(
    tech: Option<&crate::models::MediaTechInfo>,
    title_color: &str,
) -> String {
    let mut out = String::new();
    out.push_str(&bbcode::sub_heading(
        "Informations techniques",
        title_color,
    ));
    out.push('\n');

    let quality_val = tech.and_then(|t| t.quality.as_deref()).unwrap_or(" ");
    let codec_val = tech.and_then(|t| t.video_codec.as_deref()).unwrap_or(" ");
    let lang_val = tech.and_then(|t| t.language.as_deref()).unwrap_or(" ");
    let sub_val = tech.and_then(|t| t.subtitles.as_deref()).unwrap_or(" ");
    let audio_val = tech.and_then(|t| t.audio.as_deref());
    let size_val = tech.and_then(|t| t.size.as_deref());

    let mut headers: Vec<&str> = vec!["Qualite", "Codec Video", "Langue(s)", "Sous-titres"];
    let mut values: Vec<&str> = vec![quality_val, codec_val, lang_val, sub_val];

    if let Some(a) = audio_val {
        headers.push("Audio");
        values.push(a);
    }
    if let Some(s) = size_val {
        headers.push("Taille");
        values.push(s);
    }

    let mut tech_table = String::new();
    let mut header_row = String::new();
    for h in &headers {
        header_row.push_str(&bbcode::th(h));
    }
    tech_table.push_str(&bbcode::tr(&header_row));
    let mut val_row = String::new();
    for v in &values {
        val_row.push_str(&bbcode::td(&bbcode::center(v)));
    }
    tech_table.push_str(&bbcode::tr(&val_row));
    out.push_str(&bbcode::table(&tech_table));

    out
}

pub fn render_game_tech_block(
    tech: Option<&crate::models::TechInfo>,
    title_color: &str,
) -> String {
    let mut out = String::new();
    out.push_str(&bbcode::sub_heading(
        "Informations techniques",
        title_color,
    ));
    out.push('\n');

    let mut tech_headers: Vec<&str> = vec!["Plateforme", "Langue(s)", "Taille"];
    let has_install_size = tech.map_or(false, |t| !t.install_size.is_empty());
    if has_install_size {
        tech_headers.push("Taille installee");
    }

    let mut tech_table = String::new();
    let mut header_row = String::new();
    for h in &tech_headers {
        header_row.push_str(&bbcode::th(h));
    }
    tech_table.push_str(&bbcode::tr(&header_row));
    let mut values_row = String::new();
    if let Some(t) = tech {
        let mut values: Vec<&str> = vec![&t.platform, &t.languages, &t.size];
        if has_install_size {
            values.push(&t.install_size);
        }
        for val in &values {
            let display = if val.is_empty() { " " } else { val };
            values_row.push_str(&bbcode::td(&bbcode::center(display)));
        }
    } else {
        for _ in &tech_headers {
            values_row.push_str(&bbcode::td(&bbcode::center(" ")));
        }
    }
    tech_table.push_str(&bbcode::tr(&values_row));
    out.push_str(&bbcode::table(&tech_table));

    out
}

pub(crate) fn format_system_reqs(reqs: &crate::models::SystemReqs) -> String {
    let fields: [(&str, &str); 5] = [
        ("OS", &reqs.os),
        ("Processeur", &reqs.cpu),
        ("RAM", &reqs.ram),
        ("Carte graphique", &reqs.gpu),
        ("Stockage", &reqs.storage),
    ];
    fields
        .iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(label, value)| bbcode::field(label, value))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn render_game_reqs_block(
    min_reqs: Option<&crate::models::SystemReqs>,
    rec_reqs: Option<&crate::models::SystemReqs>,
    title_color: &str,
) -> String {
    let has_min = min_reqs.map_or(false, |r| !r.is_empty());
    let has_rec = rec_reqs.map_or(false, |r| !r.is_empty());
    if !has_min && !has_rec {
        return String::new();
    }

    let mut out = String::new();
    out.push_str(&bbcode::sub_heading("Configuration requise", title_color));
    out.push('\n');

    let mut tech_table = String::new();

    // Header row
    let mut header_row = String::new();
    if has_min {
        header_row.push_str(&bbcode::th("Configuration minimale"));
    }
    if has_rec {
        header_row.push_str(&bbcode::th("Configuration recommandee"));
    }
    tech_table.push_str(&bbcode::tr(&header_row));

    // Content row
    let mut content_row = String::new();
    if has_min {
        content_row.push_str(&bbcode::td(&format_system_reqs(min_reqs.unwrap())));
    }
    if has_rec {
        content_row.push_str(&bbcode::td(&format_system_reqs(rec_reqs.unwrap())));
    }
    tech_table.push_str(&bbcode::tr(&content_row));

    out.push_str(&bbcode::table(&tech_table));
    out
}

pub fn render_screenshots_block(
    screenshots: &[String],
    title_color: &str,
) -> String {
    if screenshots.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    out.push_str(&bbcode::section_heading(
        "Screenshots",
        title_color,
    ));
    out.push('\n');

    let taken: Vec<_> = screenshots.iter().take(4).collect();
    let mut inner = String::new();
    for pair in taken.chunks(2) {
        let line = pair
            .iter()
            .map(|ss| bbcode::img_width(ss, 400))
            .collect::<Vec<_>>()
            .join(" ");
        if !inner.is_empty() {
            inner.push('\n');
        }
        inner.push_str(&line);
    }
    out.push_str(&bbcode::center(&inner));

    out
}

pub fn render_poster_info_block(
    poster_url: Option<&str>,
    info_bbcode: &str,
) -> String {
    let mut out = String::new();
    let mut table_content = String::new();
    let mut row_content = String::new();
    if let Some(poster) = poster_url {
        row_content.push_str(&bbcode::td(&bbcode::center(&bbcode::img_width(poster, 300))));
    }
    row_content.push_str(&bbcode::td(info_bbcode));
    table_content.push_str(&bbcode::tr(&row_content));
    out.push_str(&bbcode::quote(&bbcode::table(&table_content)));
    out
}

pub fn render_cover_info_block(
    cover_url: Option<&str>,
    info_bbcode: &str,
) -> String {
    let mut out = String::new();
    let mut table_content = String::new();
    let mut row_content = String::new();
    if let Some(cover) = cover_url {
        row_content
            .push_str(&bbcode::td(&bbcode::center(&bbcode::img_width(cover, 264))));
    }
    row_content.push_str(&bbcode::td(info_bbcode));
    table_content.push_str(&bbcode::tr(&row_content));
    out.push_str(&bbcode::quote(&bbcode::table(&table_content)));
    out
}

/// Returns a country code for flagcdn.com from a French language name
fn language_to_country_code(lang: &str) -> Option<&'static str> {
    match lang {
        "Français" => Some("fr"),
        "Anglais" => Some("gb"),
        "Espagnol" => Some("es"),
        "Allemand" => Some("de"),
        "Italien" => Some("it"),
        "Portugais" => Some("pt"),
        "Japonais" => Some("jp"),
        "Coréen" => Some("kr"),
        "Chinois" => Some("cn"),
        "Russe" => Some("ru"),
        "Arabe" => Some("sa"),
        "Néerlandais" => Some("nl"),
        "Polonais" => Some("pl"),
        "Suédois" => Some("se"),
        "Norvégien" => Some("no"),
        "Danois" => Some("dk"),
        "Finnois" => Some("fi"),
        "Turc" => Some("tr"),
        "Hindi" => Some("in"),
        "Thaï" => Some("th"),
        "Vietnamien" => Some("vn"),
        "Hébreu" => Some("il"),
        "Grec" => Some("gr"),
        "Tchèque" => Some("cz"),
        "Hongrois" => Some("hu"),
        "Roumain" => Some("ro"),
        "Croate" => Some("hr"),
        "Serbe" => Some("rs"),
        "Bulgare" => Some("bg"),
        "Ukrainien" => Some("ua"),
        "Catalan" => Some("es"),
        "Indonésien" => Some("id"),
        "Malais" => Some("my"),
        _ => None,
    }
}

/// BBCode cell content with flag + language name
fn language_with_flag(lang: &str) -> String {
    if let Some(code) = language_to_country_code(lang) {
        format!("[img]https://flagcdn.com/w20/{}.png[/img] {}", code, lang)
    } else {
        lang.to_string()
    }
}

pub fn render_mediainfo_block(
    ma: &crate::models::MediaAnalysis,
    title_color: &str,
) -> String {
    let mut out = String::new();
    out.push_str(&bbcode::section_heading("MediaInfo", title_color));
    out.push('\n');

    // Video info table
    if let Some(v) = ma.video.first() {
        let mut headers = vec!["Codec Video", "Resolution"];
        let mut values: Vec<String> = vec![
            v.codec.clone(),
            format!("{}x{}", v.width, v.height),
        ];
        if let Some(ref fps) = v.fps {
            headers.push("FPS");
            values.push(fps.clone());
        }
        if let Some(ref d) = ma.duration {
            headers.push("Duree");
            values.push(d.clone());
        }
        if let Some(ref b) = ma.bitrate {
            headers.push("Debit");
            values.push(b.clone());
        }
        headers.push("Taille");
        values.push(ma.file_size.clone());

        let mut table = String::new();
        let mut hrow = String::new();
        for h in &headers {
            hrow.push_str(&bbcode::th(h));
        }
        table.push_str(&bbcode::tr(&hrow));
        let mut vrow = String::new();
        for val in &values {
            vrow.push_str(&bbcode::td(&bbcode::center(val)));
        }
        table.push_str(&bbcode::tr(&vrow));
        out.push_str(&bbcode::table(&table));
    }

    // Audio/Langues table: # | Langue | Canaux | Codec | Bitrate
    if !ma.audio.is_empty() {
        out.push('\n');
        out.push_str(&bbcode::sub_heading("Langues", title_color));
        out.push('\n');
        let mut table = String::new();
        let mut hrow = String::new();
        hrow.push_str(&bbcode::th("#"));
        hrow.push_str(&bbcode::th("Langue"));
        hrow.push_str(&bbcode::th("Canaux"));
        hrow.push_str(&bbcode::th("Codec"));
        hrow.push_str(&bbcode::th("Bitrate"));
        table.push_str(&bbcode::tr(&hrow));
        for (i, a) in ma.audio.iter().enumerate() {
            let mut row = String::new();
            row.push_str(&bbcode::td(&bbcode::center(&format!("{}", i + 1))));
            let lang_cell = match a.language.as_deref() {
                Some(l) => language_with_flag(l),
                None => "-".to_string(),
            };
            row.push_str(&bbcode::td(&lang_cell));
            row.push_str(&bbcode::td(&bbcode::center(&a.channels)));
            row.push_str(&bbcode::td(&bbcode::center(&a.codec)));
            row.push_str(&bbcode::td(&bbcode::center(a.bitrate.as_deref().unwrap_or("-"))));
            table.push_str(&bbcode::tr(&row));
        }
        out.push_str(&bbcode::table(&table));
    }

    // Subtitle table: # | Langue | Format | Type
    if !ma.subtitles.is_empty() {
        out.push('\n');
        out.push_str(&bbcode::sub_heading("Sous-Titres", title_color));
        out.push('\n');
        let mut table = String::new();
        let mut hrow = String::new();
        hrow.push_str(&bbcode::th("#"));
        hrow.push_str(&bbcode::th("Langue"));
        hrow.push_str(&bbcode::th("Format"));
        hrow.push_str(&bbcode::th("Type"));
        table.push_str(&bbcode::tr(&hrow));
        for (i, s) in ma.subtitles.iter().enumerate() {
            let sub_type = if s.is_forced {
                "FORCED"
            } else if let Some(ref t) = s.title {
                let tl = t.to_uppercase();
                if tl.contains("SDH") { "SDH" }
                else if tl.contains("FORCED") || tl.contains("FORCE") { "FORCED" }
                else { "FULL" }
            } else {
                "FULL"
            };
            let mut row = String::new();
            row.push_str(&bbcode::td(&bbcode::center(&format!("{}", i + 1))));
            let lang_cell = match s.language.as_deref() {
                Some(l) => language_with_flag(l),
                None => "-".to_string(),
            };
            row.push_str(&bbcode::td(&lang_cell));
            row.push_str(&bbcode::td(&bbcode::center(&s.format)));
            row.push_str(&bbcode::td(&bbcode::center(sub_type)));
            table.push_str(&bbcode::tr(&row));
        }
        out.push_str(&bbcode::table(&table));
    }

    out
}
