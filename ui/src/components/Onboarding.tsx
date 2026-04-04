import { useState } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import type { SettingsPayload } from "../types/api";

const ONBOARDING_KEY = "prezmaker_onboarding_done";

export function isOnboardingDone(): boolean {
  return localStorage.getItem(ONBOARDING_KEY) === "true";
}

export function resetOnboarding(): void {
  localStorage.removeItem(ONBOARDING_KEY);
}

function markOnboardingDone(): void {
  localStorage.setItem(ONBOARDING_KEY, "true");
}

interface Props {
  onComplete: () => void;
}

type Step = "welcome" | "tmdb" | "igdb" | "llm" | "tour" | "done";

const STEPS: Step[] = ["welcome", "tmdb", "igdb", "llm", "tour", "done"];

export default function Onboarding({ onComplete }: Props) {
  const { t } = useTranslation();
  const [step, setStep] = useState<Step>("welcome");
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
    rawg_api_key: null,
  });
  const [showKeys, setShowKeys] = useState<Record<string, boolean>>({});
  const [saving, setSaving] = useState(false);

  const stepIndex = STEPS.indexOf(step);

  const next = () => {
    const nextIdx = stepIndex + 1;
    if (nextIdx < STEPS.length) {
      setStep(STEPS[nextIdx]);
    }
  };

  const prev = () => {
    const prevIdx = stepIndex - 1;
    if (prevIdx >= 0) {
      setStep(STEPS[prevIdx]);
    }
  };

  const finish = async () => {
    setSaving(true);
    try {
      await invoke("save_settings", { settings });
    } catch {
      // Ignore save errors on onboarding
    }
    markOnboardingDone();
    setSaving(false);
    onComplete();
  };

  const toggleShow = (key: string) =>
    setShowKeys((prev) => ({ ...prev, [key]: !prev[key] }));

  const secretInput = (
    label: string,
    key: keyof SettingsPayload,
    fieldKey: string,
    placeholder?: string,
  ) => (
    <div className="flex flex-col gap-1">
      <label className="text-sm text-fg-muted">{label}</label>
      <div className="flex gap-2">
        <input
          type={showKeys[fieldKey] ? "text" : "password"}
          value={(settings[key] as string) ?? ""}
          onChange={(e) =>
            setSettings((s) => ({ ...s, [key]: e.target.value || null }))
          }
          className="flex-1 bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
          placeholder={placeholder ?? t("onboarding.pasteKeyHere")}
        />
        <button
          type="button"
          onClick={() => toggleShow(fieldKey)}
          className="bg-input border border-edge rounded px-3 py-2 text-xs text-fg-muted hover:text-fg-bright transition-colors"
        >
          {showKeys[fieldKey] ? t("common.hide") : t("common.show")}
        </button>
      </div>
    </div>
  );

  const progressDots = (
    <div className="flex justify-center gap-2 mb-6">
      {STEPS.map((s, i) => (
        <div
          key={s}
          className={`w-2.5 h-2.5 rounded-full transition-colors ${
            i === stepIndex ? "bg-blue-500" : i < stepIndex ? "bg-blue-800" : "bg-edge"
          }`}
        />
      ))}
    </div>
  );

  const navButtons = (skipLabel?: string) => (
    <div className="flex justify-between mt-8">
      <div>
        {stepIndex > 0 && step !== "done" && (
          <button
            onClick={prev}
            className="text-fg-muted hover:text-fg-bright text-sm transition-colors"
          >
            {t("common.back")}
          </button>
        )}
      </div>
      <div className="flex gap-3">
        {skipLabel && (
          <button
            onClick={next}
            className="text-fg-dim hover:text-fg text-sm transition-colors"
          >
            {skipLabel}
          </button>
        )}
        {step !== "done" && (
          <button
            onClick={next}
            className="bg-blue-600 hover:bg-blue-700 text-white px-5 py-2 rounded text-sm font-medium transition-colors"
          >
            {t("common.next")}
          </button>
        )}
      </div>
    </div>
  );

  return (
    <div className="fixed inset-0 bg-base flex items-center justify-center z-50">
      <div className="w-full max-w-xl mx-4">
        {progressDots}

        <div className="bg-surface border border-edge rounded-xl p-8 shadow-2xl">
          {/* Step: Welcome */}
          {step === "welcome" && (
            <div>
              <h1 className="text-3xl font-bold text-fg-bright mb-2 text-center">
                {t("onboarding.welcome")}
              </h1>
              <p className="text-fg-muted text-center mb-8">
                {t("onboarding.subtitle")}
              </p>

              <div className="space-y-4 text-sm text-fg">
                <div className="flex gap-3">
                  <span className="text-blue-400 text-lg">1.</span>
                  <span>{t("onboarding.step1")}</span>
                </div>
                <div className="flex gap-3">
                  <span className="text-blue-400 text-lg">2.</span>
                  <span>{t("onboarding.step2")}</span>
                </div>
                <div className="flex gap-3">
                  <span className="text-blue-400 text-lg">3.</span>
                  <span>{t("onboarding.step3")}</span>
                </div>
              </div>

              <div className="mt-6 flex flex-col gap-1">
                <label className="text-sm text-fg-muted">{t("onboarding.pseudoLabel")}</label>
                <input
                  type="text"
                  value={settings.pseudo}
                  onChange={(e) =>
                    setSettings((s) => ({ ...s, pseudo: e.target.value }))
                  }
                  className="bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
                  placeholder={t("onboarding.pseudoHint")}
                />
              </div>

              <p className="text-fg-dim text-xs text-center mt-4">
                {t("onboarding.apiIntro")}
              </p>

              {navButtons()}
            </div>
          )}

          {/* Step: TMDB */}
          {step === "tmdb" && (
            <div>
              <h2 className="text-xl font-semibold text-fg-bright mb-1">
                {t("onboarding.tmdbTitle")}
              </h2>
              <p className="text-fg-muted text-sm mb-6">
                {t("onboarding.tmdbDescription")}
              </p>

              <div className="bg-base rounded-lg p-4 mb-4 text-sm text-fg-muted">
                <p className="mb-2 font-medium text-fg">{t("onboarding.howToGetKey")}</p>
                <ol className="list-decimal list-inside space-y-1">
                  <li>{t("onboarding.tmdbStep1")}<a href="#" onClick={(e) => { e.preventDefault(); openUrl("https://www.themoviedb.org/signup"); }} className="text-blue-400 hover:underline">themoviedb.org</a></li>
                  <li>{t("onboarding.tmdbStep2")}</li>
                  <li>{t("onboarding.tmdbStep3")}</li>
                  <li>{t("onboarding.tmdbStep4")}</li>
                </ol>
              </div>

              {secretInput(t("onboarding.tmdbApiKey"), "tmdb_api_key", "tmdb")}

              {navButtons(t("common.skip"))}
            </div>
          )}

          {/* Step: IGDB */}
          {step === "igdb" && (
            <div>
              <h2 className="text-xl font-semibold text-fg-bright mb-1">
                {t("onboarding.igdbTitle")}
              </h2>
              <p className="text-fg-muted text-sm mb-6">
                {t("onboarding.igdbDescription")}
              </p>

              <div className="bg-base rounded-lg p-4 mb-4 text-sm text-fg-muted">
                <p className="mb-2 font-medium text-fg">{t("onboarding.howToGetKeys")}</p>
                <ol className="list-decimal list-inside space-y-1">
                  <li>{t("onboarding.igdbStep1")}<a href="#" onClick={(e) => { e.preventDefault(); openUrl("https://dev.twitch.tv/console/apps"); }} className="text-blue-400 hover:underline">dev.twitch.tv</a></li>
                  <li>{t("onboarding.igdbStep2")}<code className="text-xs bg-input px-1 rounded">http://localhost</code>)</li>
                  <li>{t("onboarding.igdbStep3")}</li>
                </ol>
              </div>

              <div className="space-y-3">
                {secretInput(t("onboarding.clientId"), "igdb_client_id", "igdb_id")}
                {secretInput(t("onboarding.clientSecret"), "igdb_client_secret", "igdb_secret")}
              </div>

              {navButtons(t("common.skip"))}
            </div>
          )}

          {/* Step: LLM */}
          {step === "llm" && (
            <div>
              <h2 className="text-xl font-semibold text-fg-bright mb-1">
                {t("onboarding.llmTitle")}
              </h2>
              <p className="text-fg-muted text-sm mb-6">
                {t("onboarding.llmDescription")}
              </p>

              <div className="space-y-3">
                <div className="flex flex-col gap-1">
                  <label className="text-sm text-fg-muted">{t("settings.provider")}</label>
                  <select
                    value={settings.llm_provider ?? ""}
                    onChange={(e) =>
                      setSettings((s) => ({
                        ...s,
                        llm_provider: e.target.value || null,
                        ...(e.target.value ? {} : { llm_api_key: null }),
                      }))
                    }
                    className="bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
                  >
                    <option value="">{t("common.none")}</option>
                    <option value="groq">Groq</option>
                    <option value="mistral">Mistral</option>
                    <option value="gemini">Gemini</option>
                  </select>
                </div>

                {settings.llm_provider && (
                  <>
                    {secretInput(t("onboarding.apiKey"), "llm_api_key", "llm")}
                    <p className="text-xs text-fg-dim mt-1">
                      {settings.llm_provider === "groq" && (
                        <>{t("settings.freeKeyOn")}<a href="#" onClick={(e) => { e.preventDefault(); openUrl("https://console.groq.com/keys"); }} className="text-blue-400 hover:underline">console.groq.com</a></>
                      )}
                      {settings.llm_provider === "mistral" && (
                        <>{t("settings.freeKeyOn")}<a href="#" onClick={(e) => { e.preventDefault(); openUrl("https://console.mistral.ai/api-keys"); }} className="text-blue-400 hover:underline">console.mistral.ai</a></>
                      )}
                      {settings.llm_provider === "gemini" && (
                        <>{t("settings.freeKeyOn")}<a href="#" onClick={(e) => { e.preventDefault(); openUrl("https://aistudio.google.com/apikey"); }} className="text-blue-400 hover:underline">aistudio.google.com</a></>
                      )}
                    </p>
                  </>
                )}
              </div>

              {navButtons(t("common.skip"))}
            </div>
          )}

          {/* Step: Tour */}
          {step === "tour" && (
            <div>
              <h2 className="text-xl font-semibold text-fg-bright mb-1">
                {t("onboarding.tourTitle")}
              </h2>
              <p className="text-fg-muted text-sm mb-6">
                {t("onboarding.tourIntro")}
              </p>

              <div className="space-y-4">
                <div className="flex items-start gap-4 bg-base rounded-lg p-4">
                  <div className="bg-input border border-edge rounded px-3 py-1.5 text-sm text-fg-bright shrink-0">
                    {t("onboarding.tourContentType")}
                  </div>
                  <p className="text-sm text-fg-muted">
                    {t("onboarding.tourContentTypeDesc")}
                  </p>
                </div>

                <div className="flex items-start gap-4 bg-base rounded-lg p-4">
                  <div className="bg-input border border-edge rounded p-2 shrink-0">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" className="w-5 h-5 text-fg">
                      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                      <polyline points="7 10 12 15 17 10" />
                      <line x1="12" y1="15" x2="12" y2="3" />
                    </svg>
                  </div>
                  <p className="text-sm text-fg-muted">
                    <strong className="text-fg">{t("onboarding.tourTorrent")}</strong> - {t("onboarding.tourTorrentDesc")}
                  </p>
                </div>

                <div className="flex items-start gap-4 bg-base rounded-lg p-4">
                  <div className="bg-input border border-edge rounded p-2 shrink-0">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" className="w-5 h-5 text-fg">
                      <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                      <polyline points="14 2 14 8 20 8" />
                      <line x1="16" y1="13" x2="8" y2="13" />
                      <line x1="16" y1="17" x2="8" y2="17" />
                    </svg>
                  </div>
                  <p className="text-sm text-fg-muted">
                    <strong className="text-fg">{t("onboarding.tourTemplateEditor")}</strong> - {t("onboarding.tourTemplateEditorDesc")}
                  </p>
                </div>

                <div className="flex items-start gap-4 bg-base rounded-lg p-4">
                  <div className="bg-input border border-edge rounded p-2 shrink-0">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" className="w-5 h-5 text-fg">
                      <circle cx="12" cy="12" r="3" />
                      <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
                    </svg>
                  </div>
                  <p className="text-sm text-fg-muted">
                    <strong className="text-fg">{t("onboarding.tourSettings")}</strong> - {t("onboarding.tourSettingsDesc")}
                  </p>
                </div>
              </div>

              {navButtons()}
            </div>
          )}

          {/* Step: Done */}
          {step === "done" && (
            <div className="text-center">
              <div className="text-5xl mb-4">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="w-16 h-16 mx-auto text-green-400">
                  <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
                  <polyline points="22 4 12 14.01 9 11.01" />
                </svg>
              </div>
              <h2 className="text-2xl font-bold text-fg-bright mb-2">
                {t("onboarding.ready")}
              </h2>
              <p className="text-fg-muted mb-2">
                {t("onboarding.readyDesc")}
              </p>
              <p className="text-fg-dim text-sm mb-8">
                {t("onboarding.readyHint")}
              </p>

              <button
                onClick={finish}
                disabled={saving}
                className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white px-8 py-3 rounded-lg text-sm font-medium transition-colors"
              >
                {saving ? t("common.saving") : t("onboarding.start")}
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
