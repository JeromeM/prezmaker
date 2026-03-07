export type ContentType = "film" | "serie" | "jeu" | "app";
export type TrackerType = "C411" | "torr.xyz";

export interface SearchResult {
  id: number;
  label: string;
}

export interface TechInfo {
  platform: string;
  languages: string;
  size: string;
}

export interface Game {
  title: string;
  release_date: string | null;
  year: number | null;
  synopsis: string | null;
  cover_url: string | null;
  screenshots: string[];
  genres: { name: string }[];
  platforms: string[];
  developers: string[];
  publishers: string[];
  ratings: { source: string; value: number; max: number }[];
  igdb_id: number | null;
  tech_info: TechInfo | null;
  installation: string | null;
}

export interface GameDetailsResponse {
  game: Game;
  claude_description: string | null;
}

export interface AppPayload {
  name: string;
  version: string | null;
  developer: string | null;
  description: string | null;
  website: string | null;
  license: string | null;
  platforms: string[];
  logo_url: string | null;
}

export type AppState =
  | { step: "idle" }
  | { step: "searching" }
  | { step: "selecting"; results: SearchResult[]; contentType: ContentType }
  | { step: "game_extras"; game: Game; claudeDescription: string | null }
  | { step: "app_form" }
  | { step: "generating" }
  | { step: "done"; bbcode: string; html: string }
  | { step: "error"; message: string };
