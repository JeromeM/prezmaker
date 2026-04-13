mod bbcode_to_html;
mod commands;

use commands::{process_queue_item, AppState};
use prezmaker_lib::cache::ApiCache;
use prezmaker_lib::config::Config;
use prezmaker_lib::db::Database;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::Manager;

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

    // Au démarrage, remettre les items "in_progress" à "queued" (récupération crash)
    if let Ok(n) = db.reset_stale_in_progress() {
        if n > 0 {
            eprintln!("Reset {} stale in_progress queue item(s) to queued", n);
        }
    }

    let upload_in_progress = Arc::new(AtomicBool::new(false));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_updater::Builder::new().build())?;

            // Spawn le scheduler tokio en arrière-plan : poll toutes les 30s
            // pour traiter les items dont scheduled_at <= now.
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state: tauri::State<'_, AppState> = app_handle.state();
                let db = state.db.clone();
                let config = state.config.clone();
                let upload_lock = state.upload_in_progress.clone();
                drop(state);

                let mut interval = tokio::time::interval(Duration::from_secs(30));
                interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
                loop {
                    interval.tick().await;

                    // Skip si un upload est déjà en cours
                    if upload_lock.load(Ordering::SeqCst) {
                        continue;
                    }

                    let now_iso = chrono::Utc::now().to_rfc3339();
                    let pending = match db.get_pending_scheduled(&now_iso) {
                        Ok(items) if !items.is_empty() => items,
                        _ => continue,
                    };

                    // Acquérir le verrou pour le batch entier
                    if upload_lock
                        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                        .is_err()
                    {
                        continue;
                    }

                    for item in pending {
                        process_queue_item(&item, &db, &config, &app_handle).await;
                    }

                    upload_lock.store(false, Ordering::SeqCst);
                }
            });

            Ok(())
        })
        .manage(AppState {
            config: Arc::new(Mutex::new(config)),
            cache: ApiCache::new(),
            db,
            upload_in_progress,
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
            commands::rename_content_template,
            commands::duplicate_content_template,
            commands::reorder_content_templates,
            commands::get_template_tags,
            commands::generate_from_template,
            commands::convert_bbcode,
            commands::convert_bbcode_c411,
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
            commands::c411_fetch_tmdb_metadata,
            commands::c411_fetch_rawg_metadata,
            commands::queue_add,
            commands::queue_list,
            commands::queue_remove,
            commands::queue_retry,
            commands::queue_clear_completed,
            commands::queue_count,
            commands::queue_set_schedule,
            commands::queue_reorder,
            commands::queue_process_all,
            commands::queue_process_one,
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
