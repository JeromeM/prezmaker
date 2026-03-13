import type { SearchResult, ContentType } from "../types/api";

interface Props {
  results: SearchResult[];
  contentType: ContentType;
  onSelect: (id: number, contentType: ContentType, source?: string, label?: string) => void;
  onCancel: () => void;
}

export default function ResultSelector({ results, contentType, onSelect, onCancel }: Props) {
  return (
    <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50">
      <div className="bg-surface border border-edge rounded-lg w-full max-w-md mx-4 shadow-xl">
        <div className="px-4 py-3 border-b border-edge flex items-center justify-between">
          <h2 className="text-lg font-semibold">Sélectionnez un résultat</h2>
          <button
            onClick={onCancel}
            className="text-fg-muted hover:text-fg-bright text-xl leading-none"
          >
            &times;
          </button>
        </div>
        <ul className="max-h-96 overflow-y-auto">
          {results.map((r) => (
            <li key={`${r.source ?? "default"}-${r.id}`}>
              <button
                onClick={() => onSelect(r.id, contentType, r.source, r.label)}
                className="w-full text-left px-4 py-2.5 hover:bg-input transition-colors border-b border-edge/50 last:border-b-0 flex items-center gap-3"
              >
                {r.thumbnail ? (
                  <img
                    src={r.thumbnail}
                    alt=""
                    className="w-8 h-12 object-cover rounded shrink-0"
                  />
                ) : (
                  <div className="w-8 h-12 bg-input rounded shrink-0 flex items-center justify-center text-fg-faint text-xs">
                    ?
                  </div>
                )}
                <div className="flex-1 min-w-0">
                  <span className="block truncate">{r.label}</span>
                  {r.source === "steam" && (
                    <span className="text-xs text-blue-400 bg-blue-400/10 px-1.5 py-0.5 rounded">Steam</span>
                  )}
                </div>
              </button>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
