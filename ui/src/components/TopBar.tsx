import { useState } from "react";
import type { ContentType } from "../types/api";
import TorrentImport from "./TorrentImport";

interface Props {
  onSearch: (query: string, contentType: ContentType) => void;
  loading: boolean;
  onReset: () => void;
  onOpenSettings: () => void;
  onImportTorrent: (filePath: string) => void;
  onOpenTemplateEditor: () => void;
  onOpenAbout: () => void;
  theme: "dark" | "light";
  onToggleTheme: () => void;
}

export default function TopBar({
  onSearch,
  loading,
  onReset,
  onOpenSettings,
  onImportTorrent,
  onOpenTemplateEditor,
  onOpenAbout,
  theme,
  onToggleTheme,
}: Props) {
  const [query, setQuery] = useState("");
  const [contentType, setContentType] = useState<ContentType>("film");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (contentType === "app") {
      onSearch("", "app");
    } else if (query.trim()) {
      onSearch(query.trim(), contentType);
    }
  };

  return (
    <header className="bg-surface border-b border-edge px-4 py-3">
      <form onSubmit={handleSubmit} className="flex items-center gap-3 flex-wrap">
        <select
          value={contentType}
          onChange={(e) => setContentType(e.target.value as ContentType)}
          className="bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm focus:border-blue-500 outline-none"
        >
          <option value="film">Film</option>
          <option value="serie">Série</option>
          <option value="jeu">Jeu</option>
          <option value="app">Application</option>
        </select>

        {contentType !== "app" && (
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Rechercher..."
            className="bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm flex-1 min-w-[200px] focus:border-blue-500 outline-none placeholder-fg-dim"
            disabled={loading}
          />
        )}

        <button
          type="submit"
          disabled={loading || (contentType !== "app" && !query.trim())}
          className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white px-4 py-2 rounded text-sm font-medium transition-colors"
        >
          {loading ? (
            <span className="flex items-center gap-2">
              <svg className="animate-spin h-4 w-4" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
              </svg>
              Chargement...
            </span>
          ) : contentType === "app" ? (
            "Créer"
          ) : (
            "Rechercher"
          )}
        </button>

        <button
          type="button"
          onClick={onReset}
          className="bg-gray-600 hover:bg-gray-700 text-white px-3 py-2 rounded text-sm transition-colors"
        >
          Reset
        </button>

        <div className="flex items-center gap-2 ml-auto">
          <TorrentImport onImport={onImportTorrent} disabled={loading} />

          <button
            type="button"
            onClick={onOpenTemplateEditor}
            className="text-fg-muted hover:text-fg-bright transition-colors p-2"
            title="Editeur de templates"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="w-5 h-5">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
              <polyline points="14 2 14 8 20 8" />
              <line x1="16" y1="13" x2="8" y2="13" />
              <line x1="16" y1="17" x2="8" y2="17" />
              <polyline points="10 9 9 9 8 9" />
            </svg>
          </button>

          <button
            type="button"
            onClick={onToggleTheme}
            className="text-fg-muted hover:text-fg-bright transition-colors p-2"
            title={theme === "dark" ? "Thème clair" : "Thème sombre"}
          >
            {theme === "dark" ? (
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="w-5 h-5">
                <circle cx="12" cy="12" r="5" />
                <line x1="12" y1="1" x2="12" y2="3" />
                <line x1="12" y1="21" x2="12" y2="23" />
                <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" />
                <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
                <line x1="1" y1="12" x2="3" y2="12" />
                <line x1="21" y1="12" x2="23" y2="12" />
                <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" />
                <line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />
              </svg>
            ) : (
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="w-5 h-5">
                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
              </svg>
            )}
          </button>

          <button
            type="button"
            onClick={onOpenSettings}
            className="text-fg-muted hover:text-fg-bright transition-colors p-2"
            title="Parametres"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="w-5 h-5">
              <circle cx="12" cy="12" r="3" />
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
            </svg>
          </button>

          <button
            type="button"
            onClick={onOpenAbout}
            className="text-fg-muted hover:text-fg-bright transition-colors p-2"
            title="A propos"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="w-5 h-5">
              <circle cx="12" cy="12" r="10" />
              <line x1="12" y1="16" x2="12" y2="12" />
              <line x1="12" y1="8" x2="12.01" y2="8" />
            </svg>
          </button>
        </div>
      </form>
    </header>
  );
}
