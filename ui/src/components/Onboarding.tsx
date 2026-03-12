import { useState } from "react";
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
    pseudo: "",
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
      <label className="text-sm text-gray-400">{label}</label>
      <div className="flex gap-2">
        <input
          type={showKeys[fieldKey] ? "text" : "password"}
          value={(settings[key] as string) ?? ""}
          onChange={(e) =>
            setSettings((s) => ({ ...s, [key]: e.target.value || null }))
          }
          className="flex-1 bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
          placeholder={placeholder ?? "Coller votre cle ici"}
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

  const progressDots = (
    <div className="flex justify-center gap-2 mb-6">
      {STEPS.map((s, i) => (
        <div
          key={s}
          className={`w-2.5 h-2.5 rounded-full transition-colors ${
            i === stepIndex ? "bg-blue-500" : i < stepIndex ? "bg-blue-800" : "bg-[#2a2a4a]"
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
            className="text-gray-400 hover:text-white text-sm transition-colors"
          >
            Retour
          </button>
        )}
      </div>
      <div className="flex gap-3">
        {skipLabel && (
          <button
            onClick={next}
            className="text-gray-500 hover:text-gray-300 text-sm transition-colors"
          >
            {skipLabel}
          </button>
        )}
        {step !== "done" && (
          <button
            onClick={next}
            className="bg-blue-600 hover:bg-blue-700 text-white px-5 py-2 rounded text-sm font-medium transition-colors"
          >
            Suivant
          </button>
        )}
      </div>
    </div>
  );

  return (
    <div className="fixed inset-0 bg-[#0f0f23] flex items-center justify-center z-50">
      <div className="w-full max-w-xl mx-4">
        {progressDots}

        <div className="bg-[#1a1a2e] border border-[#2a2a4a] rounded-xl p-8 shadow-2xl">
          {/* Step: Welcome */}
          {step === "welcome" && (
            <div>
              <h1 className="text-3xl font-bold text-white mb-2 text-center">
                Bienvenue sur PrezMaker
              </h1>
              <p className="text-gray-400 text-center mb-8">
                Generateur de presentations BBCode
              </p>

              <div className="space-y-4 text-sm text-gray-300">
                <div className="flex gap-3">
                  <span className="text-blue-400 text-lg">1.</span>
                  <span>Recherchez un <strong>film</strong>, une <strong>serie</strong>, un <strong>jeu</strong> ou une <strong>application</strong></span>
                </div>
                <div className="flex gap-3">
                  <span className="text-blue-400 text-lg">2.</span>
                  <span>Selectionnez le bon resultat parmi les propositions</span>
                </div>
                <div className="flex gap-3">
                  <span className="text-blue-400 text-lg">3.</span>
                  <span>Obtenez une presentation BBCode complete, prete a copier</span>
                </div>
              </div>

              <div className="mt-6 flex flex-col gap-1">
                <label className="text-sm text-gray-400">Votre pseudo (pour la signature des presentations)</label>
                <input
                  type="text"
                  value={settings.pseudo}
                  onChange={(e) =>
                    setSettings((s) => ({ ...s, pseudo: e.target.value }))
                  }
                  className="bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
                  placeholder="Laisser vide pour ne pas afficher de footer"
                />
              </div>

              <p className="text-gray-500 text-xs text-center mt-4">
                Commençons par configurer les cles API necessaires.
              </p>

              {navButtons()}
            </div>
          )}

          {/* Step: TMDB */}
          {step === "tmdb" && (
            <div>
              <h2 className="text-xl font-semibold text-white mb-1">
                TMDB - Films et Series
              </h2>
              <p className="text-gray-400 text-sm mb-6">
                Pour rechercher des films et series, vous avez besoin d'une cle API TMDB (gratuite).
              </p>

              <div className="bg-[#0f0f23] rounded-lg p-4 mb-4 text-sm text-gray-400">
                <p className="mb-2 font-medium text-gray-300">Comment obtenir une cle :</p>
                <ol className="list-decimal list-inside space-y-1">
                  <li>Creez un compte sur <a href="#" onClick={(e) => { e.preventDefault(); openUrl("https://www.themoviedb.org/signup"); }} className="text-blue-400 hover:underline">themoviedb.org</a></li>
                  <li>Allez dans Parametres &gt; API</li>
                  <li>Demandez une cle API (usage personnel)</li>
                  <li>Copiez la cle "API Key (v3 auth)"</li>
                </ol>
              </div>

              {secretInput("Cle API TMDB", "tmdb_api_key", "tmdb")}

              {navButtons("Passer")}
            </div>
          )}

          {/* Step: IGDB */}
          {step === "igdb" && (
            <div>
              <h2 className="text-xl font-semibold text-white mb-1">
                IGDB - Jeux video
              </h2>
              <p className="text-gray-400 text-sm mb-6">
                Pour rechercher des jeux, vous avez besoin d'identifiants Twitch/IGDB (gratuits).
              </p>

              <div className="bg-[#0f0f23] rounded-lg p-4 mb-4 text-sm text-gray-400">
                <p className="mb-2 font-medium text-gray-300">Comment obtenir les cles :</p>
                <ol className="list-decimal list-inside space-y-1">
                  <li>Connectez-vous sur <a href="#" onClick={(e) => { e.preventDefault(); openUrl("https://dev.twitch.tv/console/apps"); }} className="text-blue-400 hover:underline">dev.twitch.tv</a></li>
                  <li>Creez une application (nom libre, URL: <code className="text-xs bg-[#16213e] px-1 rounded">http://localhost</code>)</li>
                  <li>Copiez le Client ID et generez un Client Secret</li>
                </ol>
              </div>

              <div className="space-y-3">
                {secretInput("Client ID", "igdb_client_id", "igdb_id")}
                {secretInput("Client Secret", "igdb_client_secret", "igdb_secret")}
              </div>

              {navButtons("Passer")}
            </div>
          )}

          {/* Step: LLM */}
          {step === "llm" && (
            <div>
              <h2 className="text-xl font-semibold text-white mb-1">
                LLM - Descriptions IA (optionnel)
              </h2>
              <p className="text-gray-400 text-sm mb-6">
                Un LLM peut generer des descriptions en français pour les jeux et creer des NFO.
                C'est totalement optionnel.
              </p>

              <div className="space-y-3">
                <div className="flex flex-col gap-1">
                  <label className="text-sm text-gray-400">Provider</label>
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
                    {secretInput("Cle API", "llm_api_key", "llm")}
                    <p className="text-xs text-gray-500 mt-1">
                      {settings.llm_provider === "groq" && (
                        <>Cle gratuite sur <a href="#" onClick={(e) => { e.preventDefault(); openUrl("https://console.groq.com/keys"); }} className="text-blue-400 hover:underline">console.groq.com</a></>
                      )}
                      {settings.llm_provider === "mistral" && (
                        <>Cle gratuite sur <a href="#" onClick={(e) => { e.preventDefault(); openUrl("https://console.mistral.ai/api-keys"); }} className="text-blue-400 hover:underline">console.mistral.ai</a></>
                      )}
                      {settings.llm_provider === "gemini" && (
                        <>Cle gratuite sur <a href="#" onClick={(e) => { e.preventDefault(); openUrl("https://aistudio.google.com/apikey"); }} className="text-blue-400 hover:underline">aistudio.google.com</a></>
                      )}
                    </p>
                  </>
                )}
              </div>

              {navButtons("Passer")}
            </div>
          )}

          {/* Step: Tour */}
          {step === "tour" && (
            <div>
              <h2 className="text-xl font-semibold text-white mb-1">
                Tour de l'interface
              </h2>
              <p className="text-gray-400 text-sm mb-6">
                Voici les elements principaux de la barre d'outils :
              </p>

              <div className="space-y-4">
                <div className="flex items-start gap-4 bg-[#0f0f23] rounded-lg p-4">
                  <div className="bg-[#16213e] border border-[#2a2a4a] rounded px-3 py-1.5 text-sm text-white shrink-0">
                    Film / Serie / Jeu
                  </div>
                  <p className="text-sm text-gray-400">
                    Selectionnez le type de contenu a rechercher. Le champ de recherche s'adapte automatiquement.
                  </p>
                </div>

                <div className="flex items-start gap-4 bg-[#0f0f23] rounded-lg p-4">
                  <div className="bg-[#16213e] border border-[#2a2a4a] rounded p-2 shrink-0">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" className="w-5 h-5 text-gray-300">
                      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                      <polyline points="7 10 12 15 17 10" />
                      <line x1="12" y1="15" x2="12" y2="3" />
                    </svg>
                  </div>
                  <p className="text-sm text-gray-400">
                    <strong className="text-gray-300">Import torrent</strong> - Importez un fichier .torrent pour detecter automatiquement le contenu et pre-remplir les infos techniques.
                  </p>
                </div>

                <div className="flex items-start gap-4 bg-[#0f0f23] rounded-lg p-4">
                  <div className="bg-[#16213e] border border-[#2a2a4a] rounded p-2 shrink-0">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" className="w-5 h-5 text-gray-300">
                      <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                      <polyline points="14 2 14 8 20 8" />
                      <line x1="16" y1="13" x2="8" y2="13" />
                      <line x1="16" y1="17" x2="8" y2="17" />
                    </svg>
                  </div>
                  <p className="text-sm text-gray-400">
                    <strong className="text-gray-300">Editeur de templates</strong> - Personnalisez les templates BBCode avec des balises dynamiques et un apercu en temps reel.
                  </p>
                </div>

                <div className="flex items-start gap-4 bg-[#0f0f23] rounded-lg p-4">
                  <div className="bg-[#16213e] border border-[#2a2a4a] rounded p-2 shrink-0">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" className="w-5 h-5 text-gray-300">
                      <circle cx="12" cy="12" r="3" />
                      <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
                    </svg>
                  </div>
                  <p className="text-sm text-gray-400">
                    <strong className="text-gray-300">Parametres</strong> - Modifiez vos cles API, la couleur des titres, et les preferences.
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
              <h2 className="text-2xl font-bold text-white mb-2">
                Vous etes pret !
              </h2>
              <p className="text-gray-400 mb-2">
                La configuration est terminee.
              </p>
              <p className="text-gray-500 text-sm mb-8">
                Vous pouvez modifier vos parametres a tout moment via l'icone engrenage.
              </p>

              <button
                onClick={finish}
                disabled={saving}
                className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white px-8 py-3 rounded-lg text-sm font-medium transition-colors"
              >
                {saving ? "Sauvegarde..." : "Commencer"}
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
