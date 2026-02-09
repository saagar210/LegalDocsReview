use crate::ai::types::ContractType;

pub fn extraction_system_prompt(contract_type: &ContractType) -> String {
    format!(
        "You are a legal document analysis expert specializing in {}. \
         Extract key clauses and terms from the provided contract text. \
         You MUST respond with valid JSON only — no markdown, no explanations, no preamble.",
        contract_type.display_name()
    )
}

pub fn extraction_user_prompt(text: &str, contract_type: &ContractType) -> String {
    let schema = extraction_schema(contract_type);
    format!(
        "Analyze the following {} and extract all key clauses.\n\n\
         RULES:\n\
         1. Quote exact text from the document — do not paraphrase\n\
         2. Use null for any clause or field not found in the document\n\
         3. Respond with ONLY the JSON object below — no other text\n\n\
         JSON Schema:\n{}\n\n\
         DOCUMENT TEXT:\n---\n{}\n---",
        contract_type.display_name(),
        schema,
        text
    )
}

fn extraction_schema(contract_type: &ContractType) -> &'static str {
    match contract_type {
        ContractType::Nda => NDA_EXTRACTION_SCHEMA,
        ContractType::ServiceAgreement => SERVICE_AGREEMENT_EXTRACTION_SCHEMA,
        ContractType::Lease => LEASE_EXTRACTION_SCHEMA,
    }
}

pub fn risk_system_prompt() -> &'static str {
    "You are a legal risk assessment expert. Analyze the extracted clauses and identify potential risks. \
     You MUST respond with valid JSON only — no markdown, no explanations, no preamble."
}

pub fn risk_user_prompt(extraction_json: &str, contract_type: &ContractType) -> String {
    format!(
        "Analyze the following extracted clauses from a {} and provide a risk assessment.\n\n\
         RULES:\n\
         1. Score overall risk 0-100 (0=no risk, 100=extreme risk)\n\
         2. Set risk_level to \"low\" (0-33), \"medium\" (34-66), or \"high\" (67-100)\n\
         3. Flag specific issues with severity, description, and fix suggestions\n\
         4. Common risks: missing indemnification cap, one-sided termination, auto-renewal traps, \
            broad non-compete, unlimited liability, missing governing law\n\
         5. Respond with ONLY the JSON object below — no other text\n\n\
         JSON Schema:\n{}\n\n\
         EXTRACTED CLAUSES:\n---\n{}\n---",
        contract_type.display_name(),
        RISK_ASSESSMENT_SCHEMA,
        extraction_json
    )
}

pub fn comparison_system_prompt() -> &'static str {
    "You are a legal document comparison expert. Compare two contract versions and categorize differences. \
     You MUST respond with valid JSON only — no markdown, no explanations, no preamble."
}

pub fn comparison_user_prompt(text_a: &str, text_b: &str, contract_type: &ContractType) -> String {
    format!(
        "Compare these two versions of a {} and identify all differences.\n\n\
         RULES:\n\
         1. Categorize each difference as \"substantive\" or \"formatting\"\n\
         2. Rate significance as \"high\", \"medium\", or \"low\"\n\
         3. Quote exact text from each document\n\
         4. Respond with ONLY the JSON object below — no other text\n\n\
         JSON Schema:\n{}\n\n\
         DOCUMENT A:\n---\n{}\n---\n\n\
         DOCUMENT B:\n---\n{}\n---",
        contract_type.display_name(),
        COMPARISON_SCHEMA,
        text_a,
        text_b
    )
}

pub fn summary_system_prompt() -> &'static str {
    "You are a legal document summarizer. Write a concise, client-ready executive summary. \
     Respond with plain text only — no JSON, no markdown headers."
}

pub fn summary_user_prompt(extraction_json: &str, risk_json: &str) -> String {
    format!(
        "Write a 2-3 paragraph executive summary of this contract review for a client.\n\n\
         Include:\n\
         1. Key parties and terms\n\
         2. Notable clauses and their implications\n\
         3. Risk highlights and recommended actions\n\n\
         Keep it professional, concise, and actionable.\n\n\
         EXTRACTED CLAUSES:\n{}\n\n\
         RISK ASSESSMENT:\n{}",
        extraction_json, risk_json
    )
}

const NDA_EXTRACTION_SCHEMA: &str = r#"{
  "parties": ["Party A name", "Party B name"],
  "effective_date": "YYYY-MM-DD or null",
  "termination_date": "YYYY-MM-DD or null",
  "clauses": [
    {
      "clause_type": "definition_of_confidential_info",
      "title": "Definition of Confidential Information",
      "text": "exact quoted text",
      "section_reference": "Section X",
      "importance": "high|medium|low"
    },
    { "clause_type": "obligations_of_receiving_party", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "exclusions", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "term_and_duration", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "return_of_materials", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "remedies", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "non_solicitation", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "governing_law", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "dispute_resolution", "title": "...", "text": "...", "section_reference": "...", "importance": "..." }
  ],
  "contract_type": "nda"
}"#;

const SERVICE_AGREEMENT_EXTRACTION_SCHEMA: &str = r#"{
  "parties": ["Service Provider name", "Client name"],
  "effective_date": "YYYY-MM-DD or null",
  "termination_date": "YYYY-MM-DD or null",
  "clauses": [
    { "clause_type": "scope_of_services", "title": "Scope of Services", "text": "...", "section_reference": "...", "importance": "high" },
    { "clause_type": "payment_terms", "title": "Payment Terms", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "term_and_termination", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "indemnification", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "limitation_of_liability", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "intellectual_property", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "confidentiality", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "warranties", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "force_majeure", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "governing_law", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "dispute_resolution", "title": "...", "text": "...", "section_reference": "...", "importance": "..." }
  ],
  "contract_type": "service_agreement"
}"#;

const LEASE_EXTRACTION_SCHEMA: &str = r#"{
  "parties": ["Landlord name", "Tenant name"],
  "effective_date": "YYYY-MM-DD or null",
  "termination_date": "YYYY-MM-DD or null",
  "clauses": [
    { "clause_type": "premises_description", "title": "Premises", "text": "...", "section_reference": "...", "importance": "high" },
    { "clause_type": "rent_and_payment", "title": "Rent", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "security_deposit", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "lease_term", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "maintenance_and_repairs", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "use_restrictions", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "insurance_requirements", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "termination_and_renewal", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "default_and_remedies", "title": "...", "text": "...", "section_reference": "...", "importance": "..." },
    { "clause_type": "governing_law", "title": "...", "text": "...", "section_reference": "...", "importance": "..." }
  ],
  "contract_type": "lease"
}"#;

const RISK_ASSESSMENT_SCHEMA: &str = r#"{
  "overall_score": 45,
  "risk_level": "medium",
  "flags": [
    {
      "category": "indemnification|liability|termination|non_compete|confidentiality|payment|governing_law|other",
      "severity": "high|medium|low",
      "description": "Clear description of the risk",
      "clause_reference": "Section X or null",
      "suggestion": "Recommended action to mitigate"
    }
  ],
  "summary": "2-3 sentence risk overview"
}"#;

const COMPARISON_SCHEMA: &str = r#"{
  "differences": [
    {
      "category": "parties|payment|term|liability|indemnification|confidentiality|termination|other",
      "diff_type": "substantive|formatting",
      "description": "What changed and why it matters",
      "text_a": "Exact text from document A or null",
      "text_b": "Exact text from document B or null",
      "significance": "high|medium|low"
    }
  ],
  "summary": "Overall comparison summary"
}"#;
