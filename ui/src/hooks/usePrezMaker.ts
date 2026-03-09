import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type {
  AppState,
  ContentType,
  DetectedContentType,
  TrackerType,
  SearchResult,
  GameDetailsResponse,
  Game,
  TechInfo,
  MediaTechInfo,
  AppPayload,
  SettingsPayload,
  TorrentInfo,
} from "../types/api";

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

function buildGameTech(parsed: TorrentInfo["parsed"], sizeFormatted: string): { platform: string; languages: string; size: string } {
  return {
    platform: "PC (Windows)",
    languages: parsed.language || "",
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
  const [tracker, setTracker] = useState<TrackerType>("C411");
  const [titleColor, setTitleColor] = useState<string>("");

  useEffect(() => {
    invoke<SettingsPayload>("get_settings").then((s) => {
      setTitleColor(s.title_color || "");
    });
  }, []);

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
          tracker,
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
    [tracker, titleColor]
  );

  const selectResult = useCallback(
    async (id: number, contentType: ContentType) => {
      setState({ step: "generating" });
      try {
        if (contentType === "jeu") {
          const response = await invoke<GameDetailsResponse>(
            "fetch_game_details",
            {
              igdbId: id,
              tracker,
              titleColor: titleColor || null,
            }
          );
          setState({
            step: "game_extras",
            game: response.game,
            claudeDescription: response.claude_description,
          });
          return;
        }

        const cmd =
          contentType === "film" ? "generate_film" : "generate_serie";
        const paramKey = "tmdbId";

        const bbcode = await invoke<string>(cmd, {
          [paramKey]: id,
          tracker,
          titleColor: titleColor || null,
        });
        const html = await invoke<string>("convert_bbcode", { bbcode });
        setState({ step: "done", bbcode, html });
      } catch (e) {
        setState({ step: "error", message: String(e) });
      }
    },
    [tracker, titleColor]
  );

  const selectTorrentResult = useCallback(
    async (id: number, contentType: ContentType, torrentInfo: TorrentInfo) => {
      setState({ step: "generating" });
      try {
        if (contentType === "jeu") {
          const response = await invoke<GameDetailsResponse>(
            "fetch_game_details",
            {
              igdbId: id,
              tracker,
              titleColor: titleColor || null,
            }
          );
          setState({
            step: "game_extras",
            game: response.game,
            claudeDescription: response.claude_description,
            torrentInfo,
          });
          return;
        }

        const tech = buildMediaTech(torrentInfo.parsed, torrentInfo.size_formatted);
        const cmd =
          contentType === "film" ? "generate_film_with_tech" : "generate_serie_with_tech";

        const bbcode = await invoke<string>(cmd, {
          tmdbId: id,
          tracker,
          titleColor: titleColor || null,
          tech,
        });
        const html = await invoke<string>("convert_bbcode", { bbcode });
        setState({ step: "done", bbcode, html });
      } catch (e) {
        setState({ step: "error", message: String(e) });
      }
    },
    [tracker, titleColor]
  );

  const importTorrent = useCallback(
    async (filePath: string) => {
      setState({ step: "searching" });
      try {
        const info = await invoke<TorrentInfo>("parse_torrent", { path: filePath });
        const ct = torrentContentTypeToContentType(info.parsed.content_type);

        if (!ct) {
          // Unknown type → user picks
          setState({ step: "torrent_parsed", torrentInfo: info });
          return;
        }

        // Auto-search
        await searchForTorrent(info, ct);
      } catch (e) {
        setState({ step: "error", message: String(e) });
      }
    },
    [tracker, titleColor]
  );

  const searchForTorrent = useCallback(
    async (info: TorrentInfo, contentType: ContentType) => {
      setState({ step: "searching" });
      try {
        const query = info.parsed.title;
        const results = await invoke<SearchResult[]>("search", {
          query,
          contentType,
          tracker,
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
    [tracker, titleColor, selectTorrentResult]
  );

  const confirmTorrentContentType = useCallback(
    async (contentType: ContentType, torrentInfo: TorrentInfo) => {
      await searchForTorrent(torrentInfo, contentType);
    },
    [searchForTorrent]
  );

  const generateGame = useCallback(
    async (
      game: Game,
      description: string | null,
      installation: string | null,
      techInfo: TechInfo
    ) => {
      setState({ step: "generating" });
      try {
        const bbcode = await invoke<string>("generate_jeu", {
          payload: { game, description, installation, tech_info: techInfo },
          tracker,
          titleColor: titleColor || null,
        });
        const html = await invoke<string>("convert_bbcode", { bbcode });
        setState({ step: "done", bbcode, html });
      } catch (e) {
        setState({ step: "error", message: String(e) });
      }
    },
    [tracker, titleColor]
  );

  const generateApp = useCallback(
    async (payload: AppPayload) => {
      setState({ step: "generating" });
      try {
        const bbcode = await invoke<string>("generate_app", {
          payload,
          tracker,
          titleColor: titleColor || null,
        });
        const html = await invoke<string>("convert_bbcode", { bbcode });
        setState({ step: "done", bbcode, html });
      } catch (e) {
        setState({ step: "error", message: String(e) });
      }
    },
    [tracker, titleColor]
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
  };
}
