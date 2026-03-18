use crate::formatters::dispatch;
use crate::formatters::OutputFormat;

pub fn render_ratings_block(
    ratings: &[crate::models::Rating],
    _title_color: &str,
    fmt: OutputFormat,
) -> String {
    if ratings.is_empty() {
        return String::new();
    }

    let mut out = String::new();

    let mut ratings_table = String::new();
    let mut header_row = String::new();
    for rating in ratings {
        header_row.push_str(&dispatch::th(fmt, &dispatch::center(fmt, &rating.source, None), None));
    }
    ratings_table.push_str(&dispatch::tr(fmt, &header_row, None));
    let mut values_row = String::new();
    for rating in ratings {
        values_row.push_str(&dispatch::td(fmt, &dispatch::center(fmt,
            &dispatch::colored_rating(fmt, rating.value, rating.max), None,
        ), None));
    }
    ratings_table.push_str(&dispatch::tr(fmt, &values_row, None));
    out.push_str(&dispatch::table(fmt, &ratings_table, None));

    out
}

pub fn render_movie_tech_block(
    tech: Option<&crate::models::MediaTechInfo>,
    _title_color: &str,
    fmt: OutputFormat,
) -> String {
    let mut out = String::new();

    let quality_val = tech.and_then(|t| t.quality.as_deref()).unwrap_or(" ");
    let codec_val = tech.and_then(|t| t.video_codec.as_deref()).unwrap_or(" ");
    let lang_val = tech.and_then(|t| t.language.as_deref()).unwrap_or(" ");
    let sub_val = tech.and_then(|t| t.subtitles.as_deref()).unwrap_or(" ");
    let audio_val = tech.and_then(|t| t.audio.as_deref());
    let size_val = tech.and_then(|t| t.size.as_deref());

    let mut headers: Vec<&str> = vec!["Qualité", "Codec Vidéo", "Langue(s)", "Sous-titres"];
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
        header_row.push_str(&dispatch::th(fmt, &dispatch::center(fmt, h, None), None));
    }
    tech_table.push_str(&dispatch::tr(fmt, &header_row, None));
    let mut val_row = String::new();
    for v in &values {
        val_row.push_str(&dispatch::td(fmt, &dispatch::center(fmt, v, None), None));
    }
    tech_table.push_str(&dispatch::tr(fmt, &val_row, None));
    out.push_str(&dispatch::table(fmt, &tech_table, None));

    out
}

pub fn render_game_tech_block(
    tech: Option<&crate::models::TechInfo>,
    _title_color: &str,
    fmt: OutputFormat,
) -> String {
    let mut out = String::new();

    let mut tech_headers: Vec<&str> = vec!["Plateforme", "Langue(s)", "Taille"];
    let has_install_size = tech.map_or(false, |t| !t.install_size.is_empty());
    if has_install_size {
        tech_headers.push("Taille installée");
    }

    let mut tech_table = String::new();
    let mut header_row = String::new();
    for h in &tech_headers {
        header_row.push_str(&dispatch::th(fmt, &dispatch::center(fmt, h, None), None));
    }
    tech_table.push_str(&dispatch::tr(fmt, &header_row, None));
    let mut values_row = String::new();
    if let Some(t) = tech {
        let mut values: Vec<&str> = vec![&t.platform, &t.languages, &t.size];
        if has_install_size {
            values.push(&t.install_size);
        }
        for val in &values {
            let display = if val.is_empty() { " " } else { val };
            values_row.push_str(&dispatch::td(fmt, &dispatch::center(fmt, display, None), None));
        }
    } else {
        for _ in &tech_headers {
            values_row.push_str(&dispatch::td(fmt, &dispatch::center(fmt, " ", None), None));
        }
    }
    tech_table.push_str(&dispatch::tr(fmt, &values_row, None));
    out.push_str(&dispatch::table(fmt, &tech_table, None));

    out
}

pub(crate) fn format_system_reqs(reqs: &crate::models::SystemReqs, fmt: OutputFormat) -> String {
    let fields: [(&str, &str); 5] = [
        ("OS", &reqs.os),
        ("Processeur", &reqs.cpu),
        ("RAM", &reqs.ram),
        ("Carte graphique", &reqs.gpu),
        ("Stockage", &reqs.storage),
    ];
    let sep = match fmt {
        OutputFormat::Html => "<br>\n",
        OutputFormat::Bbcode => "\n",
    };
    fields
        .iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(label, value)| dispatch::field(fmt, label, value))
        .collect::<Vec<_>>()
        .join(sep)
}

pub fn render_game_reqs_block(
    min_reqs: Option<&crate::models::SystemReqs>,
    rec_reqs: Option<&crate::models::SystemReqs>,
    _title_color: &str,
    fmt: OutputFormat,
) -> String {
    let has_min = min_reqs.map_or(false, |r| !r.is_empty());
    let has_rec = rec_reqs.map_or(false, |r| !r.is_empty());
    if !has_min && !has_rec {
        return String::new();
    }

    let mut out = String::new();

    let mut tech_table = String::new();

    let mut header_row = String::new();
    if has_min {
        header_row.push_str(&dispatch::th(fmt, "Configuration minimale", None));
    }
    if has_rec {
        header_row.push_str(&dispatch::th(fmt, "Configuration recommandée", None));
    }
    tech_table.push_str(&dispatch::tr(fmt, &header_row, None));

    let mut content_row = String::new();
    if has_min {
        content_row.push_str(&dispatch::td(fmt, &format_system_reqs(min_reqs.unwrap(), fmt), None));
    }
    if has_rec {
        content_row.push_str(&dispatch::td(fmt, &format_system_reqs(rec_reqs.unwrap(), fmt), None));
    }
    tech_table.push_str(&dispatch::tr(fmt, &content_row, None));

    out.push_str(&dispatch::table(fmt, &tech_table, None));
    out
}

pub fn render_screenshots_block(
    screenshots: &[String],
    _title_color: &str,
    fmt: OutputFormat,
) -> String {
    if screenshots.is_empty() {
        return String::new();
    }

    let mut out = String::new();

    let taken: Vec<_> = screenshots.iter().take(4).collect();
    let mut inner = String::new();
    for pair in taken.chunks(2) {
        let line = pair
            .iter()
            .map(|ss| dispatch::img_width(fmt, ss, 400, None))
            .collect::<Vec<_>>()
            .join(" ");
        if !inner.is_empty() {
            inner.push('\n');
        }
        inner.push_str(&line);
    }
    out.push_str(&dispatch::center(fmt, &inner, None));

    out
}

pub fn render_poster_info_block(
    poster_url: Option<&str>,
    info_markup: &str,
    fmt: OutputFormat,
) -> String {
    let mut out = String::new();
    let mut table_content = String::new();
    let mut row_content = String::new();
    if let Some(poster) = poster_url {
        row_content.push_str(&dispatch::td(fmt, &dispatch::center(fmt, &dispatch::img_width(fmt, poster, 300, None), None), None));
    }
    row_content.push_str(&dispatch::td(fmt, info_markup, None));
    table_content.push_str(&dispatch::tr(fmt, &row_content, None));
    out.push_str(&dispatch::quote(fmt, &dispatch::table(fmt, &table_content, None), None));
    out
}

pub fn render_cover_info_block(
    cover_url: Option<&str>,
    info_markup: &str,
    fmt: OutputFormat,
) -> String {
    let mut out = String::new();
    let mut table_content = String::new();
    let mut row_content = String::new();
    if let Some(cover) = cover_url {
        row_content.push_str(&dispatch::td(fmt, &dispatch::center(fmt, &dispatch::img_width(fmt, cover, 264, None), None), None));
    }
    row_content.push_str(&dispatch::td(fmt, info_markup, None));
    table_content.push_str(&dispatch::tr(fmt, &row_content, None));
    out.push_str(&dispatch::quote(fmt, &dispatch::table(fmt, &table_content, None), None));
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

fn language_with_flag(lang: &str, fmt: OutputFormat) -> String {
    if let Some(code) = language_to_country_code(lang) {
        let flag = dispatch::img(fmt, &format!("https://flagcdn.com/w20/{}.png", code), None);
        format!("{} {}", flag, lang)
    } else {
        lang.to_string()
    }
}

pub fn render_mediainfo_block(
    ma: &crate::models::MediaAnalysis,
    title_color: &str,
    fmt: OutputFormat,
) -> String {
    let mut out = String::new();
    out.push_str(&dispatch::section_heading(fmt, "MediaInfo", title_color, None));
    out.push('\n');

    // Video info table
    if let Some(v) = ma.video.first() {
        let mut headers = vec!["Codec Vidéo", "Resolution"];
        let mut values: Vec<String> = vec![
            v.codec.clone(),
            format!("{}x{}", v.width, v.height),
        ];
        if let Some(ref fps) = v.fps {
            headers.push("FPS");
            values.push(fps.clone());
        }
        if let Some(ref d) = ma.duration {
            headers.push("Durée");
            values.push(d.clone());
        }
        if let Some(ref b) = ma.bitrate {
            headers.push("Débit");
            values.push(b.clone());
        }
        headers.push("Taille");
        values.push(ma.file_size.clone());

        let mut table = String::new();
        let mut hrow = String::new();
        for h in &headers {
            hrow.push_str(&dispatch::th(fmt, h, None));
        }
        table.push_str(&dispatch::tr(fmt, &hrow, None));
        let mut vrow = String::new();
        for val in &values {
            vrow.push_str(&dispatch::td(fmt, &dispatch::center(fmt, val, None), None));
        }
        table.push_str(&dispatch::tr(fmt, &vrow, None));
        out.push_str(&dispatch::table(fmt, &table, None));
    }

    // Audio/Langues table
    if !ma.audio.is_empty() {
        out.push('\n');
        out.push_str(&dispatch::sub_heading(fmt, "Langues", title_color, None));
        out.push('\n');
        let mut table = String::new();
        let mut hrow = String::new();
        hrow.push_str(&dispatch::th(fmt, "#", None));
        hrow.push_str(&dispatch::th(fmt, "Langue", None));
        hrow.push_str(&dispatch::th(fmt, "Canaux", None));
        hrow.push_str(&dispatch::th(fmt, "Codec", None));
        hrow.push_str(&dispatch::th(fmt, "Bitrate", None));
        table.push_str(&dispatch::tr(fmt, &hrow, None));
        for (i, a) in ma.audio.iter().enumerate() {
            let mut row = String::new();
            row.push_str(&dispatch::td(fmt, &dispatch::center(fmt, &format!("{}", i + 1), None), None));
            let lang_cell = match a.language.as_deref() {
                Some(l) => language_with_flag(l, fmt),
                None => "-".to_string(),
            };
            row.push_str(&dispatch::td(fmt, &lang_cell, None));
            row.push_str(&dispatch::td(fmt, &dispatch::center(fmt, &a.channels, None), None));
            row.push_str(&dispatch::td(fmt, &dispatch::center(fmt, &a.codec, None), None));
            row.push_str(&dispatch::td(fmt, &dispatch::center(fmt, a.bitrate.as_deref().unwrap_or("-"), None), None));
            table.push_str(&dispatch::tr(fmt, &row, None));
        }
        out.push_str(&dispatch::table(fmt, &table, None));
    }

    // Subtitle table
    if !ma.subtitles.is_empty() {
        out.push('\n');
        out.push_str(&dispatch::sub_heading(fmt, "Sous-Titres", title_color, None));
        out.push('\n');
        let mut table = String::new();
        let mut hrow = String::new();
        hrow.push_str(&dispatch::th(fmt, "#", None));
        hrow.push_str(&dispatch::th(fmt, "Langue", None));
        hrow.push_str(&dispatch::th(fmt, "Format", None));
        hrow.push_str(&dispatch::th(fmt, "Type", None));
        table.push_str(&dispatch::tr(fmt, &hrow, None));
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
            row.push_str(&dispatch::td(fmt, &dispatch::center(fmt, &format!("{}", i + 1), None), None));
            let lang_cell = match s.language.as_deref() {
                Some(l) => language_with_flag(l, fmt),
                None => "-".to_string(),
            };
            row.push_str(&dispatch::td(fmt, &lang_cell, None));
            row.push_str(&dispatch::td(fmt, &dispatch::center(fmt, &s.format, None), None));
            row.push_str(&dispatch::td(fmt, &dispatch::center(fmt, sub_type, None), None));
            table.push_str(&dispatch::tr(fmt, &row, None));
        }
        out.push_str(&dispatch::table(fmt, &table, None));
    }

    out
}
