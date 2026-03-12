import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { ContentType, ContentTemplate } from "../types/api";

interface Props {
  contentType: ContentType;
  onSelect: (templateName: string) => void;
  onCancel: () => void;
  onEditTemplates: () => void;
}

export default function TemplatePicker({
  contentType,
  onSelect,
  onCancel,
  onEditTemplates,
}: Props) {
  const [templates, setTemplates] = useState<ContentTemplate[]>([]);
  const [selected, setSelected] = useState("default");
  const [defaultName, setDefaultName] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setLoading(true);
    Promise.all([
      invoke<ContentTemplate[]>("list_content_templates", { contentType }),
      invoke<string | null>("get_default_template", { contentType }),
    ]).then(([list, defName]) => {
      setTemplates(list);
      setDefaultName(defName);

      // Pick the user's default if it exists in the list, otherwise first
      const initial = defName && list.some((t) => t.name === defName)
        ? defName
        : list[0]?.name ?? "default";
      setSelected(initial);
      setLoading(false);
    });
  }, [contentType]);

  // If only one template OR user has a valid default → auto-select
  useEffect(() => {
    if (loading) return;
    if (templates.length <= 1) {
      onSelect(templates[0]?.name ?? "default");
      return;
    }
    if (defaultName && templates.some((t) => t.name === defaultName)) {
      onSelect(defaultName);
    }
  }, [templates, loading, defaultName]);

  const handleSetDefault = async (name: string) => {
    // Toggle: if already default, clear it
    const newDefault = name === defaultName ? "" : name;
    await invoke("set_default_template", {
      contentType,
      templateName: newDefault,
    });
    setDefaultName(newDefault || null);
  };

  // Show spinner while loading or during auto-select (single template or has default)
  if (loading || templates.length <= 1 || (defaultName && templates.some((t) => t.name === defaultName))) {
    return (
      <div className="flex-1 flex items-center justify-center">
        <div className="flex items-center gap-3 text-gray-400">
          <svg className="animate-spin h-6 w-6" viewBox="0 0 24 24">
            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
          Generation du BBCode...
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-md mx-auto p-6">
      <h2 className="text-xl font-semibold mb-4">Choisir un template</h2>

      <div className="space-y-2 mb-4">
        {templates.map((t) => (
          <div key={t.name} className="flex items-center gap-2">
            <button
              onClick={() => setSelected(t.name)}
              className={`flex-1 text-left px-4 py-3 rounded border transition-colors ${
                selected === t.name
                  ? "border-blue-500 bg-blue-600/20"
                  : "border-[#2a2a4a] bg-[#16213e] hover:border-[#3a3a5a]"
              }`}
            >
              <span className="text-sm font-medium">
                {t.name}
                {t.is_default && (
                  <span className="text-gray-400 ml-2">(par defaut)</span>
                )}
              </span>
              {t.title_color && (
                <span
                  className="inline-block ml-2 w-3 h-3 rounded-full border border-white/20 align-middle"
                  style={{ backgroundColor: `#${t.title_color}` }}
                  title={`Couleur: #${t.title_color}`}
                />
              )}
            </button>
            <button
              onClick={() => handleSetDefault(t.name)}
              title={t.name === defaultName ? "Retirer comme favori" : "Definir comme favori"}
              className={`shrink-0 p-2 rounded transition-colors ${
                t.name === defaultName
                  ? "text-yellow-400 hover:text-yellow-300"
                  : "text-gray-600 hover:text-gray-400"
              }`}
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill={t.name === defaultName ? "currentColor" : "none"}
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
                className="w-4 h-4"
              >
                <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
              </svg>
            </button>
          </div>
        ))}
      </div>

      <p className="text-xs text-gray-600 mb-4">
        Cliquez sur l'etoile pour definir un template favori (auto-selection)
      </p>

      <div className="flex items-center gap-3">
        <button
          onClick={() => onSelect(selected)}
          className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-2 rounded text-sm font-medium transition-colors"
        >
          Utiliser ce template
        </button>
        <button
          onClick={onCancel}
          className="text-gray-400 hover:text-white text-sm transition-colors"
        >
          Annuler
        </button>
        <button
          onClick={onEditTemplates}
          className="ml-auto text-blue-400 hover:text-blue-300 text-sm transition-colors"
        >
          Editer les templates
        </button>
      </div>
    </div>
  );
}
