import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { TemplateInfo } from "../types/api";

export function useTemplates() {
  const [templates, setTemplates] = useState<TemplateInfo[]>([]);
  const [loading, setLoading] = useState(false);

  const refresh = useCallback(async () => {
    setLoading(true);
    try {
      const list = await invoke<TemplateInfo[]>("list_templates");
      setTemplates(list);
    } catch (e) {
      console.error("Failed to list templates:", e);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const load = useCallback(async (name: string): Promise<string> => {
    return invoke<string>("load_template", { name });
  }, []);

  const save = useCallback(
    async (name: string, content: string) => {
      await invoke("save_template", { name, content });
      await refresh();
    },
    [refresh],
  );

  const remove = useCallback(
    async (name: string) => {
      await invoke("delete_template", { name });
      await refresh();
    },
    [refresh],
  );

  const rename = useCallback(
    async (oldName: string, newName: string) => {
      await invoke("rename_template", { oldName, newName });
      await refresh();
    },
    [refresh],
  );

  const duplicate = useCallback(
    async (name: string, newName: string) => {
      await invoke("duplicate_template", { name, newName });
      await refresh();
    },
    [refresh],
  );

  return { templates, loading, refresh, load, save, remove, rename, duplicate };
}
