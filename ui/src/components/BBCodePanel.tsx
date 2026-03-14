import { useState } from "react";
import { useTranslation } from "react-i18next";

interface Props {
  bbcode: string;
  onChange: (bbcode: string) => void;
  textareaRef?: React.RefObject<HTMLTextAreaElement | null>;
  headerActions?: React.ReactNode;
}

export default function BBCodePanel({ bbcode, onChange, textareaRef, headerActions }: Props) {
  const { t } = useTranslation();
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(bbcode);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      // Fallback
      const ta = document.createElement("textarea");
      ta.value = bbcode;
      document.body.appendChild(ta);
      ta.select();
      document.execCommand("copy");
      document.body.removeChild(ta);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between px-3 py-2 bg-[#16213e] border-b border-[#2a2a4a]">
        <span className="text-sm font-medium text-gray-300">{t("bbcodePanel.title")}</span>
        <div className="flex items-center gap-2">
          {headerActions}
          <button
            onClick={handleCopy}
            className="text-xs bg-[#2a2a4a] hover:bg-[#3a3a5a] text-gray-300 px-3 py-1 rounded transition-colors"
          >
            {copied ? t("common.copied") : t("common.copy")}
          </button>
        </div>
      </div>
      <textarea
        ref={textareaRef}
        value={bbcode}
        onChange={(e) => onChange(e.target.value)}
        className="flex-1 bg-[#0a0a1a] text-green-300 font-mono text-xs p-3 outline-none resize-none border-none"
        spellCheck={false}
      />
    </div>
  );
}
