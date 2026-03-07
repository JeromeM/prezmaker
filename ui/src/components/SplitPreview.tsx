import { useState, useEffect, useRef } from "react";
import BBCodePanel from "./BBCodePanel";
import HtmlPreview from "./HtmlPreview";

interface Props {
  bbcode: string;
  html: string;
  onConvert: (bbcode: string) => Promise<string>;
}

export default function SplitPreview({ bbcode: initialBBCode, html: initialHtml, onConvert }: Props) {
  const [bbcode, setBBCode] = useState(initialBBCode);
  const [html, setHtml] = useState(initialHtml);
  const debounceRef = useRef<ReturnType<typeof setTimeout>>(null);

  const handleChange = (newBBCode: string) => {
    setBBCode(newBBCode);

    if (debounceRef.current) {
      clearTimeout(debounceRef.current);
    }

    debounceRef.current = setTimeout(async () => {
      const newHtml = await onConvert(newBBCode);
      setHtml(newHtml);
    }, 300);
  };

  useEffect(() => {
    return () => {
      if (debounceRef.current) clearTimeout(debounceRef.current);
    };
  }, []);

  // Raccourci Ctrl+C pour copier
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

  return (
    <div className="flex-1 flex min-h-0">
      <div className="w-1/2 flex flex-col border-r border-[#2a2a4a]">
        <BBCodePanel bbcode={bbcode} onChange={handleChange} />
      </div>
      <div className="w-1/2 flex flex-col">
        <HtmlPreview html={html} />
      </div>
    </div>
  );
}
