use crate::ai::{ContractType, ExtractionResponse, RiskFlag};

pub fn apply_rules(extraction: &ExtractionResponse, contract_type: &ContractType) -> Vec<RiskFlag> {
    let mut flags = Vec::new();

    // Universal rules
    check_missing_governing_law(extraction, &mut flags);
    check_no_termination_clause(extraction, &mut flags);

    // Contract-type-specific rules
    match contract_type {
        ContractType::Nda => apply_nda_rules(extraction, &mut flags),
        ContractType::ServiceAgreement => apply_service_rules(extraction, &mut flags),
        ContractType::Lease => apply_lease_rules(extraction, &mut flags),
    }

    flags
}

fn check_missing_governing_law(extraction: &ExtractionResponse, flags: &mut Vec<RiskFlag>) {
    let has_governing_law = extraction
        .clauses
        .iter()
        .any(|c| c.clause_type == "governing_law");

    if !has_governing_law {
        flags.push(RiskFlag {
            category: "governing_law".to_string(),
            severity: "medium".to_string(),
            description: "No governing law clause found. Disputes may be harder to resolve without a specified jurisdiction.".to_string(),
            clause_reference: None,
            suggestion: Some("Add a governing law clause specifying the applicable jurisdiction.".to_string()),
        });
    }
}

fn check_no_termination_clause(extraction: &ExtractionResponse, flags: &mut Vec<RiskFlag>) {
    let has_termination = extraction
        .clauses
        .iter()
        .any(|c| {
            c.clause_type.contains("termination")
                || c.clause_type.contains("term_and")
                || c.clause_type.contains("lease_term")
        });

    if !has_termination {
        flags.push(RiskFlag {
            category: "termination".to_string(),
            severity: "high".to_string(),
            description: "No termination clause found. Without clear termination terms, exiting this agreement may be difficult.".to_string(),
            clause_reference: None,
            suggestion: Some("Add explicit termination provisions including notice period and termination for cause/convenience.".to_string()),
        });
    }
}

fn apply_nda_rules(extraction: &ExtractionResponse, flags: &mut Vec<RiskFlag>) {
    let has_exclusions = extraction
        .clauses
        .iter()
        .any(|c| c.clause_type == "exclusions");

    if !has_exclusions {
        flags.push(RiskFlag {
            category: "confidentiality".to_string(),
            severity: "high".to_string(),
            description: "No exclusions to confidential information defined. This could mean publicly available information is improperly classified as confidential.".to_string(),
            clause_reference: None,
            suggestion: Some("Add standard exclusions: publicly available info, independently developed info, info received from third parties.".to_string()),
        });
    }

    let has_duration = extraction
        .clauses
        .iter()
        .any(|c| c.clause_type == "term_and_duration");

    if !has_duration && extraction.termination_date.is_none() {
        flags.push(RiskFlag {
            category: "termination".to_string(),
            severity: "medium".to_string(),
            description: "NDA has no specified duration or expiration. Confidentiality obligations may be perpetual.".to_string(),
            clause_reference: None,
            suggestion: Some("Specify a reasonable duration for confidentiality obligations (typically 2-5 years).".to_string()),
        });
    }
}

fn apply_service_rules(extraction: &ExtractionResponse, flags: &mut Vec<RiskFlag>) {
    let has_indemnification = extraction
        .clauses
        .iter()
        .any(|c| c.clause_type == "indemnification");

    if !has_indemnification {
        flags.push(RiskFlag {
            category: "indemnification".to_string(),
            severity: "high".to_string(),
            description: "No indemnification clause found. Without indemnification, there is no protection against third-party claims.".to_string(),
            clause_reference: None,
            suggestion: Some("Add mutual indemnification with reasonable caps tied to contract value.".to_string()),
        });
    }

    let has_liability_limit = extraction
        .clauses
        .iter()
        .any(|c| c.clause_type == "limitation_of_liability");

    if !has_liability_limit {
        flags.push(RiskFlag {
            category: "liability".to_string(),
            severity: "high".to_string(),
            description: "No limitation of liability clause found. Exposure to damages is potentially unlimited.".to_string(),
            clause_reference: None,
            suggestion: Some("Add a limitation of liability clause capping damages (typically 1-2x annual contract value).".to_string()),
        });
    }

    let has_ip = extraction
        .clauses
        .iter()
        .any(|c| c.clause_type == "intellectual_property");

    if !has_ip {
        flags.push(RiskFlag {
            category: "other".to_string(),
            severity: "medium".to_string(),
            description: "No intellectual property clause found. IP ownership of deliverables may be unclear.".to_string(),
            clause_reference: None,
            suggestion: Some("Add clear IP assignment or licensing terms for work product.".to_string()),
        });
    }
}

fn apply_lease_rules(extraction: &ExtractionResponse, flags: &mut Vec<RiskFlag>) {
    let has_security_deposit = extraction
        .clauses
        .iter()
        .any(|c| c.clause_type == "security_deposit");

    if !has_security_deposit {
        flags.push(RiskFlag {
            category: "payment".to_string(),
            severity: "medium".to_string(),
            description: "No security deposit clause found. Terms for deposit handling and return are undefined.".to_string(),
            clause_reference: None,
            suggestion: Some("Add security deposit terms including amount, conditions for withholding, and return timeline.".to_string()),
        });
    }

    let has_maintenance = extraction
        .clauses
        .iter()
        .any(|c| c.clause_type == "maintenance_and_repairs");

    if !has_maintenance {
        flags.push(RiskFlag {
            category: "other".to_string(),
            severity: "medium".to_string(),
            description: "No maintenance and repairs clause found. Responsibilities for property upkeep are unclear.".to_string(),
            clause_reference: None,
            suggestion: Some("Define maintenance responsibilities for both landlord and tenant.".to_string()),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::ExtractedClause;

    fn make_extraction(clauses: Vec<&str>) -> ExtractionResponse {
        ExtractionResponse {
            parties: vec!["A".into(), "B".into()],
            effective_date: None,
            termination_date: None,
            clauses: clauses
                .into_iter()
                .map(|ct| ExtractedClause {
                    clause_type: ct.to_string(),
                    title: ct.to_string(),
                    text: "test text".to_string(),
                    section_reference: None,
                    importance: "medium".to_string(),
                })
                .collect(),
            contract_type: "nda".to_string(),
            raw_json: "{}".to_string(),
        }
    }

    #[test]
    fn test_missing_governing_law_flagged() {
        let ext = make_extraction(vec!["confidentiality"]);
        let flags = apply_rules(&ext, &ContractType::Nda);
        assert!(flags.iter().any(|f| f.category == "governing_law"));
    }

    #[test]
    fn test_governing_law_present_no_flag() {
        let ext = make_extraction(vec!["governing_law", "term_and_duration", "exclusions"]);
        let flags = apply_rules(&ext, &ContractType::Nda);
        assert!(!flags.iter().any(|f| f.category == "governing_law"));
    }

    #[test]
    fn test_nda_missing_exclusions() {
        let ext = make_extraction(vec!["governing_law", "term_and_duration"]);
        let flags = apply_rules(&ext, &ContractType::Nda);
        assert!(flags.iter().any(|f| f.category == "confidentiality"));
    }

    #[test]
    fn test_service_missing_indemnification() {
        let ext = make_extraction(vec!["scope_of_services", "governing_law", "term_and_termination"]);
        let flags = apply_rules(&ext, &ContractType::ServiceAgreement);
        assert!(flags.iter().any(|f| f.category == "indemnification"));
        assert!(flags.iter().any(|f| f.category == "liability"));
    }

    #[test]
    fn test_lease_missing_deposit() {
        let ext = make_extraction(vec!["premises_description", "rent_and_payment", "lease_term", "governing_law"]);
        let flags = apply_rules(&ext, &ContractType::Lease);
        assert!(flags.iter().any(|f| f.category == "payment"));
    }
}
