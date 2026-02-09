use async_trait::async_trait;

use crate::error::AppResult;
use super::types::*;

#[async_trait]
pub trait AiProvider: Send + Sync {
    fn name(&self) -> &str;

    async fn extract_clauses(
        &self,
        text: &str,
        contract_type: &ContractType,
    ) -> AppResult<ExtractionResponse>;

    async fn score_risk(
        &self,
        extraction: &ExtractionResponse,
        contract_type: &ContractType,
    ) -> AppResult<RiskAssessmentResponse>;

    async fn compare_documents(
        &self,
        text_a: &str,
        text_b: &str,
        contract_type: &ContractType,
    ) -> AppResult<ComparisonResponse>;

    async fn generate_summary(
        &self,
        extraction: &ExtractionResponse,
        risk: &RiskAssessmentResponse,
    ) -> AppResult<String>;
}
