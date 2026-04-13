import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { UploadQueueItem, QueueCounts } from "../types/api";

export interface QueueAddPayload {
  torrentPath: string;
  nfoContent: string;
  title: string;
  description: string;
  categoryId: number;
  subcategoryId: number;
  optionsJson: string;
  uploaderNote?: string | null;
  descriptionFormat?: string | null;
  tmdbData?: string | null;
  rawgData?: string | null;
  scheduledAt?: string | null;
}

export function useUploadQueue() {
  const [items, setItems] = useState<UploadQueueItem[]>([]);
  const [counts, setCounts] = useState<QueueCounts>({
    queued: 0,
    in_progress: 0,
    completed: 0,
    failed: 0,
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    try {
      const [list, c] = await Promise.all([
        invoke<UploadQueueItem[]>("queue_list"),
        invoke<QueueCounts>("queue_count"),
      ]);
      setItems(list);
      setCounts(c);
      setError(null);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  // Charge initial + écoute les events backend pour rafraîchir automatiquement
  useEffect(() => {
    refresh();
    let unlistenProgress: UnlistenFn | null = null;
    let unlistenDone: UnlistenFn | null = null;
    (async () => {
      unlistenProgress = await listen<string>("queue-item-progress", () => refresh());
      unlistenDone = await listen<string>("queue-item-done", () => refresh());
    })();
    return () => {
      unlistenProgress?.();
      unlistenDone?.();
    };
  }, [refresh]);

  const addToQueue = useCallback(async (payload: QueueAddPayload) => {
    const item = await invoke<UploadQueueItem>("queue_add", {
      torrentPath: payload.torrentPath,
      nfoContent: payload.nfoContent,
      title: payload.title,
      description: payload.description,
      categoryId: payload.categoryId,
      subcategoryId: payload.subcategoryId,
      optionsJson: payload.optionsJson,
      uploaderNote: payload.uploaderNote ?? null,
      descriptionFormat: payload.descriptionFormat ?? null,
      tmdbData: payload.tmdbData ?? null,
      rawgData: payload.rawgData ?? null,
      scheduledAt: payload.scheduledAt ?? null,
    });
    await refresh();
    return item;
  }, [refresh]);

  const remove = useCallback(async (id: string) => {
    await invoke("queue_remove", { id });
    await refresh();
  }, [refresh]);

  const retry = useCallback(async (id: string) => {
    await invoke("queue_retry", { id });
    await refresh();
  }, [refresh]);

  const clearCompleted = useCallback(async () => {
    await invoke<number>("queue_clear_completed");
    await refresh();
  }, [refresh]);

  const setSchedule = useCallback(async (id: string, scheduledAt: string | null) => {
    await invoke("queue_set_schedule", { id, scheduledAt });
    await refresh();
  }, [refresh]);

  const processAll = useCallback(async () => {
    return await invoke<number>("queue_process_all");
  }, []);

  const processOne = useCallback(async (id: string) => {
    await invoke("queue_process_one", { id });
  }, []);

  return {
    items,
    counts,
    loading,
    error,
    refresh,
    addToQueue,
    remove,
    retry,
    clearCompleted,
    setSchedule,
    processAll,
    processOne,
  };
}
