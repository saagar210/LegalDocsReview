use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub id: String,
    pub document_id: String,
    pub extraction_id: String,
    pub overall_score: i32,
    pub risk_level: String,
    pub flags: String,
    pub summary: Option<String>,
    pub ai_provider: String,
    pub created_at: String,
}

pub struct CreateRiskAssessment {
    pub document_id: String,
    pub extraction_id: String,
    pub overall_score: i32,
    pub risk_level: String,
    pub flags: String,
    pub summary: Option<String>,
    pub ai_provider: String,
}

pub fn insert(conn: &Connection, ra: &CreateRiskAssessment) -> AppResult<RiskAssessment> {
    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO risk_assessments (id, document_id, extraction_id, overall_score, risk_level, flags, summary, ai_provider)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![id, ra.document_id, ra.extraction_id, ra.overall_score, ra.risk_level, ra.flags, ra.summary, ra.ai_provider],
    )?;
    get_by_id(conn, &id)
}

pub fn get_by_id(conn: &Connection, id: &str) -> AppResult<RiskAssessment> {
    conn.query_row(
        "SELECT id, document_id, extraction_id, overall_score, risk_level, flags, summary, ai_provider, created_at
         FROM risk_assessments WHERE id = ?1",
        params![id],
        |row| {
            Ok(RiskAssessment {
                id: row.get(0)?,
                document_id: row.get(1)?,
                extraction_id: row.get(2)?,
                overall_score: row.get(3)?,
                risk_level: row.get(4)?,
                flags: row.get(5)?,
                summary: row.get(6)?,
                ai_provider: row.get(7)?,
                created_at: row.get(8)?,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => AppError::NotFound(format!("RiskAssessment {id} not found")),
        other => AppError::Database(other),
    })
}

pub fn get_by_document(conn: &Connection, document_id: &str) -> AppResult<Vec<RiskAssessment>> {
    let mut stmt = conn.prepare(
        "SELECT id, document_id, extraction_id, overall_score, risk_level, flags, summary, ai_provider, created_at
         FROM risk_assessments WHERE document_id = ?1 ORDER BY created_at DESC",
    )?;
    let results = stmt
        .query_map(params![document_id], |row| {
            Ok(RiskAssessment {
                id: row.get(0)?,
                document_id: row.get(1)?,
                extraction_id: row.get(2)?,
                overall_score: row.get(3)?,
                risk_level: row.get(4)?,
                flags: row.get(5)?,
                summary: row.get(6)?,
                ai_provider: row.get(7)?,
                created_at: row.get(8)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(results)
}

pub fn get_risk_distribution(conn: &Connection) -> AppResult<RiskDistribution> {
    let low: i64 = conn.query_row(
        "SELECT COUNT(*) FROM risk_assessments WHERE risk_level = 'low'", [], |row| row.get(0),
    )?;
    let medium: i64 = conn.query_row(
        "SELECT COUNT(*) FROM risk_assessments WHERE risk_level = 'medium'", [], |row| row.get(0),
    )?;
    let high: i64 = conn.query_row(
        "SELECT COUNT(*) FROM risk_assessments WHERE risk_level = 'high'", [], |row| row.get(0),
    )?;
    Ok(RiskDistribution { low, medium, high })
}

#[derive(Debug, Serialize)]
pub struct RiskDistribution {
    pub low: i64,
    pub medium: i64,
    pub high: i64,
}
