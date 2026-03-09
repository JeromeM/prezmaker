import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { SettingsPayload } from "../types/api";
import { resetOnboarding } from "./Onboarding";

interface Props {
  onClose: () => void;
}

export default function SettingsModal({ onClose }: Props) {
  const [settings, setSettings] = useState<SettingsPayload>({
    tmdb_api_key: null,
    igdb_client_id: null,
    igdb_client_secret: null,
    language: "fr-FR",
    title_color: "c0392b",
    auto_clipboard: false,
    llm_provider: null,
    llm_api_key: null,
  });
  const [showKeys, setShowKeys] = useState<Record<string, boolean>>({});
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<SettingsPayload>("get_settings").then(setSettings);
  }, []);

  const toggleShow = (key: string) =>
    setShowKeys((prev) => ({ ...prev, [key]: !prev[key] }));

  const handleSave = async () => {
    setSaving(true);
    setError(null);
    try {
      await invoke("save_settings", { settings });
      onClose();
    } catch (e) {
      setError(String(e));
    } finally {
      setSaving(false);
    }
  };

  const secretInput = (
    label: string,
    key: keyof SettingsPayload,
    fieldKey: string
  ) => (
    <div className="flex flex-col gap-1">
      <label className="text-xs text-gray-400">{label}</label>
      <div className="flex gap-2">
        <input
          type={showKeys[fieldKey] ? "text" : "password"}
          value={(settings[key] as string) ?? ""}
          onChange={(e) =>
            setSettings((s) => ({
              ...s,
              [key]: e.target.value || null,
            }))
          }
          className="flex-1 bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
          placeholder="Non configuré"
        />
        <button
          type="button"
          onClick={() => toggleShow(fieldKey)}
          className="bg-[#16213e] border border-[#2a2a4a] rounded px-3 py-2 text-xs text-gray-400 hover:text-white transition-colors"
        >
          {showKeys[fieldKey] ? "Masquer" : "Afficher"}
        </button>
      </div>
    </div>
  );

  return (
    <div
      className="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
      onClick={(e) => e.target === e.currentTarget && onClose()}
    >
      <div className="bg-[#1a1a2e] border border-[#2a2a4a] rounded-lg w-full max-w-lg mx-4 shadow-2xl">
        <div className="flex items-center justify-between px-6 py-4 border-b border-[#2a2a4a]">
          <h2 className="text-white text-lg font-medium">Settings</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white transition-colors text-xl leading-none"
          >
            &times;
          </button>
        </div>

        <div className="px-6 py-5 space-y-6 max-h-[70vh] overflow-y-auto">
          <section>
            <h3 className="text-sm font-medium text-gray-300 mb-3">
              Clés API
            </h3>
            <div className="space-y-3">
              {secretInput("TMDB API Key", "tmdb_api_key", "tmdb")}
              {secretInput("IGDB Client ID", "igdb_client_id", "igdb_id")}
              {secretInput(
                "IGDB Client Secret",
                "igdb_client_secret",
                "igdb_secret"
              )}
            </div>
          </section>

          <section>
            <h3 className="text-sm font-medium text-gray-300 mb-3">
              LLM (descriptions jeux)
            </h3>
            <div className="space-y-3">
              <div className="flex flex-col gap-1">
                <label className="text-xs text-gray-400">Provider</label>
                <select
                  value={settings.llm_provider ?? ""}
                  onChange={(e) =>
                    setSettings((s) => ({
                      ...s,
                      llm_provider: e.target.value || null,
                      ...(e.target.value ? {} : { llm_api_key: null }),
                    }))
                  }
                  className="bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
                >
                  <option value="">(Aucun)</option>
                  <option value="groq">Groq</option>
                  <option value="mistral">Mistral</option>
                  <option value="gemini">Gemini</option>
                </select>
              </div>
              {settings.llm_provider && (
                <>
                  {secretInput("Clé API LLM", "llm_api_key", "llm")}
                  <p className="text-xs text-gray-500">
                    {settings.llm_provider === "groq" && (
                      <>Clé gratuite sur <a href="https://console.groq.com/keys" target="_blank" rel="noreferrer" className="text-blue-400 hover:underline">console.groq.com</a></>
                    )}
                    {settings.llm_provider === "mistral" && (
                      <>Clé gratuite sur <a href="https://console.mistral.ai/api-keys" target="_blank" rel="noreferrer" className="text-blue-400 hover:underline">console.mistral.ai</a></>
                    )}
                    {settings.llm_provider === "gemini" && (
                      <>Clé gratuite sur <a href="https://aistudio.google.com/apikey" target="_blank" rel="noreferrer" className="text-blue-400 hover:underline">aistudio.google.com</a></>
                    )}
                  </p>
                </>
              )}
            </div>
          </section>

          <section>
            <h3 className="text-sm font-medium text-gray-300 mb-3">
              Préférences
            </h3>
            <div className="space-y-3">
              <div className="flex flex-col gap-1">
                <label className="text-xs text-gray-400">Langue</label>
                <select
                  value={settings.language}
                  onChange={(e) =>
                    setSettings((s) => ({ ...s, language: e.target.value }))
                  }
                  className="bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
                >
                  <option value="fr-FR">Français (fr-FR)</option>
                  <option value="en-US">English (en-US)</option>
                </select>
              </div>

              <div className="flex flex-col gap-1">
                <label className="text-xs text-gray-400">
                  Couleur titre par défaut
                </label>
                <input
                  type="text"
                  value={settings.title_color}
                  onChange={(e) =>
                    setSettings((s) => ({ ...s, title_color: e.target.value }))
                  }
                  className="bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
                  placeholder="c0392b"
                />
              </div>

              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  checked={settings.auto_clipboard}
                  onChange={(e) =>
                    setSettings((s) => ({
                      ...s,
                      auto_clipboard: e.target.checked,
                    }))
                  }
                  className="accent-blue-500"
                />
                <span className="text-sm text-gray-300">
                  Copier automatiquement dans le presse-papier
                </span>
              </label>

              <button
                type="button"
                onClick={() => {
                  resetOnboarding();
                  window.location.reload();
                }}
                className="text-xs text-gray-500 hover:text-gray-300 underline transition-colors"
              >
                Relancer le tutoriel de premiere utilisation
              </button>
            </div>
          </section>

          {error && (
            <p className="text-red-400 text-sm">{error}</p>
          )}
        </div>

        <div className="flex justify-end gap-3 px-6 py-4 border-t border-[#2a2a4a]">
          <button
            onClick={onClose}
            className="bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded text-sm transition-colors"
          >
            Annuler
          </button>
          <button
            onClick={handleSave}
            disabled={saving}
            className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white px-4 py-2 rounded text-sm font-medium transition-colors"
          >
            {saving ? "Sauvegarde..." : "Sauvegarder"}
          </button>
        </div>
      </div>
    </div>
  );
}
