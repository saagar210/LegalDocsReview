import { useEffect, useState, useCallback } from "react";
import { Settings as SettingsIcon, Save } from "lucide-react";
import toast from "react-hot-toast";
import { getSetting, setSetting } from "@/lib/commands";

function Settings() {
  const [aiProvider, setAiProvider] = useState("ollama");
  const [ollamaUrl, setOllamaUrl] = useState("http://localhost:11434");
  const [ollamaModel, setOllamaModel] = useState("llama3");
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function load() {
      try {
        const [provider, url, model] = await Promise.all([
          getSetting("ai_provider"),
          getSetting("ollama_url"),
          getSetting("ollama_model"),
        ]);
        if (provider) setAiProvider(provider);
        if (url) setOllamaUrl(url);
        if (model) setOllamaModel(model);
      } finally {
        setLoading(false);
      }
    }
    load();
  }, []);

  const handleSave = useCallback(async () => {
    try {
      await Promise.all([
        setSetting("ai_provider", aiProvider),
        setSetting("ollama_url", ollamaUrl),
        setSetting("ollama_model", ollamaModel),
      ]);
      toast.success("Settings saved");
    } catch (err) {
      toast.error(
        `Failed to save: ${err instanceof Error ? err.message : String(err)}`,
      );
    }
  }, [aiProvider, ollamaUrl, ollamaModel]);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="animate-spin h-8 w-8 border-2 border-brand-600 border-t-transparent rounded-full" />
      </div>
    );
  }

  return (
    <div className="p-8 max-w-2xl">
      <div className="flex items-center gap-3 mb-8">
        <SettingsIcon className="h-6 w-6 text-gray-400" />
        <h1 className="text-2xl font-bold">Settings</h1>
      </div>

      <div className="bg-white rounded-xl border border-gray-200 p-6 space-y-6">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            AI Provider
          </label>
          <select
            value={aiProvider}
            onChange={(e) => setAiProvider(e.target.value)}
            className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-500 focus:border-transparent"
          >
            <option value="ollama">Ollama (Local)</option>
            <option value="claude">Claude API</option>
            <option value="openai">OpenAI API</option>
          </select>
        </div>

        {aiProvider === "ollama" && (
          <>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Ollama URL
              </label>
              <input
                type="text"
                value={ollamaUrl}
                onChange={(e) => setOllamaUrl(e.target.value)}
                className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-500 focus:border-transparent"
                placeholder="http://localhost:11434"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Model
              </label>
              <input
                type="text"
                value={ollamaModel}
                onChange={(e) => setOllamaModel(e.target.value)}
                className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-500 focus:border-transparent"
                placeholder="llama3"
              />
            </div>
          </>
        )}

        {aiProvider === "claude" && (
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Claude API Key
            </label>
            <input
              type="password"
              className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-500 focus:border-transparent"
              placeholder="sk-ant-..."
              onChange={(e) =>
                setSetting("claude_api_key", e.target.value)
              }
            />
            <p className="text-xs text-gray-500 mt-1">
              Stored locally on your device only
            </p>
          </div>
        )}

        {aiProvider === "openai" && (
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              OpenAI API Key
            </label>
            <input
              type="password"
              className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-500 focus:border-transparent"
              placeholder="sk-..."
              onChange={(e) =>
                setSetting("openai_api_key", e.target.value)
              }
            />
            <p className="text-xs text-gray-500 mt-1">
              Stored locally on your device only
            </p>
          </div>
        )}
      </div>

      <button
        onClick={handleSave}
        className="mt-6 flex items-center gap-2 bg-brand-600 text-white px-6 py-2.5 rounded-lg font-medium hover:bg-brand-700 transition-colors text-sm"
      >
        <Save className="h-4 w-4" />
        Save Settings
      </button>
    </div>
  );
}

export default Settings;
