mod bbcode_to_html;
mod commands;

use commands::AppState;
use prezmaker_lib::config::Config;
use std::sync::{Arc, Mutex};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = Config::load(None).unwrap_or_else(|e| {
        eprintln!("Config error: {}, using defaults", e);
        Config::default()
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            config: Arc::new(Mutex::new(config)),
        })
        .invoke_handler(tauri::generate_handler![
            commands::search,
            commands::generate_film,
            commands::generate_serie,
            commands::parse_torrent,
            commands::generate_film_with_tech,
            commands::generate_serie_with_tech,
            commands::fetch_game_details,
            commands::generate_jeu,
            commands::generate_app,
            commands::convert_bbcode,
            commands::generate_nfo,
            commands::get_settings,
            commands::save_settings,
            commands::list_templates,
            commands::load_template,
            commands::save_template,
            commands::delete_template,
            commands::rename_template,
            commands::duplicate_template,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
