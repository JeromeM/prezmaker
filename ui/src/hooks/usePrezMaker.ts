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
  PendingGeneration,
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
          titleColor: titleColor || null,
        });

        if (results.length === 0) {
          setState({ step: "error", message: "Aucun résultat trouvé" });
          return;
        }

        if (results.length === 1 && contentType !== "jeu") {
          await selectResult(results[0].id, contentType, "default", results[0].source);
          return;
        }

        setState({ step: "selecting", results, contentType });
      } catch (e) {
        setState({ step: "error", message: String(e) });
      }
    },
    [titleColor]
  );

  // --- Select result (normal flow, goes to template pick) ---
  const selectResult = useCallback(
    async (id: number, contentType: ContentType, _templateName: string = "default", source?: string) => {
      if (contentType === "jeu") {
        setState({ step: "generating" });
        try {
          const response = await invoke<GameDetailsResponse>(
            "fetch_game_details",
            { gameId: id, source: source ?? null, titleColor: titleColor || null }
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

      // Go to template pick (auto-selects if only one template)
      setState({ step: "template_pick", pending: { contentType, tmdbId: id } });
    },
    [titleColor]
  );

  // --- Torrent result (with tech info) ---
  const selectTorrentResult = useCallback(
    async (id: number, contentType: ContentType, torrentInfo: TorrentInfo, _templateName: string = "default", source?: string) => {
      if (contentType === "jeu") {
        setState({ step: "generating" });
        try {
          const response = await invoke<GameDetailsResponse>(
            "fetch_game_details",
            { gameId: id, source: source ?? null, titleColor: titleColor || null }
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
      setState({ step: "template_pick", pending: { contentType, tmdbId: id, tech } });
    },
    [titleColor]
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
          titleColor: titleColor || null,
        });

        if (results.length === 0) {
          setState({ step: "error", message: `Aucun résultat pour "${query}"` });
          return;
        }

        if (results.length === 1 && contentType !== "jeu") {
          await selectTorrentResult(results[0].id, contentType, info, "default", results[0].source);
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

  // --- Game generation (goes to template pick) ---
  const generateGame = useCallback(
    (
      game: Game,
      description: string | null,
      installation: string | null,
      techInfo: TechInfo,
    ) => {
      setState({
        step: "template_pick",
        pending: {
          contentType: "jeu",
          gamePayload: { game, description, installation, tech_info: techInfo },
        },
      });
    },
    []
  );

  // --- App generation (goes to template pick) ---
  const generateApp = useCallback(
    (payload: AppPayload) => {
      setState({
        step: "template_pick",
        pending: { contentType: "app", appPayload: payload },
      });
    },
    []
  );

  // --- Confirm template selection → actually generate ---
  const confirmTemplate = useCallback(
    async (templateName: string, pending: PendingGeneration) => {
      await generateWithTemplate(
        pending.contentType,
        templateName,
        pending.tmdbId,
        pending.tech,
        pending.gamePayload,
        pending.appPayload,
      );
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
    confirmTemplate,
    convertBBCode,
    reset,
  };
}
