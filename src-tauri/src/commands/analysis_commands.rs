use std::sync::Arc;
use tauri::State;

use crate::ai::{OllamaProvider, ClaudeProvider, OpenAiProvider, AiProvider};
use crate::analysis;
use crate::db::Database;
use crate::db::{extractions, risk_assessments, settings};
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
pub async fn analyze_document(
    db: State<'_, Database>,
    document_id: String,
) -> AppResult<AnalysisResult> {
    let provider = create_provider(&db)?;
    let (extraction, risk) = analysis::run_full_analysis(&db, provider, &document_id).await?;

    let extraction_data: serde_json::Value = serde_json::from_str(&extraction.extracted_data)
        .unwrap_or(serde_json::Value::Null);
    let risk_flags: serde_json::Value = serde_json::from_str(&risk.flags)
        .unwrap_or(serde_json::Value::Null);

    Ok(AnalysisResult {
        extraction_id: extraction.id,
        risk_assessment_id: risk.id,
        extraction_data,
        overall_score: risk.overall_score,
        risk_level: risk.risk_level,
        risk_flags,
        summary: risk.summary,
    })
}

#[tauri::command]
pub async fn get_extractions(
    db: State<'_, Database>,
    document_id: String,
) -> AppResult<Vec<extractions::Extraction>> {
    let conn = db.conn.lock().expect("db lock poisoned");
    extractions::list_by_document(&conn, &document_id)
}

#[tauri::command]
pub async fn get_risk_assessments(
    db: State<'_, Database>,
    document_id: String,
) -> AppResult<Vec<risk_assessments::RiskAssessment>> {
    let conn = db.conn.lock().expect("db lock poisoned");
    risk_assessments::get_by_document(&conn, &document_id)
}

#[tauri::command]
pub async fn get_risk_distribution(
    db: State<'_, Database>,
) -> AppResult<risk_assessments::RiskDistribution> {
    let conn = db.conn.lock().expect("db lock poisoned");
    risk_assessments::get_risk_distribution(&conn)
}

#[derive(serde::Serialize)]
pub struct AnalysisResult {
    pub extraction_id: String,
    pub risk_assessment_id: String,
    pub extraction_data: serde_json::Value,
    pub overall_score: i32,
    pub risk_level: String,
    pub risk_flags: serde_json::Value,
    pub summary: Option<String>,
}
