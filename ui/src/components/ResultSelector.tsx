import type { SearchResult, ContentType } from "../types/api";

interface Props {
  results: SearchResult[];
  contentType: ContentType;
  onSelect: (id: number, contentType: ContentType) => void;
  onCancel: () => void;
}

export default function ResultSelector({ results, contentType, onSelect, onCancel }: Props) {
  return (
    <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50">
      <div className="bg-[#1a1a2e] border border-[#2a2a4a] rounded-lg w-full max-w-md mx-4 shadow-xl">
        <div className="px-4 py-3 border-b border-[#2a2a4a] flex items-center justify-between">
          <h2 className="text-lg font-semibold">Sélectionnez un résultat</h2>
          <button
            onClick={onCancel}
            className="text-gray-400 hover:text-white text-xl leading-none"
          >
            &times;
          </button>
        </div>
        <ul className="max-h-80 overflow-y-auto">
          {results.map((r) => (
            <li key={r.id}>
              <button
                onClick={() => onSelect(r.id, contentType)}
                className="w-full text-left px-4 py-3 hover:bg-[#16213e] transition-colors border-b border-[#2a2a4a]/50 last:border-b-0"
              >
                {r.label}
              </button>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
