pub mod error;
pub mod db;
pub mod ai;
pub mod documents;
pub mod analysis;
pub mod reports;
pub mod commands;

use tauri::Manager;

use db::Database;

use commands::document_commands::*;
use commands::settings_commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Initialize database in app data directory
            let app_data = app.handle().path().app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&app_data)
                .expect("failed to create app data directory");

            let db_path = app_data.join("legal_docs_review.db");
            let database = Database::new(&db_path)
                .expect("failed to initialize database");

            app.manage(database);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            upload_document,
            extract_document_text,
            get_document,
            list_documents,
            delete_document,
            get_document_stats,
            get_setting,
            set_setting,
            get_all_settings,
            delete_setting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
