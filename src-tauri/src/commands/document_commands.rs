use std::path::PathBuf;
use tauri::{Manager, State};

use crate::db::Database;
use crate::db::documents::{self, CreateDocument, Document, DocumentStats};
use crate::documents::{compute_file_hash, get_file_size};
use crate::documents::pdf;
use crate::error::AppResult;

#[tauri::command]
pub async fn upload_document(
    db: State<'_, Database>,
    file_path: String,
    contract_type: String,
    app_handle: tauri::AppHandle,
) -> AppResult<Document> {
    let source = PathBuf::from(&file_path);

    let filename = source
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown.pdf".to_string());

    let file_hash = compute_file_hash(&source)?;
    let file_size = get_file_size(&source)?;

    // Store document in app data directory
    let app_data = app_handle
        .path()
        .app_data_dir()
        .expect("failed to get app data dir");
    let docs_dir = app_data.join("documents");
    std::fs::create_dir_all(&docs_dir)?;

    let stored_name = format!("{}_{}", &file_hash[..8], &filename);
    let stored_path = docs_dir.join(&stored_name);
    std::fs::copy(&source, &stored_path)?;

    let conn = db.conn.lock().expect("db lock poisoned");
    let doc = documents::insert(
        &conn,
        &CreateDocument {
            filename,
            original_path: file_path,
            stored_path: stored_path.to_string_lossy().to_string(),
            file_hash,
            file_size,
            contract_type,
        },
    )?;

    Ok(doc)
}

#[tauri::command]
pub async fn extract_document_text(
    db: State<'_, Database>,
    document_id: String,
) -> AppResult<Document> {
    let stored_path = {
        let conn = db.conn.lock().expect("db lock poisoned");
        let doc = documents::get_by_id(&conn, &document_id)?;
        doc.stored_path
    };

    let result = pdf::extract_text(&PathBuf::from(&stored_path));

    let conn = db.conn.lock().expect("db lock poisoned");
    match result {
        Ok(extraction) => {
            documents::update_text(&conn, &document_id, &extraction.text, extraction.page_count)?;
        }
        Err(e) => {
            documents::update_status(&conn, &document_id, "error", Some(&e.to_string()))?;
            return Err(e);
        }
    }

    documents::get_by_id(&conn, &document_id)
}

#[tauri::command]
pub async fn get_document(
    db: State<'_, Database>,
    document_id: String,
) -> AppResult<Document> {
    let conn = db.conn.lock().expect("db lock poisoned");
    documents::get_by_id(&conn, &document_id)
}

#[tauri::command]
pub async fn list_documents(db: State<'_, Database>) -> AppResult<Vec<Document>> {
    let conn = db.conn.lock().expect("db lock poisoned");
    documents::list_all(&conn)
}

#[tauri::command]
pub async fn delete_document(
    db: State<'_, Database>,
    document_id: String,
) -> AppResult<()> {
    let conn = db.conn.lock().expect("db lock poisoned");
    let doc = documents::get_by_id(&conn, &document_id)?;

    // Remove stored file
    let stored = PathBuf::from(&doc.stored_path);
    if stored.exists() {
        std::fs::remove_file(&stored)?;
    }

    documents::delete(&conn, &document_id)
}

#[tauri::command]
pub async fn get_document_stats(db: State<'_, Database>) -> AppResult<DocumentStats> {
    let conn = db.conn.lock().expect("db lock poisoned");
    documents::get_stats(&conn)
}
