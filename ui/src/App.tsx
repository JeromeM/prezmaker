import { useState } from "react";
import { usePrezMaker } from "./hooks/usePrezMaker";
import TopBar from "./components/TopBar";
import ResultSelector from "./components/ResultSelector";
import GameExtrasForm from "./components/GameExtrasForm";
import AppForm from "./components/AppForm";
import SplitPreview from "./components/SplitPreview";
import SettingsModal from "./components/SettingsModal";
import TemplateEditor from "./components/TemplateEditor";
import TorrentContentTypePicker from "./components/TorrentContentTypePicker";

function App() {
  const [showSettings, setShowSettings] = useState(false);
  const [showTemplateEditor, setShowTemplateEditor] = useState(false);
  const {
    state,
    tracker,
    setTracker,
    search,
    selectResult,
    selectTorrentResult,
    importTorrent,
    confirmTorrentContentType,
    generateGame,
    generateApp,
    convertBBCode,
    reset,
  } = usePrezMaker();

  const isLoading = state.step === "searching" || state.step === "generating";

  return (
    <div className="flex flex-col h-screen bg-[#0f0f23]">
      <TopBar
        tracker={tracker}
        onTrackerChange={setTracker}
        onSearch={search}
        loading={isLoading}
        onReset={reset}
        onOpenSettings={() => setShowSettings(true)}
        onImportTorrent={importTorrent}
        onOpenTemplateEditor={() => setShowTemplateEditor(true)}
      />

      <main className="flex-1 flex flex-col min-h-0">
        {state.step === "idle" && (
          <div className="flex-1 flex items-center justify-center text-gray-500">
            <div className="text-center">
              <p className="text-2xl mb-2">PrezMaker</p>
              <p className="text-sm">
                Sélectionnez un type de contenu et lancez une recherche
              </p>
            </div>
          </div>
        )}

        {state.step === "searching" && (
          <div className="flex-1 flex items-center justify-center">
            <div className="flex items-center gap-3 text-gray-400">
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
              Recherche en cours...
            </div>
          </div>
        )}

        {state.step === "generating" && (
          <div className="flex-1 flex items-center justify-center">
            <div className="flex items-center gap-3 text-gray-400">
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
              Génération du BBCode...
            </div>
          </div>
        )}

        {state.step === "selecting" && (
          <ResultSelector
            results={state.results}
            contentType={state.contentType}
            onSelect={selectResult}
            onCancel={reset}
          />
        )}

        {state.step === "torrent_selecting" && (
          <ResultSelector
            results={state.results}
            contentType={state.contentType}
            onSelect={(id, ct) => selectTorrentResult(id, ct, state.torrentInfo)}
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

        {state.step === "app_form" && (
          <AppForm onGenerate={generateApp} onCancel={reset} />
        )}

        {state.step === "done" && (
          <SplitPreview
            bbcode={state.bbcode}
            html={state.html}
            onConvert={convertBBCode}
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
                Réessayer
              </button>
            </div>
          </div>
        )}
      </main>
      {showSettings && (
        <SettingsModal onClose={() => setShowSettings(false)} />
      )}
      {showTemplateEditor && (
        <TemplateEditor onClose={() => setShowTemplateEditor(false)} />
      )}
    </div>
  );
}

export default App;
