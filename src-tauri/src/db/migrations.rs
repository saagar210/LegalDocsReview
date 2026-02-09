use rusqlite::Connection;
use crate::error::AppResult;

pub fn run(conn: &Connection) -> AppResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS documents (
            id TEXT PRIMARY KEY,
            filename TEXT NOT NULL,
            original_path TEXT NOT NULL,
            stored_path TEXT NOT NULL,
            file_hash TEXT NOT NULL,
            file_size INTEGER NOT NULL,
            contract_type TEXT NOT NULL,
            raw_text TEXT,
            page_count INTEGER,
            processing_status TEXT NOT NULL DEFAULT 'pending',
            error_message TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS extractions (
            id TEXT PRIMARY KEY,
            document_id TEXT NOT NULL,
            ai_provider TEXT NOT NULL,
            ai_model TEXT,
            contract_type TEXT NOT NULL,
            extracted_data TEXT NOT NULL,
            confidence_score REAL,
            processing_time_ms INTEGER,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS risk_assessments (
            id TEXT PRIMARY KEY,
            document_id TEXT NOT NULL,
            extraction_id TEXT NOT NULL,
            overall_score INTEGER NOT NULL,
            risk_level TEXT NOT NULL,
            flags TEXT NOT NULL,
            summary TEXT,
            ai_provider TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
            FOREIGN KEY (extraction_id) REFERENCES extractions(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS templates (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            contract_type TEXT NOT NULL,
            description TEXT,
            raw_text TEXT NOT NULL,
            extracted_data TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS comparisons (
            id TEXT PRIMARY KEY,
            document_a_id TEXT NOT NULL,
            document_b_id TEXT,
            template_id TEXT,
            comparison_type TEXT NOT NULL,
            differences TEXT NOT NULL,
            summary TEXT,
            ai_provider TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (document_a_id) REFERENCES documents(id) ON DELETE CASCADE,
            FOREIGN KEY (document_b_id) REFERENCES documents(id) ON DELETE SET NULL,
            FOREIGN KEY (template_id) REFERENCES templates(id) ON DELETE SET NULL
        );

        CREATE TABLE IF NOT EXISTS reports (
            id TEXT PRIMARY KEY,
            document_id TEXT NOT NULL,
            report_type TEXT NOT NULL,
            content TEXT NOT NULL,
            export_path TEXT,
            format TEXT NOT NULL DEFAULT 'pdf',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE INDEX IF NOT EXISTS idx_extractions_document ON extractions(document_id);
        CREATE INDEX IF NOT EXISTS idx_risk_document ON risk_assessments(document_id);
        CREATE INDEX IF NOT EXISTS idx_comparisons_doc_a ON comparisons(document_a_id);
        CREATE INDEX IF NOT EXISTS idx_reports_document ON reports(document_id);
        "
    )?;
    Ok(())
}
