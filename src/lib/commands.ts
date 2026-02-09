import { invoke } from "@tauri-apps/api/core";
import type { Document, DocumentStats } from "@/types";

export async function uploadDocument(
  filePath: string,
  contractType: string,
): Promise<Document> {
  return invoke<Document>("upload_document", {
    filePath,
    contractType,
  });
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
