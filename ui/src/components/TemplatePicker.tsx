import { useState, useEffect } from "react";
import { useTranslation } from "react-i18next";
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
  const { t } = useTranslation();
  const [templates, setTemplates] = useState<ContentTemplate[]>([]);
  const [selected, setSelected] = useState("default");
  const [loading, setLoading] = useState(true);
  const [favoriteName, setFavoriteName] = useState<string | null>(null);

  useEffect(() => {
    setLoading(true);
    Promise.all([
      invoke<ContentTemplate[]>("list_content_templates", { contentType }),
      invoke<string | null>("get_default_template", { contentType }),
    ]).then(([list, favName]) => {
      // Sort: user favorite first, then is_default, then by order
      const sorted = [...list].sort((a, b) => {
        if (favName) {
          if (a.name === favName) return -1;
          if (b.name === favName) return 1;
        }
        if (a.is_default !== b.is_default) return a.is_default ? -1 : 1;
        return (a.order ?? 0) - (b.order ?? 0);
      });
      setTemplates(sorted);
      setFavoriteName(favName);
      // Pre-select favorite if it exists, otherwise first
      const initial = favName && sorted.some((tpl) => tpl.name === favName)
        ? favName
        : sorted[0]?.name ?? "default";
      setSelected(initial);
      setLoading(false);
    });
  }, [contentType]);

  // If only one template, auto-select it
  useEffect(() => {
    if (!loading && templates.length <= 1) {
      onSelect(templates[0]?.name ?? "default");
    }
  }, [templates, loading]);

  // Show spinner while loading or during auto-select (single template)
  if (loading || templates.length <= 1) {
    return (
      <div className="flex-1 flex items-center justify-center">
        <div className="flex items-center gap-3 text-fg-muted">
          <svg className="animate-spin h-6 w-6" viewBox="0 0 24 24">
            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
          {t("app.generatingBBCode")}
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-md mx-auto p-6">
      <h2 className="text-xl font-semibold mb-4">{t("templatePicker.title")}</h2>

      <div className="space-y-2 mb-4">
        {templates.map((tpl) => (
          <button
            key={tpl.name}
            onClick={() => setSelected(tpl.name)}
            className={`w-full text-left px-4 py-3 rounded border transition-colors ${
              selected === tpl.name
                ? "border-blue-500 bg-blue-600/20"
                : "border-edge bg-input hover:border-[#3a3a5a]"
            }`}
          >
            <span className="text-sm font-medium">
              {tpl.name}
              {(favoriteName ? tpl.name === favoriteName : tpl.is_default) && (
                <span className="text-fg-muted ml-2">{t("templatePicker.default")}</span>
              )}
            </span>
            {tpl.title_color && (
              <span
                className="inline-block ml-2 w-3 h-3 rounded-full border border-white/20 align-middle"
                style={{ backgroundColor: `#${tpl.title_color}` }}
                title={`Couleur: #${tpl.title_color}`}
              />
            )}
          </button>
        ))}
      </div>

      <div className="flex items-center gap-3">
        <button
          onClick={() => onSelect(selected)}
          className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-2 rounded text-sm font-medium transition-colors"
        >
          {t("templatePicker.useTemplate")}
        </button>
        <button
          onClick={onCancel}
          className="text-fg-muted hover:text-fg-bright text-sm transition-colors"
        >
          {t("common.cancel")}
        </button>
        <button
          onClick={onEditTemplates}
          className="ml-auto text-blue-400 hover:text-blue-300 text-sm transition-colors"
        >
          {t("templatePicker.editTemplates")}
        </button>
      </div>
    </div>
  );
}
