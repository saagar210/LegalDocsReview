use rusqlite::{params, Connection};
use crate::error::AppResult;

pub fn get(conn: &Connection, key: &str) -> AppResult<Option<String>> {
    let result = conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        params![key],
        |row| row.get::<_, String>(0),
    );
    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub fn set(conn: &Connection, key: &str, value: &str) -> AppResult<()> {
    conn.execute(
        "INSERT INTO settings (key, value, updated_at) VALUES (?1, ?2, datetime('now'))
         ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = datetime('now')",
        params![key, value],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    #[test]
    fn test_get_set() {
        let db = Database::in_memory().unwrap();
        let conn = db.conn.lock().unwrap();

        assert_eq!(get(&conn, "ai_provider").unwrap(), None);
        set(&conn, "ai_provider", "ollama").unwrap();
        assert_eq!(get(&conn, "ai_provider").unwrap(), Some("ollama".into()));

        set(&conn, "ai_provider", "claude").unwrap();
        assert_eq!(get(&conn, "ai_provider").unwrap(), Some("claude".into()));
    }
}
