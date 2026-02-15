use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comparison {
    pub id: String,
    pub document_a_id: String,
    pub document_b_id: Option<String>,
    pub template_id: Option<String>,
    pub comparison_type: String,
    pub differences: String,
    pub summary: Option<String>,
    pub ai_provider: Option<String>,
    pub created_at: String,
}

pub struct CreateComparison<'a> {
    pub document_a_id: &'a str,
    pub document_b_id: Option<&'a str>,
    pub template_id: Option<&'a str>,
    pub comparison_type: &'a str,
    pub differences: &'a str,
    pub summary: Option<&'a str>,
    pub ai_provider: Option<&'a str>,
}

pub fn insert(conn: &Connection, comparison: &CreateComparison<'_>) -> AppResult<Comparison> {
    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO comparisons (id, document_a_id, document_b_id, template_id, comparison_type, differences, summary, ai_provider)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            id,
            comparison.document_a_id,
            comparison.document_b_id,
            comparison.template_id,
            comparison.comparison_type,
            comparison.differences,
            comparison.summary,
            comparison.ai_provider
        ],
    )?;
    get_by_id(conn, &id)
}

fn get_by_id(conn: &Connection, id: &str) -> AppResult<Comparison> {
    conn.query_row(
        "SELECT id, document_a_id, document_b_id, template_id, comparison_type, differences, summary, ai_provider, created_at
         FROM comparisons WHERE id = ?1",
        params![id],
        |row| {
            Ok(Comparison {
                id: row.get(0)?,
                document_a_id: row.get(1)?,
                document_b_id: row.get(2)?,
                template_id: row.get(3)?,
                comparison_type: row.get(4)?,
                differences: row.get(5)?,
                summary: row.get(6)?,
                ai_provider: row.get(7)?,
                created_at: row.get(8)?,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => AppError::NotFound(format!("Comparison {id} not found")),
        other => AppError::Database(other),
    })
}
