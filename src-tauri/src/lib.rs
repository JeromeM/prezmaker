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
        .manage(AppState {
            config: Arc::new(Mutex::new(config)),
        })
        .invoke_handler(tauri::generate_handler![
            commands::search,
            commands::generate_film,
            commands::generate_serie,
            commands::fetch_game_details,
            commands::generate_jeu,
            commands::generate_app,
            commands::convert_bbcode,
            commands::get_settings,
            commands::save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
