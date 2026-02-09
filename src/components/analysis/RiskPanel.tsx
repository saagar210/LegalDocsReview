import { AlertTriangle, CheckCircle, AlertCircle } from "lucide-react";
import type { RiskFlag, RiskLevel } from "@/types";
import { RISK_LEVEL_BG } from "@/types";

interface RiskPanelProps {
  score: number;
  level: RiskLevel;
  flags: RiskFlag[];
  summary: string | null;
}

function RiskMeter({ score, level }: { score: number; level: RiskLevel }) {
  const color =
    level === "low"
      ? "bg-green-500"
      : level === "medium"
        ? "bg-yellow-500"
        : "bg-red-500";

  return (
    <div className="mb-4">
      <div className="flex items-center justify-between mb-2">
        <span className="text-sm text-gray-500">Risk Score</span>
        <span className={`text-2xl font-bold ${
          level === "low" ? "text-green-600" : level === "medium" ? "text-yellow-600" : "text-red-600"
        }`}>
          {score}/100
        </span>
      </div>
      <div className="w-full bg-gray-200 rounded-full h-3">
        <div
          className={`h-3 rounded-full transition-all ${color}`}
          style={{ width: `${Math.min(score, 100)}%` }}
        />
      </div>
      <div className="flex justify-between text-xs text-gray-400 mt-1">
        <span>Low Risk</span>
        <span>High Risk</span>
      </div>
    </div>
  );
}

function FlagItem({ flag }: { flag: RiskFlag }) {
  const Icon =
    flag.severity === "high"
      ? AlertTriangle
      : flag.severity === "medium"
        ? AlertCircle
        : CheckCircle;

  const iconColor =
    flag.severity === "high"
      ? "text-red-500"
      : flag.severity === "medium"
        ? "text-yellow-500"
        : "text-green-500";

  return (
    <div className="flex gap-3 p-3 bg-gray-50 rounded-lg">
      <Icon className={`h-5 w-5 flex-shrink-0 mt-0.5 ${iconColor}`} />
      <div className="min-w-0">
        <div className="flex items-center gap-2 mb-1">
          <span className="text-xs font-medium text-gray-500 uppercase">
            {flag.category.replace(/_/g, " ")}
          </span>
          <span
            className={`text-xs px-1.5 py-0.5 rounded ${
              RISK_LEVEL_BG[flag.severity]
            }`}
          >
            {flag.severity}
          </span>
        </div>
        <p className="text-sm text-gray-700">{flag.description}</p>
        {flag.suggestion && (
          <p className="text-xs text-brand-600 mt-1">{flag.suggestion}</p>
        )}
      </div>
    </div>
  );
}

export default function RiskPanel({
  score,
  level,
  flags,
  summary,
}: RiskPanelProps) {
  return (
    <div className="bg-white rounded-xl border border-gray-200 p-6">
      <div className="flex items-center justify-between mb-4">
        <h2 className="font-semibold">Risk Assessment</h2>
        <span className={`text-xs px-2 py-1 rounded-full ${RISK_LEVEL_BG[level]}`}>
          {level.toUpperCase()}
        </span>
      </div>

      <RiskMeter score={score} level={level} />

      {summary && (
        <p className="text-sm text-gray-600 mb-4 p-3 bg-blue-50 rounded-lg">
          {summary}
        </p>
      )}

      {flags.length > 0 && (
        <div className="space-y-2">
          <h3 className="text-sm font-medium text-gray-700">
            Risk Flags ({flags.length})
          </h3>
          {flags.map((flag, i) => (
            <FlagItem key={i} flag={flag} />
          ))}
        </div>
      )}
    </div>
  );
}
