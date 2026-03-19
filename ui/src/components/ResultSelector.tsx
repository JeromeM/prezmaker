import { useTranslation } from "react-i18next";
import type { SearchResult, ContentType } from "../types/api";

interface Props {
  results: SearchResult[];
  contentType: ContentType;
  onSelect: (id: number, contentType: ContentType, source?: string, label?: string) => void;
  onCancel: () => void;
}

function ContentIcon({ contentType }: { contentType: ContentType }) {
  if (contentType === "jeu") {
    return (
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" className="w-6 h-6 text-fg-faint">
        <rect x="2" y="6" width="20" height="12" rx="2" />
        <path d="M6 12h4M8 10v4M15 11h.01M18 13h.01" />
      </svg>
    );
  }
  return (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" className="w-6 h-6 text-fg-faint">
      <rect x="2" y="2" width="20" height="20" rx="2.18" ry="2.18" />
      <line x1="7" y1="2" x2="7" y2="22" />
      <line x1="17" y1="2" x2="17" y2="22" />
      <line x1="2" y1="12" x2="22" y2="12" />
    </svg>
  );
}

function sourceBadge(source: string | undefined, contentType: ContentType) {
  let label: string;
  let cls: string;
  switch (source) {
    case "steam":
      label = "Steam";
      cls = "bg-blue-500/20 text-blue-400";
      break;
    case "igdb":
      label = "IGDB";
      cls = "bg-purple-500/20 text-purple-400";
      break;
    default:
      if (contentType === "jeu") {
        label = "IGDB";
        cls = "bg-purple-500/20 text-purple-400";
      } else {
        label = "TMDB";
        cls = "bg-sky-500/20 text-sky-400";
      }
  }
  return { label, cls };
}

export default function ResultSelector({ results, contentType, onSelect, onCancel }: Props) {
  const { t } = useTranslation();

  return (
    <div className="flex-1 flex flex-col min-h-0 p-6">
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-lg font-semibold">
          {t("resultSelector.title")}
          <span className="text-fg-muted font-normal ml-2">
            ({t("resultSelector.count", { count: results.length })})
          </span>
        </h2>
        <button
          onClick={onCancel}
          className="bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded text-sm transition-colors"
        >
          {t("common.cancel")}
        </button>
      </div>

      {/* Grid — compact cards */}
      <div className="flex-1 overflow-y-auto">
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-3 max-w-5xl mx-auto">
          {results.map((r) => {
            const badge = sourceBadge(r.source, contentType);
            return (
              <button
                key={`${r.source ?? "default"}-${r.id}`}
                onClick={() => onSelect(r.id, contentType, r.source, r.label)}
                className="group relative bg-surface border border-edge rounded-lg overflow-hidden text-left transition-all hover:border-blue-500/50 hover:shadow-lg hover:shadow-blue-500/5 focus:outline-none focus:border-blue-500"
              >
                {/* Poster */}
                <div className="aspect-[2/3] bg-input relative overflow-hidden">
                  {r.thumbnail ? (
                    <img
                      src={r.thumbnail}
                      alt={r.label}
                      className="w-full h-full object-cover"
                    />
                  ) : (
                    <div className="w-full h-full flex items-center justify-center">
                      <ContentIcon contentType={contentType} />
                    </div>
                  )}
                  {/* Hover overlay */}
                  <div className="absolute inset-0 bg-black/60 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
                    <span className="bg-blue-600 text-white px-3 py-1.5 rounded text-xs font-medium">
                      {t("resultSelector.select")}
                    </span>
                  </div>
                </div>

                {/* Info */}
                <div className="px-2 py-1.5">
                  <p className="text-xs font-medium truncate" title={r.label}>
                    {r.label}
                  </p>
                  <span className={`inline-block mt-1 text-[10px] px-1.5 py-0.5 rounded ${badge.cls}`}>
                    {badge.label}
                  </span>
                </div>
              </button>
            );
          })}
        </div>
      </div>
    </div>
  );
}
