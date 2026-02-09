import { ChevronDown, ChevronRight } from "lucide-react";
import { useState } from "react";
import type { ExtractedClause } from "@/types";

function ClauseRow({ clause }: { clause: ExtractedClause }) {
  const [expanded, setExpanded] = useState(false);

  const importanceColor = {
    high: "bg-red-100 text-red-800",
    medium: "bg-yellow-100 text-yellow-800",
    low: "bg-green-100 text-green-800",
  }[clause.importance] ?? "bg-gray-100 text-gray-800";

  return (
    <div className="border-b border-gray-100 last:border-0">
      <button
        onClick={() => setExpanded(!expanded)}
        className="w-full flex items-center justify-between px-4 py-3 hover:bg-gray-50 transition-colors text-left"
      >
        <div className="flex items-center gap-3">
          {expanded ? (
            <ChevronDown className="h-4 w-4 text-gray-400 flex-shrink-0" />
          ) : (
            <ChevronRight className="h-4 w-4 text-gray-400 flex-shrink-0" />
          )}
          <div>
            <p className="text-sm font-medium text-gray-900">{clause.title}</p>
            <p className="text-xs text-gray-500">
              {clause.clause_type.replace(/_/g, " ")}
              {clause.section_reference && ` — ${clause.section_reference}`}
            </p>
          </div>
        </div>
        <span className={`text-xs px-2 py-0.5 rounded-full ${importanceColor}`}>
          {clause.importance}
        </span>
      </button>
      {expanded && (
        <div className="px-4 pb-4 pl-11">
          <p className="text-sm text-gray-700 whitespace-pre-wrap bg-gray-50 p-3 rounded-lg">
            {clause.text}
          </p>
        </div>
      )}
    </div>
  );
}

export default function ClauseTable({
  clauses,
}: {
  clauses: ExtractedClause[];
}) {
  if (clauses.length === 0) {
    return (
      <div className="p-6 text-center text-gray-400 text-sm">
        No clauses extracted yet
      </div>
    );
  }

  return (
    <div>
      {clauses.map((clause, i) => (
        <ClauseRow key={`${clause.clause_type}-${i}`} clause={clause} />
      ))}
    </div>
  );
}
