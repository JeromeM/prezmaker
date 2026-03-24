mod bbcode_to_html;
mod commands;

use commands::AppState;
use prezmaker_lib::cache::ApiCache;
use prezmaker_lib::config::Config;
use prezmaker_lib::db::Database;
use std::sync::{Arc, Mutex};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = Config::load(None).unwrap_or_else(|e| {
        eprintln!("Config error: {}, using defaults", e);
        Config::default()
    });

    let db = Database::open().expect("Cannot open database");

    // Migrate old JSON data to SQLite on first run
    match db.migrate_from_json() {
        Ok(true) => eprintln!("Migrated existing data to SQLite database"),
        Ok(false) => {}
        Err(e) => eprintln!("Migration warning: {}", e),
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_updater::Builder::new().build())?;
            Ok(())
        })
        .manage(AppState {
            config: Arc::new(Mutex::new(config)),
            cache: ApiCache::new(),
            db,
        })
        .invoke_handler(tauri::generate_handler![
            commands::search,
            commands::generate_film,
            commands::generate_serie,
            commands::parse_torrent,
            commands::create_torrent,
            commands::generate_film_with_tech,
            commands::generate_serie_with_tech,
            commands::fetch_game_details,
            commands::fetch_steam_requirements,
            commands::generate_jeu,
            commands::generate_app,
            commands::preview_template,
            commands::list_content_templates,
            commands::get_content_template,
            commands::save_content_template,
            commands::delete_content_template,
            commands::duplicate_content_template,
            commands::reorder_content_templates,
            commands::get_template_tags,
            commands::generate_from_template,
            commands::convert_bbcode,
            commands::save_text_file,
            commands::run_mediainfo,
            commands::analyze_media,
            commands::get_settings,
            commands::save_settings,
            commands::set_default_template,
            commands::get_default_template,
            commands::list_templates,
            commands::load_template,
            commands::save_template,
            commands::delete_template,
            commands::rename_template,
            commands::duplicate_template,
            commands::export_template,
            commands::import_template,
            commands::c411_fetch_categories,
            commands::c411_fetch_options,
            commands::c411_auto_map,
            commands::c411_upload,
            commands::fetch_release_notes,
            commands::list_recent_presentations,
            commands::get_dashboard_stats,
            commands::auto_save_presentation,
            commands::create_collection,
            commands::list_collections,
            commands::rename_collection,
            commands::delete_collection,
            commands::save_to_collection,
            commands::list_collection,
            commands::get_collection_entry,
            commands::delete_collection_entry,
            commands::move_collection_entry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
