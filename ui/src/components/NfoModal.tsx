import { useState } from "react";

interface Props {
  content: string;
  onClose: () => void;
}

export default function NfoModal({ content, onClose }: Props) {
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
          <span className="text-sm font-medium text-gray-200">NFO</span>
          <div className="flex items-center gap-2">
            <button
              onClick={handleCopy}
              className="text-xs bg-[#2a2a4a] hover:bg-[#3a3a5a] text-gray-300 px-3 py-1 rounded transition-colors"
            >
              {copied ? "Copié !" : "Copier"}
            </button>
            <button
              onClick={onClose}
              className="text-xs bg-[#2a2a4a] hover:bg-[#3a3a5a] text-gray-300 px-3 py-1 rounded transition-colors"
            >
              Fermer
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
