import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { SavedPresentation } from "../types/api";

interface Props {
  onClose: () => void;
  onLoad: (bbcode: string, html: string) => void;
}

const TYPE_LABELS: Record<string, string> = {
  film: "Film",
  serie: "Série",
  jeu: "Jeu",
  app: "Application",
};

export default function CollectionBrowser({ onClose, onLoad }: Props) {
  const [entries, setEntries] = useState<SavedPresentation[]>([]);
  const [loading, setLoading] = useState(true);

  const refresh = useCallback(async () => {
    setLoading(true);
    try {
      const list = await invoke<SavedPresentation[]>("list_collection");
      setEntries(list);
    } catch (e) {
      console.error(e);
    }
    setLoading(false);
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const handleDelete = async (id: string) => {
    if (!confirm("Supprimer cette présentation ?")) return;
    try {
      await invoke("delete_collection_entry", { id });
      refresh();
    } catch (e) {
      alert(String(e));
    }
  };

  const handleLoad = async (entry: SavedPresentation) => {
    try {
      const html = await invoke<string>("convert_bbcode", { bbcode: entry.bbcode });
      onLoad(entry.bbcode, html);
    } catch (e) {
      alert(String(e));
    }
  };

  const formatDate = (iso: string) => {
    try {
      return new Date(iso).toLocaleDateString("fr-FR", {
        day: "numeric",
        month: "short",
        year: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      });
    } catch {
      return iso;
    }
  };

  return (
    <div
      className="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
      onClick={(e) => e.target === e.currentTarget && onClose()}
    >
      <div className="bg-surface border border-edge rounded-lg w-full max-w-2xl mx-4 shadow-2xl flex flex-col" style={{ height: "min(600px, 80vh)" }}>
        <div className="flex items-center justify-between px-6 py-4 border-b border-edge shrink-0">
          <h2 className="text-fg-bright text-lg font-medium">
            Collections
            {entries.length > 0 && (
              <span className="ml-2 text-sm text-fg-muted font-normal">({entries.length})</span>
            )}
          </h2>
          <button
            onClick={onClose}
            className="text-fg-muted hover:text-fg-bright transition-colors text-xl leading-none"
          >
            &times;
          </button>
        </div>

        <div className="flex-1 overflow-y-auto min-h-0">
          {loading ? (
            <div className="flex items-center justify-center py-12 text-fg-muted">
              Chargement...
            </div>
          ) : entries.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-12 text-fg-dim">
              <p>Aucune présentation sauvegardée</p>
              <p className="text-sm mt-1">
                Générez une présentation puis cliquez sur "Sauvegarder"
              </p>
            </div>
          ) : (
            <div className="divide-y divide-edge">
              {entries.map((entry) => (
                <div
                  key={entry.id}
                  className="flex items-center gap-3 px-6 py-3 hover:bg-input/50 transition-colors group"
                >
                  {entry.poster_url ? (
                    <img
                      src={entry.poster_url}
                      alt=""
                      className="w-10 h-14 object-cover rounded shrink-0"
                    />
                  ) : (
                    <div className="w-10 h-14 bg-input rounded shrink-0 flex items-center justify-center text-fg-faint text-xs">
                      ?
                    </div>
                  )}

                  <div className="flex-1 min-w-0">
                    <div className="text-sm font-medium text-fg-bright truncate">
                      {entry.title}
                    </div>
                    <div className="text-xs text-fg-dim flex items-center gap-2">
                      <span className="bg-input px-1.5 py-0.5 rounded">
                        {TYPE_LABELS[entry.content_type] ?? entry.content_type}
                      </span>
                      <span>{formatDate(entry.saved_at)}</span>
                    </div>
                  </div>

                  <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                    <button
                      onClick={() => handleLoad(entry)}
                      className="bg-blue-600 hover:bg-blue-700 text-white text-xs px-3 py-1 rounded transition-colors"
                    >
                      Charger
                    </button>
                    <button
                      onClick={() => {
                        navigator.clipboard.writeText(entry.bbcode);
                      }}
                      className="bg-input hover:bg-input-hover border border-edge text-fg text-xs px-2 py-1 rounded transition-colors"
                      title="Copier le BBCode"
                    >
                      Copier
                    </button>
                    <button
                      onClick={() => handleDelete(entry.id)}
                      className="text-red-400 hover:text-red-300 text-xs px-2 py-1 transition-colors"
                    >
                      Suppr.
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
