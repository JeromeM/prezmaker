import { useState, useEffect, useRef, useCallback } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import BBCodePanel from "./BBCodePanel";
import BBCodePalette from "./BBCodePalette";
import HtmlPreview from "./HtmlPreview";
import NfoModal from "./NfoModal";
import CollectionSaveDialog from "./CollectionSaveDialog";
import UploadDialog from "./UploadDialog";
import type { PresentationMeta, MediaAnalysis, SavedPresentation, SettingsPayload, TorrentInfo } from "../types/api";

interface Props {
  bbcode: string;
  html: string;
  onConvert: (bbcode: string) => Promise<string>;
  meta: PresentationMeta;
  nfoText?: string | null;
  mediaAnalysis?: MediaAnalysis | null;
  torrentFilePath?: string | null;
  torrentInfo?: TorrentInfo | null;
}

const PALETTE_KEY = "prezmaker_palette_collapsed";

export default function SplitPreview({ bbcode: initialBBCode, html: initialHtml, onConvert, meta, nfoText, mediaAnalysis: existingAnalysis, torrentFilePath, torrentInfo }: Props) {
  const { t } = useTranslation();
  const [bbcode, setBBCode] = useState(initialBBCode);
  const [html, setHtml] = useState(initialHtml);
  const debounceRef = useRef<ReturnType<typeof setTimeout>>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const [paletteCollapsed, setPaletteCollapsed] = useState(() => {
    return localStorage.getItem(PALETTE_KEY) === "true";
  });

  const [c411Enabled, setC411Enabled] = useState(false);

  useEffect(() => {
    invoke<SettingsPayload>("get_settings").then((s) => {
      setC411Enabled(s.c411_enabled && !!s.c411_api_key);
    });
  }, []);

  const [nfoContent, setNfoContent] = useState<string | null>(null);
  const [nfoLoading, setNfoLoading] = useState(false);
  const [saved, setSaved] = useState(false);
  const [showSaveDialog, setShowSaveDialog] = useState(false);
  const [showUpload, setShowUpload] = useState(false);
  const [savedRef, setSavedRef] = useState<{ collectionId: string; entryId: string } | null>(
    meta.savedRef ?? null
  );

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
              <div className="flex items-center gap-1.5">
                {/* Groupe : sauvegarde + NFO */}
                <button
                  onClick={handleSaveToCollection}
                  disabled={saved || !bbcode.trim()}
                  className={`text-xs px-2.5 py-1 rounded transition-colors ${
                    saved
                      ? "bg-green-700 text-white"
                      : "bg-edge hover:bg-edge-hover text-fg disabled:opacity-50"
                  }`}
                >
                  {saved ? t("collections.saved") : t("splitPreview.save")}
                </button>
                <button
                  onClick={handleGenerateNfo}
                  disabled={nfoLoading}
                  className={`text-xs px-2.5 py-1 rounded transition-colors flex items-center gap-1.5 ${
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
                  {nfoLoading ? t("collections.analyzingMedia") : t("collections.nfo")}
                </button>

                {/* Séparateur + Upload (style accentué) */}
                {torrentFilePath && c411Enabled && (
                  <>
                    <span className="w-px h-4 bg-edge mx-0.5" />
                    <button
                      onClick={() => setShowUpload(true)}
                      className="text-xs px-2.5 py-1 rounded transition-colors bg-blue-600 hover:bg-blue-700 text-white font-medium flex items-center gap-1.5"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" className="w-3.5 h-3.5">
                        <path d="M9.25 13.25a.75.75 0 0 0 1.5 0V4.636l2.955 3.129a.75.75 0 0 0 1.09-1.03l-4.25-4.5a.75.75 0 0 0-1.09 0l-4.25 4.5a.75.75 0 1 0 1.09 1.03L9.25 4.636v8.614Z" />
                        <path d="M3.5 12.75a.75.75 0 0 0-1.5 0v2.5A2.75 2.75 0 0 0 4.75 18h10.5A2.75 2.75 0 0 0 18 15.25v-2.5a.75.75 0 0 0-1.5 0v2.5c0 .69-.56 1.25-1.25 1.25H4.75c-.69 0-1.25-.56-1.25-1.25v-2.5Z" />
                      </svg>
                      {t("upload.button")}
                    </button>
                  </>
                )}
              </div>
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
      {showUpload && torrentFilePath && (
        <UploadDialog
          torrentPath={torrentFilePath}
          nfoContent={nfoContent ?? nfoText ?? null}
          bbcode={bbcode}
          meta={meta}
          torrentInfo={torrentInfo ?? null}
          onClose={() => setShowUpload(false)}
        />
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
