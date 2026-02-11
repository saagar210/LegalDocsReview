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
      setDocuments([]);
      setStats(null);
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  const removeDocument = useCallback(
    async (id: string) => {
      setError(null);
      try {
        await deleteDocCmd(id);
        await refresh();
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        setError(message);
        throw err;
      }
    },
    [refresh],
  );

  useEffect(() => {
    refresh();
  }, [refresh]);

  return { documents, stats, loading, error, refresh, removeDocument };
}
