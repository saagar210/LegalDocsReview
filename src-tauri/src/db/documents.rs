use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub filename: String,
    pub original_path: String,
    pub stored_path: String,
    pub file_hash: String,
    pub file_size: i64,
    pub contract_type: String,
    pub raw_text: Option<String>,
    pub page_count: Option<i32>,
    pub processing_status: String,
    pub error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateDocument {
    pub filename: String,
    pub original_path: String,
    pub stored_path: String,
    pub file_hash: String,
    pub file_size: i64,
    pub contract_type: String,
}

pub fn insert(conn: &Connection, doc: &CreateDocument) -> AppResult<Document> {
    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO documents (id, filename, original_path, stored_path, file_hash, file_size, contract_type)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![id, doc.filename, doc.original_path, doc.stored_path, doc.file_hash, doc.file_size, doc.contract_type],
    )?;
    get_by_id(conn, &id)
}

pub fn get_by_id(conn: &Connection, id: &str) -> AppResult<Document> {
    conn.query_row(
        "SELECT id, filename, original_path, stored_path, file_hash, file_size, contract_type,
                raw_text, page_count, processing_status, error_message, created_at, updated_at
         FROM documents WHERE id = ?1",
        params![id],
        |row| {
            Ok(Document {
                id: row.get(0)?,
                filename: row.get(1)?,
                original_path: row.get(2)?,
                stored_path: row.get(3)?,
                file_hash: row.get(4)?,
                file_size: row.get(5)?,
                contract_type: row.get(6)?,
                raw_text: row.get(7)?,
                page_count: row.get(8)?,
                processing_status: row.get(9)?,
                error_message: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => AppError::NotFound(format!("Document {id} not found")),
        other => AppError::Database(other),
    })
}

pub fn list_all(conn: &Connection) -> AppResult<Vec<Document>> {
    let mut stmt = conn.prepare(
        "SELECT id, filename, original_path, stored_path, file_hash, file_size, contract_type,
                raw_text, page_count, processing_status, error_message, created_at, updated_at
         FROM documents ORDER BY created_at DESC",
    )?;
    let docs = stmt
        .query_map([], |row| {
            Ok(Document {
                id: row.get(0)?,
                filename: row.get(1)?,
                original_path: row.get(2)?,
                stored_path: row.get(3)?,
                file_hash: row.get(4)?,
                file_size: row.get(5)?,
                contract_type: row.get(6)?,
                raw_text: row.get(7)?,
                page_count: row.get(8)?,
                processing_status: row.get(9)?,
                error_message: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(docs)
}

pub fn update_text(conn: &Connection, id: &str, raw_text: &str, page_count: i32) -> AppResult<()> {
    let rows = conn.execute(
        "UPDATE documents SET raw_text = ?1, page_count = ?2, processing_status = 'extracted',
                updated_at = datetime('now')
         WHERE id = ?3",
        params![raw_text, page_count, id],
    )?;
    if rows == 0 {
        return Err(AppError::NotFound(format!("Document {id} not found")));
    }
    Ok(())
}

pub fn update_status(conn: &Connection, id: &str, status: &str, error: Option<&str>) -> AppResult<()> {
    let rows = conn.execute(
        "UPDATE documents SET processing_status = ?1, error_message = ?2, updated_at = datetime('now')
         WHERE id = ?3",
        params![status, error, id],
    )?;
    if rows == 0 {
        return Err(AppError::NotFound(format!("Document {id} not found")));
    }
    Ok(())
}

pub fn delete(conn: &Connection, id: &str) -> AppResult<()> {
    let rows = conn.execute("DELETE FROM documents WHERE id = ?1", params![id])?;
    if rows == 0 {
        return Err(AppError::NotFound(format!("Document {id} not found")));
    }
    Ok(())
}

pub fn get_stats(conn: &Connection) -> AppResult<DocumentStats> {
    let total: i64 = conn.query_row("SELECT COUNT(*) FROM documents", [], |row| row.get(0))?;
    let analyzed: i64 = conn.query_row(
        "SELECT COUNT(*) FROM documents WHERE processing_status = 'analyzed'",
        [],
        |row| row.get(0),
    )?;
    let pending: i64 = conn.query_row(
        "SELECT COUNT(*) FROM documents WHERE processing_status IN ('pending', 'extracted')",
        [],
        |row| row.get(0),
    )?;
    let failed: i64 = conn.query_row(
        "SELECT COUNT(*) FROM documents WHERE processing_status = 'error'",
        [],
        |row| row.get(0),
    )?;
    Ok(DocumentStats {
        total,
        analyzed,
        pending,
        failed,
    })
}

#[derive(Debug, Serialize)]
pub struct DocumentStats {
    pub total: i64,
    pub analyzed: i64,
    pub pending: i64,
    pub failed: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    fn test_db() -> Database {
        Database::in_memory().expect("Failed to create test DB")
    }

    fn sample_create() -> CreateDocument {
        CreateDocument {
            filename: "test-nda.pdf".to_string(),
            original_path: "/tmp/test-nda.pdf".to_string(),
            stored_path: "/data/docs/abc.pdf".to_string(),
            file_hash: "abc123hash".to_string(),
            file_size: 1024,
            contract_type: "nda".to_string(),
        }
    }

    #[test]
    fn test_insert_and_get() {
        let db = test_db();
        let conn = db.conn.lock().unwrap();
        let doc = insert(&conn, &sample_create()).unwrap();
        assert_eq!(doc.filename, "test-nda.pdf");
        assert_eq!(doc.processing_status, "pending");

        let fetched = get_by_id(&conn, &doc.id).unwrap();
        assert_eq!(fetched.id, doc.id);
    }

    #[test]
    fn test_list_all() {
        let db = test_db();
        let conn = db.conn.lock().unwrap();
        insert(&conn, &sample_create()).unwrap();
        insert(&conn, &CreateDocument {
            filename: "lease.pdf".to_string(),
            ..sample_create()
        }).unwrap();

        let docs = list_all(&conn).unwrap();
        assert_eq!(docs.len(), 2);
    }

    #[test]
    fn test_update_text() {
        let db = test_db();
        let conn = db.conn.lock().unwrap();
        let doc = insert(&conn, &sample_create()).unwrap();

        update_text(&conn, &doc.id, "Extracted text content", 3).unwrap();
        let updated = get_by_id(&conn, &doc.id).unwrap();
        assert_eq!(updated.raw_text.as_deref(), Some("Extracted text content"));
        assert_eq!(updated.page_count, Some(3));
        assert_eq!(updated.processing_status, "extracted");
    }

    #[test]
    fn test_update_status() {
        let db = test_db();
        let conn = db.conn.lock().unwrap();
        let doc = insert(&conn, &sample_create()).unwrap();

        update_status(&conn, &doc.id, "error", Some("PDF corrupted")).unwrap();
        let updated = get_by_id(&conn, &doc.id).unwrap();
        assert_eq!(updated.processing_status, "error");
        assert_eq!(updated.error_message.as_deref(), Some("PDF corrupted"));
    }

    #[test]
    fn test_delete() {
        let db = test_db();
        let conn = db.conn.lock().unwrap();
        let doc = insert(&conn, &sample_create()).unwrap();
        delete(&conn, &doc.id).unwrap();
        assert!(get_by_id(&conn, &doc.id).is_err());
    }

    #[test]
    fn test_get_stats() {
        let db = test_db();
        let conn = db.conn.lock().unwrap();
        insert(&conn, &sample_create()).unwrap();
        let stats = get_stats(&conn).unwrap();
        assert_eq!(stats.total, 1);
        assert_eq!(stats.pending, 1);
    }

    #[test]
    fn test_not_found() {
        let db = test_db();
        let conn = db.conn.lock().unwrap();
        let result = get_by_id(&conn, "nonexistent");
        assert!(result.is_err());
    }
}
