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

  useEffect(() => {
    invoke<ContentTemplate[]>("list_content_templates", { contentType }).then(
      (list) => {
        setTemplates(list);
        if (list.length > 0) setSelected(list[0].name);
      }
    );
  }, [contentType]);

  // If only default template, auto-select it
  useEffect(() => {
    if (templates.length === 1 && templates[0].name === "default") {
      onSelect("default");
    }
  }, [templates]);

  // Don't render if auto-selected
  if (templates.length <= 1) return null;

  return (
    <div className="max-w-md mx-auto p-6">
      <h2 className="text-xl font-semibold mb-4">Choisir un template</h2>

      <div className="space-y-2 mb-4">
        {templates.map((t) => (
          <button
            key={t.name}
            onClick={() => setSelected(t.name)}
            className={`w-full text-left px-4 py-3 rounded border transition-colors ${
              selected === t.name
                ? "border-blue-500 bg-blue-600/20"
                : "border-[#2a2a4a] bg-[#16213e] hover:border-[#3a3a5a]"
            }`}
          >
            <span className="text-sm font-medium">
              {t.name}
              {t.is_default && (
                <span className="text-gray-400 ml-2">(par défaut)</span>
              )}
            </span>
          </button>
        ))}
      </div>

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
