import { useState } from "react";
import { useTranslation } from "react-i18next";
import { open, save } from "@tauri-apps/plugin-dialog";
import type { TorrentCreateOptions } from "../types/api";

const SAVED_TRACKER_KEY = "prezmaker_default_tracker";

const PIECE_SIZES = [
  { label: "16 KiB", value: 16 * 1024 },
  { label: "32 KiB", value: 32 * 1024 },
  { label: "64 KiB", value: 64 * 1024 },
  { label: "128 KiB", value: 128 * 1024 },
  { label: "256 KiB", value: 256 * 1024 },
  { label: "512 KiB", value: 512 * 1024 },
  { label: "1 MiB", value: 1024 * 1024 },
  { label: "2 MiB", value: 2 * 1024 * 1024 },
  { label: "4 MiB", value: 4 * 1024 * 1024 },
  { label: "8 MiB", value: 8 * 1024 * 1024 },
  { label: "16 MiB", value: 16 * 1024 * 1024 },
  { label: "32 MiB", value: 32 * 1024 * 1024 },
  { label: "64 MiB", value: 64 * 1024 * 1024 },
  { label: "128 MiB", value: 128 * 1024 * 1024 },
];

interface Props {
  initialPath?: string | null;
  onCreateTorrent: (opts: TorrentCreateOptions) => void;
  onCancel: () => void;
}

export default function TorrentCreator({ initialPath, onCreateTorrent, onCancel }: Props) {
  const { t } = useTranslation();
  const savedTracker = localStorage.getItem(SAVED_TRACKER_KEY) || "";
  const [sourcePath, setSourcePath] = useState(initialPath || "");
  const [pieceSize, setPieceSize] = useState<number | null>(null);
  const [isPrivate, setIsPrivate] = useState(!!savedTracker);
  const [trackers, setTrackers] = useState<string[]>([savedTracker || ""]);
  const [comment, setComment] = useState("");
  const [rememberTracker, setRememberTracker] = useState(!!savedTracker);

  const browseFile = async () => {
    const path = await open({ multiple: false, directory: false });
    if (path) setSourcePath(path as string);
  };

  const browseFolder = async () => {
    const path = await open({ multiple: false, directory: true });
    if (path) setSourcePath(path as string);
  };

  const addTracker = () => setTrackers([...trackers, ""]);

  const updateTracker = (index: number, value: string) => {
    const updated = [...trackers];
    updated[index] = value;
    setTrackers(updated);
  };

  const removeTracker = (index: number) => {
    if (trackers.length <= 1) {
      setTrackers([""]);
    } else {
      setTrackers(trackers.filter((_, i) => i !== index));
    }
  };

  const handleCreate = async () => {
    if (!sourcePath) return;

    const sourceName = sourcePath.split(/[/\\]/).pop() || "output";
    const outputPath = await save({
      defaultPath: `${sourceName}.torrent`,
      filters: [{ name: "Torrent", extensions: ["torrent"] }],
    });
    if (!outputPath) return;

    const firstTracker = trackers[0]?.trim() || "";
    if (rememberTracker && firstTracker) {
      localStorage.setItem(SAVED_TRACKER_KEY, firstTracker);
    } else {
      localStorage.removeItem(SAVED_TRACKER_KEY);
    }

    onCreateTorrent({
      source_path: sourcePath,
      output_path: outputPath,
      piece_size: pieceSize,
      private: isPrivate,
      trackers: trackers.filter((t) => t.trim() !== ""),
      comment: comment.trim() || null,
    });
  };

  const inputClass = "w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500";

  return (
    <div className="flex-1 flex items-center justify-center p-6">
      <div className="w-full max-w-lg space-y-5">
        <h2 className="text-xl font-semibold text-fg">{t("torrentCreator.title")}</h2>

        {/* Source path */}
        <div>
          <label className="block text-sm text-fg-dim mb-1">{t("torrentCreator.sourcePath")}</label>
          <div className="flex gap-2">
            <input
              type="text"
              value={sourcePath}
              readOnly
              placeholder={t("torrentCreator.noSource")}
              className={`${inputClass} flex-1 cursor-default`}
            />
            <button onClick={browseFile} className="px-3 py-2 bg-input border border-edge rounded text-sm text-fg-bright hover:border-blue-500 transition-colors whitespace-nowrap">
              {t("torrentCreator.browseFile")}
            </button>
            <button onClick={browseFolder} className="px-3 py-2 bg-input border border-edge rounded text-sm text-fg-bright hover:border-blue-500 transition-colors whitespace-nowrap">
              {t("torrentCreator.browseFolder")}
            </button>
          </div>
        </div>

        {/* Piece size */}
        <div>
          <label className="block text-sm text-fg-dim mb-1">{t("torrentCreator.pieceSize")}</label>
          <select
            value={pieceSize ?? "auto"}
            onChange={(e) => setPieceSize(e.target.value === "auto" ? null : Number(e.target.value))}
            className={inputClass}
          >
            <option value="auto" className="bg-[var(--color-input)] text-[var(--color-fg-bright)]">{t("torrentCreator.pieceSizeAuto")}</option>
            {PIECE_SIZES.map((ps) => (
              <option key={ps.value} value={ps.value} className="bg-[var(--color-input)] text-[var(--color-fg-bright)]">{ps.label}</option>
            ))}
          </select>
        </div>

        {/* Private */}
        <label className="flex items-center gap-2 text-sm text-fg cursor-pointer">
          <input
            type="checkbox"
            checked={isPrivate}
            onChange={(e) => setIsPrivate(e.target.checked)}
            className="accent-blue-500"
          />
          {t("torrentCreator.private")}
        </label>

        {/* Trackers */}
        <div>
          <label className="block text-sm text-fg-dim mb-1">{t("torrentCreator.trackers")}</label>
          <div className="space-y-2">
            {trackers.map((tracker, i) => (
              <div key={i} className="flex gap-2">
                <input
                  type="text"
                  value={tracker}
                  onChange={(e) => updateTracker(i, e.target.value)}
                  placeholder="https://tracker.example.com/announce"
                  className={`${inputClass} flex-1`}
                />
                {trackers.length > 1 && (
                  <button
                    onClick={() => removeTracker(i)}
                    className="px-2 text-fg-faint hover:text-red-400 transition-colors"
                    title={t("common.delete")}
                  >
                    &times;
                  </button>
                )}
              </div>
            ))}
          </div>
          <div className="flex items-center justify-between mt-2">
            <button
              onClick={addTracker}
              className="text-sm text-blue-400 hover:text-blue-300 transition-colors"
            >
              {t("torrentCreator.addTracker")}
            </button>
            <label className="flex items-center gap-1.5 text-xs text-fg-dim cursor-pointer">
              <input
                type="checkbox"
                checked={rememberTracker}
                onChange={(e) => setRememberTracker(e.target.checked)}
                className="accent-blue-500"
              />
              {t("torrentCreator.rememberTracker")}
            </label>
          </div>
        </div>

        {/* Comment */}
        <div>
          <label className="block text-sm text-fg-dim mb-1">{t("torrentCreator.comment")}</label>
          <input
            type="text"
            value={comment}
            onChange={(e) => setComment(e.target.value)}
            className={inputClass}
          />
        </div>

        {/* Actions */}
        <div className="flex gap-3 pt-2">
          <button
            onClick={handleCreate}
            disabled={!sourcePath}
            className="flex-1 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-600/40 disabled:cursor-not-allowed text-white px-4 py-2.5 rounded text-sm font-medium transition-colors"
          >
            {t("torrentCreator.create")}
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
