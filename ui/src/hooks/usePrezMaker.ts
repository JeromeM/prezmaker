import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type {
  AppState,
  ContentType,
  TrackerType,
  SearchResult,
  GameDetailsResponse,
  Game,
  TechInfo,
  AppPayload,
} from "../types/api";

export function usePrezMaker() {
  const [state, setState] = useState<AppState>({ step: "idle" });
  const [tracker, setTracker] = useState<TrackerType>("C411");
  const [titleColor, setTitleColor] = useState<string>("");

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
    titleColor,
    setTitleColor,
    search,
    selectResult,
    generateGame,
    generateApp,
    convertBBCode,
    reset,
  };
}
