use std::collections::HashMap;

use crate::formatters::bbcode;
use crate::formatters::OutputFormat;

use super::{format_date_fr, translate_status, RenderContext};
use super::data::{build_movie_data, build_series_data, build_app_data, build_media_analysis_data};
use super::render::render;

pub fn preview_template(
    template_body: &str,
    content_type: &str,
    title_color: &str,
    pseudo: &str,
) -> String {
    preview_template_with_format(template_body, content_type, title_color, pseudo, OutputFormat::Bbcode)
}

pub fn preview_template_with_format(
    template_body: &str,
    content_type: &str,
    title_color: &str,
    pseudo: &str,
    output_format: OutputFormat,
) -> String {
    let (data, mut ctx) = build_sample_data_with_format(content_type, title_color, output_format);
    ctx.output_format = output_format;
    render(template_body, &data, &ctx, title_color, pseudo)
}

fn build_sample_data_with_format(
    content_type: &str,
    title_color: &str,
    fmt: OutputFormat,
) -> (HashMap<String, String>, RenderContext) {
    match content_type {
        "film" => build_sample_movie(title_color),
        "serie" => build_sample_series(title_color),
        "jeu" => build_sample_game(title_color, fmt),
        "app" => build_sample_app(),
        _ => (HashMap::new(), RenderContext::default()),
    }
}

fn build_sample_movie(title_color: &str) -> (HashMap<String, String>, RenderContext) {
    use crate::models::{Movie, MediaTechInfo, Rating, Genre, Country, Person};

    let movie = Movie {
        title: "Interstellar".into(),
        original_title: Some("Interstellar".into()),
        year: Some(2014),
        release_date: Some("2014-11-05".into()),
        duration_minutes: Some(169),
        synopsis: Some("Les aventures d'un groupe d'explorateurs qui utilisent une faille dans l'espace-temps pour repousser les limites de l'exploration spatiale et conquerir les distances astronomiques.".into()),
        poster_url: Some("https://image.tmdb.org/t/p/w500/gEU2QniE6E77NI6lCU6MxlNBvIx.jpg".into()),
        backdrop_url: None,
        genres: vec![
            Genre { name: "Science-Fiction".into() },
            Genre { name: "Drame".into() },
            Genre { name: "Aventure".into() },
        ],
        countries: vec![Country { name: "Etats-Unis".into(), iso_code: Some("US".into()) }],
        directors: vec![Person { name: "Christopher Nolan".into(), role: None }],
        cast: vec![
            Person { name: "Matthew McConaughey".into(), role: Some("Cooper".into()) },
            Person { name: "Anne Hathaway".into(), role: Some("Brand".into()) },
            Person { name: "Jessica Chastain".into(), role: Some("Murph".into()) },
            Person { name: "Michael Caine".into(), role: Some("Professeur Brand".into()) },
        ],
        ratings: vec![
            Rating { source: "TMDB".into(), value: 8.4, max: 10.0 },
            Rating { source: "Allocine".into(), value: 4.2, max: 5.0 },
        ],
        tmdb_id: Some(157336),
        imdb_id: Some("tt0816692".into()),
        allocine_url: None,
    };

    let tech = MediaTechInfo {
        quality: Some("1080p".into()),
        video_codec: Some("x264".into()),
        audio: Some("DTS-HD MA 5.1".into()),
        language: Some("Multi (FR, EN)".into()),
        subtitles: Some("FR, EN".into()),
        size: Some("12.5 Go".into()),
    };

    let sample_analysis = build_sample_media_analysis();
    let mut data = build_movie_data(&movie, Some(&tech));
    build_media_analysis_data(&mut data, &sample_analysis);
    let info_bbcode = build_sample_movie_info(&movie, title_color);
    let ctx = RenderContext {
        ratings: movie.ratings.clone(),
        poster_url: movie.poster_url.clone(),
        tech: Some(tech),
        media_analysis: Some(sample_analysis),
        info_bbcode: Some(info_bbcode),
        ..Default::default()
    };
    (data, ctx)
}

fn build_sample_movie_info(movie: &crate::models::Movie, title_color: &str) -> String {
    let mut info = String::new();
    info.push_str(&bbcode::field("Origine", &movie.countries_display()));
    info.push('\n');
    if let Some(ref d) = movie.release_date {
        info.push_str(&bbcode::field("Sortie", &format_date_fr(d)));
        info.push('\n');
    }
    if let Some(ref dur) = movie.duration_formatted() {
        info.push_str(&bbcode::field("Duree", dur));
        info.push('\n');
    }
    info.push_str(&bbcode::field("Realisateur", &movie.directors_display()));
    info.push('\n');
    info.push_str(&bbcode::field("Genres", &movie.genres_display()));
    info.push('\n');
    info.push('\n');
    info.push_str(&bbcode::inline_heading("Casting", title_color));
    info.push_str("\n\n");
    info.push_str(&bbcode::field("Acteurs", &movie.cast_display(6)));
    info.push('\n');
    info
}

fn build_sample_series(title_color: &str) -> (HashMap<String, String>, RenderContext) {
    use crate::models::{Series, MediaTechInfo, Rating, Genre, Country, Person};

    let series = Series {
        title: "Breaking Bad".into(),
        original_title: Some("Breaking Bad".into()),
        year: Some(2008),
        end_year: Some(2013),
        first_air_date: Some("2008-01-20".into()),
        synopsis: Some("Un professeur de chimie atteint d'un cancer du poumon s'associe a un ancien eleve pour fabriquer et vendre de la methamphétamine.".into()),
        poster_url: Some("https://image.tmdb.org/t/p/w500/ggFHVNu6YYI5L9pCfOacjizRGt.jpg".into()),
        backdrop_url: None,
        genres: vec![
            Genre { name: "Drame".into() },
            Genre { name: "Crime".into() },
        ],
        countries: vec![Country { name: "Etats-Unis".into(), iso_code: Some("US".into()) }],
        creators: vec![Person { name: "Vince Gilligan".into(), role: None }],
        cast: vec![
            Person { name: "Bryan Cranston".into(), role: Some("Walter White".into()) },
            Person { name: "Aaron Paul".into(), role: Some("Jesse Pinkman".into()) },
            Person { name: "Anna Gunn".into(), role: Some("Skyler White".into()) },
        ],
        ratings: vec![
            Rating { source: "TMDB".into(), value: 8.9, max: 10.0 },
            Rating { source: "Allocine".into(), value: 4.6, max: 5.0 },
        ],
        seasons_count: Some(5),
        episodes_count: Some(62),
        episode_runtime: Some(47),
        status: Some("Ended".into()),
        networks: vec!["AMC".into()],
        tmdb_id: Some(1396),
        imdb_id: Some("tt0903747".into()),
        allocine_url: None,
    };

    let tech = MediaTechInfo {
        quality: Some("1080p".into()),
        video_codec: Some("x265".into()),
        audio: Some("AAC 5.1".into()),
        language: Some("Multi (FR, EN)".into()),
        subtitles: Some("FR".into()),
        size: Some("45.2 Go".into()),
    };

    let sample_analysis = build_sample_media_analysis();
    let mut data = build_series_data(&series, Some(&tech));
    build_media_analysis_data(&mut data, &sample_analysis);
    let info_bbcode = build_sample_series_info(&series, title_color);
    let ctx = RenderContext {
        ratings: series.ratings.clone(),
        poster_url: series.poster_url.clone(),
        tech: Some(tech),
        media_analysis: Some(sample_analysis),
        info_bbcode: Some(info_bbcode),
        ..Default::default()
    };
    (data, ctx)
}

fn build_sample_series_info(series: &crate::models::Series, title_color: &str) -> String {
    let mut info = String::new();
    info.push_str(&bbcode::field("Origine", &series.countries_display()));
    info.push('\n');
    if let Some(ref d) = series.first_air_date {
        info.push_str(&bbcode::field("Premiere diffusion", &format_date_fr(d)));
        info.push('\n');
    }
    if let Some(ref s) = series.status {
        info.push_str(&bbcode::field("Statut", &translate_status(s)));
        info.push('\n');
    }
    if let Some(s) = series.seasons_count {
        info.push_str(&bbcode::field("Saisons", &s.to_string()));
        info.push('\n');
    }
    if let Some(e) = series.episodes_count {
        info.push_str(&bbcode::field("Episodes", &e.to_string()));
        info.push('\n');
    }
    if let Some(ref rt) = series.runtime_formatted() {
        info.push_str(&bbcode::field("Duree par episode", rt));
        info.push('\n');
    }
    info.push_str(&bbcode::field("Createur(s)", &series.creators_display()));
    info.push('\n');
    info.push_str(&bbcode::field("Chaine / Plateforme", &series.networks_display()));
    info.push('\n');
    info.push_str(&bbcode::field("Genres", &series.genres_display()));
    info.push('\n');
    info.push('\n');
    info.push_str(&bbcode::inline_heading("Casting", title_color));
    info.push_str("\n\n");
    info.push_str(&bbcode::field("Acteurs", &series.cast_display(8)));
    info.push('\n');
    info
}

fn build_sample_game(_title_color: &str, fmt: OutputFormat) -> (HashMap<String, String>, RenderContext) {
    use crate::models::{Game, TechInfo, SystemReqs, Rating, Genre};

    let tech_info = TechInfo {
        platform: "PC (Windows)".into(),
        languages: "FR, EN, DE, ES".into(),
        size: "85.3 Go".into(),
        install_size: "120 Go".into(),
    };

    let min_reqs = SystemReqs {
        os: "Windows 10 64-bit".into(),
        cpu: "Intel Core i5-3570K / AMD FX-8310".into(),
        ram: "8 Go".into(),
        gpu: "NVIDIA GTX 970 / AMD RX 470".into(),
        storage: "70 Go SSD".into(),
    };

    let rec_reqs = SystemReqs {
        os: "Windows 10/11 64-bit".into(),
        cpu: "Intel Core i7-4790 / AMD Ryzen 3 3200G".into(),
        ram: "12 Go".into(),
        gpu: "NVIDIA GTX 1060 6Go / AMD RX 590".into(),
        storage: "70 Go SSD".into(),
    };

    let game = Game {
        title: "Cyberpunk 2077".into(),
        release_date: Some("10 decembre 2020".into()),
        year: Some(2020),
        synopsis: Some("Cyberpunk 2077 est un RPG en monde ouvert se deroulant a Night City, une megalopole obsedee par le pouvoir, le glamour et la modification corporelle.".into()),
        cover_url: Some("https://images.igdb.com/igdb/image/upload/t_cover_big/co4hkv.png".into()),
        screenshots: vec![
            "https://images.igdb.com/igdb/image/upload/t_screenshot_big/sc7ngs.jpg".into(),
            "https://images.igdb.com/igdb/image/upload/t_screenshot_big/sc7ngt.jpg".into(),
        ],
        genres: vec![
            Genre { name: "RPG".into() },
            Genre { name: "Aventure".into() },
        ],
        platforms: vec!["PC".into(), "PlayStation 5".into(), "Xbox Series X|S".into()],
        developers: vec!["CD Projekt Red".into()],
        publishers: vec!["CD Projekt".into()],
        ratings: vec![
            Rating { source: "IGDB".into(), value: 78.0, max: 100.0 },
        ],
        igdb_id: Some(1877),
        igdb_slug: Some("cyberpunk-2077".into()),
        steam_appid: Some(1091500),
        tech_info: Some(tech_info.clone()),
        installation: Some("1. Extraire l'archive\n2. Lancer le setup\n3. Jouer".into()),
        min_reqs: Some(min_reqs.clone()),
        rec_reqs: Some(rec_reqs.clone()),
    };

    let data = super::data::build_game_data_with_format(&game, fmt);
    let info_bbcode = build_sample_game_info(&game, fmt);
    let ctx = RenderContext {
        ratings: game.ratings.clone(),
        cover_url: game.cover_url.clone(),
        screenshots: game.screenshots.clone(),
        game_tech: Some(tech_info),
        min_reqs: Some(min_reqs),
        rec_reqs: Some(rec_reqs),
        info_bbcode: Some(info_bbcode),
        ..Default::default()
    };
    (data, ctx)
}

fn build_sample_game_info(game: &crate::models::Game, fmt: OutputFormat) -> String {
    use crate::formatters::dispatch;
    let mut info = String::new();
    if let Some(ref d) = game.release_date {
        info.push_str(&dispatch::field(fmt, "Date de sortie", d));
        info.push('\n');
    }
    info.push_str(&dispatch::field(fmt, "Developpeur(s)", &game.developers_display()));
    info.push('\n');
    info.push_str(&dispatch::field(fmt, "Editeur(s)", &game.publishers_display()));
    info.push('\n');
    info.push_str(&dispatch::field(fmt, "Genres", &game.genres_display()));
    info.push('\n');
    info.push_str(&dispatch::field(fmt, "Plateformes", &game.platforms_display()));
    info.push('\n');
    info
}

fn build_sample_app() -> (HashMap<String, String>, RenderContext) {
    use crate::models::Application;

    let app = Application {
        name: "qBittorrent".into(),
        version: Some("4.6.3".into()),
        developer: Some("qBittorrent Team".into()),
        description: Some("Client BitTorrent libre et open source avec une interface intuitive, un moteur de recherche integre et le support des flux RSS.".into()),
        website: Some("https://www.qbittorrent.org".into()),
        license: Some("GPLv2".into()),
        platforms: vec!["Windows".into(), "macOS".into(), "Linux".into()],
        logo_url: None,
    };

    let data = build_app_data(&app);
    let info_bbcode = build_sample_app_info(&app);
    let ctx = RenderContext {
        logo_url: app.logo_url.clone(),
        info_bbcode: Some(info_bbcode),
        ..Default::default()
    };
    (data, ctx)
}

fn build_sample_app_info(app: &crate::models::Application) -> String {
    let mut info = String::new();
    info.push_str(&bbcode::field("Nom", &app.name));
    info.push('\n');
    if let Some(ref v) = app.version {
        info.push_str(&bbcode::field("Version", v));
        info.push('\n');
    }
    if let Some(ref d) = app.developer {
        info.push_str(&bbcode::field("Developpeur", d));
        info.push('\n');
    }
    if let Some(ref l) = app.license {
        info.push_str(&bbcode::field("Licence", l));
        info.push('\n');
    }
    if let Some(ref w) = app.website {
        info.push_str(&bbcode::field("Site web", &bbcode::url(w, w)));
        info.push('\n');
    }
    info.push_str(&bbcode::field("Plateformes", &app.platforms_display()));
    info.push('\n');
    info
}

fn build_sample_media_analysis() -> crate::models::MediaAnalysis {
    use crate::models::{AudioTrack, MediaAnalysis, SubtitleTrack, VideoTrack};

    MediaAnalysis {
        format: "Matroska".into(),
        file_name: "Movie.2014.1080p.BluRay.DTS-HD.MA.5.1.x264-GRP.mkv".into(),
        file_size: "12.5 GiB".into(),
        duration: Some("2 h 49 min".into()),
        bitrate: Some("10.5 Mb/s".into()),
        video: vec![VideoTrack {
            codec: "AVC (H.264)".into(),
            width: 1920,
            height: 1080,
            fps: Some("23.976 FPS".into()),
            bitrate: Some("9500 kb/s".into()),
            language: Some("English".into()),
        }],
        audio: vec![
            AudioTrack {
                codec: "EAC3".into(),
                channels: "5.1".into(),
                sample_rate: Some("48.0 kHz".into()),
                bitrate: Some("640 kb/s".into()),
                language: Some("Français".into()),
                is_default: true,
            },
            AudioTrack {
                codec: "EAC3".into(),
                channels: "5.1".into(),
                sample_rate: Some("48.0 kHz".into()),
                bitrate: Some("640 kb/s".into()),
                language: Some("Anglais".into()),
                is_default: false,
            },
        ],
        subtitles: vec![
            SubtitleTrack {
                format: "SRT".into(),
                language: Some("Français".into()),
                title: Some("Forced".into()),
                is_default: true,
                is_forced: true,
            },
            SubtitleTrack {
                format: "SRT".into(),
                language: Some("Français".into()),
                title: None,
                is_default: false,
                is_forced: false,
            },
            SubtitleTrack {
                format: "SRT".into(),
                language: Some("Anglais".into()),
                title: None,
                is_default: false,
                is_forced: false,
            },
        ],
        raw_text: String::new(),
    }
}
