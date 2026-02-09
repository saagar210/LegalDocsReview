use tauri::State;

use crate::db::Database;
use crate::db::settings;
use crate::error::AppResult;

#[tauri::command]
pub async fn get_setting(
    db: State<'_, Database>,
    key: String,
) -> AppResult<Option<String>> {
    let conn = db.conn.lock().expect("db lock poisoned");
    settings::get(&conn, &key)
}

#[tauri::command]
pub async fn set_setting(
    db: State<'_, Database>,
    key: String,
    value: String,
) -> AppResult<()> {
    let conn = db.conn.lock().expect("db lock poisoned");
    settings::set(&conn, &key, &value)
}

#[tauri::command]
pub async fn get_all_settings(
    db: State<'_, Database>,
) -> AppResult<Vec<(String, String)>> {
    let conn = db.conn.lock().expect("db lock poisoned");
    settings::get_all(&conn)
}

#[tauri::command]
pub async fn delete_setting(
    db: State<'_, Database>,
    key: String,
) -> AppResult<()> {
    let conn = db.conn.lock().expect("db lock poisoned");
    settings::delete(&conn, &key)
}
