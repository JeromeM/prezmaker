import { useState, useEffect, useCallback } from "react";
import { check, Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

type UpdateState =
  | { step: "checking" }
  | { step: "available"; version: string; body: string; update: Update }
  | { step: "downloading"; downloaded: number; total: number }
  | { step: "installing" }
  | { step: "error"; message: string }
  | { step: "dismissed" };

export default function UpdateChecker() {
  const [state, setState] = useState<UpdateState>({ step: "checking" });

  useEffect(() => {
    let cancelled = false;

    check()
      .then((update) => {
        if (cancelled) return;
        if (update) {
          setState({
            step: "available",
            version: update.version,
            body: update.body ?? "",
            update,
          });
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

  return (
    <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-[60]">
      <div className="bg-surface border border-edge rounded-lg w-full max-w-md mx-4 shadow-2xl">
        {state.step === "available" && (
          <>
            <div className="px-6 py-4 border-b border-edge">
              <h2 className="text-fg-bright text-lg font-medium">
                Mise a jour disponible
              </h2>
            </div>
            <div className="px-6 py-6 space-y-4">
              <p className="text-fg">
                La version{" "}
                <span className="text-fg-bright font-semibold">
                  {state.version}
                </span>{" "}
                est disponible.
              </p>
              {state.body && (
                <p className="text-fg-muted text-sm whitespace-pre-line max-h-40 overflow-y-auto">
                  {state.body}
                </p>
              )}
            </div>
            <div className="flex justify-end gap-3 px-6 py-4 border-t border-edge">
              <button
                onClick={() => setState({ step: "dismissed" })}
                className="bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded text-sm transition-colors"
              >
                Plus tard
              </button>
              <button
                onClick={() => startUpdate(state.update)}
                className="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2 rounded text-sm transition-colors"
              >
                Mettre a jour
              </button>
            </div>
          </>
        )}

        {state.step === "downloading" && (
          <div className="px-6 py-6 space-y-4">
            <p className="text-fg-bright font-medium">
              Telechargement en cours...
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
            <svg
              className="animate-spin h-8 w-8 text-purple-500 mx-auto"
              viewBox="0 0 24 24"
            >
              <circle
                className="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                strokeWidth="4"
                fill="none"
              />
              <path
                className="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
              />
            </svg>
            <p className="text-fg-bright font-medium">
              Installation et redemarrage...
            </p>
          </div>
        )}

        {state.step === "error" && (
          <div className="px-6 py-6 space-y-4">
            <p className="text-red-400">
              Erreur lors de la mise a jour : {state.message}
            </p>
            <div className="flex justify-end">
              <button
                onClick={() => setState({ step: "dismissed" })}
                className="bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded text-sm transition-colors"
              >
                Fermer
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
