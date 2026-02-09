use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};
use super::prompts;
use super::provider::AiProvider;
use super::types::*;

pub struct OllamaProvider {
    client: Client,
    base_url: String,
    model: String,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    system: String,
    stream: bool,
    format: String,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    system: String,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f64,
    num_predict: i32,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

impl OllamaProvider {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            model,
        }
    }

    async fn generate_json(&self, system: &str, prompt: &str) -> AppResult<String> {
        let request = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            system: system.to_string(),
            stream: false,
            format: "json".to_string(),
            options: OllamaOptions {
                temperature: 0.1,
                num_predict: 4096,
            },
        };

        let url = format!("{}/api/generate", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::AiProvider(format!("Ollama connection failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::AiProvider(format!(
                "Ollama returned {status}: {body}"
            )));
        }

        let ollama_resp: OllamaResponse = response.json().await
            .map_err(|e| AppError::AiProvider(format!("Failed to parse Ollama response: {e}")))?;

        Ok(ollama_resp.response)
    }

    async fn generate_text(&self, system: &str, prompt: &str) -> AppResult<String> {
        let request = OllamaGenerateRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            system: system.to_string(),
            stream: false,
            options: OllamaOptions {
                temperature: 0.3,
                num_predict: 2048,
            },
        };

        let url = format!("{}/api/generate", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::AiProvider(format!("Ollama connection failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::AiProvider(format!(
                "Ollama returned {status}: {body}"
            )));
        }

        let ollama_resp: OllamaResponse = response.json().await
            .map_err(|e| AppError::AiProvider(format!("Failed to parse Ollama response: {e}")))?;

        Ok(ollama_resp.response)
    }
}

fn parse_extraction_response(json_str: &str) -> AppResult<ExtractionResponse> {
    #[derive(Deserialize)]
    struct RawExtraction {
        parties: Option<Vec<String>>,
        effective_date: Option<String>,
        termination_date: Option<String>,
        clauses: Option<Vec<RawClause>>,
        contract_type: Option<String>,
    }

    #[derive(Deserialize)]
    struct RawClause {
        clause_type: Option<String>,
        title: Option<String>,
        text: Option<String>,
        section_reference: Option<String>,
        importance: Option<String>,
    }

    let raw: RawExtraction = serde_json::from_str(json_str)
        .map_err(|e| AppError::AiProvider(format!("Failed to parse extraction JSON: {e}\nRaw: {json_str}")))?;

    let clauses = raw
        .clauses
        .unwrap_or_default()
        .into_iter()
        .filter(|c| c.text.is_some() && c.clause_type.is_some())
        .map(|c| {
            let ct = c.clause_type.unwrap_or_default();
            ExtractedClause {
            clause_type: ct.clone(),
            title: c.title.unwrap_or_else(|| ct.clone()),
            text: c.text.unwrap_or_default(),
            section_reference: c.section_reference,
            importance: c.importance.unwrap_or_else(|| "medium".to_string()),
        }})
        .collect();

    Ok(ExtractionResponse {
        parties: raw.parties.unwrap_or_default(),
        effective_date: raw.effective_date,
        termination_date: raw.termination_date,
        clauses,
        contract_type: raw.contract_type.unwrap_or_default(),
        raw_json: json_str.to_string(),
    })
}

fn parse_risk_response(json_str: &str) -> AppResult<RiskAssessmentResponse> {
    #[derive(Deserialize)]
    struct RawRisk {
        overall_score: Option<i32>,
        risk_level: Option<String>,
        flags: Option<Vec<RawFlag>>,
        summary: Option<String>,
    }

    #[derive(Deserialize)]
    struct RawFlag {
        category: Option<String>,
        severity: Option<String>,
        description: Option<String>,
        clause_reference: Option<String>,
        suggestion: Option<String>,
    }

    let raw: RawRisk = serde_json::from_str(json_str)
        .map_err(|e| AppError::AiProvider(format!("Failed to parse risk JSON: {e}\nRaw: {json_str}")))?;

    let score = raw.overall_score.unwrap_or(50);
    let level = raw.risk_level.unwrap_or_else(|| {
        if score <= 33 { "low" } else if score <= 66 { "medium" } else { "high" }.to_string()
    });

    Ok(RiskAssessmentResponse {
        overall_score: score,
        risk_level: level,
        flags: raw
            .flags
            .unwrap_or_default()
            .into_iter()
            .filter(|f| f.description.is_some())
            .map(|f| RiskFlag {
                category: f.category.unwrap_or_else(|| "other".to_string()),
                severity: f.severity.unwrap_or_else(|| "medium".to_string()),
                description: f.description.unwrap_or_default(),
                clause_reference: f.clause_reference,
                suggestion: f.suggestion,
            })
            .collect(),
        summary: raw.summary.unwrap_or_else(|| "Risk assessment completed.".to_string()),
    })
}

fn parse_comparison_response(json_str: &str) -> AppResult<ComparisonResponse> {
    #[derive(Deserialize)]
    struct RawComparison {
        differences: Option<Vec<RawDiff>>,
        summary: Option<String>,
    }

    #[derive(Deserialize)]
    struct RawDiff {
        category: Option<String>,
        diff_type: Option<String>,
        description: Option<String>,
        text_a: Option<String>,
        text_b: Option<String>,
        significance: Option<String>,
    }

    let raw: RawComparison = serde_json::from_str(json_str)
        .map_err(|e| AppError::AiProvider(format!("Failed to parse comparison JSON: {e}\nRaw: {json_str}")))?;

    Ok(ComparisonResponse {
        differences: raw
            .differences
            .unwrap_or_default()
            .into_iter()
            .filter(|d| d.description.is_some())
            .map(|d| Difference {
                category: d.category.unwrap_or_else(|| "other".to_string()),
                diff_type: d.diff_type.unwrap_or_else(|| "substantive".to_string()),
                description: d.description.unwrap_or_default(),
                text_a: d.text_a,
                text_b: d.text_b,
                significance: d.significance.unwrap_or_else(|| "medium".to_string()),
            })
            .collect(),
        summary: raw.summary.unwrap_or_else(|| "Comparison completed.".to_string()),
    })
}

#[async_trait]
impl AiProvider for OllamaProvider {
    fn name(&self) -> &str {
        "ollama"
    }

    async fn extract_clauses(
        &self,
        text: &str,
        contract_type: &ContractType,
    ) -> AppResult<ExtractionResponse> {
        let system = prompts::extraction_system_prompt(contract_type);
        let prompt = prompts::extraction_user_prompt(text, contract_type);
        let response = self.generate_json(&system, &prompt).await?;
        parse_extraction_response(&response)
    }

    async fn score_risk(
        &self,
        extraction: &ExtractionResponse,
        contract_type: &ContractType,
    ) -> AppResult<RiskAssessmentResponse> {
        let system = prompts::risk_system_prompt().to_string();
        let extraction_json = serde_json::to_string_pretty(extraction)
            .map_err(|e| AppError::AiProvider(format!("Failed to serialize extraction: {e}")))?;
        let prompt = prompts::risk_user_prompt(&extraction_json, contract_type);
        let response = self.generate_json(&system, &prompt).await?;
        parse_risk_response(&response)
    }

    async fn compare_documents(
        &self,
        text_a: &str,
        text_b: &str,
        contract_type: &ContractType,
    ) -> AppResult<ComparisonResponse> {
        let system = prompts::comparison_system_prompt().to_string();
        let prompt = prompts::comparison_user_prompt(text_a, text_b, contract_type);
        let response = self.generate_json(&system, &prompt).await?;
        parse_comparison_response(&response)
    }

    async fn generate_summary(
        &self,
        extraction: &ExtractionResponse,
        risk: &RiskAssessmentResponse,
    ) -> AppResult<String> {
        let system = prompts::summary_system_prompt().to_string();
        let extraction_json = serde_json::to_string_pretty(extraction)
            .map_err(|e| AppError::AiProvider(format!("Failed to serialize extraction: {e}")))?;
        let risk_json = serde_json::to_string_pretty(risk)
            .map_err(|e| AppError::AiProvider(format!("Failed to serialize risk: {e}")))?;
        let prompt = prompts::summary_user_prompt(&extraction_json, &risk_json);
        self.generate_text(&system, &prompt).await
    }
}

// Public wrappers for shared parsing logic
pub fn parse_extraction_response_public(json_str: &str) -> AppResult<ExtractionResponse> {
    parse_extraction_response(json_str)
}

pub fn parse_risk_response_public(json_str: &str) -> AppResult<RiskAssessmentResponse> {
    parse_risk_response(json_str)
}

pub fn parse_comparison_response_public(json_str: &str) -> AppResult<ComparisonResponse> {
    parse_comparison_response(json_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_extraction_response() {
        let json = r#"{
            "parties": ["Acme Corp", "Globex Inc"],
            "effective_date": "2024-01-01",
            "termination_date": null,
            "clauses": [
                {
                    "clause_type": "confidentiality",
                    "title": "Confidentiality",
                    "text": "All information shared shall be kept confidential.",
                    "section_reference": "Section 3",
                    "importance": "high"
                },
                {
                    "clause_type": "governing_law",
                    "title": "Governing Law",
                    "text": "This agreement shall be governed by the laws of California.",
                    "section_reference": "Section 8",
                    "importance": "medium"
                }
            ],
            "contract_type": "nda"
        }"#;

        let result = parse_extraction_response(json).unwrap();
        assert_eq!(result.parties.len(), 2);
        assert_eq!(result.parties[0], "Acme Corp");
        assert_eq!(result.clauses.len(), 2);
        assert_eq!(result.clauses[0].clause_type, "confidentiality");
        assert_eq!(result.effective_date, Some("2024-01-01".to_string()));
    }

    #[test]
    fn test_parse_extraction_with_missing_fields() {
        let json = r#"{
            "parties": ["A"],
            "clauses": [
                {"clause_type": "test", "text": "some text"},
                {"clause_type": null, "text": null}
            ]
        }"#;

        let result = parse_extraction_response(json).unwrap();
        assert_eq!(result.parties.len(), 1);
        assert_eq!(result.clauses.len(), 1);
    }

    #[test]
    fn test_parse_risk_response() {
        let json = r#"{
            "overall_score": 72,
            "risk_level": "high",
            "flags": [
                {
                    "category": "indemnification",
                    "severity": "high",
                    "description": "No indemnification cap specified",
                    "clause_reference": "Section 5",
                    "suggestion": "Add a reasonable cap"
                }
            ],
            "summary": "This contract has significant risk."
        }"#;

        let result = parse_risk_response(json).unwrap();
        assert_eq!(result.overall_score, 72);
        assert_eq!(result.risk_level, "high");
        assert_eq!(result.flags.len(), 1);
    }

    #[test]
    fn test_parse_risk_defaults() {
        let json = r#"{"flags": []}"#;
        let result = parse_risk_response(json).unwrap();
        assert_eq!(result.overall_score, 50);
        assert_eq!(result.risk_level, "medium");
    }

    #[test]
    fn test_parse_comparison_response() {
        let json = r#"{
            "differences": [
                {
                    "category": "payment",
                    "diff_type": "substantive",
                    "description": "Payment terms changed from net-30 to net-60",
                    "text_a": "Payment due within 30 days",
                    "text_b": "Payment due within 60 days",
                    "significance": "high"
                }
            ],
            "summary": "One significant change in payment terms."
        }"#;

        let result = parse_comparison_response(json).unwrap();
        assert_eq!(result.differences.len(), 1);
        assert_eq!(result.differences[0].diff_type, "substantive");
    }

    #[test]
    fn test_parse_invalid_json() {
        let result = parse_extraction_response("not json");
        assert!(result.is_err());
    }
}
