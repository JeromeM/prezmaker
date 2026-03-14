import { useState } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";

interface Props {
  content: string;
  title: string;
  onClose: () => void;
}

export default function NfoModal({ content, title, onClose }: Props) {
  const { t } = useTranslation();
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(content);
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
      await invoke("save_text_file", { path, content });
    } catch (e) {
      alert("Erreur: " + e);
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/70"
      onClick={onClose}
    >
      <div
        className="bg-[#16213e] border border-[#2a2a4a] rounded-lg shadow-2xl w-[700px] max-h-[80vh] flex flex-col"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between px-4 py-3 border-b border-[#2a2a4a]">
          <span className="text-sm font-medium text-gray-200">{t("collections.nfo")}</span>
          <div className="flex items-center gap-2">
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
        <pre className="flex-1 overflow-auto p-4 text-xs text-green-300 font-mono whitespace-pre bg-[#0a0a1a]">
          {content}
        </pre>
      </div>
    </div>
  );
}
