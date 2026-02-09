import { useState, useCallback, useEffect } from "react";
import { FileCheck, Plus, Trash2, X } from "lucide-react";
import toast from "react-hot-toast";
import {
  listTemplates,
  createTemplate,
  deleteTemplate,
} from "@/lib/commands";
import type { Template } from "@/lib/commands";
import { CONTRACT_TYPE_LABELS } from "@/types";
import type { ContractType } from "@/types";

function Templates() {
  const [templates, setTemplates] = useState<Template[]>([]);
  const [loading, setLoading] = useState(true);
  const [showCreate, setShowCreate] = useState(false);
  const [name, setName] = useState("");
  const [contractType, setContractType] = useState<ContractType>("nda");
  const [description, setDescription] = useState("");
  const [rawText, setRawText] = useState("");

  const refresh = useCallback(async () => {
    try {
      setLoading(true);
      const tpls = await listTemplates();
      setTemplates(tpls);
    } catch {
      toast.error("Failed to load templates");
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const handleCreate = useCallback(async () => {
    if (!name.trim() || !rawText.trim()) {
      toast.error("Name and text are required");
      return;
    }
    try {
      await createTemplate(
        name,
        contractType,
        description || null,
        rawText,
      );
      toast.success("Template created");
      setShowCreate(false);
      setName("");
      setDescription("");
      setRawText("");
      refresh();
    } catch (err) {
      toast.error(
        `Failed: ${err instanceof Error ? err.message : String(err)}`,
      );
    }
  }, [name, contractType, description, rawText, refresh]);

  const handleDelete = useCallback(
    async (id: string) => {
      try {
        await deleteTemplate(id);
        toast.success("Template deleted");
        refresh();
      } catch (err) {
        toast.error(
          `Failed: ${err instanceof Error ? err.message : String(err)}`,
        );
      }
    },
    [refresh],
  );

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="animate-spin h-8 w-8 border-2 border-brand-600 border-t-transparent rounded-full" />
      </div>
    );
  }

  return (
    <div className="p-8 max-w-4xl">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-2xl font-bold">Templates</h1>
          <p className="text-gray-500 mt-1">
            Gold standard contracts for benchmarking
          </p>
        </div>
        <button
          onClick={() => setShowCreate(true)}
          className="flex items-center gap-2 bg-brand-600 text-white px-4 py-2 rounded-lg hover:bg-brand-700 transition-colors text-sm font-medium"
        >
          <Plus className="h-4 w-4" />
          New Template
        </button>
      </div>

      {showCreate && (
        <div className="bg-white rounded-xl border border-gray-200 p-6 mb-6">
          <div className="flex items-center justify-between mb-4">
            <h2 className="font-semibold">Create Template</h2>
            <button
              onClick={() => setShowCreate(false)}
              className="p-1 hover:bg-gray-100 rounded"
            >
              <X className="h-4 w-4" />
            </button>
          </div>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Name
              </label>
              <input
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
                placeholder="Standard NDA Template"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Contract Type
              </label>
              <select
                value={contractType}
                onChange={(e) =>
                  setContractType(e.target.value as ContractType)
                }
                className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
              >
                {(
                  Object.entries(CONTRACT_TYPE_LABELS) as [
                    ContractType,
                    string,
                  ][]
                ).map(([value, label]) => (
                  <option key={value} value={value}>
                    {label}
                  </option>
                ))}
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Description
              </label>
              <input
                type="text"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
                placeholder="Optional description"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Contract Text
              </label>
              <textarea
                value={rawText}
                onChange={(e) => setRawText(e.target.value)}
                rows={8}
                className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm font-mono"
                placeholder="Paste the full contract text here..."
              />
            </div>
            <button
              onClick={handleCreate}
              className="bg-brand-600 text-white px-6 py-2 rounded-lg hover:bg-brand-700 transition-colors text-sm font-medium"
            >
              Create Template
            </button>
          </div>
        </div>
      )}

      {templates.length === 0 && !showCreate ? (
        <div className="bg-white rounded-xl border border-gray-200 p-12 text-center">
          <FileCheck className="h-12 w-12 text-gray-300 mx-auto mb-4" />
          <p className="text-gray-500">No templates yet</p>
          <button
            onClick={() => setShowCreate(true)}
            className="text-brand-600 hover:text-brand-700 text-sm font-medium mt-2"
          >
            Create your first template
          </button>
        </div>
      ) : (
        <div className="space-y-3">
          {templates.map((tpl) => (
            <div
              key={tpl.id}
              className="bg-white rounded-xl border border-gray-200 p-4 flex items-center justify-between"
            >
              <div>
                <p className="font-medium text-gray-900">{tpl.name}</p>
                <p className="text-xs text-gray-500">
                  {CONTRACT_TYPE_LABELS[tpl.contract_type as ContractType] ??
                    tpl.contract_type}{" "}
                  &middot; {tpl.raw_text.length.toLocaleString()} chars
                  {tpl.description && ` — ${tpl.description}`}
                </p>
              </div>
              <button
                onClick={() => handleDelete(tpl.id)}
                className="p-2 text-red-400 hover:text-red-600 hover:bg-red-50 rounded-lg transition-colors"
              >
                <Trash2 className="h-4 w-4" />
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

export default Templates;
