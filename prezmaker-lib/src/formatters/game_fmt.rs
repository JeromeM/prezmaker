use crate::formatters::bbcode::*;
use crate::models::{Game, Tracker};

pub fn format_game(game: &Game, title_color: &str, tracker: Tracker, pseudo: &str) -> String {
    let mut out = String::new();

    // Header
    let title_upper = format!("\u{1F3AE} {} \u{1F3AE}", game.title.to_uppercase());
    out.push_str(&heading_title_for(tracker, &title_upper, title_color));
    out.push('\n');
    out.push('\n');
    out.push_str(&hr_for(tracker));
    out.push('\n');
    out.push('\n');

    // Section Informations
    out.push_str(&section_heading_for(tracker, "Informations", title_color));
    out.push('\n');
    out.push('\n');

    let mut info = String::new();
    if let Some(ref date) = game.release_date {
        info.push_str(&field_for(tracker, "Date de sortie", date));
        info.push('\n');
    }
    if !game.developers.is_empty() {
        info.push_str(&field_for(tracker, "Developpeur(s)", &game.developers_display()));
        info.push('\n');
    }
    if !game.publishers.is_empty() {
        info.push_str(&field_for(tracker, "Editeur(s)", &game.publishers_display()));
        info.push('\n');
    }
    if !game.genres.is_empty() {
        info.push_str(&field_for(tracker, "Genres", &game.genres_display()));
        info.push('\n');
    }
    if !game.platforms.is_empty() {
        info.push_str(&field_for(tracker, "Plateformes", &game.platforms_display()));
        info.push('\n');
    }

    match tracker {
        Tracker::C411 => {
            let mut table_content = String::new();
            let mut row_content = String::new();
            if let Some(ref cover) = game.cover_url {
                row_content.push_str(&td(&center(&img_width(cover, 264))));
            }
            row_content.push_str(&td(&info));
            table_content.push_str(&tr(&row_content));
            out.push_str(&quote(&table(&table_content)));
        }
        Tracker::TorrXyz => {
            let mut row_content = String::new();
            if let Some(ref cover) = game.cover_url {
                row_content.push_str(&td(&format!("\n{}\n", center(&img_sized_for(tracker, cover, 264, 352)))));
                row_content.push_str(&td(""));
            }
            row_content.push_str(&td(&format!("\n{}\n", quote(&info))));
            let table_content = tr(&row_content);
            out.push_str(&center(&table(&table_content)));
        }
    }

    out.push('\n');
    out.push('\n');
    out.push_str(&hr_for(tracker));
    out.push('\n');
    out.push('\n');

    // Description
    if let Some(ref synopsis) = game.synopsis {
        if !synopsis.is_empty() {
            out.push_str(&section_heading_for(tracker, "Description", title_color));
            out.push('\n');
            out.push('\n');
            out.push_str(&quote_for(tracker, synopsis));
            out.push('\n');
            out.push('\n');
            out.push_str(&hr_for(tracker));
            out.push('\n');
            out.push('\n');
        }
    }

    // Screenshots
    if !game.screenshots.is_empty() {
        out.push_str(&section_heading_for(tracker, "Screenshots", title_color));
        out.push('\n');
        out.push('\n');
        let screenshots: Vec<_> = game.screenshots.iter().take(4).collect();
        let mut inner = String::new();
        for pair in screenshots.chunks(2) {
            let line = pair
                .iter()
                .map(|ss| img_width(ss, 400))
                .collect::<Vec<_>>()
                .join(" ");
            if !inner.is_empty() {
                inner.push('\n');
            }
            inner.push_str(&line);
        }
        out.push_str(&center(&inner));
        out.push('\n');
        out.push('\n');
        out.push_str(&hr_for(tracker));
        out.push('\n');
        out.push('\n');
    }

    // Notes (apres screenshots)
    if !game.ratings.is_empty() {
        out.push_str(&section_heading_for(tracker, "Notes", title_color));
        out.push('\n');
        out.push('\n');

        match tracker {
            Tracker::C411 => {
                let mut ratings_table = String::new();
                let mut header_row = String::new();
                for rating in &game.ratings {
                    header_row.push_str(&th(&rating.source));
                }
                ratings_table.push_str(&tr(&header_row));
                let mut values_row = String::new();
                for rating in &game.ratings {
                    values_row.push_str(&td(&center(&colored_rating_for(tracker, rating.value, rating.max))));
                }
                ratings_table.push_str(&tr(&values_row));
                out.push_str(&table(&ratings_table));
            }
            Tracker::TorrXyz => {
                let mut header_row = String::new();
                let mut values_row = String::new();
                for (i, rating) in game.ratings.iter().enumerate() {
                    if i > 0 {
                        header_row.push_str(&th("        "));
                        values_row.push_str(&td(""));
                    }
                    header_row.push_str(&rating_header_torrxyz(&rating.source));
                    values_row.push_str(&td(&colored_rating_for(tracker, rating.value, rating.max)));
                }
                let ratings_table = format!("{}{}", tr(&header_row), tr(&values_row));
                out.push_str(&center(&table(&ratings_table)));
            }
        }
        out.push('\n');
        out.push_str(&hr_for(tracker));
        out.push('\n');
        out.push('\n');
    }

    // Informations techniques
    out.push_str(&sub_heading_for(tracker, "Informations techniques", title_color));
    out.push('\n');
    out.push('\n');

    let mut tech_headers: Vec<&str> = vec!["Plateforme", "Langue(s)", "Taille"];
    let has_install_size = game.tech_info.as_ref().map_or(false, |t| !t.install_size.is_empty());
    if has_install_size {
        tech_headers.push("Taille installee");
    }
    match tracker {
        Tracker::C411 => {
            let mut tech_table = String::new();
            let mut header_row = String::new();
            for h in &tech_headers {
                header_row.push_str(&th(h));
            }
            tech_table.push_str(&tr(&header_row));
            let mut values_row = String::new();
            if let Some(ref tech) = game.tech_info {
                let mut values: Vec<&str> = vec![&tech.platform, &tech.languages, &tech.size];
                if has_install_size {
                    values.push(&tech.install_size);
                }
                for val in &values {
                    let display = if val.is_empty() { " " } else { val };
                    values_row.push_str(&td(&center(display)));
                }
            } else {
                for _ in &tech_headers {
                    values_row.push_str(&td(&center(" ")));
                }
            }
            tech_table.push_str(&tr(&values_row));
            out.push_str(&table(&tech_table));
        }
        Tracker::TorrXyz => {
            if let Some(ref tech) = game.tech_info {
                let mut values: Vec<&str> = vec![&tech.platform, &tech.languages, &tech.size];
                if has_install_size {
                    values.push(&tech.install_size);
                }
                for (h, val) in tech_headers.iter().zip(values.iter()) {
                    let display = if val.is_empty() { " " } else { val };
                    out.push_str(&center(&field_for(tracker, h, display)));
                    out.push('\n');
                }
            } else {
                for h in &tech_headers {
                    out.push_str(&center(&field_for(tracker, h, " ")));
                    out.push('\n');
                }
            }
        }
    }
    out.push('\n');
    out.push_str(&hr_for(tracker));
    out.push('\n');
    out.push('\n');

    // Section Installation
    if let Some(ref install) = game.installation {
        out.push_str(&sub_heading_for(tracker, "Installation", title_color));
        out.push('\n');
        out.push('\n');
        match tracker {
            Tracker::C411 => {
                out.push_str(&quote(install));
            }
            Tracker::TorrXyz => {
                out.push_str(&format!("[quote]{}\n[/quote]\n[left][/left]", center(&color("aaaaaa", install))));
            }
        }
        out.push('\n');
        out.push('\n');
        out.push_str(&hr_for(tracker));
        out.push('\n');
        out.push('\n');
    }

    // Signature
    let footer = footer_for(tracker, pseudo);
    if !footer.is_empty() {
        out.push_str(&footer);
        out.push('\n');
    }

    out
}
