use crate::formatters::bbcode::*;
use crate::models::{MediaTechInfo, Series};

pub fn format_series(series: &Series, title_color: &str, pseudo: &str) -> String {
    format_series_with_tech(series, title_color, None, pseudo)
}

pub fn format_series_with_tech(series: &Series, title_color: &str, tech: Option<&MediaTechInfo>, pseudo: &str) -> String {
    let mut out = String::new();

    // Header
    let title_upper = format!("\u{1F4FA} {} \u{1F4FA}", series.title.to_uppercase());
    out.push_str(&heading_title(&title_upper, title_color));
    out.push('\n');
    out.push('\n');

    // Section Informations
    out.push_str(&section_heading("Informations", title_color));
    out.push('\n');
    out.push('\n');

    let mut info = String::new();
    if !series.countries.is_empty() {
        info.push_str(&field("Origine", &series.countries_display()));
        info.push('\n');
    }
    if let Some(ref date) = series.first_air_date {
        info.push_str(&field("Premiere diffusion", &format_date(date)));
        info.push('\n');
    }
    if let Some(ref status) = series.status {
        info.push_str(&field("Statut", &translate_status(status)));
        info.push('\n');
    }
    if let Some(seasons) = series.seasons_count {
        info.push_str(&field("Saisons", &seasons.to_string()));
        info.push('\n');
    }
    if let Some(episodes) = series.episodes_count {
        info.push_str(&field("Episodes", &episodes.to_string()));
        info.push('\n');
    }
    if let Some(ref runtime) = series.runtime_formatted() {
        info.push_str(&field("Duree par episode", runtime));
        info.push('\n');
    }
    if !series.creators.is_empty() {
        info.push_str(&field("Createur(s)", &series.creators_display()));
        info.push('\n');
    }
    if !series.networks.is_empty() {
        info.push_str(&field("Chaine / Plateforme", &series.networks_display()));
        info.push('\n');
    }
    if !series.genres.is_empty() {
        info.push_str(&field("Genres", &series.genres_display()));
        info.push('\n');
    }

    // Casting
    if !series.cast.is_empty() {
        info.push('\n');
        info.push_str(&inline_heading("Casting", title_color));
        info.push('\n');
        info.push('\n');
        info.push_str(&field("Acteurs", &series.cast_display(8)));
        info.push('\n');
    }

    {
        let mut table_content = String::new();
        let mut row_content = String::new();
        if let Some(ref poster) = series.poster_url {
            row_content.push_str(&td(&center(&img_width(poster, 300))));
        }
        row_content.push_str(&td(&info));
        table_content.push_str(&tr(&row_content));
        out.push_str(&quote(&table(&table_content)));
    }

    out.push('\n');
    out.push('\n');

    // Ratings
    if !series.ratings.is_empty() {
        out.push_str(&section_heading("Notes", title_color));
        out.push('\n');
        out.push('\n');

        let mut ratings_table = String::new();
        let mut header_row = String::new();
        for rating in &series.ratings {
            header_row.push_str(&th(&rating.source));
        }
        ratings_table.push_str(&tr(&header_row));
        let mut values_row = String::new();
        for rating in &series.ratings {
            values_row.push_str(&td(&center(&colored_rating(rating.value, rating.max))));
        }
        ratings_table.push_str(&tr(&values_row));
        out.push_str(&table(&ratings_table));

        out.push('\n');
        out.push('\n');
    }

    // Synopsis
    if let Some(ref synopsis) = series.synopsis {
        if !synopsis.is_empty() {
            out.push_str(&section_heading("Synopsis", title_color));
            out.push('\n');
            out.push('\n');
            out.push_str(&quote(synopsis));
            out.push('\n');
            out.push('\n');
        }
    }

    // Technical info
    out.push_str(&sub_heading("Informations techniques", title_color));
    out.push('\n');
    out.push('\n');

    {
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
            header_row.push_str(&th(h));
        }
        tech_table.push_str(&tr(&header_row));
        let mut val_row = String::new();
        for v in &values {
            val_row.push_str(&td(&center(v)));
        }
        tech_table.push_str(&tr(&val_row));
        out.push_str(&table(&tech_table));
    }
    out.push('\n');

    let footer = footer(pseudo);
    if !footer.is_empty() {
        out.push_str(&footer);
        out.push('\n');
    }

    out
}

fn format_date(date_str: &str) -> String {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() == 3 {
        let month_name = match parts[1] {
            "01" => "janvier",
            "02" => "fevrier",
            "03" => "mars",
            "04" => "avril",
            "05" => "mai",
            "06" => "juin",
            "07" => "juillet",
            "08" => "aout",
            "09" => "septembre",
            "10" => "octobre",
            "11" => "novembre",
            "12" => "decembre",
            _ => parts[1],
        };
        let day = parts[2].trim_start_matches('0');
        format!("{} {} {}", day, month_name, parts[0])
    } else {
        date_str.to_string()
    }
}

fn translate_status(status: &str) -> String {
    match status {
        "Returning Series" => "En cours".to_string(),
        "Ended" => "Terminee".to_string(),
        "Canceled" => "Annulee".to_string(),
        "In Production" => "En production".to_string(),
        "Planned" => "Planifiee".to_string(),
        _ => status.to_string(),
    }
}
