use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};
use super::prompts;
use super::provider::AiProvider;
use super::types::*;

pub struct ClaudeProvider {
    client: Client,
    api_key: String,
    model: String,
}

#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: i32,
    system: String,
    messages: Vec<ClaudeMessage>,
}

#[derive(Serialize)]
struct ClaudeMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
}

#[derive(Deserialize)]
struct ClaudeContent {
    text: Option<String>,
}

impl ClaudeProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: model.unwrap_or_else(|| "claude-sonnet-4-5-20250929".to_string()),
        }
    }

    async fn call_api(&self, system: &str, prompt: &str, max_tokens: i32) -> AppResult<String> {
        let request = ClaudeRequest {
            model: self.model.clone(),
            max_tokens,
            system: system.to_string(),
            messages: vec![ClaudeMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::AiProvider(format!("Claude API connection failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::AiProvider(format!(
                "Claude API returned {status}: {body}"
            )));
        }

        let claude_resp: ClaudeResponse = response.json().await
            .map_err(|e| AppError::AiProvider(format!("Failed to parse Claude response: {e}")))?;

        claude_resp
            .content
            .first()
            .and_then(|c| c.text.clone())
            .ok_or_else(|| AppError::AiProvider("Empty response from Claude".to_string()))
    }
}

fn extract_json_from_text(text: &str) -> &str {
    // Claude sometimes wraps JSON in markdown code blocks
    if let Some(start) = text.find("```json") {
        let json_start = start + 7;
        if let Some(end) = text[json_start..].find("```") {
            return text[json_start..json_start + end].trim();
        }
    }
    if let Some(start) = text.find("```") {
        let json_start = start + 3;
        if let Some(end) = text[json_start..].find("```") {
            return text[json_start..json_start + end].trim();
        }
    }
    text.trim()
}

#[async_trait]
impl AiProvider for ClaudeProvider {
    fn name(&self) -> &str {
        "claude"
    }

    async fn extract_clauses(
        &self,
        text: &str,
        contract_type: &ContractType,
    ) -> AppResult<ExtractionResponse> {
        let system = prompts::extraction_system_prompt(contract_type);
        let prompt = prompts::extraction_user_prompt(text, contract_type);
        let response = self.call_api(&system, &prompt, 4096).await?;
        let json_str = extract_json_from_text(&response);
        super::ollama::parse_extraction_response_public(json_str)
    }

    async fn score_risk(
        &self,
        extraction: &ExtractionResponse,
        contract_type: &ContractType,
    ) -> AppResult<RiskAssessmentResponse> {
        let system = prompts::risk_system_prompt().to_string();
        let extraction_json = serde_json::to_string_pretty(extraction)
            .map_err(|e| AppError::AiProvider(format!("Serialize error: {e}")))?;
        let prompt = prompts::risk_user_prompt(&extraction_json, contract_type);
        let response = self.call_api(&system, &prompt, 2048).await?;
        let json_str = extract_json_from_text(&response);
        super::ollama::parse_risk_response_public(json_str)
    }

    async fn compare_documents(
        &self,
        text_a: &str,
        text_b: &str,
        contract_type: &ContractType,
    ) -> AppResult<ComparisonResponse> {
        let system = prompts::comparison_system_prompt().to_string();
        let prompt = prompts::comparison_user_prompt(text_a, text_b, contract_type);
        let response = self.call_api(&system, &prompt, 4096).await?;
        let json_str = extract_json_from_text(&response);
        super::ollama::parse_comparison_response_public(json_str)
    }

    async fn generate_summary(
        &self,
        extraction: &ExtractionResponse,
        risk: &RiskAssessmentResponse,
    ) -> AppResult<String> {
        let system = prompts::summary_system_prompt().to_string();
        let extraction_json = serde_json::to_string_pretty(extraction)
            .map_err(|e| AppError::AiProvider(format!("Serialize error: {e}")))?;
        let risk_json = serde_json::to_string_pretty(risk)
            .map_err(|e| AppError::AiProvider(format!("Serialize error: {e}")))?;
        let prompt = prompts::summary_user_prompt(&extraction_json, &risk_json);
        self.call_api(&system, &prompt, 2048).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_from_markdown() {
        let text = "Here's the JSON:\n```json\n{\"key\": \"value\"}\n```\nDone!";
        assert_eq!(extract_json_from_text(text), r#"{"key": "value"}"#);
    }

    #[test]
    fn test_extract_json_plain() {
        let text = r#"{"key": "value"}"#;
        assert_eq!(extract_json_from_text(text), r#"{"key": "value"}"#);
    }
}
