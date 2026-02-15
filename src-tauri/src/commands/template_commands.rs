use tauri::State;

use crate::db::Database;
use crate::db::templates::{self, Template};
use crate::error::AppResult;

#[tauri::command]
pub async fn create_template(
    db: State<'_, Database>,
    name: String,
    contract_type: String,
    description: Option<String>,
    raw_text: String,
) -> AppResult<Template> {
    let conn = db.conn.lock().expect("db lock poisoned");
    templates::insert(&conn, &name, &contract_type, description.as_deref(), &raw_text)
}

#[tauri::command]
pub async fn list_templates(db: State<'_, Database>) -> AppResult<Vec<Template>> {
    let conn = db.conn.lock().expect("db lock poisoned");
    templates::list_all(&conn)
}

#[tauri::command]
pub async fn delete_template(
    db: State<'_, Database>,
    template_id: String,
) -> AppResult<()> {
    let conn = db.conn.lock().expect("db lock poisoned");
    templates::delete(&conn, &template_id)
}
