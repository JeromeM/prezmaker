import { useState, useEffect, useCallback } from "react";
import { useTranslation } from "react-i18next";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-dialog";
import { usePrezMaker } from "./hooks/usePrezMaker";
import { useTheme } from "./hooks/useTheme";
import TopBar from "./components/TopBar";
import ResultSelector from "./components/ResultSelector";
import GameExtrasForm from "./components/GameExtrasForm";
import MovieExtrasForm from "./components/MovieExtrasForm";
import AppForm from "./components/AppForm";
import SplitPreview from "./components/SplitPreview";
import SettingsModal from "./components/SettingsModal";
import TemplateEditor from "./components/TemplateEditor";
import TemplatePicker from "./components/TemplatePicker";
import AboutModal from "./components/AboutModal";
import TorrentContentTypePicker from "./components/TorrentContentTypePicker";
import Onboarding, { isOnboardingDone } from "./components/Onboarding";
import UpdateChecker from "./components/UpdateChecker";
import CollectionBrowser from "./components/CollectionBrowser";

function App() {
  const { t } = useTranslation();
  const [showSettings, setShowSettings] = useState(false);
  const [showTemplateEditor, setShowTemplateEditor] = useState(false);
  const [showCollections, setShowCollections] = useState(false);
  const [showAbout, setShowAbout] = useState(false);
  const [onboardingDone, setOnboardingDone] = useState(isOnboardingDone);
  const {
    state,
    search,
    selectResult,
    selectTorrentResult,
    importTorrent,
    confirmTorrentContentType,
    generateMovie,
    generateGame,
    generateApp,
    confirmTemplate,
    convertBBCode,
    loadPresentation,
    reset,
  } = usePrezMaker();

  const { theme, setTheme } = useTheme();
  const [dragging, setDragging] = useState(false);

  // Global drag-drop listener for .torrent files
  useEffect(() => {
    const unlisten = getCurrentWindow().onDragDropEvent((event) => {
      if (event.payload.type === "enter") {
        const hasTorrent = event.payload.paths.some((p) =>
          p.toLowerCase().endsWith(".torrent")
        );
        if (hasTorrent) setDragging(true);
      } else if (event.payload.type === "drop") {
        setDragging(false);
        const torrent = event.payload.paths.find((p) =>
          p.toLowerCase().endsWith(".torrent")
        );
        if (torrent) importTorrent(torrent);
      } else if (event.payload.type === "leave") {
        setDragging(false);
      }
    });
    return () => { unlisten.then((fn) => fn()); };
  }, [importTorrent]);

  const handleIdleClick = useCallback(async () => {
    const path = await open({
      filters: [{ name: "Torrent", extensions: ["torrent"] }],
      multiple: false,
    });
    if (path) importTorrent(path as string);
  }, [importTorrent]);

  const isLoading = state.step === "searching" || state.step === "generating";

  if (!onboardingDone) {
    return <Onboarding onComplete={() => setOnboardingDone(true)} />;
  }

  return (
    <div className="flex flex-col h-screen bg-base">
      <UpdateChecker />
      <TopBar
        onSearch={search}
        loading={isLoading}
        onReset={reset}
        onOpenSettings={() => setShowSettings(true)}
        onImportTorrent={importTorrent}
        onOpenTemplateEditor={() => setShowTemplateEditor(true)}
        onOpenCollections={() => setShowCollections(true)}
        onOpenAbout={() => setShowAbout(true)}
      />

      <main className="flex-1 flex flex-col min-h-0">
        {state.step === "idle" && (
          <div
            onClick={handleIdleClick}
            className={`flex-1 flex items-center justify-center cursor-pointer transition-colors ${
              dragging
                ? "bg-blue-600/10 border-2 border-dashed border-blue-500"
                : ""
            }`}
          >
            <div className="text-center pointer-events-none select-none">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="1.5"
                strokeLinecap="round"
                strokeLinejoin="round"
                className={`w-12 h-12 mx-auto mb-4 transition-colors ${
                  dragging ? "text-blue-400" : "text-fg-faint"
                }`}
              >
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="7 10 12 15 17 10" />
                <line x1="12" y1="15" x2="12" y2="3" />
              </svg>
              <p className={`text-lg mb-2 transition-colors ${
                dragging ? "text-blue-400" : "text-fg-dim"
              }`}>
                {dragging
                  ? t("app.dropTorrent")
                  : t("app.dragOrClick")}
              </p>
              <p className="text-sm text-fg-faint">
                {t("app.orSearchAbove")}
              </p>
            </div>
          </div>
        )}

        {state.step === "searching" && (
          <div className="flex-1 flex items-center justify-center">
            <div className="flex items-center gap-3 text-fg-muted">
              <svg className="animate-spin h-6 w-6" viewBox="0 0 24 24">
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
              {t("app.searching")}
            </div>
          </div>
        )}

        {state.step === "generating" && (
          <div className="flex-1 flex items-center justify-center">
            <div className="flex flex-col items-center gap-2">
              <div className="flex items-center gap-3 text-fg-muted">
                <svg className="animate-spin h-6 w-6" viewBox="0 0 24 24">
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
                {t("app.generating")}
              </div>
              {state.message && (
                <p className="text-sm text-fg-faint">{state.message}</p>
              )}
            </div>
          </div>
        )}

        {state.step === "selecting" && (
          <ResultSelector
            results={state.results}
            contentType={state.contentType}
            onSelect={(id, ct, source, label) => selectResult(id, ct, "default", source, label)}
            onCancel={reset}
          />
        )}

        {state.step === "torrent_selecting" && (
          <ResultSelector
            results={state.results}
            contentType={state.contentType}
            onSelect={(id, ct, source, label) => selectTorrentResult(id, ct, state.torrentInfo, "default", source, label)}
            onCancel={reset}
          />
        )}

        {state.step === "torrent_parsed" && (
          <TorrentContentTypePicker
            torrentInfo={state.torrentInfo}
            onConfirm={confirmTorrentContentType}
            onCancel={reset}
          />
        )}

        {state.step === "game_extras" && (
          <GameExtrasForm
            game={state.game}
            claudeDescription={state.claudeDescription}
            onGenerate={generateGame}
            onCancel={reset}
            torrentInfo={state.torrentInfo}
          />
        )}

        {state.step === "movie_extras" && (
          <MovieExtrasForm
            contentType={state.contentType}
            tmdbId={state.tmdbId}
            title={state.title}
            tech={state.tech}
            onGenerate={generateMovie}
            onCancel={reset}
          />
        )}

        {state.step === "template_pick" && (
          <TemplatePicker
            contentType={state.pending.contentType}
            onSelect={(templateName) => confirmTemplate(templateName, state.pending)}
            onCancel={reset}
            onEditTemplates={() => setShowTemplateEditor(true)}
          />
        )}

        {state.step === "app_form" && (
          <AppForm onGenerate={generateApp} onCancel={reset} />
        )}

        {state.step === "done" && (
          <SplitPreview
            bbcode={state.bbcode}
            html={state.html}
            onConvert={convertBBCode}
            meta={state.meta}
            nfoText={state.nfoText}
            mediaAnalysis={state.mediaAnalysis}
          />
        )}

        {state.step === "error" && (
          <div className="flex-1 flex items-center justify-center">
            <div className="bg-red-900/30 border border-red-700 rounded-lg px-6 py-4 max-w-md text-center">
              <p className="text-red-400 mb-3">{state.message}</p>
              <button
                onClick={reset}
                className="bg-red-700 hover:bg-red-800 text-white px-4 py-2 rounded text-sm transition-colors"
              >
                {t("common.retry")}
              </button>
            </div>
          </div>
        )}
      </main>
      {dragging && state.step !== "idle" && (
        <div className="absolute inset-0 z-50 bg-base/80 flex items-center justify-center pointer-events-none">
          <div className="border-2 border-dashed border-blue-500 rounded-xl p-12 text-center">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="1.5"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="w-12 h-12 mx-auto mb-4 text-blue-400"
            >
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
              <polyline points="7 10 12 15 17 10" />
              <line x1="12" y1="15" x2="12" y2="3" />
            </svg>
            <p className="text-blue-400 text-lg">{t("app.dropTorrent")}</p>
          </div>
        </div>
      )}
      {showSettings && (
        <SettingsModal onClose={() => setShowSettings(false)} theme={theme} onSetTheme={setTheme} />
      )}
      {showTemplateEditor && (
        <TemplateEditor onClose={() => setShowTemplateEditor(false)} />
      )}
      {showCollections && (
        <CollectionBrowser
          onClose={() => setShowCollections(false)}
          onLoad={(bbcode, html, meta) => {
            setShowCollections(false);
            loadPresentation(bbcode, html, meta);
          }}
        />
      )}
      {showAbout && (
        <AboutModal onClose={() => setShowAbout(false)} />
      )}
    </div>
  );
}

export default App;
