import { act, renderHook, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { useDocuments } from "@/hooks/useDocuments";

const mocks = vi.hoisted(() => ({
  listDocuments: vi.fn(),
  getDocumentStats: vi.fn(),
  deleteDocument: vi.fn(),
}));

vi.mock("@/lib/commands", () => ({
  listDocuments: mocks.listDocuments,
  getDocumentStats: mocks.getDocumentStats,
  deleteDocument: mocks.deleteDocument,
}));

const sampleDoc = {
  id: "doc-1",
  filename: "contract.pdf",
  original_path: "/tmp/contract.pdf",
  stored_path: "/data/contract.pdf",
  file_hash: "abc123",
  file_size: 1024,
  contract_type: "nda" as const,
  raw_text: null,
  page_count: null,
  processing_status: "pending" as const,
  error_message: null,
  created_at: "2026-01-01T00:00:00Z",
  updated_at: "2026-01-01T00:00:00Z",
};

const sampleStats = {
  total: 1,
  analyzed: 0,
  pending: 1,
  failed: 0,
};

describe("useDocuments", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("loads documents and stats on mount", async () => {
    mocks.listDocuments.mockResolvedValue([sampleDoc]);
    mocks.getDocumentStats.mockResolvedValue(sampleStats);

    const { result } = renderHook(() => useDocuments());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    expect(result.current.documents).toEqual([sampleDoc]);
    expect(result.current.stats).toEqual(sampleStats);
    expect(result.current.error).toBeNull();
  });

  it("sets an error when initial load fails", async () => {
    mocks.listDocuments.mockRejectedValue(new Error("load failed"));
    mocks.getDocumentStats.mockResolvedValue(sampleStats);

    const { result } = renderHook(() => useDocuments());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    expect(result.current.error).toBe("load failed");
    expect(result.current.documents).toEqual([]);
    expect(result.current.stats).toBeNull();
  });

  it("deletes a document and refreshes data", async () => {
    mocks.listDocuments
      .mockResolvedValueOnce([sampleDoc])
      .mockResolvedValueOnce([]);
    mocks.getDocumentStats
      .mockResolvedValueOnce(sampleStats)
      .mockResolvedValueOnce({ ...sampleStats, total: 0, pending: 0 });
    mocks.deleteDocument.mockResolvedValue(undefined);

    const { result } = renderHook(() => useDocuments());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    await act(async () => {
      await result.current.removeDocument("doc-1");
    });

    expect(mocks.deleteDocument).toHaveBeenCalledWith("doc-1");
    expect(result.current.documents).toEqual([]);
    expect(result.current.stats?.total).toBe(0);
  });

  it("surfaces delete errors and preserves existing data", async () => {
    mocks.listDocuments.mockResolvedValue([sampleDoc]);
    mocks.getDocumentStats.mockResolvedValue(sampleStats);
    mocks.deleteDocument.mockRejectedValue(new Error("delete failed"));

    const { result } = renderHook(() => useDocuments());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    await act(async () => {
      await expect(result.current.removeDocument("doc-1")).rejects.toThrow(
        "delete failed",
      );
    });

    await waitFor(() => {
      expect(result.current.error).toBe("delete failed");
    });
    expect(result.current.documents).toEqual([sampleDoc]);
    expect(result.current.stats).toEqual(sampleStats);
  });
});
