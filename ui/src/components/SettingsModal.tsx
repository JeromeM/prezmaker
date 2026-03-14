import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import type { SettingsPayload } from "../types/api";
import { resetOnboarding } from "./Onboarding";

interface Props {
  onClose: () => void;
  theme: "dark" | "light";
  onSetTheme: (theme: "dark" | "light") => void;
}

type Tab = "general" | "api" | "llm" | "modules";

const TAB_IDS: Tab[] = ["general", "api", "llm", "modules"];

const inputClass =
  "w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500";

const TAB_LABELS: Record<Tab, string> = {
  general: "settings.general",
  api: "settings.apiKeys",
  llm: "settings.aiLlm",
  modules: "settings.modules",
};

export default function SettingsModal({ onClose, theme, onSetTheme }: Props) {
  const { t, i18n } = useTranslation();
  const [tab, setTab] = useState<Tab>("general");
  const [settings, setSettings] = useState<SettingsPayload>({
    tmdb_api_key: null,
    igdb_client_id: null,
    igdb_client_secret: null,
    language: "fr-FR",
    title_color: "c0392b",
    default_templates: {},
    auto_clipboard: false,
    llm_provider: null,
    llm_api_key: null,
    groq_api_key: null,
    mistral_api_key: null,
    gemini_api_key: null,
    pseudo: "",
    c411_enabled: false,
    c411_api_key: null,
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
    fieldKey: string,
    placeholder?: string
  ) => (
    <div className="flex flex-col gap-1">
      <label className="text-xs text-fg-muted">{label}</label>
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
          className={inputClass}
          placeholder={placeholder ?? t("settings.notConfigured")}
        />
        <button
          type="button"
          onClick={() => toggleShow(fieldKey)}
          className="bg-input border border-edge rounded px-3 py-2 text-xs text-fg-muted hover:text-fg-bright transition-colors shrink-0"
        >
          {showKeys[fieldKey] ? t("common.hide") : t("common.show")}
        </button>
      </div>
    </div>
  );

  return (
    <div
      className="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
      onClick={(e) => e.target === e.currentTarget && onClose()}
    >
      <div className="bg-surface border border-edge rounded-lg w-full max-w-2xl mx-4 shadow-2xl flex flex-col" style={{ height: "min(620px, 85vh)" }}>
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-edge shrink-0">
          <h2 className="text-fg-bright text-lg font-medium">{t("settings.title")}</h2>
          <button
            onClick={onClose}
            className="text-fg-muted hover:text-fg-bright transition-colors text-xl leading-none"
          >
            &times;
          </button>
        </div>

        {/* Body: sidebar tabs + content */}
        <div className="flex flex-1 min-h-0">
          {/* Tab sidebar */}
          <nav className="w-40 border-r border-edge py-2 shrink-0">
            {TAB_IDS.map((tabId) => (
              <button
                key={tabId}
                onClick={() => setTab(tabId)}
                className={`w-full text-left px-4 py-2.5 text-sm transition-colors ${
                  tab === tabId
                    ? "bg-blue-600/20 text-blue-300 border-r-2 border-blue-500"
                    : "text-fg-muted hover:text-fg-bright hover:bg-input"
                }`}
              >
                {t(TAB_LABELS[tabId])}
              </button>
            ))}
          </nav>

          {/* Tab content */}
          <div className="flex-1 px-6 py-5 overflow-y-auto">
            {tab === "general" && (
              <div className="space-y-4">
                <div className="flex flex-col gap-1">
                  <label className="text-xs text-fg-muted">{t("settings.theme")}</label>
                  <select
                    value={theme}
                    onChange={(e) => onSetTheme(e.target.value as "dark" | "light")}
                    className={inputClass}
                  >
                    <option value="dark">{t("settings.dark")}</option>
                    <option value="light">{t("settings.light")}</option>
                  </select>
                </div>

                <div className="flex flex-col gap-1">
                  <label className="text-xs text-fg-muted">{t("settings.uiLanguage")}</label>
                  <select
                    value={i18n.language.startsWith("fr") ? "fr" : "en"}
                    onChange={(e) => {
                      i18n.changeLanguage(e.target.value);
                      localStorage.setItem("prezmaker_ui_lang", e.target.value);
                    }}
                    className={inputClass}
                  >
                    <option value="fr">Français</option>
                    <option value="en">English</option>
                  </select>
                </div>

                <div className="flex flex-col gap-1">
                  <label className="text-xs text-fg-muted">{t("settings.language")}</label>
                  <select
                    value={settings.language}
                    onChange={(e) =>
                      setSettings((s) => ({ ...s, language: e.target.value }))
                    }
                    className={inputClass}
                  >
                    <option value="fr-FR">Français (fr-FR)</option>
                    <option value="en-US">English (en-US)</option>
                  </select>
                </div>

                <div className="flex flex-col gap-1">
                  <label className="text-xs text-fg-muted">{t("settings.pseudo")}</label>
                  <input
                    type="text"
                    value={settings.pseudo}
                    onChange={(e) =>
                      setSettings((s) => ({ ...s, pseudo: e.target.value }))
                    }
                    className={inputClass}
                    placeholder={t("settings.pseudoHint")}
                  />
                </div>

                <div className="flex flex-col gap-1">
                  <label className="text-xs text-fg-muted">
                    {t("settings.defaultTitleColor")}
                  </label>
                  <div className="flex items-center gap-2">
                    <label className="relative w-8 h-8 rounded border border-edge cursor-pointer shrink-0 overflow-hidden">
                      <div
                        className="absolute inset-0"
                        style={{ backgroundColor: `#${settings.title_color || "c0392b"}` }}
                      />
                      <input
                        type="color"
                        value={`#${settings.title_color || "c0392b"}`}
                        onChange={(e) =>
                          setSettings((s) => ({ ...s, title_color: e.target.value.replace("#", "") }))
                        }
                        className="absolute inset-0 opacity-0 cursor-pointer w-full h-full"
                      />
                    </label>
                    <input
                      type="text"
                      value={settings.title_color}
                      onChange={(e) =>
                        setSettings((s) => ({ ...s, title_color: e.target.value.replace("#", "") }))
                      }
                      className={inputClass + " max-w-32 font-mono"}
                      placeholder="c0392b"
                      maxLength={6}
                    />
                    <span className="text-[11px] text-fg-dim">
                      {t("settings.colorHint")}
                    </span>
                  </div>
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
                  <span className="text-sm text-fg">
                    {t("settings.autoClipboard")}
                  </span>
                </label>

                <div className="pt-2">
                  <button
                    type="button"
                    onClick={() => {
                      resetOnboarding();
                      window.location.reload();
                    }}
                    className="text-xs text-fg-dim hover:text-fg underline transition-colors"
                  >
                    {t("settings.restartOnboarding")}
                  </button>
                </div>
              </div>
            )}

            {tab === "api" && (
              <div className="space-y-4">
                <p className="text-xs text-fg-dim mb-2">
                  {t("settings.apiKeysDescription")}
                </p>
                {secretInput("TMDB API Key", "tmdb_api_key", "tmdb")}
                {secretInput("IGDB Client ID", "igdb_client_id", "igdb_id")}
                {secretInput("IGDB Client Secret", "igdb_client_secret", "igdb_secret")}
              </div>
            )}

            {tab === "llm" && (
              <div className="space-y-4">
                <p className="text-xs text-fg-dim mb-2">
                  {t("settings.llmDescription")}
                </p>
                <div className="flex flex-col gap-1">
                  <label className="text-xs text-fg-muted">{t("settings.provider")}</label>
                  <select
                    value={settings.llm_provider ?? ""}
                    onChange={(e) =>
                      setSettings((s) => ({
                        ...s,
                        llm_provider: e.target.value || null,
                      }))
                    }
                    className={inputClass}
                  >
                    <option value="">{t("common.none")}</option>
                    <option value="groq">Groq</option>
                    <option value="mistral">Mistral</option>
                    <option value="gemini">Gemini</option>
                  </select>
                </div>

                {secretInput(t("settings.groqApiKey"), "groq_api_key", "groq")}
                <p className="text-xs text-fg-dim -mt-2">
                  {t("settings.freeKeyOn")} <a href="#" onClick={(e) => { e.preventDefault(); openUrl("https://console.groq.com/keys"); }} className="text-blue-400 hover:underline">console.groq.com</a>
                </p>

                {secretInput(t("settings.mistralApiKey"), "mistral_api_key", "mistral")}
                <p className="text-xs text-fg-dim -mt-2">
                  {t("settings.freeKeyOn")} <a href="#" onClick={(e) => { e.preventDefault(); openUrl("https://console.mistral.ai/api-keys"); }} className="text-blue-400 hover:underline">console.mistral.ai</a>
                </p>

                {secretInput(t("settings.geminiApiKey"), "gemini_api_key", "gemini")}
                <p className="text-xs text-fg-dim -mt-2">
                  {t("settings.freeKeyOn")} <a href="#" onClick={(e) => { e.preventDefault(); openUrl("https://aistudio.google.com/apikey"); }} className="text-blue-400 hover:underline">aistudio.google.com</a>
                </p>
              </div>
            )}

            {tab === "modules" && (
              <div className="space-y-6">
                <p className="text-xs text-fg-dim mb-2">
                  {t("settings.modulesDescription")}
                </p>

                {/* C411 */}
                <div className="border border-edge rounded-lg p-4 space-y-3">
                  <div className="flex items-center justify-between">
                    <h3 className="text-sm font-medium text-fg-bright">C411</h3>
                    <label className="flex items-center gap-2 cursor-pointer">
                      <input
                        type="checkbox"
                        checked={settings.c411_enabled}
                        onChange={(e) =>
                          setSettings((s) => ({
                            ...s,
                            c411_enabled: e.target.checked,
                          }))
                        }
                        className="accent-blue-500"
                      />
                      <span className="text-xs text-fg-muted">{t("settings.enabled")}</span>
                    </label>
                  </div>
                  <p className="text-xs text-fg-dim">{t("settings.c411Description")}</p>
                  {secretInput(t("settings.c411ApiKey"), "c411_api_key", "c411")}
                </div>
              </div>
            )}

            {error && (
              <p className="text-red-400 text-sm mt-4">{error}</p>
            )}
          </div>
        </div>

        {/* Footer */}
        <div className="flex justify-end gap-3 px-6 py-4 border-t border-edge shrink-0">
          <button
            onClick={onClose}
            className="bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded text-sm transition-colors"
          >
            {t("common.cancel")}
          </button>
          <button
            onClick={handleSave}
            disabled={saving}
            className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white px-4 py-2 rounded text-sm font-medium transition-colors"
          >
            {saving ? t("common.saving") : t("common.save")}
          </button>
        </div>
      </div>
    </div>
  );
}
