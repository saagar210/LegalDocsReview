use std::sync::Arc;
use tauri::{Manager, State};

use crate::ai::{AiProvider, ExtractionResponse, RiskAssessmentResponse, OllamaProvider, ClaudeProvider, OpenAiProvider};
use crate::db::Database;
use crate::db::{extractions, reports, risk_assessments, settings};
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
pub async fn generate_report(
    db: State<'_, Database>,
    document_id: String,
    app_handle: tauri::AppHandle,
) -> AppResult<reports::Report> {
    // Get latest extraction and risk assessment
    let (extraction_data, risk_data) = {
        let conn = db.conn.lock().expect("db lock poisoned");
        let exts = extractions::list_by_document(&conn, &document_id)?;
        let ext = exts.first().ok_or_else(|| {
            AppError::NotFound("No extraction found. Run analysis first.".to_string())
        })?;
        let risks = risk_assessments::get_by_document(&conn, &document_id)?;
        let risk = risks.first().ok_or_else(|| {
            AppError::NotFound("No risk assessment found. Run analysis first.".to_string())
        })?;
        (ext.extracted_data.clone(), risk.clone())
    };

    let extraction: ExtractionResponse = serde_json::from_str(&extraction_data)
        .map_err(|e| AppError::AiProvider(format!("Failed to parse extraction: {e}")))?;

    let risk_flags_parsed: Vec<crate::ai::RiskFlag> = serde_json::from_str(&risk_data.flags)
        .unwrap_or_default();
    let risk_response = RiskAssessmentResponse {
        overall_score: risk_data.overall_score,
        risk_level: risk_data.risk_level.clone(),
        flags: risk_flags_parsed,
        summary: risk_data.summary.clone().unwrap_or_default(),
    };

    // Generate AI summary
    let provider = create_provider(&db)?;
    let summary = provider.generate_summary(&extraction, &risk_response).await?;

    // Build report content
    let report_content = build_report_content(&extraction, &risk_response, &summary);

    // Save report
    let conn = db.conn.lock().expect("db lock poisoned");
    let report = reports::insert(&conn, &document_id, "full_analysis", &report_content, "text")?;

    // Also export as text file
    let app_data = app_handle.path().app_data_dir()
        .expect("failed to get app data dir");
    let reports_dir = app_data.join("reports");
    std::fs::create_dir_all(&reports_dir)?;
    let filename = format!("report_{}.txt", &report.id[..8]);
    let export_path = reports_dir.join(&filename);
    std::fs::write(&export_path, &report_content)?;

    Ok(report)
}

fn build_report_content(
    extraction: &ExtractionResponse,
    risk: &RiskAssessmentResponse,
    summary: &str,
) -> String {
    let mut content = String::new();

    content.push_str("═══════════════════════════════════════════════════\n");
    content.push_str("        LEGAL DOCUMENT REVIEW REPORT\n");
    content.push_str("═══════════════════════════════════════════════════\n\n");

    content.push_str("EXECUTIVE SUMMARY\n");
    content.push_str("─────────────────\n");
    content.push_str(summary);
    content.push_str("\n\n");

    content.push_str("KEY PARTIES\n");
    content.push_str("───────────\n");
    for party in &extraction.parties {
        content.push_str(&format!("  • {party}\n"));
    }
    content.push('\n');

    if let Some(date) = &extraction.effective_date {
        content.push_str(&format!("Effective Date: {date}\n"));
    }
    if let Some(date) = &extraction.termination_date {
        content.push_str(&format!("Termination Date: {date}\n"));
    }
    content.push('\n');

    content.push_str("EXTRACTED CLAUSES\n");
    content.push_str("─────────────────\n");
    for clause in &extraction.clauses {
        let ref_str = clause.section_reference.as_deref().unwrap_or("N/A");
        content.push_str(&format!(
            "\n[{}] {} (Ref: {})\n  Importance: {}\n  Text: {}\n",
            clause.clause_type.to_uppercase(),
            clause.title,
            ref_str,
            clause.importance,
            clause.text
        ));
    }
    content.push('\n');

    content.push_str("RISK ASSESSMENT\n");
    content.push_str("───────────────\n");
    content.push_str(&format!(
        "Overall Score: {}/100 ({})\n\n",
        risk.overall_score,
        risk.risk_level.to_uppercase()
    ));

    if !risk.flags.is_empty() {
        content.push_str("Risk Flags:\n");
        for flag in &risk.flags {
            let ref_str = flag.clause_reference.as_deref().unwrap_or("General");
            content.push_str(&format!(
                "\n  [{} - {}] {}\n    Ref: {}\n",
                flag.severity.to_uppercase(),
                flag.category.to_uppercase(),
                flag.description,
                ref_str,
            ));
            if let Some(suggestion) = &flag.suggestion {
                content.push_str(&format!("    Suggestion: {suggestion}\n"));
            }
        }
    }

    content.push_str("\n═══════════════════════════════════════════════════\n");
    content.push_str("Generated by Legal Document Review Assistant\n");

    content
}

#[tauri::command]
pub async fn get_reports(
    db: State<'_, Database>,
    document_id: String,
) -> AppResult<Vec<reports::Report>> {
    let conn = db.conn.lock().expect("db lock poisoned");
    reports::list_by_document(&conn, &document_id)
}
