import { useCallback, useEffect, useState } from "react";
import type { Document, DocumentStats } from "@/types";
import {
  listDocuments,
  getDocumentStats,
  deleteDocument as deleteDocCmd,
} from "@/lib/commands";

export function useDocuments() {
  const [documents, setDocuments] = useState<Document[]>([]);
  const [stats, setStats] = useState<DocumentStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const [docs, docStats] = await Promise.all([
        listDocuments(),
        getDocumentStats(),
      ]);
      setDocuments(docs);
      setStats(docStats);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  const removeDocument = useCallback(
    async (id: string) => {
      await deleteDocCmd(id);
      await refresh();
    },
    [refresh],
  );

  useEffect(() => {
    refresh();
  }, [refresh]);

  return { documents, stats, loading, error, refresh, removeDocument };
}
