use std::sync::Arc;
use tauri::State;

use crate::ai::{AiProvider, ContractType, OllamaProvider, ClaudeProvider, OpenAiProvider};
use crate::db::Database;
use crate::db::{comparisons, documents, settings};
use crate::error::{AppError, AppResult};

fn create_provider(db: &Database) -> AppResult<Arc<dyn AiProvider>> {
    let conn = db.conn.lock().expect("db lock poisoned");
    let provider_name = settings::get(&conn, "ai_provider")?
        .unwrap_or_else(|| "ollama".to_string());

    match provider_name.as_str() {
        "ollama" => {
            let url = settings::get(&conn, "ollama_url")?
                .unwrap_or_else(|| "http://localhost:11434".to_string());
            let model = settings::get(&conn, "ollama_model")?
                .unwrap_or_else(|| "llama3".to_string());
            Ok(Arc::new(OllamaProvider::new(url, model)))
        }
        "claude" => {
            let api_key = settings::get(&conn, "claude_api_key")?
                .ok_or_else(|| AppError::Validation("Claude API key not configured".to_string()))?;
            let model = settings::get(&conn, "claude_model")?;
            Ok(Arc::new(ClaudeProvider::new(api_key, model)))
        }
        "openai" => {
            let api_key = settings::get(&conn, "openai_api_key")?
                .ok_or_else(|| AppError::Validation("OpenAI API key not configured".to_string()))?;
            let model = settings::get(&conn, "openai_model")?;
            Ok(Arc::new(OpenAiProvider::new(api_key, model)))
        }
        other => Err(AppError::Validation(format!("Unknown AI provider: {other}"))),
    }
}

#[tauri::command]
pub async fn compare_documents(
    db: State<'_, Database>,
    document_a_id: String,
    document_b_id: String,
) -> AppResult<comparisons::Comparison> {
    let (text_a, text_b, contract_type_str) = {
        let conn = db.conn.lock().expect("db lock poisoned");
        let doc_a = documents::get_by_id(&conn, &document_a_id)?;
        let doc_b = documents::get_by_id(&conn, &document_b_id)?;

        let text_a = doc_a.raw_text.ok_or_else(|| {
            AppError::Validation("Document A has no extracted text".to_string())
        })?;
        let text_b = doc_b.raw_text.ok_or_else(|| {
            AppError::Validation("Document B has no extracted text".to_string())
        })?;

        (text_a, text_b, doc_a.contract_type)
    };

    let contract_type = ContractType::from_str(&contract_type_str)
        .ok_or_else(|| AppError::Validation(format!("Unknown contract type: {contract_type_str}")))?;

    let provider = create_provider(&db)?;
    let result = provider.compare_documents(&text_a, &text_b, &contract_type).await?;

    let differences_json = serde_json::to_string(&result.differences)
        .map_err(|e| AppError::Json(e))?;

    let conn = db.conn.lock().expect("db lock poisoned");
    comparisons::insert(
        &conn,
        &document_a_id,
        Some(&document_b_id),
        None,
        "document_vs_document",
        &differences_json,
        Some(&result.summary),
        Some(provider.name()),
    )
}

#[tauri::command]
pub async fn get_comparisons(
    db: State<'_, Database>,
    document_id: String,
) -> AppResult<Vec<comparisons::Comparison>> {
    let conn = db.conn.lock().expect("db lock poisoned");
    comparisons::list_by_document(&conn, &document_id)
}
