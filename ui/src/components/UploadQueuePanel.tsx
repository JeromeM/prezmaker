import { useState } from "react";
import { useTranslation } from "react-i18next";
import { useUploadQueue } from "../hooks/useUploadQueue";
import type { UploadQueueItem } from "../types/api";

interface Props {
  onClose: () => void;
}

const STATUS_COLORS: Record<string, string> = {
  queued: "bg-blue-900/30 border-blue-700 text-blue-300",
  in_progress: "bg-yellow-900/30 border-yellow-700 text-yellow-300",
  completed: "bg-green-900/30 border-green-700 text-green-300",
  failed: "bg-red-900/30 border-red-700 text-red-300",
};

function formatDate(iso: string | null): string {
  if (!iso) return "";
  return new Date(iso).toLocaleString();
}

function formatBytes(n: number): string {
  if (n < 1024) return `${n} B`;
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`;
  return `${(n / 1024 / 1024).toFixed(1)} MB`;
}

function ScheduleEditor({
  item,
  onSet,
  onCancel,
}: {
  item: UploadQueueItem;
  onSet: (iso: string | null) => void;
  onCancel: () => void;
}) {
  const { t } = useTranslation();
  // datetime-local format: yyyy-MM-ddTHH:mm
  const initial = item.scheduled_at
    ? new Date(item.scheduled_at).toISOString().slice(0, 16)
    : "";
  const [value, setValue] = useState(initial);

  return (
    <div className="flex items-center gap-2 mt-2">
      <input
        type="datetime-local"
        value={value}
        onChange={(e) => setValue(e.target.value)}
        className="bg-input text-fg-bright border border-edge rounded px-2 py-1 text-xs outline-none focus:border-blue-500"
      />
      <button
        onClick={() => onSet(value ? new Date(value).toISOString() : null)}
        className="bg-blue-600 hover:bg-blue-700 text-white px-2 py-1 rounded text-xs"
      >
        {t("common.save")}
      </button>
      {item.scheduled_at && (
        <button
          onClick={() => onSet(null)}
          className="bg-input border border-edge text-fg-muted hover:text-fg px-2 py-1 rounded text-xs"
        >
          {t("queue.removeSchedule")}
        </button>
      )}
      <button
        onClick={onCancel}
        className="text-fg-dim hover:text-fg-bright text-xs"
      >
        &times;
      </button>
    </div>
  );
}

export default function UploadQueuePanel({ onClose }: Props) {
  const { t } = useTranslation();
  const {
    items,
    counts,
    loading,
    error,
    addToQueue: _addToQueue,
    remove,
    retry,
    clearCompleted,
    setSchedule,
    processAll,
    processOne,
  } = useUploadQueue();
  void _addToQueue; // unused here

  const [actionError, setActionError] = useState<string | null>(null);
  const [editingScheduleId, setEditingScheduleId] = useState<string | null>(null);
  const [expandedResponseId, setExpandedResponseId] = useState<string | null>(null);

  const handleProcessAll = async () => {
    setActionError(null);
    try {
      await processAll();
    } catch (e) {
      setActionError(String(e));
    }
  };

  const handleProcessOne = async (id: string) => {
    setActionError(null);
    try {
      await processOne(id);
    } catch (e) {
      setActionError(String(e));
    }
  };

  const isAnyInProgress = counts.in_progress > 0;
  const queuedCount = counts.queued;

  return (
    <div
      className="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
      onMouseDown={(e) => e.target === e.currentTarget && onClose()}
    >
      <div
        className="bg-surface border border-edge rounded-lg w-full max-w-3xl mx-4 shadow-2xl flex flex-col"
        style={{ height: "min(700px, 88vh)" }}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-edge shrink-0">
          <div className="flex items-center gap-3">
            <h2 className="text-fg-bright text-lg font-medium">{t("queue.title")}</h2>
            <div className="flex gap-1.5 text-[11px]">
              <span className="px-2 py-0.5 rounded bg-blue-900/30 border border-blue-700 text-blue-300">
                {t("queue.statusQueued")}: {counts.queued}
              </span>
              {counts.in_progress > 0 && (
                <span className="px-2 py-0.5 rounded bg-yellow-900/30 border border-yellow-700 text-yellow-300">
                  {t("queue.statusInProgress")}: {counts.in_progress}
                </span>
              )}
              {counts.completed > 0 && (
                <span className="px-2 py-0.5 rounded bg-green-900/30 border border-green-700 text-green-300">
                  {t("queue.statusCompleted")}: {counts.completed}
                </span>
              )}
              {counts.failed > 0 && (
                <span className="px-2 py-0.5 rounded bg-red-900/30 border border-red-700 text-red-300">
                  {t("queue.statusFailed")}: {counts.failed}
                </span>
              )}
            </div>
          </div>
          <button
            onClick={onClose}
            className="text-fg-muted hover:text-fg-bright transition-colors text-xl leading-none"
          >
            &times;
          </button>
        </div>

        {/* Toolbar */}
        <div className="flex items-center gap-2 px-6 py-3 border-b border-edge shrink-0">
          <button
            onClick={handleProcessAll}
            disabled={isAnyInProgress || queuedCount === 0}
            className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white px-3 py-1.5 rounded text-sm font-medium transition-colors flex items-center gap-2"
          >
            {isAnyInProgress && (
              <svg className="animate-spin h-4 w-4" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
              </svg>
            )}
            {t("queue.sendAll")}
          </button>
          {counts.completed > 0 && (
            <button
              onClick={() => clearCompleted()}
              className="bg-input border border-edge text-fg hover:text-fg-bright px-3 py-1.5 rounded text-sm transition-colors"
            >
              {t("queue.clearCompleted")}
            </button>
          )}
          <div className="flex-1" />
          <span className="text-xs text-fg-dim">
            {items.length} {t("queue.items")}
          </span>
        </div>

        {/* Body */}
        <div className="flex-1 overflow-y-auto px-6 py-4 space-y-2">
          {loading && items.length === 0 ? (
            <p className="text-fg-dim text-sm text-center py-8">{t("common.loading")}</p>
          ) : items.length === 0 ? (
            <p className="text-fg-dim text-sm text-center py-8">{t("queue.empty")}</p>
          ) : (
            items.map((item) => (
              <div
                key={item.id}
                className="border border-edge rounded p-3 flex flex-col gap-2 hover:border-edge-bright transition-colors"
              >
                <div className="flex items-start gap-2">
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 flex-wrap">
                      <span
                        className={`text-[10px] px-2 py-0.5 rounded border font-medium ${
                          STATUS_COLORS[item.status] ?? STATUS_COLORS.queued
                        }`}
                      >
                        {t(`queue.status_${item.status}`)}
                      </span>
                      <span className="text-fg-bright text-sm font-medium truncate">
                        {item.title}
                      </span>
                    </div>
                    <div className="text-xs text-fg-dim mt-1 flex flex-wrap gap-x-3">
                      <span title={item.torrent_filename}>
                        {item.torrent_filename.split(/[/\\]/).pop()} ({formatBytes(item.torrent_size)})
                      </span>
                      <span>
                        {t("queue.descriptionFormat")}: {item.description_format ?? "standard"}
                      </span>
                      {item.has_tmdb_data && <span>TMDB ✓</span>}
                      {item.has_rawg_data && <span>RAWG ✓</span>}
                    </div>
                    {item.scheduled_at && (
                      <p className="text-[11px] text-yellow-400 mt-1">
                        🕒 {t("queue.scheduledFor")}: {formatDate(item.scheduled_at)}
                      </p>
                    )}
                    {item.error_message && (
                      <p className="text-[11px] text-red-400 mt-1 break-all">
                        {item.error_message}
                      </p>
                    )}
                    {item.last_response && (
                      <button
                        onClick={() =>
                          setExpandedResponseId(expandedResponseId === item.id ? null : item.id)
                        }
                        className="text-[10px] text-fg-dim hover:text-fg mt-1"
                      >
                        {expandedResponseId === item.id ? "▼" : "▶"} {t("queue.viewResponse")}
                      </button>
                    )}
                    {expandedResponseId === item.id && item.last_response && (
                      <pre className="mt-2 p-2 bg-input border border-edge rounded text-[10px] text-fg-muted overflow-x-auto whitespace-pre-wrap break-all max-h-40">
                        {item.last_response}
                      </pre>
                    )}
                  </div>

                  <div className="flex flex-col gap-1 shrink-0">
                    {item.status === "queued" && (
                      <button
                        onClick={() => handleProcessOne(item.id)}
                        disabled={isAnyInProgress}
                        className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white px-2 py-1 rounded text-[11px] transition-colors"
                      >
                        {t("queue.sendNow")}
                      </button>
                    )}
                    {item.status === "failed" && (
                      <button
                        onClick={() => retry(item.id)}
                        className="bg-yellow-600 hover:bg-yellow-700 text-white px-2 py-1 rounded text-[11px] transition-colors"
                      >
                        {t("queue.retry")}
                      </button>
                    )}
                    {(item.status === "queued" || item.status === "failed") && (
                      <button
                        onClick={() =>
                          setEditingScheduleId(editingScheduleId === item.id ? null : item.id)
                        }
                        className="bg-input border border-edge text-fg-muted hover:text-fg px-2 py-1 rounded text-[11px] transition-colors"
                      >
                        {item.scheduled_at ? t("queue.editSchedule") : t("queue.schedule")}
                      </button>
                    )}
                    {item.status !== "in_progress" && (
                      <button
                        onClick={() => remove(item.id)}
                        className="bg-red-700 hover:bg-red-800 text-white px-2 py-1 rounded text-[11px] transition-colors"
                      >
                        {t("common.delete")}
                      </button>
                    )}
                  </div>
                </div>

                {editingScheduleId === item.id && (
                  <ScheduleEditor
                    item={item}
                    onSet={async (iso) => {
                      await setSchedule(item.id, iso);
                      setEditingScheduleId(null);
                    }}
                    onCancel={() => setEditingScheduleId(null)}
                  />
                )}
              </div>
            ))
          )}

          {(error || actionError) && (
            <div className="bg-red-900/30 border border-red-700 rounded p-3 mt-3">
              <p className="text-red-400 text-sm">{error ?? actionError}</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
