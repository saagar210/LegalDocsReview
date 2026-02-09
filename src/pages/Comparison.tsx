import { useState, useCallback, useEffect } from "react";
import { GitCompareArrows, Loader2, ArrowRight } from "lucide-react";
import toast from "react-hot-toast";
import {
  listDocuments,
  compareDocuments,
} from "@/lib/commands";
import type { Comparison as ComparisonType } from "@/lib/commands";
import type { Document } from "@/types";
import { CONTRACT_TYPE_LABELS } from "@/types";

interface Difference {
  category: string;
  diff_type: string;
  description: string;
  text_a: string | null;
  text_b: string | null;
  significance: string;
}

function DiffCard({ diff }: { diff: Difference }) {
  const sigColor = {
    high: "border-red-200 bg-red-50",
    medium: "border-yellow-200 bg-yellow-50",
    low: "border-gray-200 bg-gray-50",
  }[diff.significance] ?? "border-gray-200 bg-gray-50";

  const typeBadge =
    diff.diff_type === "substantive"
      ? "bg-red-100 text-red-800"
      : "bg-gray-100 text-gray-600";

  return (
    <div className={`border rounded-lg p-4 ${sigColor}`}>
      <div className="flex items-center gap-2 mb-2">
        <span className="text-xs font-medium uppercase text-gray-500">
          {diff.category.replace(/_/g, " ")}
        </span>
        <span className={`text-xs px-1.5 py-0.5 rounded ${typeBadge}`}>
          {diff.diff_type}
        </span>
        <span className="text-xs text-gray-400 ml-auto">
          {diff.significance} significance
        </span>
      </div>
      <p className="text-sm text-gray-700 mb-3">{diff.description}</p>
      {(diff.text_a || diff.text_b) && (
        <div className="grid grid-cols-2 gap-3">
          {diff.text_a && (
            <div className="p-2 bg-white rounded border border-red-200">
              <p className="text-xs text-red-600 font-medium mb-1">
                Document A
              </p>
              <p className="text-xs text-gray-600">{diff.text_a}</p>
            </div>
          )}
          {diff.text_b && (
            <div className="p-2 bg-white rounded border border-green-200">
              <p className="text-xs text-green-600 font-medium mb-1">
                Document B
              </p>
              <p className="text-xs text-gray-600">{diff.text_b}</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

function Comparison() {
  const [documents, setDocuments] = useState<Document[]>([]);
  const [docAId, setDocAId] = useState("");
  const [docBId, setDocBId] = useState("");
  const [comparing, setComparing] = useState(false);
  const [result, setResult] = useState<ComparisonType | null>(null);
  const [differences, setDifferences] = useState<Difference[]>([]);

  useEffect(() => {
    listDocuments()
      .then((docs) =>
        setDocuments(docs.filter((d) => d.raw_text !== null)),
      )
      .catch(() => toast.error("Failed to load documents"));
  }, []);

  const handleCompare = useCallback(async () => {
    if (!docAId || !docBId) {
      toast.error("Select two documents to compare");
      return;
    }
    if (docAId === docBId) {
      toast.error("Select two different documents");
      return;
    }
    setComparing(true);
    try {
      const comp = await compareDocuments(docAId, docBId);
      setResult(comp);
      const diffs = JSON.parse(comp.differences) as Difference[];
      setDifferences(diffs);
      toast.success("Comparison complete");
    } catch (err) {
      toast.error(
        `Comparison failed: ${err instanceof Error ? err.message : String(err)}`,
      );
    } finally {
      setComparing(false);
    }
  }, [docAId, docBId]);

  return (
    <div className="p-8 max-w-5xl">
      <div className="mb-8">
        <h1 className="text-2xl font-bold">Document Comparison</h1>
        <p className="text-gray-500 mt-1">
          Compare two contract versions to identify differences
        </p>
      </div>

      <div className="bg-white rounded-xl border border-gray-200 p-6 mb-6">
        <div className="flex items-end gap-4">
          <div className="flex-1">
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Document A
            </label>
            <select
              value={docAId}
              onChange={(e) => setDocAId(e.target.value)}
              className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
            >
              <option value="">Select document...</option>
              {documents.map((doc) => (
                <option key={doc.id} value={doc.id}>
                  {doc.filename} ({CONTRACT_TYPE_LABELS[doc.contract_type]})
                </option>
              ))}
            </select>
          </div>
          <ArrowRight className="h-5 w-5 text-gray-400 mb-2" />
          <div className="flex-1">
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Document B
            </label>
            <select
              value={docBId}
              onChange={(e) => setDocBId(e.target.value)}
              className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
            >
              <option value="">Select document...</option>
              {documents.map((doc) => (
                <option key={doc.id} value={doc.id}>
                  {doc.filename} ({CONTRACT_TYPE_LABELS[doc.contract_type]})
                </option>
              ))}
            </select>
          </div>
          <button
            onClick={handleCompare}
            disabled={comparing || !docAId || !docBId}
            className="flex items-center gap-2 bg-brand-600 text-white px-6 py-2 rounded-lg hover:bg-brand-700 disabled:opacity-50 transition-colors text-sm font-medium whitespace-nowrap"
          >
            {comparing ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              <GitCompareArrows className="h-4 w-4" />
            )}
            {comparing ? "Comparing..." : "Compare"}
          </button>
        </div>
      </div>

      {result && (
        <>
          {result.summary && (
            <div className="bg-blue-50 border border-blue-200 rounded-xl p-4 mb-6">
              <p className="text-sm text-blue-800">{result.summary}</p>
            </div>
          )}

          <div className="space-y-4">
            <h2 className="font-semibold">
              Differences ({differences.length})
            </h2>
            {differences.length === 0 ? (
              <p className="text-gray-500 text-sm">
                No significant differences found
              </p>
            ) : (
              differences.map((diff, i) => <DiffCard key={i} diff={diff} />)
            )}
          </div>
        </>
      )}
    </div>
  );
}

export default Comparison;
