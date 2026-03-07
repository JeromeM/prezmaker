import { useState } from "react";

interface Props {
  bbcode: string;
  onChange: (bbcode: string) => void;
}

export default function BBCodePanel({ bbcode, onChange }: Props) {
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
        <span className="text-sm font-medium text-gray-300">BBCode</span>
        <button
          onClick={handleCopy}
          className="text-xs bg-[#2a2a4a] hover:bg-[#3a3a5a] text-gray-300 px-3 py-1 rounded transition-colors"
        >
          {copied ? "Copié !" : "Copier"}
        </button>
      </div>
      <textarea
        value={bbcode}
        onChange={(e) => onChange(e.target.value)}
        className="flex-1 bg-[#0a0a1a] text-green-300 font-mono text-xs p-3 outline-none resize-none border-none"
        spellCheck={false}
      />
    </div>
  );
}
