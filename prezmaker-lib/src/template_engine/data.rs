use std::collections::HashMap;

use crate::formatters::bbcode;
use crate::formatters::OutputFormat;

use super::{format_date_fr, translate_status};
use super::blocks::format_system_reqs;

pub fn build_movie_data(
    movie: &crate::models::Movie,
    tech: Option<&crate::models::MediaTechInfo>,
) -> HashMap<String, String> {
    let mut data = HashMap::new();

    data.insert("titre".into(), movie.title.clone());
    data.insert("titre_maj".into(), movie.title.to_uppercase());
    if let Some(ref ot) = movie.original_title {
        data.insert("titre_original".into(), ot.clone());
    }
    if let Some(y) = movie.year {
        data.insert("annee".into(), y.to_string());
    }
    if let Some(ref d) = movie.release_date {
        data.insert("date_sortie".into(), format_date_fr(d));
    }
    if let Some(ref dur) = movie.duration_formatted() {
        data.insert("duree".into(), dur.clone());
    }
    if !movie.directors.is_empty() {
        data.insert("realisateurs".into(), movie.directors_display());
    }
    if !movie.genres.is_empty() {
        data.insert("genres".into(), movie.genres_display());
    }
    if !movie.countries.is_empty() {
        data.insert("pays".into(), movie.countries_display());
    }
    if !movie.cast.is_empty() {
        data.insert("casting".into(), movie.cast_display(6));
    }
    if let Some(ref s) = movie.synopsis {
        if !s.is_empty() {
            data.insert("synopsis".into(), s.clone());
        }
    }
    if let Some(ref url) = movie.poster_url {
        data.insert("poster_url".into(), url.clone());
    }

    // Links
    if let Some(id) = movie.tmdb_id {
        let tmdb_link = format!("https://www.themoviedb.org/movie/{}", id);
        data.insert("tmdb_link".into(), tmdb_link.clone());
        data.insert("link".into(), tmdb_link);
    }
    if let Some(ref id) = movie.imdb_id {
        data.insert("imdb_link".into(), format!("https://www.imdb.com/title/{}/", id));
    }
    if let Some(ref url) = movie.allocine_url {
        data.insert("allocine_link".into(), url.clone());
    }

    // Ratings as individual tags
    build_ratings_data(&mut data, &movie.ratings);

    // Tech info
    if let Some(t) = tech {
        build_media_tech_data(&mut data, t);
    }

    data
}

pub fn build_series_data(
    series: &crate::models::Series,
    tech: Option<&crate::models::MediaTechInfo>,
) -> HashMap<String, String> {
    let mut data = HashMap::new();

    data.insert("titre".into(), series.title.clone());
    data.insert("titre_maj".into(), series.title.to_uppercase());
    if let Some(ref ot) = series.original_title {
        data.insert("titre_original".into(), ot.clone());
    }
    if let Some(y) = series.year {
        data.insert("annee".into(), y.to_string());
    }
    if let Some(ref d) = series.first_air_date {
        data.insert("premiere_diffusion".into(), format_date_fr(d));
    }
    if let Some(ref s) = series.status {
        data.insert("statut".into(), translate_status(s));
    }
    if let Some(s) = series.seasons_count {
        data.insert("saisons".into(), s.to_string());
    }
    if let Some(e) = series.episodes_count {
        data.insert("episodes".into(), e.to_string());
    }
    if let Some(ref rt) = series.runtime_formatted() {
        data.insert("duree_episode".into(), rt.clone());
    }
    if !series.creators.is_empty() {
        data.insert("createurs".into(), series.creators_display());
    }
    if !series.networks.is_empty() {
        data.insert("chaines".into(), series.networks_display());
    }
    if !series.genres.is_empty() {
        data.insert("genres".into(), series.genres_display());
    }
    if !series.countries.is_empty() {
        data.insert("pays".into(), series.countries_display());
    }
    if !series.cast.is_empty() {
        data.insert("casting".into(), series.cast_display(8));
    }
    if let Some(ref s) = series.synopsis {
        if !s.is_empty() {
            data.insert("synopsis".into(), s.clone());
        }
    }
    if let Some(ref url) = series.poster_url {
        data.insert("poster_url".into(), url.clone());
    }

    // Links
    if let Some(id) = series.tmdb_id {
        let tmdb_link = format!("https://www.themoviedb.org/tv/{}", id);
        data.insert("tmdb_link".into(), tmdb_link.clone());
        data.insert("link".into(), tmdb_link);
    }
    if let Some(ref id) = series.imdb_id {
        data.insert("imdb_link".into(), format!("https://www.imdb.com/title/{}/", id));
    }
    if let Some(ref url) = series.allocine_url {
        data.insert("allocine_link".into(), url.clone());
    }

    build_ratings_data(&mut data, &series.ratings);

    if let Some(t) = tech {
        build_media_tech_data(&mut data, t);
    }

    data
}

pub fn build_game_data(game: &crate::models::Game) -> HashMap<String, String> {
    build_game_data_with_format(game, OutputFormat::Bbcode)
}

pub fn build_game_data_with_format(game: &crate::models::Game, fmt: OutputFormat) -> HashMap<String, String> {
    let mut data = HashMap::new();

    data.insert("titre".into(), game.title.clone());
    data.insert("titre_maj".into(), game.title.to_uppercase());
    if let Some(ref d) = game.release_date {
        data.insert("date_sortie".into(), d.clone());
    }
    if let Some(y) = game.year {
        data.insert("annee".into(), y.to_string());
    }
    if let Some(ref s) = game.synopsis {
        if !s.is_empty() {
            data.insert("synopsis".into(), s.clone());
        }
    }
    if let Some(ref url) = game.cover_url {
        data.insert("cover_url".into(), url.clone());
    }
    if !game.genres.is_empty() {
        data.insert("genres".into(), game.genres_display());
    }
    if !game.platforms.is_empty() {
        data.insert("plateformes".into(), game.platforms_display());
    }
    if !game.developers.is_empty() {
        data.insert("developpeurs".into(), game.developers_display());
    }
    if !game.publishers.is_empty() {
        data.insert("editeurs".into(), game.publishers_display());
    }
    if !game.screenshots.is_empty() {
        // Store individual screenshot URLs
        for (i, ss) in game.screenshots.iter().take(4).enumerate() {
            data.insert(format!("screenshot_{}", i + 1), ss.clone());
        }
        data.insert("screenshots".into(), "true".into());
    }
    if let Some(ref install) = game.installation {
        data.insert("installation".into(), install.clone());
    }
    if let Some(ref tech) = game.tech_info {
        data.insert("tech_plateforme".into(), tech.platform.clone());
        data.insert("tech_langues".into(), tech.languages.clone());
        data.insert("tech_taille".into(), tech.size.clone());
        if !tech.install_size.is_empty() {
            data.insert("tech_taille_installee".into(), tech.install_size.clone());
        }
    }

    // System requirements
    if let Some(ref reqs) = game.min_reqs {
        if !reqs.is_empty() {
            data.insert("config_mini".into(), format_system_reqs(reqs, fmt));
        }
    }
    if let Some(ref reqs) = game.rec_reqs {
        if !reqs.is_empty() {
            data.insert("config_reco".into(), format_system_reqs(reqs, fmt));
        }
    }

    // Links
    if let Some(ref slug) = game.igdb_slug {
        let igdb_link = format!("https://www.igdb.com/games/{}", slug);
        data.insert("igdb_link".into(), igdb_link.clone());
        data.insert("link".into(), igdb_link);
    }
    if let Some(appid) = game.steam_appid {
        let steam_link = format!("https://store.steampowered.com/app/{}/", appid);
        data.insert("steam_link".into(), steam_link.clone());
        // If no IGDB link, use Steam as primary
        if !data.contains_key("link") {
            data.insert("link".into(), steam_link);
        }
    }

    build_ratings_data(&mut data, &game.ratings);

    data
}

pub fn build_app_data(app: &crate::models::Application) -> HashMap<String, String> {
    let mut data = HashMap::new();

    data.insert("nom".into(), app.name.clone());
    data.insert("nom_maj".into(), app.name.to_uppercase());
    if let Some(ref v) = app.version {
        data.insert("version".into(), v.clone());
    }
    if let Some(ref d) = app.developer {
        data.insert("developpeur".into(), d.clone());
    }
    if let Some(ref d) = app.description {
        if !d.is_empty() {
            data.insert("description".into(), d.clone());
        }
    }
    if let Some(ref w) = app.website {
        data.insert("site_web".into(), w.clone());
        data.insert("link".into(), w.clone());
    }
    if let Some(ref l) = app.license {
        data.insert("licence".into(), l.clone());
    }
    if !app.platforms.is_empty() {
        data.insert("plateformes".into(), app.platforms_display());
    }
    if let Some(ref url) = app.logo_url {
        data.insert("logo_url".into(), url.clone());
    }

    data
}

/// Inject MediaAnalysis data into the template data map.
pub fn build_media_analysis_data(data: &mut HashMap<String, String>, ma: &crate::models::MediaAnalysis) {
    data.insert("mi_format".into(), ma.format.clone());
    data.insert("mi_file_size".into(), ma.file_size.clone());
    if let Some(ref d) = ma.duration {
        data.insert("mi_duration".into(), d.clone());
    }
    if let Some(ref b) = ma.bitrate {
        data.insert("mi_bitrate".into(), b.clone());
    }
    if let Some(codec) = ma.video_codec() {
        data.insert("mi_video_codec".into(), codec.to_string());
    }
    if let Some(res) = ma.resolution() {
        data.insert("mi_resolution".into(), res);
    }
    if let Some(ref v) = ma.video.first() {
        if let Some(ref fps) = v.fps {
            data.insert("mi_video_fps".into(), fps.clone());
        }
    }
    let audio_langs = ma.audio_languages();
    if !audio_langs.is_empty() {
        data.insert("mi_audio_langs".into(), audio_langs);
    }
    let sub_langs = ma.subtitle_languages();
    if !sub_langs.is_empty() {
        data.insert("mi_subtitle_langs".into(), sub_langs);
    }
    data.insert("mi_audio_count".into(), ma.audio.len().to_string());
    data.insert("mi_subtitle_count".into(), ma.subtitles.len().to_string());
    // Flag for conditionals
    data.insert("has_mediainfo".into(), "true".into());
}

pub(crate) fn build_ratings_data(data: &mut HashMap<String, String>, ratings: &[crate::models::Rating]) {
    if !ratings.is_empty() {
        data.insert("has_ratings".into(), "true".into());
        data.insert("ratings_count".into(), ratings.len().to_string());
    }
    for (i, rating) in ratings.iter().enumerate() {
        let idx = i + 1;
        data.insert(format!("rating_{}_source", idx), rating.source.clone());
        data.insert(format!("rating_{}_value", idx), format!("{:.1}", rating.value));
        data.insert(format!("rating_{}_max", idx), format!("{}", rating.max as u32));
        data.insert(
            format!("rating_{}_display", idx),
            bbcode::colored_rating(rating.value, rating.max),
        );
    }
}

fn build_media_tech_data(data: &mut HashMap<String, String>, tech: &crate::models::MediaTechInfo) {
    if let Some(ref q) = tech.quality {
        data.insert("tech_qualite".into(), q.clone());
    }
    if let Some(ref c) = tech.video_codec {
        data.insert("tech_codec".into(), c.clone());
    }
    if let Some(ref a) = tech.audio {
        data.insert("tech_audio".into(), a.clone());
    }
    if let Some(ref l) = tech.language {
        data.insert("tech_langue".into(), l.clone());
    }
    if let Some(ref s) = tech.subtitles {
        data.insert("tech_soustitres".into(), s.clone());
    }
    if let Some(ref s) = tech.size {
        data.insert("tech_taille".into(), s.clone());
    }
}
