use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};
use super::prompts;
use super::provider::AiProvider;
use super::types::*;

pub struct OpenAiProvider {
    client: Client,
    api_key: String,
    model: String,
}

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    temperature: f64,
    max_tokens: i32,
    response_format: Option<ResponseFormat>,
}

#[derive(Serialize)]
struct ResponseFormat {
    r#type: String,
}

#[derive(Serialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessageResponse,
}

#[derive(Deserialize)]
struct OpenAiMessageResponse {
    content: Option<String>,
}

impl OpenAiProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: model.unwrap_or_else(|| "gpt-4o".to_string()),
        }
    }

    async fn call_api(&self, system: &str, prompt: &str, max_tokens: i32, json_mode: bool) -> AppResult<String> {
        let request = OpenAiRequest {
            model: self.model.clone(),
            messages: vec![
                OpenAiMessage {
                    role: "system".to_string(),
                    content: system.to_string(),
                },
                OpenAiMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            temperature: if json_mode { 0.1 } else { 0.3 },
            max_tokens,
            response_format: if json_mode {
                Some(ResponseFormat {
                    r#type: "json_object".to_string(),
                })
            } else {
                None
            },
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::AiProvider(format!("OpenAI connection failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::AiProvider(format!(
                "OpenAI returned {status}: {body}"
            )));
        }

        let oai_resp: OpenAiResponse = response.json().await
            .map_err(|e| AppError::AiProvider(format!("Failed to parse OpenAI response: {e}")))?;

        oai_resp
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .ok_or_else(|| AppError::AiProvider("Empty response from OpenAI".to_string()))
    }
}

#[async_trait]
impl AiProvider for OpenAiProvider {
    fn name(&self) -> &str {
        "openai"
    }

    async fn extract_clauses(
        &self,
        text: &str,
        contract_type: &ContractType,
    ) -> AppResult<ExtractionResponse> {
        let system = prompts::extraction_system_prompt(contract_type);
        let prompt = prompts::extraction_user_prompt(text, contract_type);
        let response = self.call_api(&system, &prompt, 4096, true).await?;
        super::ollama::parse_extraction_response_public(&response)
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
        let response = self.call_api(&system, &prompt, 2048, true).await?;
        super::ollama::parse_risk_response_public(&response)
    }

    async fn compare_documents(
        &self,
        text_a: &str,
        text_b: &str,
        contract_type: &ContractType,
    ) -> AppResult<ComparisonResponse> {
        let system = prompts::comparison_system_prompt().to_string();
        let prompt = prompts::comparison_user_prompt(text_a, text_b, contract_type);
        let response = self.call_api(&system, &prompt, 4096, true).await?;
        super::ollama::parse_comparison_response_public(&response)
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
        self.call_api(&system, &prompt, 2048, false).await
    }
}
