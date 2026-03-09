import { useState, useRef, useEffect } from "react";
import type { TemplateInfo } from "../types/api";

interface Props {
  templates: TemplateInfo[];
  loading: boolean;
  onSave: (name: string) => void;
  onLoad: (name: string) => void;
  onDelete: (name: string) => void;
  onRename: (oldName: string, newName: string) => void;
  onDuplicate: (name: string, newName: string) => void;
}

export default function TemplateManager({
  templates,
  loading,
  onSave,
  onLoad,
  onDelete,
  onRename,
  onDuplicate,
}: Props) {
  const [open, setOpen] = useState(false);
  const [saveName, setSaveName] = useState("");
  const [renamingIdx, setRenamingIdx] = useState<number | null>(null);
  const [renameValue, setRenameValue] = useState("");
  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) return;
    const handler = (e: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, [open]);

  const handleSave = () => {
    const name = saveName.trim();
    if (!name) return;
    onSave(name);
    setSaveName("");
  };

  const handleRenameSubmit = (oldName: string) => {
    const newName = renameValue.trim();
    if (newName && newName !== oldName) {
      onRename(oldName, newName);
    }
    setRenamingIdx(null);
    setRenameValue("");
  };

  const handleDuplicate = (name: string) => {
    const newName = `${name} (copie)`;
    onDuplicate(name, newName);
  };

  return (
    <div className="relative" ref={dropdownRef}>
      <button
        onClick={() => setOpen(!open)}
        className="text-xs bg-[#2a2a4a] hover:bg-[#3a3a5a] text-gray-300 px-3 py-1 rounded transition-colors"
      >
        Templates
      </button>

      {open && (
        <div className="absolute right-0 top-full mt-1 w-72 bg-[#1a1a2e] border border-[#2a2a4a] rounded-lg shadow-xl z-50 overflow-hidden">
          {/* Save section */}
          <div className="flex gap-1 p-2 border-b border-[#2a2a4a]">
            <input
              type="text"
              value={saveName}
              onChange={(e) => setSaveName(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && handleSave()}
              placeholder="Nom du template..."
              className="flex-1 bg-[#0a0a1a] text-gray-200 text-xs px-2 py-1 rounded border border-[#2a2a4a] outline-none focus:border-blue-500"
            />
            <button
              onClick={handleSave}
              disabled={!saveName.trim()}
              className="text-xs bg-blue-600 hover:bg-blue-700 disabled:opacity-40 text-white px-2 py-1 rounded transition-colors"
            >
              Sauver
            </button>
          </div>

          {/* Template list */}
          <div className="max-h-64 overflow-y-auto">
            {loading && (
              <div className="text-xs text-gray-500 p-3 text-center">Chargement...</div>
            )}
            {!loading && templates.length === 0 && (
              <div className="text-xs text-gray-500 p-3 text-center">Aucun template</div>
            )}
            {templates.map((t, i) => (
              <div
                key={t.name}
                className="flex items-center gap-1 px-2 py-1.5 hover:bg-[#2a2a4a] group"
              >
                {renamingIdx === i ? (
                  <input
                    autoFocus
                    value={renameValue}
                    onChange={(e) => setRenameValue(e.target.value)}
                    onKeyDown={(e) => {
                      if (e.key === "Enter") handleRenameSubmit(t.name);
                      if (e.key === "Escape") setRenamingIdx(null);
                    }}
                    onBlur={() => handleRenameSubmit(t.name)}
                    className="flex-1 bg-[#0a0a1a] text-gray-200 text-xs px-1 py-0.5 rounded border border-blue-500 outline-none"
                  />
                ) : (
                  <button
                    onClick={() => {
                      onLoad(t.name);
                      setOpen(false);
                    }}
                    className="flex-1 text-left text-xs text-gray-300 hover:text-white truncate"
                    title={`Charger "${t.name}"`}
                  >
                    {t.name}
                  </button>
                )}
                <div className="flex gap-0.5 opacity-0 group-hover:opacity-100 transition-opacity shrink-0">
                  <button
                    onClick={() => {
                      setRenamingIdx(i);
                      setRenameValue(t.name);
                    }}
                    className="text-[10px] text-gray-500 hover:text-yellow-400 px-1"
                    title="Renommer"
                  >
                    ✎
                  </button>
                  <button
                    onClick={() => handleDuplicate(t.name)}
                    className="text-[10px] text-gray-500 hover:text-blue-400 px-1"
                    title="Dupliquer"
                  >
                    ⧉
                  </button>
                  <button
                    onClick={() => onDelete(t.name)}
                    className="text-[10px] text-gray-500 hover:text-red-400 px-1"
                    title="Supprimer"
                  >
                    ✕
                  </button>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
