import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { ContentType, MediaTechInfo, MediaAnalysis } from "../types/api";

interface Props {
  contentType: ContentType;
  tmdbId: number;
  title?: string;
  tech?: MediaTechInfo | null;
  onGenerate: (
    contentType: ContentType,
    tmdbId: number,
    tech: MediaTechInfo | null,
    mediaAnalysis: MediaAnalysis | null,
    title?: string,
  ) => void;
  onCancel: () => void;
}

const inputClass =
  "w-full bg-input text-fg-bright border border-edge rounded px-3 py-2 text-sm outline-none focus:border-blue-500";

export default function MovieExtrasForm({ contentType, tmdbId, title, tech, onGenerate, onCancel }: Props) {
  const [analysis, setAnalysis] = useState<MediaAnalysis | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleOpenFile = useCallback(async () => {
    const path = await open({
      filters: [{
        name: "Video",
        extensions: ["mkv", "mp4", "m4v", "mov", "webm", "avi", "wmv", "ts", "m2ts"],
      }],
      multiple: false,
    });
    if (!path) return;
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<MediaAnalysis>("analyze_media", { path });
      setAnalysis(result);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  const handleSubmit = useCallback(() => {
    onGenerate(contentType, tmdbId, tech ?? null, analysis, title);
  }, [contentType, tmdbId, tech, analysis, title, onGenerate]);

  const typeLabel = contentType === "serie" ? "la serie" : "le film";

  return (
    <div className="max-w-2xl mx-auto p-6 overflow-y-auto">
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-semibold">{title || "Presentation"}</h2>
        <button onClick={onCancel} className="text-fg-muted hover:text-fg-bright text-sm">
          Annuler
        </button>
      </div>

      <div className="space-y-6">
        {/* File picker section */}
        <div className="bg-surface border border-edge rounded-lg p-5">
          <h3 className="text-sm font-medium text-fg mb-3">Analyse du fichier media</h3>
          <p className="text-xs text-fg-muted mb-4">
            Selectionnez le fichier video pour {typeLabel} afin d'extraire les informations MediaInfo
            (codec, pistes audio, sous-titres...) et les inclure dans la presentation.
          </p>

          <button
            onClick={handleOpenFile}
            disabled={loading}
            className={`flex items-center gap-2 px-4 py-2 rounded text-sm font-medium transition-colors ${
              loading
                ? "bg-blue-600 text-white cursor-wait"
                : analysis
                  ? "bg-green-700 hover:bg-green-800 text-white"
                  : "bg-blue-600 hover:bg-blue-700 text-white"
            }`}
          >
            {loading && (
              <span
                className="inline-block h-3.5 w-3.5 border-2 border-white/30 border-t-white rounded-full"
                style={{ animation: "spin 1s linear infinite" }}
              />
            )}
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z" />
            </svg>
            {loading ? "Analyse en cours..." : analysis ? "Changer de fichier" : "Ouvrir un fichier video"}
          </button>

          {error && (
            <p className="text-red-400 text-xs mt-2">{error}</p>
          )}
        </div>

        {/* Analysis results */}
        {analysis && (
          <div className="space-y-4">
            {/* General info */}
            <div className="bg-surface border border-edge rounded-lg p-4">
              <h4 className="text-sm font-medium text-fg-bright mb-3">Informations generales</h4>
              <div className="grid grid-cols-2 gap-x-6 gap-y-1.5 text-xs">
                <Field label="Fichier" value={analysis.file_name} />
                <Field label="Format" value={analysis.format} />
                <Field label="Taille" value={analysis.file_size} />
                {analysis.duration && <Field label="Duree" value={analysis.duration} />}
                {analysis.bitrate && <Field label="Debit" value={analysis.bitrate} />}
                {analysis.video[0] && (
                  <>
                    <Field label="Codec video" value={analysis.video[0].codec} />
                    <Field label="Resolution" value={`${analysis.video[0].width}x${analysis.video[0].height}`} />
                    {analysis.video[0].fps && <Field label="FPS" value={analysis.video[0].fps} />}
                  </>
                )}
              </div>
            </div>

            {/* Audio tracks */}
            {analysis.audio.length > 0 && (
              <div className="bg-surface border border-edge rounded-lg p-4">
                <h4 className="text-sm font-medium text-fg-bright mb-3">
                  Pistes audio ({analysis.audio.length})
                </h4>
                <table className="w-full text-xs">
                  <thead>
                    <tr className="text-fg-muted border-b border-edge">
                      <th className="text-left py-1.5 pr-3">#</th>
                      <th className="text-left py-1.5 pr-3">Codec</th>
                      <th className="text-left py-1.5 pr-3">Canaux</th>
                      <th className="text-left py-1.5">Langue</th>
                    </tr>
                  </thead>
                  <tbody>
                    {analysis.audio.map((a, i) => (
                      <tr key={i} className="border-b border-edge/50 last:border-0">
                        <td className="py-1.5 pr-3 text-fg-dim">{i + 1}</td>
                        <td className="py-1.5 pr-3 text-fg-bright">{a.codec}</td>
                        <td className="py-1.5 pr-3 text-fg">{a.channels}</td>
                        <td className="py-1.5 text-fg">{a.language || "-"}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}

            {/* Subtitle tracks */}
            {analysis.subtitles.length > 0 && (
              <div className="bg-surface border border-edge rounded-lg p-4">
                <h4 className="text-sm font-medium text-fg-bright mb-3">
                  Sous-titres ({analysis.subtitles.length})
                </h4>
                <table className="w-full text-xs">
                  <thead>
                    <tr className="text-fg-muted border-b border-edge">
                      <th className="text-left py-1.5 pr-3">#</th>
                      <th className="text-left py-1.5 pr-3">Format</th>
                      <th className="text-left py-1.5">Langue</th>
                    </tr>
                  </thead>
                  <tbody>
                    {analysis.subtitles.map((s, i) => (
                      <tr key={i} className="border-b border-edge/50 last:border-0">
                        <td className="py-1.5 pr-3 text-fg-dim">{i + 1}</td>
                        <td className="py-1.5 pr-3 text-fg-bright">{s.format}</td>
                        <td className="py-1.5 text-fg">{s.language || "-"}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </div>
        )}

        {/* Action buttons */}
        <div className="flex gap-3">
          <button
            onClick={handleSubmit}
            className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-2 rounded text-sm font-medium transition-colors"
          >
            {analysis ? "Generer le BBCode" : "Continuer sans MediaInfo"}
          </button>
          {!analysis && (
            <p className="text-xs text-fg-dim self-center">
              Vous pouvez continuer sans analyser de fichier media
            </p>
          )}
        </div>
      </div>
    </div>
  );
}

function Field({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex gap-2">
      <span className="text-fg-muted whitespace-nowrap">{label} :</span>
      <span className="text-fg-bright truncate" title={value}>{value}</span>
    </div>
  );
}
