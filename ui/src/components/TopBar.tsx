import { useState } from "react";
import type { ContentType, TrackerType } from "../types/api";

interface Props {
  tracker: TrackerType;
  onTrackerChange: (t: TrackerType) => void;
  titleColor: string;
  onTitleColorChange: (c: string) => void;
  onSearch: (query: string, contentType: ContentType) => void;
  loading: boolean;
  onReset: () => void;
}

export default function TopBar({
  tracker,
  onTrackerChange,
  titleColor,
  onTitleColorChange,
  onSearch,
  loading,
  onReset,
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
    <header className="bg-[#1a1a2e] border-b border-[#2a2a4a] px-4 py-3">
      <form onSubmit={handleSubmit} className="flex items-center gap-3 flex-wrap">
        <select
          value={contentType}
          onChange={(e) => setContentType(e.target.value as ContentType)}
          className="bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm focus:border-blue-500 outline-none"
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
            className="bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm flex-1 min-w-[200px] focus:border-blue-500 outline-none placeholder-gray-500"
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
          <label className="text-xs text-gray-400">Couleur titre:</label>
          <input
            type="text"
            value={titleColor}
            onChange={(e) => onTitleColorChange(e.target.value)}
            placeholder="c0392b"
            className="bg-[#16213e] text-white border border-[#2a2a4a] rounded px-2 py-1 text-xs w-20 outline-none focus:border-blue-500"
          />

          <select
            value={tracker}
            onChange={(e) => onTrackerChange(e.target.value as TrackerType)}
            className="bg-[#16213e] text-white border border-[#2a2a4a] rounded px-3 py-2 text-sm focus:border-blue-500 outline-none"
          >
            <option value="C411">C411</option>
            <option value="torr.xyz">torr.xyz</option>
          </select>
        </div>
      </form>
    </header>
  );
}
