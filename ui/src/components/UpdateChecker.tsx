import { useState, useEffect, useCallback } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { check, Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

interface ReleaseNotes {
  title: string;
  body: string;
}

type UpdateState =
  | { step: "checking" }
  | { step: "available"; version: string; update: Update; notes: ReleaseNotes | null; loadingNotes: boolean }
  | { step: "downloading"; downloaded: number; total: number }
  | { step: "installing" }
  | { step: "error"; message: string }
  | { step: "dismissed" };

export default function UpdateChecker() {
  const { t } = useTranslation();
  const [state, setState] = useState<UpdateState>({ step: "checking" });

  useEffect(() => {
    let cancelled = false;

    check()
      .then(async (update) => {
        if (cancelled) return;
        if (update) {
          setState({
            step: "available",
            version: update.version,
            update,
            notes: null,
            loadingNotes: true,
          });
          // Fetch release notes in background
          try {
            const notes = await invoke<ReleaseNotes>("fetch_release_notes", {
              version: update.version,
            });
            if (!cancelled) {
              setState((prev) =>
                prev.step === "available"
                  ? { ...prev, notes, loadingNotes: false }
                  : prev
              );
            }
          } catch {
            if (!cancelled) {
              setState((prev) =>
                prev.step === "available"
                  ? { ...prev, loadingNotes: false }
                  : prev
              );
            }
          }
        } else {
          setState({ step: "dismissed" });
        }
      })
      .catch(() => {
        if (!cancelled) setState({ step: "dismissed" });
      });

    return () => {
      cancelled = true;
    };
  }, []);

  const startUpdate = useCallback(async (update: Update) => {
    setState({ step: "downloading", downloaded: 0, total: 0 });
    try {
      await update.downloadAndInstall((event) => {
        if (event.event === "Started" && event.data.contentLength) {
          setState((prev) =>
            prev.step === "downloading"
              ? { ...prev, total: event.data.contentLength! }
              : prev
          );
        } else if (event.event === "Progress") {
          setState((prev) =>
            prev.step === "downloading"
              ? { ...prev, downloaded: prev.downloaded + event.data.chunkLength }
              : prev
          );
        } else if (event.event === "Finished") {
          setState({ step: "installing" });
        }
      });
      await relaunch();
    } catch (e) {
      setState({
        step: "error",
        message: e instanceof Error ? e.message : String(e),
      });
    }
  }, []);

  if (state.step === "checking" || state.step === "dismissed") return null;

  // Extract summary from release title (format: "v1.31.0 — Description")
  const extractFeatures = (notes: ReleaseNotes | null, version: string): string[] => {
    if (!notes) return [];
    // First try the body
    if (notes.body) {
      return notes.body
        .split("\n")
        .map((l) => l.replace(/^[-*•]\s*/, "").trim())
        .filter((l) => l.length > 0);
    }
    // Fallback: extract from title after the dash
    const titleParts = notes.title.split("—");
    if (titleParts.length > 1) {
      return titleParts[1]
        .split(",")
        .map((s) => s.trim())
        .filter((s) => s.length > 0);
    }
    return [`PrezMaker v${version}`];
  };

  return (
    <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-[60]">
      <div className="bg-surface border border-edge rounded-lg w-full max-w-md mx-4 shadow-2xl">
        {state.step === "available" && (
          <>
            <div className="px-6 py-4 border-b border-edge">
              <h2 className="text-fg-bright text-lg font-medium">
                {t("update.title")}
              </h2>
              <p className="text-fg-muted text-sm mt-1">
                {t("update.versionAvailable", { version: state.version })}
              </p>
            </div>
            <div className="px-6 py-4">
              {state.loadingNotes ? (
                <div className="flex items-center gap-2 text-fg-muted text-sm py-2">
                  <svg className="animate-spin h-4 w-4" viewBox="0 0 24 24">
                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                  </svg>
                  {t("update.loadingNotes")}
                </div>
              ) : (
                <div className="space-y-2">
                  <h3 className="text-sm font-semibold text-fg">
                    {t("update.whatsNew")}
                  </h3>
                  <ul className="space-y-1.5 max-h-48 overflow-y-auto">
                    {extractFeatures(state.notes, state.version).map((feature, i) => (
                      <li key={i} className="flex items-start gap-2 text-sm text-fg-muted">
                        <span className="text-purple-400 mt-0.5 shrink-0">•</span>
                        <span>{feature}</span>
                      </li>
                    ))}
                  </ul>
                </div>
              )}
            </div>
            <div className="flex justify-end gap-3 px-6 py-4 border-t border-edge">
              <button
                onClick={() => setState({ step: "dismissed" })}
                className="bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded text-sm transition-colors"
              >
                {t("update.later")}
              </button>
              <button
                onClick={() => startUpdate(state.update)}
                className="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2 rounded text-sm transition-colors"
              >
                {t("update.update")}
              </button>
            </div>
          </>
        )}

        {state.step === "downloading" && (
          <div className="px-6 py-6 space-y-4">
            <p className="text-fg-bright font-medium">
              {t("update.downloading")}
            </p>
            <div className="w-full bg-edge rounded-full h-3">
              <div
                className="bg-purple-600 h-3 rounded-full transition-all duration-300"
                style={{
                  width:
                    state.total > 0
                      ? `${Math.min((state.downloaded / state.total) * 100, 100)}%`
                      : "0%",
                }}
              />
            </div>
            <p className="text-fg-muted text-sm text-center">
              {state.total > 0
                ? `${(state.downloaded / 1024 / 1024).toFixed(1)} / ${(state.total / 1024 / 1024).toFixed(1)} Mo`
                : `${(state.downloaded / 1024 / 1024).toFixed(1)} Mo`}
            </p>
          </div>
        )}

        {state.step === "installing" && (
          <div className="px-6 py-6 text-center space-y-3">
            <svg className="animate-spin h-8 w-8 text-purple-500 mx-auto" viewBox="0 0 24 24">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
            </svg>
            <p className="text-fg-bright font-medium">
              {t("update.installing")}
            </p>
          </div>
        )}

        {state.step === "error" && (
          <div className="px-6 py-6 space-y-4">
            <p className="text-red-400">
              {t("update.error")} {state.message}
            </p>
            <div className="flex justify-end">
              <button
                onClick={() => setState({ step: "dismissed" })}
                className="bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded text-sm transition-colors"
              >
                {t("common.close")}
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
