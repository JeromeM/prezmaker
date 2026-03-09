import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type {
  AppState,
  ContentType,
  DetectedContentType,
  SearchResult,
  GameDetailsResponse,
  Game,
  TechInfo,
  MediaTechInfo,
  AppPayload,
  SettingsPayload,
  TorrentInfo,
} from "../types/api";

const TRACKER = "C411";

function buildMediaTech(parsed: TorrentInfo["parsed"], sizeFormatted: string): MediaTechInfo {
  return {
    quality: parsed.quality,
    video_codec: parsed.video_codec,
    audio: parsed.audio,
    language: parsed.language,
    subtitles: null,
    size: sizeFormatted,
  };
}

function torrentContentTypeToContentType(t: DetectedContentType): ContentType | null {
  switch (t) {
    case "Film": return "film";
    case "Serie": return "serie";
    case "Jeu": return "jeu";
    default: return null;
  }
}

export function usePrezMaker() {
  const [state, setState] = useState<AppState>({ step: "idle" });
  const [titleColor, setTitleColor] = useState<string>("");

  useEffect(() => {
    invoke<SettingsPayload>("get_settings").then((s) => {
      setTitleColor(s.title_color || "");
    });
  }, []);

  // --- Helper: generate via template ---
  const generateWithTemplate = useCallback(
    async (
      contentType: ContentType,
      templateName: string,
      tmdbId?: number,
      tech?: MediaTechInfo | null,
      gamePayload?: { game: Game; description: string | null; installation: string | null; tech_info: TechInfo },
      appPayload?: AppPayload,
    ) => {
      setState({ step: "generating" });
      try {
        const bbcode = await invoke<string>("generate_from_template", {
          contentType,
          tmdbId: tmdbId ?? null,
          tracker: TRACKER,
          titleColor: titleColor || null,
          templateName,
          tech: tech ?? null,
          gamePayload: gamePayload ?? null,
          appPayload: appPayload ?? null,
        });
        const html = await invoke<string>("convert_bbcode", { bbcode });
        setState({ step: "done", bbcode, html });
      } catch (e) {
        setState({ step: "error", message: String(e) });
      }
    },
    [titleColor]
  );

  // --- Search ---
  const search = useCallback(
    async (query: string, contentType: ContentType) => {
      if (contentType === "app") {
        setState({ step: "app_form" });
        return;
      }

      setState({ step: "searching" });
      try {
        const results = await invoke<SearchResult[]>("search", {
          query,
          contentType,
          tracker: TRACKER,
          titleColor: titleColor || null,
        });

        if (results.length === 0) {
          setState({ step: "error", message: "Aucun résultat trouvé" });
          return;
        }

        if (results.length === 1 && contentType !== "jeu") {
          await selectResult(results[0].id, contentType);
          return;
        }

        setState({ step: "selecting", results, contentType });
      } catch (e) {
        setState({ step: "error", message: String(e) });
      }
    },
    [titleColor]
  );

  // --- Select result (normal flow, uses template) ---
  const selectResult = useCallback(
    async (id: number, contentType: ContentType, templateName: string = "default") => {
      if (contentType === "jeu") {
        setState({ step: "generating" });
        try {
          const response = await invoke<GameDetailsResponse>(
            "fetch_game_details",
            { igdbId: id, tracker: TRACKER, titleColor: titleColor || null }
          );
          setState({
            step: "game_extras",
            game: response.game,
            claudeDescription: response.claude_description,
          });
        } catch (e) {
          setState({ step: "error", message: String(e) });
        }
        return;
      }

      await generateWithTemplate(contentType, templateName, id);
    },
    [titleColor, generateWithTemplate]
  );

  // --- Torrent result (with tech info) ---
  const selectTorrentResult = useCallback(
    async (id: number, contentType: ContentType, torrentInfo: TorrentInfo, templateName: string = "default") => {
      if (contentType === "jeu") {
        setState({ step: "generating" });
        try {
          const response = await invoke<GameDetailsResponse>(
            "fetch_game_details",
            { igdbId: id, tracker: TRACKER, titleColor: titleColor || null }
          );
          setState({
            step: "game_extras",
            game: response.game,
            claudeDescription: response.claude_description,
            torrentInfo,
          });
        } catch (e) {
          setState({ step: "error", message: String(e) });
        }
        return;
      }

      const tech = buildMediaTech(torrentInfo.parsed, torrentInfo.size_formatted);
      await generateWithTemplate(contentType, templateName, id, tech);
    },
    [titleColor, generateWithTemplate]
  );

  // --- Torrent import ---
  const importTorrent = useCallback(
    async (filePath: string) => {
      setState({ step: "searching" });
      try {
        const info = await invoke<TorrentInfo>("parse_torrent", { path: filePath });
        const ct = torrentContentTypeToContentType(info.parsed.content_type);

        if (!ct) {
          setState({ step: "torrent_parsed", torrentInfo: info });
          return;
        }

        await searchForTorrent(info, ct);
      } catch (e) {
        setState({ step: "error", message: String(e) });
      }
    },
    [titleColor]
  );

  const searchForTorrent = useCallback(
    async (info: TorrentInfo, contentType: ContentType) => {
      setState({ step: "searching" });
      try {
        const query = info.parsed.title;
        const results = await invoke<SearchResult[]>("search", {
          query,
          contentType,
          tracker: TRACKER,
          titleColor: titleColor || null,
        });

        if (results.length === 0) {
          setState({ step: "error", message: `Aucun résultat pour "${query}"` });
          return;
        }

        if (results.length === 1 && contentType !== "jeu") {
          await selectTorrentResult(results[0].id, contentType, info);
          return;
        }

        setState({ step: "torrent_selecting", results, contentType, torrentInfo: info });
      } catch (e) {
        setState({ step: "error", message: String(e) });
      }
    },
    [titleColor, selectTorrentResult]
  );

  const confirmTorrentContentType = useCallback(
    async (contentType: ContentType, torrentInfo: TorrentInfo) => {
      await searchForTorrent(torrentInfo, contentType);
    },
    [searchForTorrent]
  );

  // --- Game generation (with template) ---
  const generateGame = useCallback(
    async (
      game: Game,
      description: string | null,
      installation: string | null,
      techInfo: TechInfo,
      templateName: string = "default",
    ) => {
      await generateWithTemplate("jeu", templateName, undefined, undefined, {
        game,
        description,
        installation,
        tech_info: techInfo,
      });
    },
    [generateWithTemplate]
  );

  // --- App generation (with template) ---
  const generateApp = useCallback(
    async (payload: AppPayload, templateName: string = "default") => {
      await generateWithTemplate("app", templateName, undefined, undefined, undefined, payload);
    },
    [generateWithTemplate]
  );

  const convertBBCode = useCallback(async (bbcode: string) => {
    try {
      return await invoke<string>("convert_bbcode", { bbcode });
    } catch {
      return "";
    }
  }, []);

  const reset = useCallback(() => {
    setState({ step: "idle" });
  }, []);

  return {
    state,
    search,
    selectResult,
    selectTorrentResult,
    importTorrent,
    confirmTorrentContentType,
    generateGame,
    generateApp,
    convertBBCode,
    reset,
  };
}
