import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Collection, SavedPresentation, PresentationMeta, ContentType } from "../types/api";

interface Props {
  onClose: () => void;
  onLoad: (bbcode: string, html: string, meta: PresentationMeta) => void;
}

const TYPE_LABELS: Record<string, string> = {
  film: "Film",
  serie: "Série",
  jeu: "Jeu",
  app: "Application",
};

export default function CollectionBrowser({ onClose, onLoad }: Props) {
  const [collections, setCollections] = useState<Collection[]>([]);
  const [selectedCol, setSelectedCol] = useState<string | null>(null);
  const [entries, setEntries] = useState<SavedPresentation[]>([]);
  const [loading, setLoading] = useState(true);
  const [renamingId, setRenamingId] = useState<string | null>(null);
  const [renameValue, setRenameValue] = useState("");
  const [creatingNew, setCreatingNew] = useState(false);
  const [newName, setNewName] = useState("");

  const refreshCollections = useCallback(async () => {
    setLoading(true);
    try {
      const list = await invoke<Collection[]>("list_collections");
      setCollections(list);
      if (list.length > 0 && !selectedCol) {
        setSelectedCol(list[0].id);
      } else if (list.length === 0) {
        setSelectedCol(null);
        setEntries([]);
      } else if (selectedCol && !list.find((c) => c.id === selectedCol)) {
        setSelectedCol(list[0]?.id ?? null);
      }
    } catch (e) {
      console.error(e);
    }
    setLoading(false);
  }, [selectedCol]);

  const refreshEntries = useCallback(async (colId: string) => {
    try {
      const list = await invoke<SavedPresentation[]>("list_collection", { collectionId: colId });
      setEntries(list);
    } catch (e) {
      console.error(e);
      setEntries([]);
    }
  }, []);

  useEffect(() => {
    refreshCollections();
  }, []);

  useEffect(() => {
    if (selectedCol) refreshEntries(selectedCol);
    else setEntries([]);
  }, [selectedCol, refreshEntries]);

  const handleCreateCollection = async () => {
    const name = newName.trim();
    if (!name) return;
    try {
      const col = await invoke<Collection>("create_collection", { name });
      setCollections((prev) => [...prev, col]);
      setSelectedCol(col.id);
      setNewName("");
      setCreatingNew(false);
    } catch (e) {
      alert(String(e));
    }
  };

  const handleRename = async (id: string) => {
    const name = renameValue.trim();
    if (!name) return;
    try {
      await invoke("rename_collection", { id, name });
      setCollections((prev) => prev.map((c) => (c.id === id ? { ...c, name } : c)));
      setRenamingId(null);
    } catch (e) {
      alert(String(e));
    }
  };

  const handleDeleteCollection = async (id: string) => {
    const col = collections.find((c) => c.id === id);
    if (!confirm(`Supprimer la collection "${col?.name}" et tout son contenu ?`)) return;
    try {
      await invoke("delete_collection", { id });
      const remaining = collections.filter((c) => c.id !== id);
      setCollections(remaining);
      if (selectedCol === id) {
        setSelectedCol(remaining[0]?.id ?? null);
      }
    } catch (e) {
      alert(String(e));
    }
  };

  const handleDeleteEntry = async (entryId: string) => {
    if (!selectedCol || !confirm("Supprimer cette présentation ?")) return;
    try {
      await invoke("delete_collection_entry", { collectionId: selectedCol, id: entryId });
      setEntries((prev) => prev.filter((e) => e.id !== entryId));
    } catch (e) {
      alert(String(e));
    }
  };

  const handleLoad = async (entry: SavedPresentation) => {
    try {
      const html = await invoke<string>("convert_bbcode", { bbcode: entry.bbcode });
      const meta: PresentationMeta = {
        title: entry.title,
        contentType: entry.content_type as ContentType,
        posterUrl: entry.poster_url,
        savedRef: { collectionId: entry.collection_id, entryId: entry.id },
      };
      onLoad(entry.bbcode, html, meta);
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
      <div
        className="bg-surface border border-edge rounded-lg w-full max-w-4xl mx-4 shadow-2xl flex flex-col"
        style={{ height: "min(650px, 85vh)" }}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-edge shrink-0">
          <h2 className="text-fg-bright text-lg font-medium">Collections</h2>
          <button
            onClick={onClose}
            className="text-fg-muted hover:text-fg-bright transition-colors text-xl leading-none"
          >
            &times;
          </button>
        </div>

        {/* Body: sidebar + content */}
        <div className="flex flex-1 min-h-0">
          {/* Sidebar */}
          <div className="w-56 shrink-0 border-r border-edge flex flex-col">
            <div className="flex-1 overflow-y-auto p-2 space-y-0.5">
              {loading ? (
                <p className="text-fg-dim text-sm px-2 py-4 text-center">Chargement...</p>
              ) : collections.length === 0 ? (
                <p className="text-fg-dim text-sm px-2 py-4 text-center">Aucune collection</p>
              ) : (
                collections.map((col) => (
                  <div
                    key={col.id}
                    className={`group flex items-center gap-1 rounded transition-colors ${
                      selectedCol === col.id ? "bg-blue-600/20" : "hover:bg-input/50"
                    }`}
                  >
                    {renamingId === col.id ? (
                      <input
                        type="text"
                        value={renameValue}
                        onChange={(e) => setRenameValue(e.target.value)}
                        onKeyDown={(e) => {
                          if (e.key === "Enter") handleRename(col.id);
                          if (e.key === "Escape") setRenamingId(null);
                        }}
                        onBlur={() => handleRename(col.id)}
                        className="flex-1 bg-input border border-edge rounded px-2 py-1 text-sm text-fg focus:outline-none focus:border-blue-500 m-0.5"
                        autoFocus
                      />
                    ) : (
                      <>
                        <button
                          onClick={() => setSelectedCol(col.id)}
                          className={`flex-1 text-left px-3 py-2 text-sm truncate ${
                            selectedCol === col.id ? "text-fg-bright font-medium" : "text-fg"
                          }`}
                        >
                          {col.name}
                        </button>
                        <div className="flex items-center opacity-0 group-hover:opacity-100 transition-opacity pr-1">
                          <button
                            onClick={() => {
                              setRenamingId(col.id);
                              setRenameValue(col.name);
                            }}
                            className="p-1 text-fg-faint hover:text-fg-muted transition-colors"
                            title="Renommer"
                          >
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="w-3.5 h-3.5">
                              <path d="M17 3a2.85 2.85 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" />
                            </svg>
                          </button>
                          <button
                            onClick={() => handleDeleteCollection(col.id)}
                            className="p-1 text-fg-faint hover:text-red-400 transition-colors"
                            title="Supprimer"
                          >
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="w-3.5 h-3.5">
                              <path d="M3 6h18" /><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
                            </svg>
                          </button>
                        </div>
                      </>
                    )}
                  </div>
                ))
              )}
            </div>

            {/* New collection */}
            <div className="border-t border-edge p-2">
              {creatingNew ? (
                <div className="flex gap-1">
                  <input
                    type="text"
                    value={newName}
                    onChange={(e) => setNewName(e.target.value)}
                    onKeyDown={(e) => {
                      if (e.key === "Enter") handleCreateCollection();
                      if (e.key === "Escape") { setCreatingNew(false); setNewName(""); }
                    }}
                    placeholder="Nom..."
                    className="flex-1 bg-input border border-edge rounded px-2 py-1 text-sm text-fg placeholder:text-fg-faint focus:outline-none focus:border-blue-500 min-w-0"
                    autoFocus
                  />
                  <button
                    onClick={handleCreateCollection}
                    disabled={!newName.trim()}
                    className="bg-blue-600 hover:bg-blue-700 text-white text-xs px-2 py-1 rounded transition-colors disabled:opacity-50"
                  >
                    OK
                  </button>
                </div>
              ) : (
                <button
                  onClick={() => setCreatingNew(true)}
                  className="w-full text-sm text-blue-400 hover:text-blue-300 transition-colors py-1"
                >
                  + Nouvelle collection
                </button>
              )}
            </div>
          </div>

          {/* Entries */}
          <div className="flex-1 overflow-y-auto min-h-0">
            {!selectedCol ? (
              <div className="flex items-center justify-center h-full text-fg-dim text-sm">
                Sélectionnez ou créez une collection
              </div>
            ) : entries.length === 0 ? (
              <div className="flex flex-col items-center justify-center h-full text-fg-dim">
                <p>Aucune présentation</p>
                <p className="text-sm mt-1 text-fg-faint">
                  Générez une présentation puis cliquez sur "Sauvegarder"
                </p>
              </div>
            ) : (
              <div className="divide-y divide-edge">
                {entries.map((entry) => (
                  <div
                    key={entry.id}
                    className="flex items-center gap-3 px-5 py-3 hover:bg-input/50 transition-colors group"
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
                        onClick={() => navigator.clipboard.writeText(entry.bbcode)}
                        className="bg-input hover:bg-input-hover border border-edge text-fg text-xs px-2 py-1 rounded transition-colors"
                        title="Copier le BBCode"
                      >
                        Copier
                      </button>
                      <button
                        onClick={() => handleDeleteEntry(entry.id)}
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
    </div>
  );
}
