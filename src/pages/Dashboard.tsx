import { Link } from "react-router";
import {
  FileText,
  Upload,
  AlertTriangle,
  CheckCircle,
  Clock,
} from "lucide-react";
import { useDocuments } from "@/hooks/useDocuments";
import { CONTRACT_TYPE_LABELS, STATUS_LABELS } from "@/types";
import type { Document } from "@/types";

function StatCard({
  label,
  value,
  icon: Icon,
  color,
}: {
  label: string;
  value: number;
  icon: React.ComponentType<{ className?: string }>;
  color: string;
}) {
  return (
    <div className="bg-white rounded-xl border border-gray-200 p-6">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm text-gray-500">{label}</p>
          <p className="text-3xl font-bold mt-1">{value}</p>
        </div>
        <div className={`p-3 rounded-lg ${color}`}>
          <Icon className="h-6 w-6" />
        </div>
      </div>
    </div>
  );
}

function DocumentRow({ doc }: { doc: Document }) {
  return (
    <Link
      to={`/documents/${doc.id}`}
      className="flex items-center justify-between px-4 py-3 hover:bg-gray-50 transition-colors"
    >
      <div className="flex items-center gap-3">
        <FileText className="h-5 w-5 text-gray-400" />
        <div>
          <p className="text-sm font-medium text-gray-900">{doc.filename}</p>
          <p className="text-xs text-gray-500">
            {CONTRACT_TYPE_LABELS[doc.contract_type]} &middot;{" "}
            {new Date(doc.created_at).toLocaleDateString()}
          </p>
        </div>
      </div>
      <span
        className={`text-xs px-2 py-1 rounded-full ${
          doc.processing_status === "analyzed"
            ? "bg-green-100 text-green-800"
            : doc.processing_status === "error"
              ? "bg-red-100 text-red-800"
              : "bg-gray-100 text-gray-600"
        }`}
      >
        {STATUS_LABELS[doc.processing_status]}
      </span>
    </Link>
  );
}

function Dashboard() {
  const { documents, stats, loading, error } = useDocuments();

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="animate-spin h-8 w-8 border-2 border-brand-600 border-t-transparent rounded-full" />
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-8">
        <div className="bg-red-50 text-red-700 p-4 rounded-lg">
          Failed to load dashboard: {error}
        </div>
      </div>
    );
  }

  return (
    <div className="p-8 max-w-6xl">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-2xl font-bold">Dashboard</h1>
          <p className="text-gray-500 mt-1">
            Overview of your document reviews
          </p>
        </div>
        <Link
          to="/upload"
          className="flex items-center gap-2 bg-brand-600 text-white px-4 py-2 rounded-lg hover:bg-brand-700 transition-colors text-sm font-medium"
        >
          <Upload className="h-4 w-4" />
          Upload Document
        </Link>
      </div>

      <div className="grid grid-cols-4 gap-4 mb-8">
        <StatCard
          label="Total Documents"
          value={stats?.total ?? 0}
          icon={FileText}
          color="bg-brand-50 text-brand-600"
        />
        <StatCard
          label="Analyzed"
          value={stats?.analyzed ?? 0}
          icon={CheckCircle}
          color="bg-green-50 text-green-600"
        />
        <StatCard
          label="Pending"
          value={stats?.pending ?? 0}
          icon={Clock}
          color="bg-yellow-50 text-yellow-600"
        />
        <StatCard
          label="Failed"
          value={stats?.failed ?? 0}
          icon={AlertTriangle}
          color="bg-red-50 text-red-600"
        />
      </div>

      <div className="bg-white rounded-xl border border-gray-200">
        <div className="px-4 py-3 border-b border-gray-200">
          <h2 className="font-semibold">Recent Documents</h2>
        </div>
        {documents.length === 0 ? (
          <div className="p-12 text-center">
            <FileText className="h-12 w-12 text-gray-300 mx-auto mb-4" />
            <p className="text-gray-500">No documents yet</p>
            <Link
              to="/upload"
              className="text-brand-600 hover:text-brand-700 text-sm font-medium mt-2 inline-block"
            >
              Upload your first document
            </Link>
          </div>
        ) : (
          <div className="divide-y divide-gray-100">
            {documents.slice(0, 10).map((doc) => (
              <DocumentRow key={doc.id} doc={doc} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

export default Dashboard;
