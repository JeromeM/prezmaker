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
      <div className="px-3 py-2 bg-input border-b border-edge">
        <span className="text-sm font-medium text-fg">Aperçu HTML</span>
      </div>
      <div className="flex-1 bg-surface">
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
