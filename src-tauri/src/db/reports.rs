use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: String,
    pub document_id: String,
    pub report_type: String,
    pub content: String,
    pub export_path: Option<String>,
    pub format: String,
    pub created_at: String,
}

pub fn insert(
    conn: &Connection,
    document_id: &str,
    report_type: &str,
    content: &str,
    format: &str,
) -> AppResult<Report> {
    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO reports (id, document_id, report_type, content, format) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, document_id, report_type, content, format],
    )?;
    get_by_id(conn, &id)
}

pub fn get_by_id(conn: &Connection, id: &str) -> AppResult<Report> {
    conn.query_row(
        "SELECT id, document_id, report_type, content, export_path, format, created_at
         FROM reports WHERE id = ?1",
        params![id],
        |row| {
            Ok(Report {
                id: row.get(0)?,
                document_id: row.get(1)?,
                report_type: row.get(2)?,
                content: row.get(3)?,
                export_path: row.get(4)?,
                format: row.get(5)?,
                created_at: row.get(6)?,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => AppError::NotFound(format!("Report {id} not found")),
        other => AppError::Database(other),
    })
}

pub fn list_by_document(conn: &Connection, document_id: &str) -> AppResult<Vec<Report>> {
    let mut stmt = conn.prepare(
        "SELECT id, document_id, report_type, content, export_path, format, created_at
         FROM reports WHERE document_id = ?1 ORDER BY created_at DESC",
    )?;
    let results = stmt
        .query_map(params![document_id], |row| {
            Ok(Report {
                id: row.get(0)?,
                document_id: row.get(1)?,
                report_type: row.get(2)?,
                content: row.get(3)?,
                export_path: row.get(4)?,
                format: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(results)
}
