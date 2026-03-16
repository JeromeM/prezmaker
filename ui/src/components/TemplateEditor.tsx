import { useState, useEffect, useCallback, useRef, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { openUrl } from "@tauri-apps/plugin-opener";
import { useTranslation } from "react-i18next";
import type { ContentType, ContentTemplate, TemplateTag, SettingsPayload, OutputFormat } from "../types/api";
import { wrapHtmlDocument } from "../utils/wrapHtml";

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

const CLOSING_TAGS = new Set(Object.values(BLOCK_PAIRS));

// --- Depth-aware tag extraction (handles nested {{...}}) ---

/** Extract top-level {{...}} tag positions from text, handling nesting */
function extractTagPositions(text: string): { start: number; end: number }[] {
  const positions: { start: number; end: number }[] = [];
  let i = 0;
  while (i < text.length - 1) {
    if (text[i] === '{' && text[i + 1] === '{') {
      let depth = 1;
      let j = i + 2;
      while (j < text.length - 1 && depth > 0) {
        if (text[j] === '{' && text[j + 1] === '{') {
          depth++; j += 2;
        } else if (text[j] === '}' && text[j + 1] === '}') {
          depth--;
          if (depth === 0) { positions.push({ start: i, end: j + 2 }); break; }
          j += 2;
        } else { j++; }
      }
      i = (depth === 0) ? j + 2 : i + 1;
    } else { i++; }
  }
  return positions;
}

/** Get the tag name (first word before : or space) from a full tag string */
function getTagName(tag: string): string {
  const inner = tag.slice(2, -2);
  const nameEnd = inner.search(/[\s:]/);
  return (nameEnd >= 0 ? inner.substring(0, nameEnd) : inner).toLowerCase().trim();
}

/** Check if tag has args (contains : after the tag name) */
function tagHasArgs(tag: string): boolean {
  const inner = tag.slice(2, -2);
  const nameEnd = inner.search(/[\s:]/);
  if (nameEnd < 0) return false;
  // #if always has "args" (condition) but that's via space not colon
  if (inner[nameEnd] === ':') return true;
  return false;
}

function isIndentOpen(tag: string): boolean {
  const name = getTagName(tag);
  if (name === '#if') return true;
  // Block pairs open only without args (e.g. {{quote}} but NOT {{quote:content}})
  if (name in BLOCK_PAIRS && !tagHasArgs(tag)) return true;
  return false;
}

function isIndentClose(tag: string): boolean {
  const name = getTagName(tag);
  if (name === '/if') return true;
  if (CLOSING_TAGS.has(name)) return true;
  return false;
}

/** Auto-indent a template body based on block nesting */
function autoIndent(body: string): string {
  const lines = body.split("\n");
  const result: string[] = [];
  let depth = 0;
  const INDENT = "    "; // 4 spaces

  for (const rawLine of lines) {
    const line = rawLine.trim();
    if (!line) {
      result.push("");
      continue;
    }

    // Extract tags with proper nesting support
    const tagPositions = extractTagPositions(line);
    const tags = tagPositions.map(p => line.substring(p.start, p.end));

    // Text without tags
    let textOnly = line;
    for (const tag of tags) textOnly = textOnly.replace(tag, "");
    const textWithoutTags = textOnly.trim();

    // Count opens and closes on this line
    let lineOpens = 0;
    let lineCloses = 0;
    for (const tag of tags) {
      if (isIndentClose(tag)) lineCloses++;
      if (isIndentOpen(tag)) lineOpens++;
    }

    // If line is ONLY closing tags (no other content, no opens), dedent before
    // If line has mixed content, dedent only by the closes that come first
    const isOnlyClose = textWithoutTags === "" && lineOpens === 0 && lineCloses > 0;
    const isOnlyOpen = textWithoutTags === "" && lineCloses === 0 && lineOpens > 0;

    if (isOnlyClose) {
      depth = Math.max(0, depth - lineCloses);
      result.push(INDENT.repeat(depth) + line);
    } else if (isOnlyOpen) {
      result.push(INDENT.repeat(depth) + line);
      depth += lineOpens;
    } else {
      // Mixed line or content line: dedent for leading closes, then re-indent for opens
      depth = Math.max(0, depth - lineCloses);
      result.push(INDENT.repeat(depth) + line);
      depth += lineOpens;
    }
  }

  return result.join("\n");
}

// Category display order
const CATEGORY_ORDER = [
  "Mise en page",
  "Formatage",
  "Images",
  "Tableaux",
  "Raccourcis",
  "Donnees",
  "Donnees techniques",
  "Liens",
  "Notes",
  "Conditionnel",
];

// Only "Mise en page" is open by default
const DEFAULT_COLLAPSED = new Set(
  CATEGORY_ORDER.filter(c => c !== "Mise en page")
);

// Tags that accept a color as last arg
const COLOR_TAGS = new Set(["heading", "section", "sub_section", "inline_heading"]);

// --- Syntax highlighting ---

interface HighlightSpan {
  text: string;
  className: string;
  error?: string;
}

/** Find last index in array matching predicate (ES2020-compatible) */
function findLastIdx<T>(arr: T[], pred: (item: T) => boolean): number {
  for (let i = arr.length - 1; i >= 0; i--) {
    if (pred(arr[i])) return i;
  }
  return -1;
}

function findUnmatchedTags(body: string): Map<number, string> {
  const unmatched = new Map<number, string>();
  const positions = extractTagPositions(body);
  const stack: { name: string; pos: number }[] = [];

  for (const p of positions) {
    const tag = body.substring(p.start, p.end);
    const name = getTagName(tag);
    const hasArgs = tagHasArgs(tag);

    if (name === "/if") {
      const idx = findLastIdx(stack, (e) => e.name === "#if");
      if (idx >= 0) { stack.splice(idx, 1); } else { unmatched.set(p.start, "{{/if}} sans {{#if}} correspondant"); }
    } else if (name === "#if") {
      stack.push({ name: "#if", pos: p.start });
    } else if (name.startsWith("/")) {
      const base = name.slice(1);
      const idx = findLastIdx(stack, (e) => e.name === base);
      if (idx >= 0) { stack.splice(idx, 1); } else { unmatched.set(p.start, `{{/${base}}} sans {{${base}}} correspondant`); }
    } else if (!hasArgs && name in BLOCK_PAIRS) {
      stack.push({ name, pos: p.start });
    }
  }
  for (const s of stack) {
    const closing = s.name === "#if" ? "{{/if}}" : `{{/${s.name}}}`;
    unmatched.set(s.pos, `${closing} manquant`);
  }
  return unmatched;
}

const LAYOUT_TAGS = new Set([
  ...Object.keys(BLOCK_PAIRS),
  ...Object.values(BLOCK_PAIRS).map(v => v),
  ...Array.from(COLOR_TAGS),
  "hr", "br", "footer", "field", "color", "size", "url", "img", "img_poster",
  "img_cover", "img_logo", "spoiler", "td", "th",
  "poster_info", "cover_info", "logo_info", "ratings_table",
  "tech_table", "game_tech_table", "game_reqs_table", "app_tech_table", "screenshots_grid",
]);

function classifyTag(tag: string): string {
  const name = getTagName(tag);
  if (name === "#if" || name === "/if") return "hl-cond";
  if (name.startsWith("/") && CLOSING_TAGS.has(name)) return "hl-closing";
  if (LAYOUT_TAGS.has(name) || LAYOUT_TAGS.has("/" + name.replace("/", ""))) return "hl-layout";
  return "hl-data";
}

function highlightTemplate(body: string): HighlightSpan[] {
  const spans: HighlightSpan[] = [];
  const unmatchedMap = findUnmatchedTags(body);
  const positions = extractTagPositions(body);
  let lastIndex = 0;

  for (const p of positions) {
    // Text before this tag
    if (p.start > lastIndex) {
      spans.push({ text: body.slice(lastIndex, p.start), className: "hl-text" });
    }

    const tagText = body.substring(p.start, p.end);
    let cls = classifyTag(tagText);
    const errorMsg = unmatchedMap.get(p.start);
    if (errorMsg) cls += " hl-unmatched";

    spans.push({ text: tagText, className: cls, error: errorMsg });
    lastIndex = p.end;
  }

  // Trailing text
  if (lastIndex < body.length) {
    spans.push({ text: body.slice(lastIndex), className: "hl-text" });
  }

  return spans;
}

// --- Color picker popup ---

function ColorPickerPopup({ value, onChange, onClose }: {
  value: string;
  onChange: (hex: string) => void;
  onClose: () => void;
}) {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handler = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) onClose();
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, [onClose]);

  return (
    <div ref={ref} className="absolute top-full left-0 mt-1 z-50 bg-surface border border-edge rounded p-2 shadow-lg">
      <input
        type="color"
        value={`#${value}`}
        onChange={(e) => onChange(e.target.value.replace("#", ""))}
        className="w-8 h-8 cursor-pointer border-0 bg-transparent"
      />
      <input
        type="text"
        value={value}
        onChange={(e) => {
          const v = e.target.value.replace("#", "").slice(0, 6);
          if (/^[0-9a-fA-F]*$/.test(v)) onChange(v);
        }}
        className="ml-1 w-20 bg-input text-fg-bright border border-edge rounded px-1 py-0.5 text-xs font-mono"
        maxLength={6}
        placeholder="hex"
      />
    </div>
  );
}

// --- Main component ---

export default function TemplateEditor({ onClose }: Props) {
  const { t } = useTranslation();
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
  const [globalColor, setGlobalColor] = useState("c0392b");
  const [customColor, setCustomColor] = useState<string | null>(null); // null = use global
  const [showTitleColorPicker, setShowTitleColorPicker] = useState(false);
  const [favoriteName, setFavoriteName] = useState<string | null>(null);
  const titleColor = customColor ?? globalColor;
  const [showColorPicker, setShowColorPicker] = useState(false);
  const [pickedColor, setPickedColor] = useState("e74c3c");
  const [tagSearch, setTagSearch] = useState("");
  const [outputFormat, setOutputFormat] = useState<OutputFormat>("bbcode");
  const debounceRef = useRef<ReturnType<typeof setTimeout>>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const highlightRef = useRef<HTMLDivElement>(null);

  // Load global title color from settings
  useEffect(() => {
    invoke<SettingsPayload>("get_settings").then((s) => {
      setGlobalColor(s.title_color || "c0392b");
    });
  }, []);

  const updatePreview = useCallback(async (templateBody: string, ct: string, color: string, fmt: OutputFormat) => {
    try {
      const rendered = await invoke<string>("preview_template", {
        body: templateBody,
        contentType: ct,
        titleColor: color,
        outputFormat: fmt,
      });
      if (fmt === "html") {
        setPreviewHtml(wrapHtmlDocument(rendered));
      } else {
        // BBCode output: convert to HTML for preview display
        const html = await invoke<string>("convert_bbcode", { bbcode: rendered });
        setPreviewHtml(html);
      }
    } catch (e) {
      console.error("Preview error:", e);
    }
  }, []);

  const debouncedPreview = useCallback((templateBody: string, ct: string, color: string, fmt: OutputFormat) => {
    if (debounceRef.current) clearTimeout(debounceRef.current);
    debounceRef.current = setTimeout(() => updatePreview(templateBody, ct, color, fmt), 400);
  }, [updatePreview]);

  useEffect(() => {
    return () => { if (debounceRef.current) clearTimeout(debounceRef.current); };
  }, []);

  // Open links from iframe preview in system browser
  useEffect(() => {
    const handler = (e: MessageEvent) => {
      if (e.data?.type === "open-url" && typeof e.data.url === "string") {
        openUrl(e.data.url);
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, []);

  const loadTemplates = useCallback(async (ct: string) => {
    try {
      const list = await invoke<ContentTemplate[]>("list_content_templates", { contentType: ct });
      setTemplates(list);
      const current = list.find((t) => t.name === selected) || list[0];
      if (current) {
        setSelected(current.name);
        setBody(autoIndent(current.body));
        setCustomColor(current.title_color ?? null);
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
    invoke<string | null>("get_default_template", { contentType }).then(setFavoriteName);
  }, [contentType]);

  // Update preview when body, contentType or titleColor changes
  useEffect(() => {
    if (body) debouncedPreview(body, contentType, titleColor, outputFormat);
  }, [body, contentType, titleColor, outputFormat, debouncedPreview]);

  // Sync scroll between textarea and highlight overlay
  const syncScroll = useCallback(() => {
    if (textareaRef.current && highlightRef.current) {
      highlightRef.current.scrollTop = textareaRef.current.scrollTop;
      highlightRef.current.scrollLeft = textareaRef.current.scrollLeft;
    }
  }, []);

  // Compute highlighted spans
  const highlightedSpans = useMemo(() => highlightTemplate(body), [body]);

  // Group tags by category
  const tagsByCategory = useMemo(() => {
    const map = new Map<string, TemplateTag[]>();
    for (const t of tags) {
      const list = map.get(t.category) || [];
      list.push(t);
      map.set(t.category, list);
    }
    const sorted: [string, TemplateTag[]][] = [];
    for (const cat of CATEGORY_ORDER) {
      const list = map.get(cat);
      if (list) sorted.push([cat, list]);
    }
    for (const [cat, list] of map) {
      if (!CATEGORY_ORDER.includes(cat)) {
        sorted.push([cat, list]);
      }
    }
    return sorted;
  }, [tags]);

  // Filter tags by search query
  const filteredTagsByCategory = useMemo(() => {
    if (!tagSearch.trim()) return tagsByCategory;
    const q = tagSearch.toLowerCase();
    const result: [string, TemplateTag[]][] = [];
    for (const [cat, catTags] of tagsByCategory) {
      const filtered = catTags.filter(
        (t) => t.name.toLowerCase().includes(q) || t.description.toLowerCase().includes(q)
      );
      if (filtered.length > 0) result.push([cat, filtered]);
    }
    return result;
  }, [tagsByCategory, tagSearch]);

  const handleSelectTemplate = async (name: string) => {
    if (dirty && !confirm(t("templateEditor.unsavedChanges"))) return;
    try {
      const tpl = await invoke<ContentTemplate>("get_content_template", {
        contentType,
        name,
      });
      setSelected(tpl.name);
      setBody(autoIndent(tpl.body));
      setCustomColor(tpl.title_color ?? null);
      setDirty(false);
    } catch (e) {
      console.error(e);
    }
  };

  const handleSave = async () => {
    setSaving(true);
    try {
      await invoke("save_content_template", { contentType, name: selected, body, titleColor: customColor });
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
      setBody(autoIndent(tpl.body));
      setCustomColor(tpl.title_color ?? null);
      setDirty(false);
    } catch (e) {
      alert("Erreur: " + e);
    }
  };

  const handleDelete = async () => {
    if (selected === "default") return;
    if (!confirm(t("templateEditor.confirmDelete", { name: selected }))) return;
    try {
      await invoke("delete_content_template", { contentType, name: selected });
      setSelected("default");
      await loadTemplates(contentType);
    } catch (e) {
      alert("Erreur: " + e);
    }
  };

  const handleExport = async () => {
    const path = await save({
      defaultPath: `${selected}.json`,
      filters: [{ name: "Template", extensions: ["json"] }],
    });
    if (!path) return;
    try {
      await invoke("export_template", { contentType, name: selected, path });
    } catch (e) {
      alert("Erreur export: " + e);
    }
  };

  const handleImport = async () => {
    const path = await open({
      filters: [{ name: "Template", extensions: ["json"] }],
      multiple: false,
    });
    if (!path) return;
    try {
      const tpl = await invoke<ContentTemplate>("import_template", { path });
      // Switch to the imported template's content type if different
      if (tpl.content_type !== contentType) {
        setContentType(tpl.content_type as ContentType);
      }
      setSelected(tpl.name);
      await loadTemplates(tpl.content_type);
      setBody(autoIndent(tpl.body));
      setCustomColor(tpl.title_color ?? null);
      setDirty(false);
    } catch (e) {
      alert("Erreur import: " + e);
    }
  };

  const insertTag = (tagName: string) => {
    const textarea = textareaRef.current;
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
        const cursor = selectedText ? start + open.length + selectedText.length + close.length : start + open.length;
        textarea.setSelectionRange(selectedText ? start : cursor, cursor);
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
        const cursor = selectedText ? start + open.length + selectedText.length + close.length : start + open.length;
        textarea.setSelectionRange(selectedText ? start : cursor, cursor);
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

  const insertColorTag = () => {
    const textarea = textareaRef.current;
    if (!textarea) return;
    const start = textarea.selectionStart;
    const end = textarea.selectionEnd;
    const selectedText = body.substring(start, end) || "texte";
    const tag = `{{color:${pickedColor}:${selectedText}}}`;
    const newBody = body.substring(0, start) + tag + body.substring(end);
    setBody(newBody);
    setDirty(true);
    setShowColorPicker(false);
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
      <div className="bg-surface border border-edge rounded-lg w-[95vw] h-[90vh] flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between px-4 py-3 border-b border-edge">
          <div className="flex items-center gap-3">
            <h2 className="text-lg font-semibold">{t("templateEditor.title")}</h2>
            <select
              value={contentType}
              onChange={(e) => {
                if (dirty && !confirm(t("templateEditor.unsavedChanges"))) return;
                setContentType(e.target.value as ContentType);
                setSelected("default");
                setDirty(false);
              }}
              className="bg-input text-fg-bright border border-edge rounded px-2 py-1 text-sm"
            >
              <option value="film">Film</option>
              <option value="serie">Série</option>
              <option value="jeu">Jeu</option>
              <option value="app">Application</option>
            </select>
            {/* Output format toggle */}
            <div className="flex items-center gap-0.5 bg-input rounded-md p-0.5">
              <button
                onClick={() => setOutputFormat("bbcode")}
                className={`text-xs px-2.5 py-1 rounded transition-colors font-medium ${
                  outputFormat === "bbcode"
                    ? "bg-blue-600 text-white shadow-sm"
                    : "text-fg-muted hover:text-fg-bright"
                }`}
              >
                BBCode
              </button>
              <button
                onClick={() => setOutputFormat("html")}
                className={`text-xs px-2.5 py-1 rounded transition-colors font-medium ${
                  outputFormat === "html"
                    ? "bg-blue-600 text-white shadow-sm"
                    : "text-fg-muted hover:text-fg-bright"
                }`}
              >
                HTML
              </button>
            </div>
          </div>
          <button onClick={onClose} className="text-fg-muted hover:text-fg-bright text-xl leading-none">&times;</button>
        </div>

        {/* Template selector bar */}
        <div className="flex items-center gap-2 px-4 py-2 border-b border-edge bg-input/50">
          <select
            value={selected}
            onChange={(e) => handleSelectTemplate(e.target.value)}
            className="bg-input border border-edge rounded px-2 py-1 text-sm text-fg max-w-xs"
          >
            {templates.map((t) => (
              <option key={t.name} value={t.name}>
                {t.name === favoriteName ? `★ ${t.name}` : t.name}
              </option>
            ))}
          </select>

          <button
            onClick={async () => {
              const newFav = selected === favoriteName ? "" : selected;
              await invoke("set_default_template", { contentType, templateName: newFav });
              setFavoriteName(newFav || null);
            }}
            title={selected === favoriteName ? t("templateEditor.unsetDefault") : t("templateEditor.setDefault")}
            className={`p-1.5 rounded transition-colors ${
              selected === favoriteName
                ? "text-yellow-400 hover:text-yellow-300"
                : "text-fg-faint hover:text-fg-muted"
            }`}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill={selected === favoriteName ? "currentColor" : "none"}
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="w-4 h-4"
            >
              <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
            </svg>
          </button>

          <button
            onClick={() => setShowNewDialog(true)}
            className="bg-green-700 hover:bg-green-600 text-white px-3 py-1 rounded text-sm"
          >
            {t("common.new")}
          </button>

          <button
            onClick={handleSave}
            disabled={!dirty || saving}
            className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white px-3 py-1 rounded text-sm"
          >
            {saving ? "..." : t("common.save")}
          </button>

          {selected !== "default" && (
            <button
              onClick={handleDelete}
              className="bg-red-700 hover:bg-red-600 text-white px-3 py-1 rounded text-sm"
            >
              {t("common.delete")}
            </button>
          )}

          <button
            onClick={handleExport}
            className="bg-input hover:bg-input-hover border border-edge text-fg hover:text-fg-bright px-3 py-1 rounded text-sm"
            title={t("common.export")}
          >
            {t("common.export")}
          </button>

          <button
            onClick={handleImport}
            className="bg-input hover:bg-input-hover border border-edge text-fg hover:text-fg-bright px-3 py-1 rounded text-sm"
            title={t("common.import")}
          >
            {t("common.import")}
          </button>

          {dirty && <span className="text-yellow-400 text-xs">{t("templateEditor.modified")}</span>}

          {/* Per-template title color picker — right-aligned */}
          <div className="relative flex items-center gap-1 ml-auto">
            <span className="text-xs text-fg-muted">{t("templateEditor.titleColor")}</span>
            <button
              onClick={() => setShowTitleColorPicker(!showTitleColorPicker)}
              className="w-6 h-6 rounded border border-edge cursor-pointer"
              style={{ backgroundColor: `#${titleColor}` }}
              title={`Couleur des titres : #${titleColor}${customColor ? '' : ' (défaut)'}`}
            />
            {customColor && (
              <button
                onClick={() => { setCustomColor(null); setDirty(true); }}
                className="text-[10px] text-fg-dim hover:text-fg"
                title={t("templateEditor.resetColor")}
              >
                {t("templateEditor.resetColor")}
              </button>
            )}
            {!customColor && (
              <span className="text-[10px] text-fg-dim">défaut</span>
            )}
            {showTitleColorPicker && (
              <ColorPickerPopup
                value={titleColor}
                onChange={(v) => { setCustomColor(v); setDirty(true); }}
                onClose={() => setShowTitleColorPicker(false)}
              />
            )}
          </div>
        </div>

        {/* New dialog */}
        {showNewDialog && (
          <div className="px-4 py-2 border-b border-edge bg-base flex items-center gap-2">
            <input
              type="text"
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              placeholder={t("templateEditor.templateName")}
              className="bg-input text-fg-bright border border-edge rounded px-2 py-1 text-sm flex-1 max-w-xs"
              autoFocus
              onKeyDown={(e) => e.key === "Enter" && handleNew()}
            />
            <button onClick={handleNew} className="bg-green-700 hover:bg-green-600 text-white px-3 py-1 rounded text-sm">
              {t("common.create")}
            </button>
            <button
              onClick={() => { setShowNewDialog(false); setNewName(""); }}
              className="text-fg-muted hover:text-fg-bright text-sm"
            >
              {t("common.cancel")}
            </button>
          </div>
        )}

        {/* Main content: 3 columns */}
        <div className="flex flex-1 min-h-0">
          {/* Tag reference sidebar */}
          <div className="w-72 border-r border-edge flex flex-col bg-input/30">
            <div className="px-3 py-2 border-b border-edge">
              <input
                type="text"
                value={tagSearch}
                onChange={(e) => setTagSearch(e.target.value)}
                placeholder={t("templateEditor.searchTags")}
                className="w-full bg-base text-fg-bright border border-edge rounded px-2 py-1 text-xs outline-none focus:border-blue-500 placeholder-fg-faint"
              />
            </div>
            <div className="flex-1 overflow-y-auto">
              {/* HTML style helpers */}
              {outputFormat === "html" && !tagSearch.trim() && (
                <div className="border-b border-edge pb-1">
                  <div className="px-3 py-1.5 text-xs font-semibold text-fg-muted uppercase tracking-wider">
                    Styles HTML
                  </div>
                  {[
                    { label: "| style", insert: " | ", desc: "Ajouter un style CSS" },
                    { label: "background", insert: " | background:#1a2744", desc: "Couleur de fond" },
                    { label: "border-radius", insert: " | border-radius:8px", desc: "Coins arrondis" },
                    { label: "padding", insert: " | padding:12px 16px", desc: "Espacement interne" },
                    { label: "border-left", insert: " | border-left:4px solid #c0392b", desc: "Bordure gauche" },
                    { label: "box-shadow", insert: " | box-shadow:0 4px 12px rgba(0,0,0,0.3)", desc: "Ombre" },
                    { label: "gradient hr", insert: " | border:none;height:2px;background:linear-gradient(90deg,transparent,#c0392b,transparent)", desc: "Séparateur dégradé" },
                  ].map((item) => (
                    <button
                      key={item.label}
                      onClick={() => {
                        const ta = textareaRef.current;
                        if (!ta) return;
                        const pos = ta.selectionStart;
                        // Find the end of the current tag (before }})
                        const before = body.substring(0, pos);
                        const tagEnd = body.indexOf("}}", pos);
                        if (tagEnd >= 0) {
                          const newBody = body.substring(0, tagEnd) + item.insert + body.substring(tagEnd);
                          setBody(newBody);
                          setDirty(true);
                          setTimeout(() => {
                            ta.focus();
                            ta.setSelectionRange(tagEnd + item.insert.length, tagEnd + item.insert.length);
                          }, 0);
                        } else {
                          // No tag context, insert at cursor
                          const newBody = before + item.insert + body.substring(pos);
                          setBody(newBody);
                          setDirty(true);
                        }
                      }}
                      title={item.desc}
                      className="w-full text-left px-3 py-1 hover:bg-edge transition-colors group"
                    >
                      <div className="text-xs font-mono text-orange-400 group-hover:text-orange-300">{item.label}</div>
                      <div className="text-[11px] text-fg-dim">{item.desc}</div>
                    </button>
                  ))}
                  {/* HTML-exclusive tags */}
                  <div className="px-3 py-1.5 text-xs font-semibold text-fg-muted uppercase tracking-wider mt-1">
                    Tags HTML
                  </div>
                  {[
                    { tag: "details:Titre", desc: "Section dépliable" },
                    { tag: "/details", desc: "Fin section dépliable" },
                    { tag: "summary:Titre", desc: "Titre du dépliable" },
                    { tag: "p:contenu", desc: "Paragraphe" },
                    { tag: "pre:code", desc: "Bloc pré-formaté" },
                  ].map((item) => (
                    <button
                      key={item.tag}
                      onClick={() => insertTag(item.tag)}
                      className="w-full text-left px-3 py-1 hover:bg-edge transition-colors group"
                    >
                      <div className="text-xs font-mono text-blue-400 group-hover:text-blue-300">{`{{${item.tag}}}`}</div>
                      <div className="text-[11px] text-fg-dim">{item.desc}</div>
                    </button>
                  ))}
                </div>
              )}
              {filteredTagsByCategory.map(([category, categoryTags]) => (
                <div key={category}>
                  <button
                    onClick={() => toggleCategory(category)}
                    className="w-full flex items-center gap-1.5 px-3 py-1.5 text-xs font-semibold text-fg-muted uppercase tracking-wider hover:bg-edge/50 transition-colors"
                  >
                    <span className="text-[10px]">{collapsed.has(category) ? "\u25B6" : "\u25BC"}</span>
                    {category}
                  </button>
                  {(!collapsed.has(category) || tagSearch.trim()) && categoryTags.map((t) => {
                    const tagName = t.name.split(":")[0].toLowerCase();
                    const isColorTag = tagName === "color";
                    return (
                      <div key={t.name} className="flex items-center">
                        <button
                          onClick={() => {
                            if (isColorTag) {
                              setShowColorPicker(true);
                            } else {
                              insertTag(t.name);
                            }
                          }}
                          title={`${t.description}${t.example ? '\nExemple : ' + t.example : ''}`}
                          className="flex-1 text-left px-3 py-1 hover:bg-edge transition-colors group min-w-0"
                        >
                          <div className="text-xs font-mono text-blue-400 group-hover:text-blue-300 truncate">
                            {"{{" + t.name + "}}"}
                          </div>
                          <div className="text-[11px] text-fg-dim group-hover:text-fg-muted leading-tight">
                            {t.description}
                          </div>
                        </button>
                        {isColorTag && (
                          <div className="relative pr-2">
                            <button
                              onClick={() => setShowColorPicker(!showColorPicker)}
                              className="w-4 h-4 rounded border border-edge cursor-pointer"
                              style={{ backgroundColor: `#${pickedColor}` }}
                              title="Choisir une couleur"
                            />
                            {showColorPicker && (
                              <div className="absolute right-0 top-full mt-1 z-50 bg-surface border border-edge rounded p-2 shadow-lg flex flex-col gap-1">
                                <div className="flex items-center gap-1">
                                  <input
                                    type="color"
                                    value={`#${pickedColor}`}
                                    onChange={(e) => setPickedColor(e.target.value.replace("#", ""))}
                                    className="w-8 h-8 cursor-pointer border-0 bg-transparent"
                                  />
                                  <input
                                    type="text"
                                    value={pickedColor}
                                    onChange={(e) => {
                                      const v = e.target.value.replace("#", "").slice(0, 6);
                                      if (/^[0-9a-fA-F]*$/.test(v)) setPickedColor(v);
                                    }}
                                    className="w-16 bg-input text-fg-bright border border-edge rounded px-1 py-0.5 text-xs font-mono"
                                    maxLength={6}
                                  />
                                </div>
                                <button
                                  onClick={insertColorTag}
                                  className="bg-blue-600 hover:bg-blue-700 text-white text-xs px-2 py-1 rounded"
                                >
                                  Insérer
                                </button>
                              </div>
                            )}
                          </div>
                        )}
                      </div>
                    );
                  })}
                </div>
              ))}
            </div>
          </div>

          {/* Editor with syntax highlighting */}
          <div className="flex-1 flex flex-col min-w-0 border-r border-edge">
            <div className="px-3 py-2 border-b border-edge bg-input">
              <span className="text-sm font-medium text-fg">Template</span>
            </div>
            <div className="flex-1 relative overflow-hidden">
              {/* Highlight underlay */}
              <div
                ref={highlightRef}
                className="absolute inset-0 overflow-hidden pointer-events-none p-4 font-mono text-sm whitespace-pre-wrap break-words"
                style={{ wordBreak: "break-all" }}
                aria-hidden="true"
              >
                {highlightedSpans.map((span, i) => (
                  <span
                    key={i}
                    className={span.className}
                    title={span.error}
                    style={span.error ? { pointerEvents: "auto", cursor: "help" } : undefined}
                  >{span.text}</span>
                ))}
                {/* Extra line to match textarea scrollable area */}
                {"\n"}
              </div>
              {/* Transparent textarea on top */}
              <textarea
                ref={textareaRef}
                id="template-body"
                value={body}
                onChange={(e) => { setBody(e.target.value); setDirty(true); }}
                onScroll={syncScroll}
                onKeyDown={(e) => {
                  if (e.key === "Tab") {
                    e.preventDefault();
                    const ta = textareaRef.current!;
                    const start = ta.selectionStart;
                    const end = ta.selectionEnd;

                    if (start === end && !e.shiftKey) {
                      // Simple tab: insert 4 spaces
                      const newBody = body.substring(0, start) + "    " + body.substring(end);
                      setBody(newBody);
                      setDirty(true);
                      requestAnimationFrame(() => { ta.selectionStart = ta.selectionEnd = start + 4; });
                    } else {
                      // Multi-line indent/dedent
                      const lineStart = body.lastIndexOf("\n", start - 1) + 1;
                      const lineEnd = end === start ? body.indexOf("\n", end) : body.lastIndexOf("\n", end - 1) + 1 < lineStart ? end : (() => { const i = body.indexOf("\n", end); return i < 0 ? body.length : i; })();
                      const block = body.substring(lineStart, lineEnd);
                      const lines = block.split("\n");
                      const modified = lines.map((line) =>
                        e.shiftKey
                          ? line.startsWith("    ") ? line.substring(4) : line.replace(/^\t/, "")
                          : "    " + line
                      );
                      const newBlock = modified.join("\n");
                      const newBody = body.substring(0, lineStart) + newBlock + body.substring(lineEnd);
                      setBody(newBody);
                      setDirty(true);
                      requestAnimationFrame(() => {
                        ta.selectionStart = lineStart;
                        ta.selectionEnd = lineStart + newBlock.length;
                      });
                    }
                  }
                }}
                className="absolute inset-0 w-full h-full bg-transparent text-transparent caret-gray-200 font-mono text-sm p-4 resize-none outline-none border-none"
                style={{ caretColor: "#e0e0e0", WebkitTextFillColor: "transparent" }}
                spellCheck={false}
              />
            </div>
          </div>

          {/* Preview */}
          <div className="flex-1 flex flex-col min-w-0">
            <div className="px-3 py-2 border-b border-edge bg-input">
              <span className="text-sm font-medium text-fg">{t("templateEditor.preview")}</span>
            </div>
            <div className="flex-1 bg-surface">
              <iframe
                srcDoc={previewHtml}
                className="w-full h-full border-none"
                sandbox="allow-same-origin allow-scripts"
                title="Aperçu template"
              />
            </div>
          </div>
        </div>
      </div>

      {/* Syntax highlighting styles */}
      <style>{`
        .hl-text { color: #c8c8d8; }
        .hl-layout { color: #5dade2; }
        .hl-data { color: #58d68d; }
        .hl-cond { color: #bb8fce; font-weight: 600; }
        .hl-closing { color: #5dade2; opacity: 0.7; }
        .hl-unmatched { text-decoration: wavy underline #e74c3c; text-underline-offset: 3px; }
      `}</style>
    </div>
  );
}
