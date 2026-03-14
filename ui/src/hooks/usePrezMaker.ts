import { useState, useCallback, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  AppState,
  ContentType,
  DetectedContentType,
  SearchResult,
  GameDetailsResponse,
  GenerationResult,
  Game,
  TechInfo,
  MediaTechInfo,
  MediaAnalysis,
  AppPayload,
  PendingGeneration,
  PresentationMeta,
  SettingsPayload,
  TorrentInfo,
  TorrentCreateOptions,
  TorrentCreateProgress,
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
  const [torrentFilePath, setTorrentFilePath] = useState<string | null>(null);
  const [torrentInfo, setTorrentInfo] = useState<TorrentInfo | null>(null);
  const unlistenRef = useRef<UnlistenFn | null>(null);

  const unlistenCreationRef = useRef<UnlistenFn | null>(null);

  useEffect(() => {
    invoke<SettingsPayload>("get_settings").then((s) => {
      setTitleColor(s.title_color || "");
    });
    // Listen for progress events from backend
    listen<string>("generation-progress", (event) => {
      setState((prev) => {
        if (prev.step === "generating") {
          return { step: "generating", message: event.payload };
        }
        return prev;
      });
    }).then((unlisten) => {
      unlistenRef.current = unlisten;
    });
    // Listen for torrent creation progress
    listen<TorrentCreateProgress>("torrent-creation-progress", (event) => {
      setState((prev) => {
        if (prev.step === "torrent_creating") {
          return { step: "torrent_creating", progress: event.payload };
        }
        return prev;
      });
    }).then((unlisten) => {
      unlistenCreationRef.current = unlisten;
    });
    return () => {
      unlistenRef.current?.();
      unlistenCreationRef.current?.();
    };
  }, []);

  // --- Helper: generate via template ---
  const generateWithTemplate = useCallback(
    async (
      contentType: ContentType,
      templateName: string,
      tmdbId?: number,
      tech?: MediaTechInfo | null,
      mediaAnalysis?: MediaAnalysis | null,
      gamePayload?: { game: Game; description: string | null; installation: string | null; tech_info: TechInfo },
      appPayload?: AppPayload,
      meta?: PresentationMeta,
    ) => {
      setState({ step: "generating" });
      try {
        const result = await invoke<GenerationResult>("generate_from_template", {
          contentType,
          tmdbId: tmdbId ?? null,
          titleColor: titleColor || null,
          templateName,
          tech: tech ?? null,
          mediaAnalysis: mediaAnalysis ?? null,
          gamePayload: gamePayload ?? null,
          appPayload: appPayload ?? null,
        });
        const html = await invoke<string>("convert_bbcode", { bbcode: result.bbcode });
        const presentationMeta: PresentationMeta = meta ?? {
          title: "Présentation",
          contentType,
          posterUrl: null,
        };
        setState({ step: "done", bbcode: result.bbcode, html, meta: presentationMeta, nfoText: result.nfo_text, mediaAnalysis: mediaAnalysis ?? null });
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

        if (results.length === 1) {
          await selectResult(results[0].id, contentType, "default", results[0].source, results[0].label);
          return;
        }

        setState({ step: "selecting", results, contentType });
      } catch (e) {
        setState({ step: "error", message: String(e) });
      }
    },
    [titleColor]
  );

  // --- Select result (normal flow) ---
  const selectResult = useCallback(
    async (id: number, contentType: ContentType, _templateName: string = "default", source?: string, label?: string) => {
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

      if (contentType === "film" || contentType === "serie") {
        setState({ step: "movie_extras", contentType, tmdbId: id, title: label });
        return;
      }

      setState({ step: "template_pick", pending: { contentType, tmdbId: id, title: label } });
    },
    [titleColor]
  );

  // --- Torrent result (with tech info) ---
  const selectTorrentResult = useCallback(
    async (id: number, contentType: ContentType, torrentInfo: TorrentInfo, _templateName: string = "default", source?: string, label?: string) => {
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

      if (contentType === "film" || contentType === "serie") {
        const tech = buildMediaTech(torrentInfo.parsed, torrentInfo.size_formatted);
        setState({ step: "movie_extras", contentType, tmdbId: id, title: label, tech, torrentInfo });
        return;
      }

      const tech = buildMediaTech(torrentInfo.parsed, torrentInfo.size_formatted);
      setState({ step: "template_pick", pending: { contentType, tmdbId: id, tech, title: label } });
    },
    [titleColor]
  );

  // --- Torrent import ---
  const importTorrent = useCallback(
    async (filePath: string) => {
      setState({ step: "searching" });
      try {
        const info = await invoke<TorrentInfo>("parse_torrent", { path: filePath });
        setTorrentFilePath(filePath);
        setTorrentInfo(info);
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

        if (results.length === 1) {
          await selectTorrentResult(results[0].id, contentType, info, "default", results[0].source, results[0].label);
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
          title: game.title,
          posterUrl: game.cover_url,
        },
      });
    },
    []
  );

  // --- Movie/Serie generation (goes to template pick) ---
  const generateMovie = useCallback(
    (
      contentType: ContentType,
      tmdbId: number,
      tech: MediaTechInfo | null,
      mediaAnalysis: MediaAnalysis | null,
      title?: string,
    ) => {
      setState({
        step: "template_pick",
        pending: { contentType, tmdbId, tech: tech ?? undefined, mediaAnalysis, title },
      });
    },
    []
  );

  // --- App generation (goes to template pick) ---
  const generateApp = useCallback(
    (payload: AppPayload) => {
      setState({
        step: "template_pick",
        pending: { contentType: "app", appPayload: payload, title: payload.name, posterUrl: payload.logo_url },
      });
    },
    []
  );

  // --- Confirm template selection → actually generate ---
  const confirmTemplate = useCallback(
    async (templateName: string, pending: PendingGeneration) => {
      const meta: PresentationMeta = {
        title: pending.title ?? "Présentation",
        contentType: pending.contentType,
        posterUrl: pending.posterUrl ?? null,
      };
      await generateWithTemplate(
        pending.contentType,
        templateName,
        pending.tmdbId,
        pending.tech,
        pending.mediaAnalysis,
        pending.gamePayload,
        pending.appPayload,
        meta,
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

  const openTorrentCreator = useCallback((initialPath?: string | null) => {
    setState({ step: "torrent_creator", initialPath: initialPath ?? null });
  }, []);

  const createTorrent = useCallback(
    async (opts: TorrentCreateOptions) => {
      setState({ step: "torrent_creating", progress: null });
      try {
        const info = await invoke<TorrentInfo>("create_torrent", { payload: opts });
        setTorrentFilePath(opts.output_path);
        setTorrentInfo(info);
        const ct = torrentContentTypeToContentType(info.parsed.content_type);
        if (ct) {
          await searchForTorrent(info, ct);
        } else {
          setState({ step: "torrent_parsed", torrentInfo: info });
        }
      } catch (e) {
        setState({ step: "error", message: String(e) });
      }
    },
    [titleColor, searchForTorrent]
  );

  const loadPresentation = useCallback((bbcode: string, html: string, meta?: PresentationMeta) => {
    setState({
      step: "done",
      bbcode,
      html,
      meta: meta ?? { title: "Collection", contentType: "film", posterUrl: null },
    });
  }, []);

  const reset = useCallback(() => {
    setState({ step: "idle" });
    setTorrentFilePath(null);
    setTorrentInfo(null);
  }, []);

  return {
    state,
    search,
    selectResult,
    selectTorrentResult,
    importTorrent,
    confirmTorrentContentType,
    openTorrentCreator,
    createTorrent,
    generateMovie,
    generateGame,
    generateApp,
    confirmTemplate,
    convertBBCode,
    loadPresentation,
    reset,
    torrentFilePath,
    torrentInfo,
  };
}
