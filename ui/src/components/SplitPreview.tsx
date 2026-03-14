import { useState, useEffect, useRef, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import BBCodePanel from "./BBCodePanel";
import BBCodePalette from "./BBCodePalette";
import HtmlPreview from "./HtmlPreview";
import NfoModal from "./NfoModal";
import TemplateManager from "./TemplateManager";
import CollectionSaveDialog from "./CollectionSaveDialog";
import { useTemplates } from "../hooks/useTemplates";
import type { PresentationMeta, MediaAnalysis, SavedPresentation } from "../types/api";

interface Props {
  bbcode: string;
  html: string;
  onConvert: (bbcode: string) => Promise<string>;
  meta: PresentationMeta;
  nfoText?: string | null;
  mediaAnalysis?: MediaAnalysis | null;
}

const PALETTE_KEY = "prezmaker_palette_collapsed";

export default function SplitPreview({ bbcode: initialBBCode, html: initialHtml, onConvert, meta, nfoText, mediaAnalysis: existingAnalysis }: Props) {
  const [bbcode, setBBCode] = useState(initialBBCode);
  const [html, setHtml] = useState(initialHtml);
  const debounceRef = useRef<ReturnType<typeof setTimeout>>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const [paletteCollapsed, setPaletteCollapsed] = useState(() => {
    return localStorage.getItem(PALETTE_KEY) === "true";
  });

  const [nfoContent, setNfoContent] = useState<string | null>(null);
  const [nfoLoading, setNfoLoading] = useState(false);
  const [saved, setSaved] = useState(false);
  const [showSaveDialog, setShowSaveDialog] = useState(false);
  const [savedRef, setSavedRef] = useState<{ collectionId: string; entryId: string } | null>(
    meta.savedRef ?? null
  );

  const { templates, loading, load, save, remove, rename, duplicate } = useTemplates();

  const handleChange = useCallback(
    (newBBCode: string) => {
      setBBCode(newBBCode);
      if (debounceRef.current) clearTimeout(debounceRef.current);
      debounceRef.current = setTimeout(async () => {
        const newHtml = await onConvert(newBBCode);
        setHtml(newHtml);
      }, 300);
    },
    [onConvert],
  );

  useEffect(() => {
    return () => {
      if (debounceRef.current) clearTimeout(debounceRef.current);
    };
  }, []);

  // Ctrl+C shortcut
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === "c") {
        const selection = window.getSelection();
        if (!selection || selection.toString() === "") {
          e.preventDefault();
          navigator.clipboard.writeText(bbcode);
        }
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [bbcode]);

  const togglePalette = useCallback(() => {
    setPaletteCollapsed((prev) => {
      const next = !prev;
      localStorage.setItem(PALETTE_KEY, String(next));
      return next;
    });
  }, []);

  const insertTag = useCallback(
    (open: string, close?: string, placeholder?: string) => {
      const ta = textareaRef.current;
      if (!ta) return;

      const start = ta.selectionStart;
      const end = ta.selectionEnd;
      const text = ta.value;
      const selected = text.slice(start, end);

      let insertion: string;
      let cursorPos: number;

      if (close) {
        if (selected) {
          insertion = open + selected + close;
          cursorPos = start + insertion.length;
        } else {
          const inner = placeholder || "";
          insertion = open + inner + close;
          cursorPos = start + open.length;
        }
      } else {
        insertion = open;
        cursorPos = start + open.length;
      }

      const newBBCode = text.slice(0, start) + insertion + text.slice(end);
      handleChange(newBBCode);

      requestAnimationFrame(() => {
        ta.focus();
        if (close && !selected && placeholder) {
          ta.setSelectionRange(start + open.length, start + open.length + placeholder.length);
        } else {
          ta.setSelectionRange(cursorPos, cursorPos);
        }
      });
    },
    [handleChange],
  );

  const handleLoadTemplate = useCallback(
    async (name: string) => {
      try {
        const content = await load(name);
        handleChange(content);
      } catch (e) {
        console.error("Failed to load template:", e);
      }
    },
    [load, handleChange],
  );

  const handleSaveTemplate = useCallback(
    (name: string) => {
      save(name, bbcode).catch((e) => console.error("Failed to save template:", e));
    },
    [save, bbcode],
  );

  const handleDeleteTemplate = useCallback(
    (name: string) => {
      remove(name).catch((e) => console.error("Failed to delete template:", e));
    },
    [remove],
  );

  const handleRenameTemplate = useCallback(
    (oldName: string, newName: string) => {
      rename(oldName, newName).catch((e) => console.error("Failed to rename template:", e));
    },
    [rename],
  );

  const handleDuplicateTemplate = useCallback(
    (name: string, newName: string) => {
      duplicate(name, newName).catch((e) => console.error("Failed to duplicate template:", e));
    },
    [duplicate],
  );

  const handleGenerateNfo = useCallback(async () => {
    // If we have pre-generated NFO text, use it directly
    if (nfoText) {
      setNfoContent(nfoText);
      return;
    }
    // Fallback: if we have raw MediaInfo text from analysis
    if (existingAnalysis?.raw_text) {
      setNfoContent(existingAnalysis.raw_text);
      return;
    }
    // Last resort: pick a file and get raw mediainfo
    const path = await open({
      filters: [{ name: "Media", extensions: ["mkv", "mp4", "avi", "wmv", "flv", "mov", "ts", "m2ts", "iso"] }],
      multiple: false,
    });
    if (!path) return;
    setNfoLoading(true);
    try {
      const result = await invoke<string>("run_mediainfo", { path });
      setNfoContent(result);
    } catch (e) {
      alert(String(e));
    } finally {
      setNfoLoading(false);
    }
  }, [nfoText, existingAnalysis]);

  const doSave = useCallback(async (collectionId: string, entryId?: string) => {
    try {
      const result = await invoke<SavedPresentation>("save_to_collection", {
        collectionId,
        entryId: entryId ?? null,
        title: meta.title,
        contentType: meta.contentType,
        bbcode,
        posterUrl: meta.posterUrl,
      });
      setSavedRef({ collectionId: result.collection_id, entryId: result.id });
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (e) {
      alert(String(e));
    }
  }, [bbcode, meta]);

  const handleSaveToCollection = useCallback(() => {
    if (savedRef) {
      doSave(savedRef.collectionId, savedRef.entryId);
    } else {
      setShowSaveDialog(true);
    }
  }, [savedRef, doSave]);

  const templateActions = (
    <TemplateManager
      templates={templates}
      loading={loading}
      onSave={handleSaveTemplate}
      onLoad={handleLoadTemplate}
      onDelete={handleDeleteTemplate}
      onRename={handleRenameTemplate}
      onDuplicate={handleDuplicateTemplate}
    />
  );

  return (
    <div className="flex-1 flex min-h-0">
      <div className="w-1/2 flex border-r border-edge">
        <BBCodePalette
          collapsed={paletteCollapsed}
          onToggle={togglePalette}
          onInsertTag={insertTag}
        />
        <div className="flex-1 flex flex-col min-w-0">
          <BBCodePanel
            bbcode={bbcode}
            onChange={handleChange}
            textareaRef={textareaRef}
            headerActions={
              <>
                {templateActions}
                <button
                  onClick={handleSaveToCollection}
                  disabled={saved || !bbcode.trim()}
                  className={`text-xs px-3 py-1 rounded transition-colors ${
                    saved
                      ? "bg-green-700 text-white"
                      : "bg-edge hover:bg-edge-hover text-fg disabled:opacity-50"
                  }`}
                >
                  {saved ? "Sauvegardé !" : "Sauvegarder"}
                </button>
                <button
                  onClick={handleGenerateNfo}
                  disabled={nfoLoading}
                  className={`text-xs px-3 py-1 rounded transition-colors flex items-center gap-1.5 ${
                    nfoLoading
                      ? "bg-blue-600 text-white cursor-wait"
                      : "bg-edge hover:bg-edge-hover text-fg disabled:opacity-50"
                  }`}
                >
                  {nfoLoading && (
                    <span
                      className="inline-block h-3 w-3 border-2 border-white/30 border-t-white rounded-full"
                      style={{ animation: "spin 1s linear infinite" }}
                    />
                  )}
                  {nfoLoading ? "Analyse MediaInfo..." : "NFO"}
                </button>
              </>
            }
          />
        </div>
      </div>
      <div className="w-1/2 flex flex-col">
        <HtmlPreview html={html} />
      </div>
      {nfoContent && (
        <NfoModal content={nfoContent} title={meta.title} onClose={() => setNfoContent(null)} />
      )}
      {showSaveDialog && (
        <CollectionSaveDialog
          onSave={(collectionId) => {
            setShowSaveDialog(false);
            doSave(collectionId);
          }}
          onClose={() => setShowSaveDialog(false)}
        />
      )}
    </div>
  );
}
