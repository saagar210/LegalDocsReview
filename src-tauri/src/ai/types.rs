use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContractType {
    Nda,
    ServiceAgreement,
    Lease,
}

impl std::str::FromStr for ContractType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nda" => Ok(Self::Nda),
            "service_agreement" => Ok(Self::ServiceAgreement),
            "lease" => Ok(Self::Lease),
            _ => Err(()),
        }
    }
}

impl ContractType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Nda => "Non-Disclosure Agreement",
            Self::ServiceAgreement => "Service Agreement",
            Self::Lease => "Lease Agreement",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResponse {
    pub parties: Vec<String>,
    pub effective_date: Option<String>,
    pub termination_date: Option<String>,
    pub clauses: Vec<ExtractedClause>,
    pub contract_type: String,
    pub raw_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedClause {
    pub clause_type: String,
    pub title: String,
    pub text: String,
    pub section_reference: Option<String>,
    pub importance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessmentResponse {
    pub overall_score: i32,
    pub risk_level: String,
    pub flags: Vec<RiskFlag>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFlag {
    pub category: String,
    pub severity: String,
    pub description: String,
    pub clause_reference: Option<String>,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResponse {
    pub differences: Vec<Difference>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Difference {
    pub category: String,
    pub diff_type: String,
    pub description: String,
    pub text_a: Option<String>,
    pub text_b: Option<String>,
    pub significance: String,
}
