use crate::formatters::bbcode::*;
use crate::models::{MediaTechInfo, Movie};

pub fn format_movie(movie: &Movie, title_color: &str, pseudo: &str) -> String {
    format_movie_with_tech(movie, title_color, None, pseudo)
}

pub fn format_movie_with_tech(movie: &Movie, title_color: &str, tech: Option<&MediaTechInfo>, pseudo: &str) -> String {
    let mut out = String::new();

    // Header
    let title_upper = format!("\u{1F3AC} {} \u{1F3AC}", movie.title.to_uppercase());
    out.push_str(&heading_title(&title_upper, title_color));
    out.push('\n');
    out.push('\n');

    // Section Informations
    out.push_str(&section_heading("Informations", title_color));
    out.push('\n');
    out.push('\n');

    let mut info = String::new();
    if !movie.countries.is_empty() {
        info.push_str(&field("Origine", &movie.countries_display()));
        info.push('\n');
    }
    if let Some(ref date) = movie.release_date {
        info.push_str(&field("Sortie", &format_release_date(date)));
        info.push('\n');
    }
    if let Some(ref dur) = movie.duration_formatted() {
        info.push_str(&field("Duree", dur));
        info.push('\n');
    }
    if !movie.directors.is_empty() {
        info.push_str(&field("Realisateur", &movie.directors_display()));
        info.push('\n');
    }
    if !movie.genres.is_empty() {
        info.push_str(&field("Genres", &movie.genres_display()));
        info.push('\n');
    }

    // Casting
    if !movie.cast.is_empty() {
        info.push('\n');
        info.push_str(&inline_heading("Casting", title_color));
        info.push('\n');
        info.push('\n');
        info.push_str(&field("Acteurs", &movie.cast_display(6)));
        info.push('\n');
    }

    {
        let mut table_content = String::new();
        let mut row_content = String::new();
        if let Some(ref poster) = movie.poster_url {
            row_content.push_str(&td(&center(&img_width(poster, 300))));
        }
        row_content.push_str(&td(&info));
        table_content.push_str(&tr(&row_content));
        out.push_str(&quote(&table(&table_content)));
    }

    out.push('\n');
    out.push('\n');

    // Ratings
    if !movie.ratings.is_empty() {
        out.push_str(&section_heading("Notes", title_color));
        out.push('\n');
        out.push('\n');

        let mut ratings_table = String::new();
        let mut header_row = String::new();
        for rating in &movie.ratings {
            header_row.push_str(&th(&rating.source));
        }
        ratings_table.push_str(&tr(&header_row));
        let mut values_row = String::new();
        for rating in &movie.ratings {
            values_row.push_str(&td(&center(&colored_rating(rating.value, rating.max))));
        }
        ratings_table.push_str(&tr(&values_row));
        out.push_str(&table(&ratings_table));

        out.push('\n');
        out.push('\n');
    }

    // Synopsis
    if let Some(ref synopsis) = movie.synopsis {
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
    out.push('\n');

    // Footer
    let footer = footer(pseudo);
    if !footer.is_empty() {
        out.push_str(&footer);
        out.push('\n');
    }

    out
}

fn format_release_date(date_str: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Country, Genre, Person, Rating};

    fn sample_movie() -> Movie {
        Movie {
            title: "Intouchables".to_string(),
            original_title: Some("Intouchables".to_string()),
            year: Some(2011),
            release_date: Some("2011-11-02".to_string()),
            duration_minutes: Some(112),
            synopsis: Some("Philippe, un riche aristocrate...".to_string()),
            poster_url: Some("https://image.tmdb.org/t/p/w500/poster.jpg".to_string()),
            backdrop_url: None,
            genres: vec![
                Genre { name: "Comedie".to_string() },
                Genre { name: "Drame".to_string() },
            ],
            countries: vec![Country {
                name: "France".to_string(),
                iso_code: Some("FR".to_string()),
            }],
            directors: vec![
                Person { name: "Olivier Nakache".to_string(), role: Some("Director".to_string()) },
                Person { name: "Eric Toledano".to_string(), role: Some("Director".to_string()) },
            ],
            cast: vec![
                Person { name: "Francois Cluzet".to_string(), role: None },
                Person { name: "Omar Sy".to_string(), role: None },
            ],
            ratings: vec![
                Rating { source: "TMDB".to_string(), value: 8.2, max: 10.0 },
                Rating { source: "Allocine Presse".to_string(), value: 3.8, max: 5.0 },
            ],
            tmdb_id: Some(77338),
            imdb_id: None,
            allocine_url: None,
        }
    }

    #[test]
    fn test_format_release_date() {
        assert_eq!(format_release_date("2011-11-02"), "2 novembre 2011");
        assert_eq!(format_release_date("1999-03-31"), "31 mars 1999");
    }

    #[test]
    fn test_format_movie_contains_title() {
        let movie = sample_movie();
        let bbcode = format_movie(&movie, "c0392b", "TestUser");
        assert!(bbcode.contains("INTOUCHABLES"));
        assert!(bbcode.contains("[h1]"));
        assert!(bbcode.contains("[color=#c0392b]"));
    }

    #[test]
    fn test_format_movie_ratings_colors() {
        let movie = sample_movie();
        let bbcode = format_movie(&movie, "c0392b", "TestUser");
        assert!(bbcode.contains("8.2"));
        assert!(bbcode.contains("[color=#27ae60]"));
    }

    #[test]
    fn test_format_movie_contains_info() {
        let movie = sample_movie();
        let bbcode = format_movie(&movie, "c0392b", "TestUser");
        assert!(bbcode.contains("France"));
        assert!(bbcode.contains("2 novembre 2011"));
        assert!(bbcode.contains("1h et 52min"));
        assert!(bbcode.contains("Olivier Nakache"));
        assert!(bbcode.contains("Comedie, Drame"));
    }
}
