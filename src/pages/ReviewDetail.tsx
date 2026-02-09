import { useEffect, useState, useCallback } from "react";
import { useParams, useNavigate } from "react-router";
import {
  ArrowLeft,
  FileText,
  Trash2,
  Brain,
  FileBarChart,
  Loader2,
} from "lucide-react";
import toast from "react-hot-toast";
import {
  getDocument,
  deleteDocument,
  analyzeDocument,
  getExtractions,
  getRiskAssessments,
  generateReport,
} from "@/lib/commands";
import type { AnalysisResult } from "@/lib/commands";
import { CONTRACT_TYPE_LABELS, STATUS_LABELS } from "@/types";
import type { Document, ExtractedClause, RiskFlag, RiskLevel } from "@/types";
import ClauseTable from "@/components/analysis/ClauseTable";
import RiskPanel from "@/components/analysis/RiskPanel";

function ReviewDetail() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [doc, setDoc] = useState<Document | null>(null);
  const [loading, setLoading] = useState(true);
  const [analyzing, setAnalyzing] = useState(false);
  const [generatingReport, setGeneratingReport] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [clauses, setClauses] = useState<ExtractedClause[]>([]);
  const [riskScore, setRiskScore] = useState<number | null>(null);
  const [riskLevel, setRiskLevel] = useState<RiskLevel | null>(null);
  const [riskFlags, setRiskFlags] = useState<RiskFlag[]>([]);
  const [riskSummary, setRiskSummary] = useState<string | null>(null);

  const loadDocument = useCallback(async () => {
    if (!id) return;
    try {
      setLoading(true);
      const document = await getDocument(id);
      setDoc(document);

      // Load existing analysis if available
      const [exts, risks] = await Promise.all([
        getExtractions(id).catch(() => []),
        getRiskAssessments(id).catch(() => []),
      ]);

      if (exts.length > 0) {
        const latestExt = exts[0];
        if (latestExt) {
          try {
            const data = JSON.parse(latestExt.extracted_data) as {
              clauses?: ExtractedClause[];
            };
            setClauses(data.clauses ?? []);
          } catch {
            // extraction data not parseable
          }
        }
      }

      if (risks.length > 0) {
        const latestRisk = risks[0];
        if (latestRisk) {
          setRiskScore(latestRisk.overall_score);
          setRiskLevel(latestRisk.risk_level as RiskLevel);
          setRiskSummary(latestRisk.summary);
          try {
            const flags = JSON.parse(latestRisk.flags) as RiskFlag[];
            setRiskFlags(flags);
          } catch {
            // flags not parseable
          }
        }
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, [id]);

  useEffect(() => {
    loadDocument();
  }, [loadDocument]);

  const handleAnalyze = useCallback(async () => {
    if (!id) return;
    setAnalyzing(true);
    try {
      const result: AnalysisResult = await analyzeDocument(id);
      const data = result.extraction_data as {
        clauses?: ExtractedClause[];
      };
      setClauses(data.clauses ?? []);
      setRiskScore(result.overall_score);
      setRiskLevel(result.risk_level as RiskLevel);
      setRiskFlags(result.risk_flags as unknown as RiskFlag[]);
      setRiskSummary(result.summary);
      toast.success("Analysis complete");
      // Reload document to get updated status
      const updated = await getDocument(id);
      setDoc(updated);
    } catch (err) {
      toast.error(
        `Analysis failed: ${err instanceof Error ? err.message : String(err)}`,
      );
    } finally {
      setAnalyzing(false);
    }
  }, [id]);

  const handleGenerateReport = useCallback(async () => {
    if (!id) return;
    setGeneratingReport(true);
    try {
      await generateReport(id);
      toast.success("Report generated");
    } catch (err) {
      toast.error(
        `Report failed: ${err instanceof Error ? err.message : String(err)}`,
      );
    } finally {
      setGeneratingReport(false);
    }
  }, [id]);

  const handleDelete = useCallback(async () => {
    if (!id) return;
    try {
      await deleteDocument(id);
      toast.success("Document deleted");
      navigate("/");
    } catch (err) {
      toast.error(
        `Delete failed: ${err instanceof Error ? err.message : String(err)}`,
      );
    }
  }, [id, navigate]);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="animate-spin h-8 w-8 border-2 border-brand-600 border-t-transparent rounded-full" />
      </div>
    );
  }

  if (error || !doc) {
    return (
      <div className="p-8">
        <div className="bg-red-50 text-red-700 p-4 rounded-lg">
          {error ?? "Document not found"}
        </div>
      </div>
    );
  }

  const canAnalyze =
    doc.raw_text &&
    (doc.processing_status === "extracted" ||
      doc.processing_status === "analyzed");

  return (
    <div className="p-8 max-w-7xl">
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-4">
          <button
            onClick={() => navigate("/")}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <ArrowLeft className="h-5 w-5" />
          </button>
          <div>
            <h1 className="text-2xl font-bold">{doc.filename}</h1>
            <p className="text-gray-500 text-sm">
              {CONTRACT_TYPE_LABELS[doc.contract_type]} &middot;{" "}
              {STATUS_LABELS[doc.processing_status]}
            </p>
          </div>
        </div>
        <div className="flex items-center gap-2">
          {canAnalyze && (
            <button
              onClick={handleAnalyze}
              disabled={analyzing}
              className="flex items-center gap-2 bg-brand-600 text-white px-4 py-2 rounded-lg hover:bg-brand-700 disabled:opacity-50 transition-colors text-sm font-medium"
            >
              {analyzing ? (
                <Loader2 className="h-4 w-4 animate-spin" />
              ) : (
                <Brain className="h-4 w-4" />
              )}
              {analyzing ? "Analyzing..." : "Run AI Analysis"}
            </button>
          )}
          {riskScore !== null && (
            <button
              onClick={handleGenerateReport}
              disabled={generatingReport}
              className="flex items-center gap-2 bg-gray-800 text-white px-4 py-2 rounded-lg hover:bg-gray-900 disabled:opacity-50 transition-colors text-sm font-medium"
            >
              {generatingReport ? (
                <Loader2 className="h-4 w-4 animate-spin" />
              ) : (
                <FileBarChart className="h-4 w-4" />
              )}
              Generate Report
            </button>
          )}
          <button
            onClick={handleDelete}
            className="flex items-center gap-2 text-red-500 hover:text-red-700 hover:bg-red-50 px-3 py-2 rounded-lg transition-colors text-sm"
          >
            <Trash2 className="h-4 w-4" />
            Delete
          </button>
        </div>
      </div>

      <div className="grid grid-cols-3 gap-6">
        <div className="col-span-2 space-y-6">
          {/* Document metadata */}
          <div className="bg-white rounded-xl border border-gray-200 p-6">
            <h2 className="font-semibold mb-4">Document Details</h2>
            <dl className="grid grid-cols-2 gap-4 text-sm">
              <div>
                <dt className="text-gray-500">File Size</dt>
                <dd className="font-medium">
                  {(doc.file_size / 1024).toFixed(1)} KB
                </dd>
              </div>
              <div>
                <dt className="text-gray-500">Pages</dt>
                <dd className="font-medium">{doc.page_count ?? "N/A"}</dd>
              </div>
              <div>
                <dt className="text-gray-500">Uploaded</dt>
                <dd className="font-medium">
                  {new Date(doc.created_at).toLocaleString()}
                </dd>
              </div>
              <div>
                <dt className="text-gray-500">Hash</dt>
                <dd className="font-mono text-xs">{doc.file_hash}</dd>
              </div>
            </dl>
          </div>

          {/* Extracted clauses */}
          {clauses.length > 0 && (
            <div className="bg-white rounded-xl border border-gray-200">
              <div className="px-4 py-3 border-b border-gray-200">
                <h2 className="font-semibold">
                  Extracted Clauses ({clauses.length})
                </h2>
              </div>
              <ClauseTable clauses={clauses} />
            </div>
          )}

          {/* Extracted text preview */}
          {doc.raw_text && clauses.length === 0 && (
            <div className="bg-white rounded-xl border border-gray-200 p-6">
              <h2 className="font-semibold mb-4">Extracted Text</h2>
              <pre className="text-sm text-gray-700 whitespace-pre-wrap max-h-96 overflow-y-auto bg-gray-50 p-4 rounded-lg">
                {doc.raw_text}
              </pre>
            </div>
          )}

          {doc.error_message && (
            <div className="bg-red-50 border border-red-200 rounded-xl p-6">
              <h2 className="font-semibold text-red-800 mb-2">Error</h2>
              <p className="text-sm text-red-700">{doc.error_message}</p>
            </div>
          )}
        </div>

        <div className="space-y-6">
          {/* Status card */}
          <div className="bg-white rounded-xl border border-gray-200 p-6">
            <h2 className="font-semibold mb-4">Status</h2>
            <div className="flex items-center gap-3">
              <FileText className="h-8 w-8 text-brand-500" />
              <div>
                <p className="font-medium">
                  {STATUS_LABELS[doc.processing_status]}
                </p>
                <p className="text-xs text-gray-500">
                  {doc.processing_status === "extracted"
                    ? "Ready for AI analysis"
                    : doc.processing_status === "pending"
                      ? "Text extraction needed"
                      : doc.processing_status === "analyzed"
                        ? "Analysis complete"
                        : ""}
                </p>
              </div>
            </div>
          </div>

          {/* Risk panel */}
          {riskScore !== null && riskLevel && (
            <RiskPanel
              score={riskScore}
              level={riskLevel}
              flags={riskFlags}
              summary={riskSummary}
            />
          )}

          {/* Placeholder if no analysis yet */}
          {riskScore === null && (
            <div className="bg-gray-50 rounded-xl border border-dashed border-gray-300 p-6 text-center">
              <Brain className="h-8 w-8 text-gray-300 mx-auto mb-2" />
              <p className="text-sm text-gray-400">
                Run AI analysis to see risk assessment
              </p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default ReviewDetail;
