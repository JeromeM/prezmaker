import { useTranslation } from "react-i18next";
import TorrentImport from "./TorrentImport";

interface Props {
  onReset: () => void;
  onOpenSettings: () => void;
  onImportTorrent: (filePath: string) => void;
  onOpenTemplateEditor: () => void;
  onOpenCollections: () => void;
  onOpenAbout: () => void;
  showHome?: boolean;
}

export default function TopBar({
  onReset,
  onOpenSettings,
  onImportTorrent,
  onOpenTemplateEditor,
  onOpenCollections,
  onOpenAbout,
  showHome,
}: Props) {
  const { t } = useTranslation();

  return (
    <header className="bg-surface border-b border-edge px-4 py-2">
      <div className="flex items-center gap-1">
        {showHome && (
          <button
            type="button"
            onClick={onReset}
            className="flex items-center gap-1.5 text-fg-muted hover:text-fg-bright transition-colors px-2.5 py-1.5 rounded hover:bg-input text-sm"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="w-4 h-4">
              <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
              <polyline points="9 22 9 12 15 12 15 22" />
            </svg>
            {t("topBar.home")}
          </button>
        )}

        <div className="flex items-center gap-1 ml-auto">
          <TorrentImport onImport={onImportTorrent} disabled={false} />

          <button
            type="button"
            onClick={onOpenTemplateEditor}
            className="flex items-center gap-1.5 text-fg-muted hover:text-fg-bright transition-colors px-2.5 py-1.5 rounded hover:bg-input text-sm"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="w-4 h-4">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
              <polyline points="14 2 14 8 20 8" />
              <line x1="16" y1="13" x2="8" y2="13" />
              <line x1="16" y1="17" x2="8" y2="17" />
            </svg>
            {t("topBar.templateEditor")}
          </button>

          <button
            type="button"
            onClick={onOpenCollections}
            className="flex items-center gap-1.5 text-fg-muted hover:text-fg-bright transition-colors px-2.5 py-1.5 rounded hover:bg-input text-sm"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="w-4 h-4">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
            </svg>
            {t("topBar.collections")}
          </button>

          <button
            type="button"
            onClick={onOpenSettings}
            className="flex items-center gap-1.5 text-fg-muted hover:text-fg-bright transition-colors px-2.5 py-1.5 rounded hover:bg-input text-sm"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="w-4 h-4">
              <circle cx="12" cy="12" r="3" />
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
            </svg>
            {t("topBar.settings")}
          </button>

          <button
            type="button"
            onClick={onOpenAbout}
            className="flex items-center gap-1.5 text-fg-muted hover:text-fg-bright transition-colors px-2.5 py-1.5 rounded hover:bg-input text-sm"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="w-4 h-4">
              <circle cx="12" cy="12" r="10" />
              <line x1="12" y1="16" x2="12" y2="12" />
              <line x1="12" y1="8" x2="12.01" y2="8" />
            </svg>
            {t("topBar.about")}
          </button>
        </div>
      </div>
    </header>
  );
}
