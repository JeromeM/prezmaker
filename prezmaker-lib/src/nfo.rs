use crate::models::{Application, Game, MediaAnalysis, Movie, Series};

const LINE_WIDTH: usize = 60;

fn separator() -> String {
    "=".repeat(LINE_WIDTH)
}

fn center_text(text: &str) -> String {
    if text.len() >= LINE_WIDTH {
        return text.to_string();
    }
    let pad = (LINE_WIDTH - text.len()) / 2;
    format!("{}{}", " ".repeat(pad), text)
}

fn dotted_field(label: &str, value: &str) -> String {
    let min_dots = 3;
    let target = 22; // label + dots total width
    let dots = if label.len() + min_dots < target {
        target - label.len()
    } else {
        min_dots
    };
    format!("  {} {} : {}", label, ".".repeat(dots), value)
}

fn wrap_text(text: &str, width: usize, indent: usize) -> String {
    let prefix = " ".repeat(indent);
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        if paragraph.trim().is_empty() {
            lines.push(String::new());
            continue;
        }
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            if line.is_empty() {
                line = format!("{}{}", prefix, word);
            } else if line.len() + 1 + word.len() > width {
                lines.push(line);
                line = format!("{}{}", prefix, word);
            } else {
                line.push(' ');
                line.push_str(word);
            }
        }
        if !line.is_empty() {
            lines.push(line);
        }
    }
    lines.join("\n")
}

fn section_header(title: &str) -> String {
    format!(
        "{}\n{}\n{}",
        separator(),
        center_text(&title.to_uppercase()),
        separator()
    )
}

fn title_banner(title: &str) -> String {
    let upper = title.to_uppercase();
    format!(
        "{}\n{}\n{}",
        separator(),
        center_text(&upper),
        separator()
    )
}

// --- Game NFO ---

pub fn generate_game_nfo(game: &Game, pseudo: &str) -> String {
    let mut out = String::new();

    // Title banner
    out.push_str(&title_banner(&game.title));
    out.push_str("\n\n");

    // Info section
    out.push_str(&section_header("Informations"));
    out.push('\n');
    out.push_str(&dotted_field("Type", "Jeu"));
    out.push('\n');
    if !game.genres.is_empty() {
        out.push_str(&dotted_field("Genre", &game.genres_display()));
        out.push('\n');
    }
    if let Some(year) = game.year {
        out.push_str(&dotted_field("Annee", &year.to_string()));
        out.push('\n');
    }
    if let Some(ref date) = game.release_date {
        out.push_str(&dotted_field("Date de sortie", date));
        out.push('\n');
    }
    if !game.platforms.is_empty() {
        out.push_str(&dotted_field("Plateforme", &game.platforms_display()));
        out.push('\n');
    }
    if !game.developers.is_empty() {
        out.push_str(&dotted_field("Developpeur", &game.developers_display()));
        out.push('\n');
    }
    if !game.publishers.is_empty() {
        out.push_str(&dotted_field("Editeur", &game.publishers_display()));
        out.push('\n');
    }
    if let Some(ref tech) = game.tech_info {
        if !tech.languages.is_empty() {
            out.push_str(&dotted_field("Langue", &tech.languages));
            out.push('\n');
        }
    }
    out.push('\n');

    // Description
    if let Some(ref synopsis) = game.synopsis {
        out.push_str(&section_header("Description"));
        out.push('\n');
        out.push_str(&wrap_text(synopsis, LINE_WIDTH, 2));
        out.push_str("\n\n");
    }

    // System Requirements
    let has_reqs = game.min_reqs.as_ref().map_or(false, |r| !r.is_empty())
        || game.rec_reqs.as_ref().map_or(false, |r| !r.is_empty());
    if has_reqs {
        out.push_str(&section_header("Configuration requise"));
        out.push('\n');
        if let Some(ref min) = game.min_reqs {
            if !min.is_empty() {
                out.push_str("  [ Minimum ]\n");
                if !min.os.is_empty() {
                    out.push_str(&dotted_field("OS", &min.os));
                    out.push('\n');
                }
                if !min.cpu.is_empty() {
                    out.push_str(&dotted_field("CPU", &min.cpu));
                    out.push('\n');
                }
                if !min.ram.is_empty() {
                    out.push_str(&dotted_field("RAM", &min.ram));
                    out.push('\n');
                }
                if !min.gpu.is_empty() {
                    out.push_str(&dotted_field("GPU", &min.gpu));
                    out.push('\n');
                }
                if !min.storage.is_empty() {
                    out.push_str(&dotted_field("Stockage", &min.storage));
                    out.push('\n');
                }
                out.push('\n');
            }
        }
        if let Some(ref rec) = game.rec_reqs {
            if !rec.is_empty() {
                out.push_str("  [ Recommande ]\n");
                if !rec.os.is_empty() {
                    out.push_str(&dotted_field("OS", &rec.os));
                    out.push('\n');
                }
                if !rec.cpu.is_empty() {
                    out.push_str(&dotted_field("CPU", &rec.cpu));
                    out.push('\n');
                }
                if !rec.ram.is_empty() {
                    out.push_str(&dotted_field("RAM", &rec.ram));
                    out.push('\n');
                }
                if !rec.gpu.is_empty() {
                    out.push_str(&dotted_field("GPU", &rec.gpu));
                    out.push('\n');
                }
                if !rec.storage.is_empty() {
                    out.push_str(&dotted_field("Stockage", &rec.storage));
                    out.push('\n');
                }
                out.push('\n');
            }
        }
    }

    // Tech info
    if let Some(ref tech) = game.tech_info {
        out.push_str(&section_header("Fiche technique"));
        out.push('\n');
        if !tech.platform.is_empty() {
            out.push_str(&dotted_field("Plateforme", &tech.platform));
            out.push('\n');
        }
        if !tech.size.is_empty() {
            out.push_str(&dotted_field("Taille", &tech.size));
            out.push('\n');
        }
        if !tech.install_size.is_empty() {
            out.push_str(&dotted_field("Taille installee", &tech.install_size));
            out.push('\n');
        }
        out.push('\n');
    }

    // Installation
    if let Some(ref install) = game.installation {
        if !install.is_empty() {
            out.push_str(&section_header("Installation"));
            out.push('\n');
            out.push_str(&wrap_text(install, LINE_WIDTH, 2));
            out.push_str("\n\n");
        }
    }

    // Upload info
    out.push_str(&section_header("Upload"));
    out.push('\n');
    out.push_str(&dotted_field("Uploader", pseudo));
    out.push('\n');
    out.push_str(&separator());
    out.push('\n');

    out
}

// --- App NFO ---

pub fn generate_app_nfo(app: &Application, pseudo: &str) -> String {
    let mut out = String::new();

    out.push_str(&title_banner(&app.name));
    out.push_str("\n\n");

    out.push_str(&section_header("Informations"));
    out.push('\n');
    out.push_str(&dotted_field("Type", "Application"));
    out.push('\n');
    out.push_str(&dotted_field("Nom", &app.name));
    out.push('\n');
    if let Some(ref version) = app.version {
        out.push_str(&dotted_field("Version", version));
        out.push('\n');
    }
    if let Some(ref dev) = app.developer {
        out.push_str(&dotted_field("Developpeur", dev));
        out.push('\n');
    }
    if let Some(ref license) = app.license {
        out.push_str(&dotted_field("Licence", license));
        out.push('\n');
    }
    if let Some(ref website) = app.website {
        out.push_str(&dotted_field("Site web", website));
        out.push('\n');
    }
    if !app.platforms.is_empty() {
        out.push_str(&dotted_field("Plateformes", &app.platforms_display()));
        out.push('\n');
    }
    out.push('\n');

    if let Some(ref desc) = app.description {
        if !desc.is_empty() {
            out.push_str(&section_header("Description"));
            out.push('\n');
            out.push_str(&wrap_text(desc, LINE_WIDTH, 2));
            out.push_str("\n\n");
        }
    }

    out.push_str(&section_header("Upload"));
    out.push('\n');
    out.push_str(&dotted_field("Uploader", pseudo));
    out.push('\n');
    out.push_str(&separator());
    out.push('\n');

    out
}

// --- Movie NFO ---

pub fn generate_movie_nfo(
    movie: &Movie,
    media_analysis: Option<&MediaAnalysis>,
    pseudo: &str,
) -> String {
    let mut out = String::new();

    out.push_str(&title_banner(&movie.title));
    out.push_str("\n\n");

    // Info section
    out.push_str(&section_header("Infos sur le film"));
    out.push('\n');
    out.push_str(&dotted_field("Type", "Film"));
    out.push('\n');
    if let Some(year) = movie.year {
        out.push_str(&dotted_field("Annee", &year.to_string()));
        out.push('\n');
    }
    if !movie.directors.is_empty() {
        out.push_str(&dotted_field("Realisateur", &movie.directors_display()));
        out.push('\n');
    }
    if !movie.genres.is_empty() {
        out.push_str(&dotted_field("Genre", &movie.genres_display()));
        out.push('\n');
    }
    if !movie.countries.is_empty() {
        out.push_str(&dotted_field("Pays", &movie.countries_display()));
        out.push('\n');
    }
    if let Some(ref dur) = movie.duration_formatted() {
        out.push_str(&dotted_field("Duree", dur));
        out.push('\n');
    }
    if !movie.cast.is_empty() {
        out.push_str(&dotted_field("Acteurs", &movie.cast_display(6)));
        out.push('\n');
    }
    out.push('\n');

    // Synopsis
    if let Some(ref synopsis) = movie.synopsis {
        if !synopsis.is_empty() {
            out.push_str(&section_header("Resume"));
            out.push('\n');
            out.push_str(&wrap_text(synopsis, LINE_WIDTH, 2));
            out.push_str("\n\n");
        }
    }

    // Fiche technique (MediaInfo)
    if let Some(ma) = media_analysis {
        out.push_str(&section_header("Fiche technique"));
        out.push('\n');
        out.push_str(&dotted_field("Fichier", &ma.file_name));
        out.push('\n');
        out.push_str(&dotted_field("Taille", &ma.file_size));
        out.push('\n');
        out.push_str(&dotted_field("Format", &ma.format));
        out.push('\n');
        if let Some(ref dur) = ma.duration {
            out.push_str(&dotted_field("Duree", dur));
            out.push('\n');
        }
        if let Some(ref br) = ma.bitrate {
            out.push_str(&dotted_field("Debit", br));
            out.push('\n');
        }
        out.push('\n');

        // Video tracks
        if !ma.video.is_empty() {
            out.push_str("  [ Video ]\n");
            for (i, v) in ma.video.iter().enumerate() {
                let label = if ma.video.len() > 1 {
                    format!("Piste {}", i + 1)
                } else {
                    "Codec".to_string()
                };
                out.push_str(&dotted_field(&label, &v.codec));
                out.push('\n');
                out.push_str(&dotted_field("Resolution", &format!("{}x{}", v.width, v.height)));
                out.push('\n');
                if let Some(ref fps) = v.fps {
                    out.push_str(&dotted_field("FPS", fps));
                    out.push('\n');
                }
                if let Some(ref br) = v.bitrate {
                    out.push_str(&dotted_field("Debit video", br));
                    out.push('\n');
                }
            }
            out.push('\n');
        }

        // Audio tracks
        if !ma.audio.is_empty() {
            out.push_str("  [ Audio ]\n");
            for (i, a) in ma.audio.iter().enumerate() {
                let lang = a.language.as_deref().unwrap_or("?");
                out.push_str(&dotted_field(
                    &format!("Piste {}", i + 1),
                    &format!("{} {} ({})", a.codec, a.channels, lang),
                ));
                out.push('\n');
            }
            out.push('\n');
        }

        // Subtitles
        if !ma.subtitles.is_empty() {
            out.push_str("  [ Sous-titres ]\n");
            for (i, s) in ma.subtitles.iter().enumerate() {
                let lang = s.language.as_deref().unwrap_or("?");
                out.push_str(&dotted_field(
                    &format!("Piste {}", i + 1),
                    &format!("{} ({})", s.format, lang),
                ));
                out.push('\n');
            }
            out.push('\n');
        }

        // Raw MediaInfo
        if !ma.raw_text.is_empty() {
            out.push_str(&section_header("MediaInfo"));
            out.push('\n');
            out.push_str(&ma.raw_text);
            out.push_str("\n\n");
        }
    }

    // Upload info
    out.push_str(&section_header("Upload"));
    out.push('\n');
    out.push_str(&dotted_field("Uploader", pseudo));
    out.push('\n');
    out.push_str(&separator());
    out.push('\n');

    out
}

// --- Series NFO ---

pub fn generate_series_nfo(
    series: &Series,
    media_analysis: Option<&MediaAnalysis>,
    pseudo: &str,
) -> String {
    let mut out = String::new();

    out.push_str(&title_banner(&series.title));
    out.push_str("\n\n");

    // Info section
    out.push_str(&section_header("Infos sur la serie"));
    out.push('\n');
    out.push_str(&dotted_field("Type", "Serie"));
    out.push('\n');
    if series.year.is_some() {
        out.push_str(&dotted_field("Annee", &series.year_display()));
        out.push('\n');
    }
    if !series.creators.is_empty() {
        out.push_str(&dotted_field("Createur(s)", &series.creators_display()));
        out.push('\n');
    }
    if !series.genres.is_empty() {
        out.push_str(&dotted_field("Genre", &series.genres_display()));
        out.push('\n');
    }
    if !series.countries.is_empty() {
        out.push_str(&dotted_field("Pays", &series.countries_display()));
        out.push('\n');
    }
    if let Some(seasons) = series.seasons_count {
        out.push_str(&dotted_field("Saisons", &seasons.to_string()));
        out.push('\n');
    }
    if let Some(episodes) = series.episodes_count {
        out.push_str(&dotted_field("Episodes", &episodes.to_string()));
        out.push('\n');
    }
    if let Some(ref runtime) = series.runtime_formatted() {
        out.push_str(&dotted_field("Duree par episode", runtime));
        out.push('\n');
    }
    if !series.networks.is_empty() {
        out.push_str(&dotted_field("Chaine", &series.networks_display()));
        out.push('\n');
    }
    if let Some(ref status) = series.status {
        out.push_str(&dotted_field("Statut", status));
        out.push('\n');
    }
    if !series.cast.is_empty() {
        out.push_str(&dotted_field("Acteurs", &series.cast_display(6)));
        out.push('\n');
    }
    out.push('\n');

    // Synopsis
    if let Some(ref synopsis) = series.synopsis {
        if !synopsis.is_empty() {
            out.push_str(&section_header("Resume"));
            out.push('\n');
            out.push_str(&wrap_text(synopsis, LINE_WIDTH, 2));
            out.push_str("\n\n");
        }
    }

    // Fiche technique (MediaInfo) — same as movie
    if let Some(ma) = media_analysis {
        out.push_str(&section_header("Fiche technique"));
        out.push('\n');
        out.push_str(&dotted_field("Fichier", &ma.file_name));
        out.push('\n');
        out.push_str(&dotted_field("Taille", &ma.file_size));
        out.push('\n');
        out.push_str(&dotted_field("Format", &ma.format));
        out.push('\n');
        if let Some(ref dur) = ma.duration {
            out.push_str(&dotted_field("Duree", dur));
            out.push('\n');
        }
        if let Some(ref br) = ma.bitrate {
            out.push_str(&dotted_field("Debit", br));
            out.push('\n');
        }
        out.push('\n');

        if !ma.video.is_empty() {
            out.push_str("  [ Video ]\n");
            for (i, v) in ma.video.iter().enumerate() {
                let label = if ma.video.len() > 1 {
                    format!("Piste {}", i + 1)
                } else {
                    "Codec".to_string()
                };
                out.push_str(&dotted_field(&label, &v.codec));
                out.push('\n');
                out.push_str(&dotted_field("Resolution", &format!("{}x{}", v.width, v.height)));
                out.push('\n');
                if let Some(ref fps) = v.fps {
                    out.push_str(&dotted_field("FPS", fps));
                    out.push('\n');
                }
                if let Some(ref br) = v.bitrate {
                    out.push_str(&dotted_field("Debit video", br));
                    out.push('\n');
                }
            }
            out.push('\n');
        }

        if !ma.audio.is_empty() {
            out.push_str("  [ Audio ]\n");
            for (i, a) in ma.audio.iter().enumerate() {
                let lang = a.language.as_deref().unwrap_or("?");
                out.push_str(&dotted_field(
                    &format!("Piste {}", i + 1),
                    &format!("{} {} ({})", a.codec, a.channels, lang),
                ));
                out.push('\n');
            }
            out.push('\n');
        }

        if !ma.subtitles.is_empty() {
            out.push_str("  [ Sous-titres ]\n");
            for (i, s) in ma.subtitles.iter().enumerate() {
                let lang = s.language.as_deref().unwrap_or("?");
                out.push_str(&dotted_field(
                    &format!("Piste {}", i + 1),
                    &format!("{} ({})", s.format, lang),
                ));
                out.push('\n');
            }
            out.push('\n');
        }

        if !ma.raw_text.is_empty() {
            out.push_str(&section_header("MediaInfo"));
            out.push('\n');
            out.push_str(&ma.raw_text);
            out.push_str("\n\n");
        }
    }

    // Upload info
    out.push_str(&section_header("Upload"));
    out.push('\n');
    out.push_str(&dotted_field("Uploader", pseudo));
    out.push('\n');
    out.push_str(&separator());
    out.push('\n');

    out
}
