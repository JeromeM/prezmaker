import { useState, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import type { Collection } from "../types/api";

interface Props {
  onSave: (collectionId: string) => void;
  onClose: () => void;
}

export default function CollectionSaveDialog({ onSave, onClose }: Props) {
  const { t } = useTranslation();
  const [collections, setCollections] = useState<Collection[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [newName, setNewName] = useState("");
  const [creating, setCreating] = useState(false);

  useEffect(() => {
    invoke<Collection[]>("list_collections").then((list) => {
      setCollections(list);
      if (list.length > 0) setSelectedId(list[0].id);
    });
  }, []);

  const handleCreate = async () => {
    const name = newName.trim();
    if (!name) return;
    try {
      const col = await invoke<Collection>("create_collection", { name });
      setCollections((prev) => [...prev, col]);
      setSelectedId(col.id);
      setNewName("");
      setCreating(false);
    } catch (e) {
      alert(String(e));
    }
  };

  const handleConfirm = () => {
    if (selectedId) onSave(selectedId);
  };

  return (
    <div
      className="fixed inset-0 bg-black/60 flex items-center justify-center z-[60]"
      onMouseDown={(e) => e.target === e.currentTarget && onClose()}
    >
      <div className="bg-surface border border-edge rounded-lg w-full max-w-sm mx-4 shadow-2xl">
        <div className="flex items-center justify-between px-5 py-3 border-b border-edge">
          <h3 className="text-fg-bright font-medium">{t("collections.saveTitle")}</h3>
          <button onClick={onClose} className="text-fg-muted hover:text-fg-bright text-xl leading-none">
            &times;
          </button>
        </div>

        <div className="p-5 space-y-3">
          {collections.length === 0 && !creating ? (
            <p className="text-fg-dim text-sm">{t("collections.noCollectionCreate")}</p>
          ) : (
            <div className="space-y-1 max-h-48 overflow-y-auto">
              {collections.map((col) => (
                <label
                  key={col.id}
                  className={`flex items-center gap-2 px-3 py-2 rounded cursor-pointer transition-colors ${
                    selectedId === col.id ? "bg-blue-600/20 border border-blue-500/50" : "hover:bg-input/50 border border-transparent"
                  }`}
                >
                  <input
                    type="radio"
                    name="collection"
                    checked={selectedId === col.id}
                    onChange={() => setSelectedId(col.id)}
                    className="accent-blue-500"
                  />
                  <span className="text-sm text-fg">{col.name}</span>
                </label>
              ))}
            </div>
          )}

          {creating ? (
            <div className="flex gap-2">
              <input
                type="text"
                value={newName}
                onChange={(e) => setNewName(e.target.value)}
                onKeyDown={(e) => e.key === "Enter" && handleCreate()}
                placeholder={t("collections.collectionName")}
                className="flex-1 bg-input border border-edge rounded px-2 py-1.5 text-sm text-fg placeholder:text-fg-faint focus:outline-none focus:border-blue-500"
                autoFocus
              />
              <button
                onClick={handleCreate}
                disabled={!newName.trim()}
                className="bg-blue-600 hover:bg-blue-700 text-white text-sm px-3 py-1.5 rounded transition-colors disabled:opacity-50"
              >
                {t("common.create")}
              </button>
              <button
                onClick={() => { setCreating(false); setNewName(""); }}
                className="text-fg-muted hover:text-fg text-sm px-2"
              >
                {t("common.cancel")}
              </button>
            </div>
          ) : (
            <button
              onClick={() => setCreating(true)}
              className="text-sm text-blue-400 hover:text-blue-300 transition-colors"
            >
              {t("collections.newCollection")}
            </button>
          )}
        </div>

        <div className="flex justify-end gap-2 px-5 py-3 border-t border-edge">
          <button
            onClick={onClose}
            className="text-sm text-fg-muted hover:text-fg px-3 py-1.5 transition-colors"
          >
            {t("common.cancel")}
          </button>
          <button
            onClick={handleConfirm}
            disabled={!selectedId}
            className="bg-blue-600 hover:bg-blue-700 text-white text-sm px-4 py-1.5 rounded transition-colors disabled:opacity-50"
          >
            {t("common.save")}
          </button>
        </div>
      </div>
    </div>
  );
}
