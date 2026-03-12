import { useEffect } from "react";
import { openUrl } from "@tauri-apps/plugin-opener";

interface Props {
  html: string;
}

export default function HtmlPreview({ html }: Props) {
  useEffect(() => {
    const handler = (e: MessageEvent) => {
      if (e.data?.type === "open-url" && typeof e.data.url === "string") {
        openUrl(e.data.url);
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, []);

  return (
    <div className="flex flex-col h-full">
      <div className="px-3 py-2 bg-[#16213e] border-b border-[#2a2a4a]">
        <span className="text-sm font-medium text-gray-300">Aperçu HTML</span>
      </div>
      <div className="flex-1 bg-[#1a1a2e]">
        <iframe
          srcDoc={html}
          className="w-full h-full border-none"
          sandbox="allow-same-origin allow-scripts"
          title="Aperçu BBCode"
        />
      </div>
    </div>
  );
}
