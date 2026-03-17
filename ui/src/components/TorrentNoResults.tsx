import { useState } from "react";
import { useTranslation } from "react-i18next";
import type { ContentType, TorrentInfo } from "../types/api";

interface Props {
  torrentInfo: TorrentInfo;
  contentType: ContentType;
  initialQuery: string;
  onRetry: (query: string, contentType: ContentType, torrentInfo: TorrentInfo) => void;
  onCancel: () => void;
}

export default function TorrentNoResults({ torrentInfo, contentType, initialQuery, onRetry, onCancel }: Props) {
  const { t } = useTranslation();
  const [query, setQuery] = useState(initialQuery);
  const [ct, setCt] = useState<ContentType>(contentType);

  return (
    <div className="flex-1 flex items-center justify-center p-6">
      <div className="w-full max-w-md space-y-5">
        <div className="bg-yellow-900/20 border border-yellow-700/50 rounded-lg p-4">
          <p className="text-yellow-400 font-medium mb-1">{t("torrentNoResults.title")}</p>
          <p className="text-fg-muted text-sm">{t("torrentNoResults.description")}</p>
        </div>

        {/* Torrent info */}
        <div className="bg-input rounded-lg p-3 space-y-1">
          <p className="text-xs text-fg-dim">{t("torrentPicker.torrent")}</p>
          <p className="text-sm text-fg-bright font-mono truncate">{torrentInfo.meta.name}</p>
          <p className="text-xs text-fg-dim">{t("torrentPicker.size")} {torrentInfo.size_formatted}</p>
        </div>

        {/* Search form */}
        <div className="space-y-3">
          <div>
            <label className="block text-sm text-fg-dim mb-1">{t("torrentNoResults.contentType")}</label>
            <select
              value={ct}
              onChange={(e) => setCt(e.target.value as ContentType)}
              className="w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
            >
              <option value="film">{t("common.film")}</option>
              <option value="serie">{t("common.serie")}</option>
              <option value="jeu">{t("common.jeu")}</option>
            </select>
          </div>

          <div>
            <label className="block text-sm text-fg-dim mb-1">{t("torrentNoResults.searchQuery")}</label>
            <input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && query.trim() && onRetry(query.trim(), ct, torrentInfo)}
              className="w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500"
              autoFocus
            />
          </div>
        </div>

        <div className="flex gap-3">
          <button
            onClick={() => query.trim() && onRetry(query.trim(), ct, torrentInfo)}
            disabled={!query.trim()}
            className="flex-1 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-600/40 text-white px-4 py-2.5 rounded text-sm font-medium transition-colors"
          >
            {t("common.search")}
          </button>
          <button
            onClick={onCancel}
            className="px-4 py-2.5 bg-input border border-edge rounded text-sm text-fg-bright hover:border-blue-500 transition-colors"
          >
            {t("common.cancel")}
          </button>
        </div>
      </div>
    </div>
  );
}
