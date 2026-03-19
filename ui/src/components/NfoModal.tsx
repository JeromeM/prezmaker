import { useState, useRef, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";

interface Props {
  content: string;
  title: string;
  onClose: () => void;
  onUpdate?: (content: string) => void;
}

export default function NfoModal({ content, title, onClose, onUpdate }: Props) {
  const { t } = useTranslation();
  const [copied, setCopied] = useState(false);
  const [editing, setEditing] = useState(false);
  const [editContent, setEditContent] = useState(content);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (editing && textareaRef.current) {
      textareaRef.current.focus();
    }
  }, [editing]);

  const currentContent = editing ? editContent : content;

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(currentContent);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      // fallback
    }
  };

  const handleDownload = async () => {
    const path = await save({
      defaultPath: `${title.replace(/[/\\:*?"<>|]/g, "_")}.nfo`,
      filters: [{ name: "NFO", extensions: ["nfo", "txt"] }],
    });
    if (!path) return;
    try {
      await invoke("save_text_file", { path, content: currentContent });
    } catch (e) {
      alert("Erreur: " + e);
    }
  };

  const handleToggleEdit = () => {
    if (editing) {
      // Save edits
      if (onUpdate && editContent !== content) {
        onUpdate(editContent);
      }
      setEditing(false);
    } else {
      setEditContent(content);
      setEditing(true);
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/70"
      onMouseDown={(e) => e.target === e.currentTarget && onClose()}
    >
      <div
        className="bg-[#16213e] border border-[#2a2a4a] rounded-lg shadow-2xl w-[700px] min-h-[60vh] max-h-[80vh] flex flex-col"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between px-4 py-3 border-b border-[#2a2a4a]">
          <span className="text-sm font-medium text-gray-200">{t("collections.nfo")}</span>
          <div className="flex items-center gap-2">
            <button
              onClick={handleToggleEdit}
              className={`text-xs px-3 py-1 rounded transition-colors ${
                editing
                  ? "bg-green-700 hover:bg-green-600 text-white"
                  : "bg-[#2a2a4a] hover:bg-[#3a3a5a] text-gray-300"
              }`}
            >
              {editing ? t("common.save") : t("common.edit")}
            </button>
            <button
              onClick={handleCopy}
              className="text-xs bg-[#2a2a4a] hover:bg-[#3a3a5a] text-gray-300 px-3 py-1 rounded transition-colors"
            >
              {copied ? t("common.copied") : t("common.copy")}
            </button>
            <button
              onClick={handleDownload}
              className="text-xs bg-[#2a2a4a] hover:bg-[#3a3a5a] text-gray-300 px-3 py-1 rounded transition-colors"
            >
              {t("common.download")}
            </button>
            <button
              onClick={onClose}
              className="text-xs bg-[#2a2a4a] hover:bg-[#3a3a5a] text-gray-300 px-3 py-1 rounded transition-colors"
            >
              {t("common.close")}
            </button>
          </div>
        </div>
        {editing ? (
          <textarea
            ref={textareaRef}
            value={editContent}
            onChange={(e) => setEditContent(e.target.value)}
            className="flex-1 overflow-auto p-4 text-xs text-green-300 font-mono whitespace-pre bg-[#0a0a1a] outline-none resize-none border-none"
            spellCheck={false}
          />
        ) : (
          <pre className="flex-1 overflow-auto p-4 text-xs text-green-300 font-mono whitespace-pre bg-[#0a0a1a]">
            {content}
          </pre>
        )}
      </div>
    </div>
  );
}
