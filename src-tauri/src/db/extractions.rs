use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extraction {
    pub id: String,
    pub document_id: String,
    pub ai_provider: String,
    pub ai_model: Option<String>,
    pub contract_type: String,
    pub extracted_data: String,
    pub confidence_score: Option<f64>,
    pub processing_time_ms: Option<i64>,
    pub created_at: String,
}

pub struct CreateExtraction {
    pub document_id: String,
    pub ai_provider: String,
    pub ai_model: Option<String>,
    pub contract_type: String,
    pub extracted_data: String,
    pub confidence_score: Option<f64>,
    pub processing_time_ms: Option<i64>,
}

pub fn insert(conn: &Connection, ext: &CreateExtraction) -> AppResult<Extraction> {
    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO extractions (id, document_id, ai_provider, ai_model, contract_type, extracted_data, confidence_score, processing_time_ms)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![id, ext.document_id, ext.ai_provider, ext.ai_model, ext.contract_type, ext.extracted_data, ext.confidence_score, ext.processing_time_ms],
    )?;
    get_by_id(conn, &id)
}

pub fn get_by_id(conn: &Connection, id: &str) -> AppResult<Extraction> {
    conn.query_row(
        "SELECT id, document_id, ai_provider, ai_model, contract_type, extracted_data, confidence_score, processing_time_ms, created_at
         FROM extractions WHERE id = ?1",
        params![id],
        |row| {
            Ok(Extraction {
                id: row.get(0)?,
                document_id: row.get(1)?,
                ai_provider: row.get(2)?,
                ai_model: row.get(3)?,
                contract_type: row.get(4)?,
                extracted_data: row.get(5)?,
                confidence_score: row.get(6)?,
                processing_time_ms: row.get(7)?,
                created_at: row.get(8)?,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => AppError::NotFound(format!("Extraction {id} not found")),
        other => AppError::Database(other),
    })
}

pub fn list_by_document(conn: &Connection, document_id: &str) -> AppResult<Vec<Extraction>> {
    let mut stmt = conn.prepare(
        "SELECT id, document_id, ai_provider, ai_model, contract_type, extracted_data, confidence_score, processing_time_ms, created_at
         FROM extractions WHERE document_id = ?1 ORDER BY created_at DESC",
    )?;
    let results = stmt
        .query_map(params![document_id], |row| {
            Ok(Extraction {
                id: row.get(0)?,
                document_id: row.get(1)?,
                ai_provider: row.get(2)?,
                ai_model: row.get(3)?,
                contract_type: row.get(4)?,
                extracted_data: row.get(5)?,
                confidence_score: row.get(6)?,
                processing_time_ms: row.get(7)?,
                created_at: row.get(8)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use crate::db::documents;

    fn setup() -> Database {
        Database::in_memory().expect("Failed to create test DB")
    }

    fn insert_doc(conn: &Connection) -> String {
        let doc = documents::insert(conn, &documents::CreateDocument {
            filename: "test.pdf".into(),
            original_path: "/tmp/test.pdf".into(),
            stored_path: "/data/test.pdf".into(),
            file_hash: "hash123".into(),
            file_size: 1024,
            contract_type: "nda".into(),
        }).unwrap();
        doc.id
    }

    #[test]
    fn test_insert_and_get() {
        let db = setup();
        let conn = db.conn.lock().unwrap();
        let doc_id = insert_doc(&conn);

        let ext = insert(&conn, &CreateExtraction {
            document_id: doc_id.clone(),
            ai_provider: "ollama".into(),
            ai_model: Some("llama3".into()),
            contract_type: "nda".into(),
            extracted_data: r#"{"parties": ["A", "B"]}"#.into(),
            confidence_score: Some(0.85),
            processing_time_ms: Some(1500),
        }).unwrap();

        assert_eq!(ext.document_id, doc_id);
        assert_eq!(ext.ai_provider, "ollama");
    }

    #[test]
    fn test_list_by_document() {
        let db = setup();
        let conn = db.conn.lock().unwrap();
        let doc_id = insert_doc(&conn);

        insert(&conn, &CreateExtraction {
            document_id: doc_id.clone(),
            ai_provider: "ollama".into(),
            ai_model: None,
            contract_type: "nda".into(),
            extracted_data: "{}".into(),
            confidence_score: None,
            processing_time_ms: None,
        }).unwrap();

        let results = list_by_document(&conn, &doc_id).unwrap();
        assert_eq!(results.len(), 1);
    }
}
