import { invoke } from "@tauri-apps/api/core";
import type {
  Document,
  DocumentStats,
  Extraction,
  RiskAssessment,
  RiskDistribution,
} from "@/types";

// Documents
export async function uploadDocument(
  filePath: string,
  contractType: string,
): Promise<Document> {
  return invoke<Document>("upload_document", { filePath, contractType });
}

export async function extractDocumentText(
  documentId: string,
): Promise<Document> {
  return invoke<Document>("extract_document_text", { documentId });
}

export async function getDocument(documentId: string): Promise<Document> {
  return invoke<Document>("get_document", { documentId });
}

export async function listDocuments(): Promise<Document[]> {
  return invoke<Document[]>("list_documents");
}

export async function deleteDocument(documentId: string): Promise<void> {
  return invoke<void>("delete_document", { documentId });
}

export async function getDocumentStats(): Promise<DocumentStats> {
  return invoke<DocumentStats>("get_document_stats");
}

// Settings
export async function getSetting(key: string): Promise<string | null> {
  return invoke<string | null>("get_setting", { key });
}

export async function setSetting(key: string, value: string): Promise<void> {
  return invoke<void>("set_setting", { key, value });
}

export async function getAllSettings(): Promise<[string, string][]> {
  return invoke<[string, string][]>("get_all_settings");
}

export async function deleteSetting(key: string): Promise<void> {
  return invoke<void>("delete_setting", { key });
}

// Analysis
export interface AnalysisResult {
  extraction_id: string;
  risk_assessment_id: string;
  extraction_data: Record<string, unknown>;
  overall_score: number;
  risk_level: string;
  risk_flags: Record<string, unknown>[];
  summary: string | null;
}

export async function analyzeDocument(
  documentId: string,
): Promise<AnalysisResult> {
  return invoke<AnalysisResult>("analyze_document", { documentId });
}

export async function getExtractions(
  documentId: string,
): Promise<Extraction[]> {
  return invoke<Extraction[]>("get_extractions", { documentId });
}

export async function getRiskAssessments(
  documentId: string,
): Promise<RiskAssessment[]> {
  return invoke<RiskAssessment[]>("get_risk_assessments", { documentId });
}

export async function getRiskDistribution(): Promise<RiskDistribution> {
  return invoke<RiskDistribution>("get_risk_distribution");
}

// Comparison
export interface Comparison {
  id: string;
  document_a_id: string;
  document_b_id: string | null;
  template_id: string | null;
  comparison_type: string;
  differences: string;
  summary: string | null;
  ai_provider: string | null;
  created_at: string;
}

export async function compareDocuments(
  documentAId: string,
  documentBId: string,
): Promise<Comparison> {
  return invoke<Comparison>("compare_documents", { documentAId, documentBId });
}

export async function getComparisons(
  documentId: string,
): Promise<Comparison[]> {
  return invoke<Comparison[]>("get_comparisons", { documentId });
}

// Templates
export interface Template {
  id: string;
  name: string;
  contract_type: string;
  description: string | null;
  raw_text: string;
  extracted_data: string | null;
  created_at: string;
  updated_at: string;
}

export async function createTemplate(
  name: string,
  contractType: string,
  description: string | null,
  rawText: string,
): Promise<Template> {
  return invoke<Template>("create_template", {
    name,
    contractType,
    description,
    rawText,
  });
}

export async function listTemplates(): Promise<Template[]> {
  return invoke<Template[]>("list_templates");
}

export async function getTemplate(templateId: string): Promise<Template> {
  return invoke<Template>("get_template", { templateId });
}

export async function deleteTemplate(templateId: string): Promise<void> {
  return invoke<void>("delete_template", { templateId });
}

// Reports
export interface Report {
  id: string;
  document_id: string;
  report_type: string;
  content: string;
  export_path: string | null;
  format: string;
  created_at: string;
}

export async function generateReport(documentId: string): Promise<Report> {
  return invoke<Report>("generate_report", { documentId });
}

export async function getReports(documentId: string): Promise<Report[]> {
  return invoke<Report[]>("get_reports", { documentId });
}
