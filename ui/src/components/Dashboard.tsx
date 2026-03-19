import { useState, useEffect, useCallback } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { ContentType, SavedPresentation, PresentationMeta, DashboardStats } from "../types/api";

interface Props {
  onSearch: (query: string, contentType: ContentType) => void;
  onOpenTorrentCreator: () => void;
  onImportTorrent: (filePath: string) => void;
  loadPresentation: (bbcode: string, html: string, meta?: PresentationMeta, torrentPath?: string | null, nfoText?: string | null) => void;
  loading: boolean;
  dragging: boolean;
}

function StatCard({ value, label }: { value: number; label: string }) {
  return (
    <div className="bg-surface border border-edge rounded-lg px-6 py-4 text-center">
      <p className="text-2xl font-bold text-fg-bright">{value}</p>
      <p className="text-sm text-fg-muted mt-1">{label}</p>
    </div>
  );
}

export default function Dashboard({ onSearch, onOpenTorrentCreator, onImportTorrent, loadPresentation, loading, dragging }: Props) {
  const { t } = useTranslation();
  const [query, setQuery] = useState("");
  const [contentType, setContentType] = useState<ContentType>("film");
  const [stats, setStats] = useState<DashboardStats | null>(null);
  const [recents, setRecents] = useState<SavedPresentation[]>([]);

  useEffect(() => {
    invoke<DashboardStats>("get_dashboard_stats").then(setStats).catch(() => {});
    invoke<SavedPresentation[]>("list_recent_presentations", { limit: 8 }).then(setRecents).catch(() => {});
  }, []);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (contentType === "app") {
      onSearch("", "app");
    } else if (query.trim()) {
      onSearch(query.trim(), contentType);
    }
  };

  const handleImportClick = useCallback(async () => {
    const path = await open({
      filters: [{ name: "Torrent", extensions: ["torrent"] }],
      multiple: false,
    });
    if (path) onImportTorrent(path as string);
  }, [onImportTorrent]);

  const handleLoadRecent = useCallback(async (entry: SavedPresentation) => {
    try {
      const html = await invoke<string>("convert_bbcode", { bbcode: entry.bbcode });
      const meta: PresentationMeta = {
        title: entry.title,
        contentType: entry.content_type as ContentType,
        posterUrl: entry.poster_url,
        savedRef: { collectionId: entry.collection_id, entryId: entry.id },
      };
      loadPresentation(entry.bbcode, html, meta, entry.torrent_path, entry.nfo_text);
    } catch (e) {
      console.error("Failed to load presentation:", e);
    }
  }, [loadPresentation]);

  const contentTypeIcon = (ct: string) => {
    switch (ct) {
      case "film":
      case "serie":
        return (
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" className="w-8 h-8 text-fg-faint">
            <rect x="2" y="2" width="20" height="20" rx="2.18" ry="2.18" />
            <line x1="7" y1="2" x2="7" y2="22" />
            <line x1="17" y1="2" x2="17" y2="22" />
            <line x1="2" y1="12" x2="22" y2="12" />
          </svg>
        );
      case "jeu":
        return (
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" className="w-8 h-8 text-fg-faint">
            <rect x="2" y="6" width="20" height="12" rx="2" />
            <path d="M6 12h4M8 10v4M15 11h.01M18 13h.01" />
          </svg>
        );
      default:
        return (
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" className="w-8 h-8 text-fg-faint">
            <rect x="3" y="3" width="7" height="7" />
            <rect x="14" y="3" width="7" height="7" />
            <rect x="3" y="14" width="7" height="7" />
            <rect x="14" y="14" width="7" height="7" />
          </svg>
        );
    }
  };

  return (
    <div className={`flex-1 flex flex-col items-center justify-start overflow-y-auto px-6 py-10 transition-colors ${
      dragging ? "bg-blue-600/10" : ""
    }`}>
      <div className="w-full max-w-3xl space-y-8">
        {/* Title */}
        <h1 className="text-3xl font-bold text-center text-fg-bright">PrezMaker</h1>

        {/* Search bar */}
        <form onSubmit={handleSubmit} className="flex items-center gap-2">
          <select
            value={contentType}
            onChange={(e) => setContentType(e.target.value as ContentType)}
            className="bg-input text-fg-bright border border-edge rounded-lg px-3 py-2.5 text-sm focus:border-blue-500 outline-none"
          >
            <option value="film">{t("common.film")}</option>
            <option value="serie">{t("common.serie")}</option>
            <option value="jeu">{t("common.jeu")}</option>
            <option value="app">{t("common.app")}</option>
          </select>

          {contentType !== "app" && (
            <input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder={t("dashboard.searchPlaceholder")}
              className="bg-input text-fg-bright border border-edge rounded-lg px-4 py-2.5 text-sm flex-1 min-w-0 focus:border-blue-500 outline-none placeholder-fg-dim"
              disabled={loading}
            />
          )}

          <button
            type="submit"
            disabled={loading || (contentType !== "app" && !query.trim())}
            className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white px-5 py-2.5 rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
          >
            {contentType === "app" ? t("common.create") : t("common.search")}
          </button>
        </form>

        {/* Stats */}
        {stats && (
          <div className="grid grid-cols-3 gap-4">
            <StatCard value={stats.presentation_count} label={t("dashboard.presentations")} />
            <StatCard value={stats.template_count} label={t("dashboard.templates")} />
            <StatCard value={stats.collection_count} label={t("dashboard.collections")} />
          </div>
        )}

        {/* Recent presentations */}
        {recents.length > 0 && (
          <div>
            <h2 className="text-lg font-semibold mb-3">{t("dashboard.recent")}</h2>
            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
              {recents.map((p) => (
                <button
                  key={p.id}
                  onClick={() => handleLoadRecent(p)}
                  className="group bg-surface border border-edge rounded-lg overflow-hidden text-left transition-all hover:border-blue-500/50 hover:shadow-lg hover:shadow-blue-500/5"
                >
                  <div className="aspect-[2/3] bg-input relative overflow-hidden">
                    {p.poster_url ? (
                      <img
                        src={p.poster_url}
                        alt={p.title}
                        className="w-full h-full object-cover"
                      />
                    ) : (
                      <div className="w-full h-full flex items-center justify-center">
                        {contentTypeIcon(p.content_type)}
                      </div>
                    )}
                    <div className="absolute inset-0 bg-black/60 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
                      <span className="bg-blue-600 text-white px-3 py-1.5 rounded-lg text-xs font-medium">
                        {t("common.load")}
                      </span>
                    </div>
                  </div>
                  <div className="p-2">
                    <p className="text-xs font-medium truncate" title={p.title}>
                      {p.title}
                    </p>
                  </div>
                </button>
              ))}
            </div>
          </div>
        )}

        {recents.length === 0 && stats && (
          <div className="text-center py-4">
            <p className="text-fg-muted text-sm">{t("dashboard.noRecent")}</p>
            <p className="text-fg-faint text-xs mt-1">{t("dashboard.noRecentHint")}</p>
          </div>
        )}

        {/* Torrent actions */}
        <div className="flex gap-4 justify-center">
          <button
            onClick={() => onOpenTorrentCreator()}
            className="flex items-center gap-2 px-5 py-2.5 rounded-lg border border-dashed border-border hover:border-blue-500/50 hover:bg-surface-raised/50 transition-all text-sm text-fg-dim hover:text-fg group"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" className="w-5 h-5 text-fg-faint group-hover:text-blue-400 transition-colors">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8Z" />
              <polyline points="14 2 14 8 20 8" />
              <line x1="12" y1="18" x2="12" y2="12" />
              <line x1="9" y1="15" x2="15" y2="15" />
            </svg>
            {t("app.createTorrent")}
          </button>

          <button
            onClick={handleImportClick}
            className="flex items-center gap-2 px-5 py-2.5 rounded-lg border border-dashed border-border hover:border-blue-500/50 hover:bg-surface-raised/50 transition-all text-sm text-fg-dim hover:text-fg group"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" className="w-5 h-5 text-fg-faint group-hover:text-blue-400 transition-colors">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
              <polyline points="7 10 12 15 17 10" />
              <line x1="12" y1="15" x2="12" y2="3" />
            </svg>
            {t("app.importTorrent")}
          </button>
        </div>
      </div>
    </div>
  );
}
