use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub contract_type: String,
    pub description: Option<String>,
    pub raw_text: String,
    pub extracted_data: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn insert(conn: &Connection, name: &str, contract_type: &str, description: Option<&str>, raw_text: &str) -> AppResult<Template> {
    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO templates (id, name, contract_type, description, raw_text) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, name, contract_type, description, raw_text],
    )?;
    get_by_id(conn, &id)
}

pub fn get_by_id(conn: &Connection, id: &str) -> AppResult<Template> {
    conn.query_row(
        "SELECT id, name, contract_type, description, raw_text, extracted_data, created_at, updated_at
         FROM templates WHERE id = ?1",
        params![id],
        |row| {
            Ok(Template {
                id: row.get(0)?,
                name: row.get(1)?,
                contract_type: row.get(2)?,
                description: row.get(3)?,
                raw_text: row.get(4)?,
                extracted_data: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => AppError::NotFound(format!("Template {id} not found")),
        other => AppError::Database(other),
    })
}

pub fn list_all(conn: &Connection) -> AppResult<Vec<Template>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, contract_type, description, raw_text, extracted_data, created_at, updated_at
         FROM templates ORDER BY name",
    )?;
    let results = stmt
        .query_map([], |row| {
            Ok(Template {
                id: row.get(0)?,
                name: row.get(1)?,
                contract_type: row.get(2)?,
                description: row.get(3)?,
                raw_text: row.get(4)?,
                extracted_data: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(results)
}

pub fn delete(conn: &Connection, id: &str) -> AppResult<()> {
    let rows = conn.execute("DELETE FROM templates WHERE id = ?1", params![id])?;
    if rows == 0 {
        return Err(AppError::NotFound(format!("Template {id} not found")));
    }
    Ok(())
}
