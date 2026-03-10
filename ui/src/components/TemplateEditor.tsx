import { useState, useEffect, useCallback, useRef, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { ContentType, ContentTemplate, TemplateTag } from "../types/api";

interface Props {
  onClose: () => void;
}

const BLOCK_PAIRS: Record<string, string> = {
  "center": "/center",
  "quote": "/quote",
  "bold": "/bold",
  "italic": "/italic",
  "underline": "/underline",
  "table": "/table",
  "tr": "/tr",
};

// Category display order
const CATEGORY_ORDER = [
  "Mise en page",
  "Formatage",
  "Images",
  "Tableaux",
  "Raccourcis",
  "Donnees",
  "Donnees techniques",
  "Notes",
  "Conditionnel",
];

// Categories collapsed by default
const DEFAULT_COLLAPSED = new Set(["Images", "Tableaux", "Donnees techniques", "Notes"]);

export default function TemplateEditor({ onClose }: Props) {
  const [contentType, setContentType] = useState<ContentType>("film");
  const [templates, setTemplates] = useState<ContentTemplate[]>([]);
  const [selected, setSelected] = useState<string>("default");
  const [body, setBody] = useState("");
  const [tags, setTags] = useState<TemplateTag[]>([]);
  const [dirty, setDirty] = useState(false);
  const [saving, setSaving] = useState(false);
  const [showNewDialog, setShowNewDialog] = useState(false);
  const [newName, setNewName] = useState("");
  const [previewHtml, setPreviewHtml] = useState("");
  const [collapsed, setCollapsed] = useState<Set<string>>(new Set(DEFAULT_COLLAPSED));
  const debounceRef = useRef<ReturnType<typeof setTimeout>>(null);

  const updatePreview = useCallback(async (templateBody: string, ct: string) => {
    try {
      const bbcode = await invoke<string>("preview_template", {
        body: templateBody,
        contentType: ct,
        titleColor: null,
      });
      const html = await invoke<string>("convert_bbcode", { bbcode });
      setPreviewHtml(html);
    } catch (e) {
      console.error("Preview error:", e);
    }
  }, []);

  const debouncedPreview = useCallback((templateBody: string, ct: string) => {
    if (debounceRef.current) clearTimeout(debounceRef.current);
    debounceRef.current = setTimeout(() => updatePreview(templateBody, ct), 400);
  }, [updatePreview]);

  useEffect(() => {
    return () => { if (debounceRef.current) clearTimeout(debounceRef.current); };
  }, []);

  const loadTemplates = useCallback(async (ct: string) => {
    try {
      const list = await invoke<ContentTemplate[]>("list_content_templates", { contentType: ct });
      setTemplates(list);
      const current = list.find((t) => t.name === selected) || list[0];
      if (current) {
        setSelected(current.name);
        setBody(current.body);
      }
      setDirty(false);
    } catch (e) {
      console.error("Failed to load templates:", e);
    }
  }, [selected]);

  const loadTags = useCallback(async (ct: string) => {
    const t = await invoke<TemplateTag[]>("get_template_tags", { contentType: ct });
    setTags(t);
  }, []);

  useEffect(() => {
    loadTemplates(contentType);
    loadTags(contentType);
  }, [contentType]);

  // Update preview when body or contentType changes
  useEffect(() => {
    if (body) debouncedPreview(body, contentType);
  }, [body, contentType, debouncedPreview]);

  // Group tags by category
  const tagsByCategory = useMemo(() => {
    const map = new Map<string, TemplateTag[]>();
    for (const t of tags) {
      const list = map.get(t.category) || [];
      list.push(t);
      map.set(t.category, list);
    }
    // Sort by CATEGORY_ORDER
    const sorted: [string, TemplateTag[]][] = [];
    for (const cat of CATEGORY_ORDER) {
      const list = map.get(cat);
      if (list) sorted.push([cat, list]);
    }
    // Any remaining categories not in the order
    for (const [cat, list] of map) {
      if (!CATEGORY_ORDER.includes(cat)) {
        sorted.push([cat, list]);
      }
    }
    return sorted;
  }, [tags]);

  const handleSelectTemplate = async (name: string) => {
    if (dirty && !confirm("Modifications non sauvegardées. Continuer ?")) return;
    try {
      const tpl = await invoke<ContentTemplate>("get_content_template", {
        contentType,
        name,
      });
      setSelected(tpl.name);
      setBody(tpl.body);
      setDirty(false);
    } catch (e) {
      console.error(e);
    }
  };

  const handleSave = async () => {
    setSaving(true);
    try {
      await invoke("save_content_template", { contentType, name: selected, body });
      setDirty(false);
      await loadTemplates(contentType);
    } catch (e) {
      alert("Erreur: " + e);
    }
    setSaving(false);
  };

  const handleNew = async () => {
    const safe = newName.trim();
    if (!safe) return;
    try {
      await invoke("duplicate_content_template", {
        contentType,
        name: "default",
        newName: safe,
      });
      setShowNewDialog(false);
      setNewName("");
      setSelected(safe);
      await loadTemplates(contentType);
      const tpl = await invoke<ContentTemplate>("get_content_template", {
        contentType,
        name: safe,
      });
      setBody(tpl.body);
      setDirty(false);
    } catch (e) {
      alert("Erreur: " + e);
    }
  };

  const handleDelete = async () => {
    if (selected === "default") return;
    if (!confirm(`Supprimer le template "${selected}" ?`)) return;
    try {
      await invoke("delete_content_template", { contentType, name: selected });
      setSelected("default");
      await loadTemplates(contentType);
    } catch (e) {
      alert("Erreur: " + e);
    }
  };

  const insertTag = (tagName: string) => {
    const textarea = document.getElementById("template-body") as HTMLTextAreaElement | null;
    if (!textarea) return;
    const start = textarea.selectionStart;
    const end = textarea.selectionEnd;
    const selectedText = body.substring(start, end);

    // Check if it's a display-only pair tag (like "center}}...{{/center")
    const pairMatch = tagName.match(/^(\w+)\}\}\.\.\.\{\{\/\1$/);
    if (pairMatch) {
      const pairName = pairMatch[1];
      const open = `{{${pairName}}}`;
      const close = `{{/${pairName}}}`;
      const newBody = body.substring(0, start) + open + selectedText + close + body.substring(end);
      setBody(newBody);
      setDirty(true);
      setTimeout(() => {
        textarea.focus();
        if (selectedText) {
          textarea.setSelectionRange(start, start + open.length + selectedText.length + close.length);
        } else {
          textarea.setSelectionRange(start + open.length, start + open.length);
        }
      }, 0);
      return;
    }

    // Check if this tag is a block pair opener
    if (tagName in BLOCK_PAIRS) {
      const open = `{{${tagName}}}`;
      const close = `{{${BLOCK_PAIRS[tagName]}}}`;
      const newBody = body.substring(0, start) + open + selectedText + close + body.substring(end);
      setBody(newBody);
      setDirty(true);
      setTimeout(() => {
        textarea.focus();
        if (selectedText) {
          textarea.setSelectionRange(start, start + open.length + selectedText.length + close.length);
        } else {
          textarea.setSelectionRange(start + open.length, start + open.length);
        }
      }, 0);
      return;
    }

    // Default: insert tag
    const tag = `{{${tagName}}}`;
    const newBody = body.substring(0, start) + tag + body.substring(end);
    setBody(newBody);
    setDirty(true);
    setTimeout(() => {
      textarea.focus();
      textarea.setSelectionRange(start + tag.length, start + tag.length);
    }, 0);
  };

  const toggleCategory = (cat: string) => {
    setCollapsed(prev => {
      const next = new Set(prev);
      if (next.has(cat)) {
        next.delete(cat);
      } else {
        next.add(cat);
      }
      return next;
    });
  };

  return (
    <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50">
      <div className="bg-[#1a1a2e] border border-[#2a2a4a] rounded-lg w-[95vw] h-[90vh] flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between px-4 py-3 border-b border-[#2a2a4a]">
          <div className="flex items-center gap-3">
            <h2 className="text-lg font-semibold">Editeur de templates</h2>
            <select
              value={contentType}
              onChange={(e) => {
                if (dirty && !confirm("Modifications non sauvegardées. Continuer ?")) return;
                setContentType(e.target.value as ContentType);
                setSelected("default");
                setDirty(false);
              }}
              className="bg-[#16213e] text-white border border-[#2a2a4a] rounded px-2 py-1 text-sm"
            >
              <option value="film">Film</option>
              <option value="serie">Série</option>
              <option value="jeu">Jeu</option>
              <option value="app">Application</option>
            </select>
          </div>
          <button onClick={onClose} className="text-gray-400 hover:text-white text-xl leading-none">&times;</button>
        </div>

        {/* Template selector bar */}
        <div className="flex items-center gap-2 px-4 py-2 border-b border-[#2a2a4a] bg-[#16213e]/50">
          <select
            value={selected}
            onChange={(e) => handleSelectTemplate(e.target.value)}
            className="bg-[#16213e] text-white border border-[#2a2a4a] rounded px-2 py-1 text-sm flex-1 max-w-xs"
          >
            {templates.map((t) => (
              <option key={t.name} value={t.name}>
                {t.name}{t.is_default ? " (par défaut)" : ""}
              </option>
            ))}
          </select>

          <button
            onClick={() => setShowNewDialog(true)}
            className="bg-green-700 hover:bg-green-600 text-white px-3 py-1 rounded text-sm"
          >
            Nouveau
          </button>

          <button
            onClick={handleSave}
            disabled={!dirty || saving}
            className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white px-3 py-1 rounded text-sm"
          >
            {saving ? "..." : "Sauvegarder"}
          </button>

          {selected !== "default" && (
            <button
              onClick={handleDelete}
              className="bg-red-700 hover:bg-red-600 text-white px-3 py-1 rounded text-sm"
            >
              Supprimer
            </button>
          )}

          {dirty && <span className="text-yellow-400 text-xs">modifié</span>}
        </div>

        {/* New dialog */}
        {showNewDialog && (
          <div className="px-4 py-2 border-b border-[#2a2a4a] bg-[#0f0f23] flex items-center gap-2">
            <input
              type="text"
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              placeholder="Nom du nouveau template..."
              className="bg-[#16213e] text-white border border-[#2a2a4a] rounded px-2 py-1 text-sm flex-1 max-w-xs"
              autoFocus
              onKeyDown={(e) => e.key === "Enter" && handleNew()}
            />
            <button onClick={handleNew} className="bg-green-700 hover:bg-green-600 text-white px-3 py-1 rounded text-sm">
              Créer
            </button>
            <button
              onClick={() => { setShowNewDialog(false); setNewName(""); }}
              className="text-gray-400 hover:text-white text-sm"
            >
              Annuler
            </button>
          </div>
        )}

        {/* Main content: 3 columns */}
        <div className="flex flex-1 min-h-0">
          {/* Tag reference sidebar */}
          <div className="w-72 border-r border-[#2a2a4a] flex flex-col bg-[#16213e]/30">
            <div className="px-3 py-2 border-b border-[#2a2a4a] text-sm font-medium text-gray-300">
              Balises
            </div>
            <div className="flex-1 overflow-y-auto">
              {tagsByCategory.map(([category, categoryTags]) => (
                <div key={category}>
                  <button
                    onClick={() => toggleCategory(category)}
                    className="w-full flex items-center gap-1.5 px-3 py-1.5 text-xs font-semibold text-gray-400 uppercase tracking-wider hover:bg-[#2a2a4a]/50 transition-colors"
                  >
                    <span className="text-[10px]">{collapsed.has(category) ? "\u25B6" : "\u25BC"}</span>
                    {category}
                  </button>
                  {!collapsed.has(category) && categoryTags.map((t) => (
                    <button
                      key={t.name}
                      onClick={() => insertTag(t.name)}
                      title={`${t.description}${t.example ? '\nExemple : ' + t.example : ''}`}
                      className="w-full text-left px-3 py-1 hover:bg-[#2a2a4a] transition-colors group"
                    >
                      <div className="text-xs font-mono text-blue-400 group-hover:text-blue-300">
                        {"{{" + t.name + "}}"}
                      </div>
                      <div className="text-[11px] text-gray-500 group-hover:text-gray-400 leading-tight">
                        {t.description}
                      </div>
                    </button>
                  ))}
                </div>
              ))}
            </div>
          </div>

          {/* Editor */}
          <div className="flex-1 flex flex-col min-w-0 border-r border-[#2a2a4a]">
            <div className="px-3 py-2 border-b border-[#2a2a4a] bg-[#16213e]">
              <span className="text-sm font-medium text-gray-300">Template</span>
            </div>
            <textarea
              id="template-body"
              value={body}
              onChange={(e) => { setBody(e.target.value); setDirty(true); }}
              className="flex-1 bg-[#0f0f23] text-gray-200 font-mono text-sm p-4 resize-none outline-none border-none"
              spellCheck={false}
            />
          </div>

          {/* Preview */}
          <div className="flex-1 flex flex-col min-w-0">
            <div className="px-3 py-2 border-b border-[#2a2a4a] bg-[#16213e]">
              <span className="text-sm font-medium text-gray-300">Aperçu (données fictives)</span>
            </div>
            <div className="flex-1 bg-[#1a1a2e]">
              <iframe
                srcDoc={previewHtml}
                className="w-full h-full border-none"
                sandbox="allow-same-origin"
                title="Aperçu template"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
