export type ContractType = "nda" | "service_agreement" | "lease";

export type ProcessingStatus =
  | "pending"
  | "extracted"
  | "analyzing"
  | "analyzed"
  | "error";

export type RiskLevel = "low" | "medium" | "high";

export interface Document {
  id: string;
  filename: string;
  original_path: string;
  stored_path: string;
  file_hash: string;
  file_size: number;
  contract_type: ContractType;
  raw_text: string | null;
  page_count: number | null;
  processing_status: ProcessingStatus;
  error_message: string | null;
  created_at: string;
  updated_at: string;
}

export interface DocumentStats {
  total: number;
  analyzed: number;
  pending: number;
  failed: number;
}

export interface Extraction {
  id: string;
  document_id: string;
  ai_provider: string;
  ai_model: string | null;
  contract_type: ContractType;
  extracted_data: string;
  confidence_score: number | null;
  processing_time_ms: number | null;
  created_at: string;
}

export interface ExtractedClause {
  clause_type: string;
  title: string;
  text: string;
  section_reference: string | null;
  importance: "high" | "medium" | "low";
}

export interface ExtractionData {
  parties: string[];
  effective_date: string | null;
  termination_date: string | null;
  clauses: ExtractedClause[];
  contract_type: string;
}

export interface RiskAssessment {
  id: string;
  document_id: string;
  extraction_id: string;
  overall_score: number;
  risk_level: RiskLevel;
  flags: string;
  summary: string | null;
  ai_provider: string;
  created_at: string;
}

export interface RiskFlag {
  category: string;
  severity: RiskLevel;
  description: string;
  clause_reference: string | null;
  suggestion: string | null;
}

export interface RiskDistribution {
  low: number;
  medium: number;
  high: number;
}

export const CONTRACT_TYPE_LABELS: Record<ContractType, string> = {
  nda: "Non-Disclosure Agreement",
  service_agreement: "Service Agreement",
  lease: "Lease Agreement",
};

export const RISK_LEVEL_COLORS: Record<RiskLevel, string> = {
  low: "text-risk-low",
  medium: "text-risk-medium",
  high: "text-risk-high",
};

export const RISK_LEVEL_BG: Record<RiskLevel, string> = {
  low: "bg-green-100 text-green-800",
  medium: "bg-yellow-100 text-yellow-800",
  high: "bg-red-100 text-red-800",
};

export const STATUS_LABELS: Record<ProcessingStatus, string> = {
  pending: "Pending",
  extracted: "Text Extracted",
  analyzing: "Analyzing...",
  analyzed: "Analyzed",
  error: "Error",
};
