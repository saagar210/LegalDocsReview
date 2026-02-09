pub mod risk_rules;

use std::sync::Arc;
use std::time::Instant;

use crate::ai::{AiProvider, ContractType, ExtractionResponse};
use crate::db::{Database, documents, extractions, risk_assessments};
use crate::error::{AppError, AppResult};

pub async fn run_extraction(
    db: &Database,
    provider: Arc<dyn AiProvider>,
    document_id: &str,
) -> AppResult<extractions::Extraction> {
    let (raw_text, contract_type_str) = {
        let conn = db.conn.lock().expect("db lock poisoned");
        let doc = documents::get_by_id(&conn, document_id)?;
        let text = doc.raw_text.ok_or_else(|| {
            AppError::Validation("Document text not yet extracted".to_string())
        })?;
        (text, doc.contract_type)
    };

    let contract_type = ContractType::from_str(&contract_type_str)
        .ok_or_else(|| AppError::Validation(format!("Unknown contract type: {contract_type_str}")))?;

    // Update status to analyzing
    {
        let conn = db.conn.lock().expect("db lock poisoned");
        documents::update_status(&conn, document_id, "analyzing", None)?;
    }

    let start = Instant::now();
    let extraction = provider.extract_clauses(&raw_text, &contract_type).await;
    let elapsed_ms = start.elapsed().as_millis() as i64;

    match extraction {
        Ok(result) => {
            let conn = db.conn.lock().expect("db lock poisoned");
            let extraction_record = extractions::insert(
                &conn,
                &extractions::CreateExtraction {
                    document_id: document_id.to_string(),
                    ai_provider: provider.name().to_string(),
                    ai_model: None,
                    contract_type: contract_type_str,
                    extracted_data: serde_json::to_string(&result)
                        .map_err(|e| AppError::Json(e))?,
                    confidence_score: None,
                    processing_time_ms: Some(elapsed_ms),
                },
            )?;
            documents::update_status(&conn, document_id, "extracted", None)?;
            Ok(extraction_record)
        }
        Err(e) => {
            let conn = db.conn.lock().expect("db lock poisoned");
            documents::update_status(&conn, document_id, "error", Some(&e.to_string()))?;
            Err(e)
        }
    }
}

pub async fn run_risk_assessment(
    db: &Database,
    provider: Arc<dyn AiProvider>,
    document_id: &str,
    extraction_id: &str,
) -> AppResult<risk_assessments::RiskAssessment> {
    let (extraction_data, contract_type_str) = {
        let conn = db.conn.lock().expect("db lock poisoned");
        let ext = extractions::get_by_id(&conn, extraction_id)?;
        (ext.extracted_data, ext.contract_type)
    };

    let contract_type = ContractType::from_str(&contract_type_str)
        .ok_or_else(|| AppError::Validation(format!("Unknown contract type: {contract_type_str}")))?;

    let extraction: ExtractionResponse = serde_json::from_str(&extraction_data)
        .map_err(|e| AppError::AiProvider(format!("Failed to parse stored extraction: {e}")))?;

    let mut risk_result = provider.score_risk(&extraction, &contract_type).await?;

    // Apply rule-based risk checks
    let rule_flags = risk_rules::apply_rules(&extraction, &contract_type);
    risk_result.flags.extend(rule_flags);

    // Recalculate score if rule-based flags bumped severity
    if risk_result.flags.iter().any(|f| f.severity == "high") && risk_result.overall_score < 67 {
        risk_result.overall_score = 67.max(risk_result.overall_score);
        risk_result.risk_level = "high".to_string();
    }

    let conn = db.conn.lock().expect("db lock poisoned");
    let ra = risk_assessments::insert(
        &conn,
        &risk_assessments::CreateRiskAssessment {
            document_id: document_id.to_string(),
            extraction_id: extraction_id.to_string(),
            overall_score: risk_result.overall_score,
            risk_level: risk_result.risk_level.clone(),
            flags: serde_json::to_string(&risk_result.flags)
                .map_err(|e| AppError::Json(e))?,
            summary: Some(risk_result.summary),
            ai_provider: provider.name().to_string(),
        },
    )?;

    documents::update_status(&conn, document_id, "analyzed", None)?;

    Ok(ra)
}

pub async fn run_full_analysis(
    db: &Database,
    provider: Arc<dyn AiProvider>,
    document_id: &str,
) -> AppResult<(extractions::Extraction, risk_assessments::RiskAssessment)> {
    let extraction = run_extraction(db, provider.clone(), document_id).await?;
    let risk = run_risk_assessment(db, provider, document_id, &extraction.id).await?;
    Ok((extraction, risk))
}
