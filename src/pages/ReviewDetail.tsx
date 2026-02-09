import { useEffect, useState, useCallback } from "react";
import { useParams, useNavigate } from "react-router";
import { ArrowLeft, FileText, Trash2 } from "lucide-react";
import toast from "react-hot-toast";
import { getDocument, deleteDocument } from "@/lib/commands";
import { CONTRACT_TYPE_LABELS, STATUS_LABELS } from "@/types";
import type { Document } from "@/types";

function ReviewDetail() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [doc, setDoc] = useState<Document | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadDocument = useCallback(async () => {
    if (!id) return;
    try {
      setLoading(true);
      const document = await getDocument(id);
      setDoc(document);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, [id]);

  useEffect(() => {
    loadDocument();
  }, [loadDocument]);

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

  return (
    <div className="p-8 max-w-6xl">
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
        <button
          onClick={handleDelete}
          className="flex items-center gap-2 text-red-500 hover:text-red-700 hover:bg-red-50 px-3 py-2 rounded-lg transition-colors text-sm"
        >
          <Trash2 className="h-4 w-4" />
          Delete
        </button>
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

          {/* Extracted text preview */}
          {doc.raw_text && (
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
                      : ""}
                </p>
              </div>
            </div>
          </div>

          {/* Placeholder for risk panel (Sprint 3) */}
          <div className="bg-gray-50 rounded-xl border border-dashed border-gray-300 p-6 text-center">
            <p className="text-sm text-gray-400">
              Risk analysis will appear here after AI processing
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default ReviewDetail;
