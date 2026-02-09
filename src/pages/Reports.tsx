import { useEffect, useState, useCallback } from "react";
import { useParams, useNavigate } from "react-router";
import { ArrowLeft, FileBarChart, Download, Copy } from "lucide-react";
import toast from "react-hot-toast";
import { getDocument, getReports } from "@/lib/commands";
import type { Report } from "@/lib/commands";
import type { Document } from "@/types";

function Reports() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [doc, setDoc] = useState<Document | null>(null);
  const [reports, setReports] = useState<Report[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedReport, setSelectedReport] = useState<Report | null>(null);

  const load = useCallback(async () => {
    if (!id) return;
    try {
      setLoading(true);
      const [document, rpts] = await Promise.all([
        getDocument(id),
        getReports(id),
      ]);
      setDoc(document);
      setReports(rpts);
      if (rpts.length > 0) {
        setSelectedReport(rpts[0] ?? null);
      }
    } catch (err) {
      toast.error(
        `Failed to load: ${err instanceof Error ? err.message : String(err)}`,
      );
    } finally {
      setLoading(false);
    }
  }, [id]);

  useEffect(() => {
    load();
  }, [load]);

  const handleCopy = useCallback(() => {
    if (!selectedReport) return;
    navigator.clipboard.writeText(selectedReport.content);
    toast.success("Report copied to clipboard");
  }, [selectedReport]);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="animate-spin h-8 w-8 border-2 border-brand-600 border-t-transparent rounded-full" />
      </div>
    );
  }

  return (
    <div className="p-8 max-w-5xl">
      <div className="flex items-center gap-4 mb-6">
        <button
          onClick={() => navigate(`/documents/${id ?? ""}`)}
          className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
        >
          <ArrowLeft className="h-5 w-5" />
        </button>
        <div>
          <h1 className="text-2xl font-bold">Reports</h1>
          <p className="text-gray-500 text-sm">{doc?.filename}</p>
        </div>
      </div>

      {reports.length === 0 ? (
        <div className="bg-white rounded-xl border border-gray-200 p-12 text-center">
          <FileBarChart className="h-12 w-12 text-gray-300 mx-auto mb-4" />
          <p className="text-gray-500">No reports generated yet</p>
          <button
            onClick={() => navigate(`/documents/${id ?? ""}`)}
            className="text-brand-600 hover:text-brand-700 text-sm font-medium mt-2"
          >
            Go to document to generate a report
          </button>
        </div>
      ) : (
        <div className="grid grid-cols-4 gap-6">
          <div className="space-y-2">
            <h2 className="text-sm font-medium text-gray-500 mb-3">
              Generated Reports
            </h2>
            {reports.map((report) => (
              <button
                key={report.id}
                onClick={() => setSelectedReport(report)}
                className={`w-full text-left p-3 rounded-lg border text-sm transition-colors ${
                  selectedReport?.id === report.id
                    ? "border-brand-500 bg-brand-50"
                    : "border-gray-200 hover:border-gray-300"
                }`}
              >
                <p className="font-medium text-gray-900">
                  {report.report_type.replace(/_/g, " ")}
                </p>
                <p className="text-xs text-gray-500">
                  {new Date(report.created_at).toLocaleString()}
                </p>
              </button>
            ))}
          </div>

          <div className="col-span-3">
            {selectedReport && (
              <div className="bg-white rounded-xl border border-gray-200">
                <div className="flex items-center justify-between px-6 py-4 border-b border-gray-200">
                  <h2 className="font-semibold">Report Preview</h2>
                  <div className="flex items-center gap-2">
                    <button
                      onClick={handleCopy}
                      className="flex items-center gap-1 text-sm text-gray-500 hover:text-gray-700 px-3 py-1.5 rounded-lg hover:bg-gray-100"
                    >
                      <Copy className="h-3.5 w-3.5" />
                      Copy
                    </button>
                    {selectedReport.export_path && (
                      <button className="flex items-center gap-1 text-sm text-brand-600 hover:text-brand-700 px-3 py-1.5 rounded-lg hover:bg-brand-50">
                        <Download className="h-3.5 w-3.5" />
                        Open File
                      </button>
                    )}
                  </div>
                </div>
                <pre className="p-6 text-sm text-gray-700 whitespace-pre-wrap max-h-[600px] overflow-y-auto font-mono">
                  {selectedReport.content}
                </pre>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

export default Reports;
