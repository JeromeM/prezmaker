export type ContentType = "film" | "serie" | "jeu" | "app";

export interface SearchResult {
  id: number;
  label: string;
  source?: string;
}

export interface TechInfo {
  platform: string;
  languages: string;
  size: string;
  install_size: string;
}

export interface MediaTechInfo {
  quality: string | null;
  video_codec: string | null;
  audio: string | null;
  language: string | null;
  subtitles: string | null;
  size: string | null;
}

export interface SystemReqs {
  os: string;
  cpu: string;
  ram: string;
  gpu: string;
  storage: string;
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
  min_reqs: SystemReqs | null;
  rec_reqs: SystemReqs | null;
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

export interface TemplateInfo {
  name: string;
  size: number;
  modified: number;
}

export interface SettingsPayload {
  tmdb_api_key: string | null;
  igdb_client_id: string | null;
  igdb_client_secret: string | null;
  language: string;
  title_color: string;
  auto_clipboard: boolean;
  llm_provider: string | null;
  llm_api_key: string | null;
  pseudo: string;
}

// Content Templates

export interface ContentTemplate {
  name: string;
  content_type: string;
  body: string;
  is_default: boolean;
}

export interface TemplateTag {
  name: string;
  description: string;
  category: string;
  example?: string;
}

// Torrent types

export type DetectedContentType = "Film" | "Serie" | "Jeu" | "Unknown";

export interface TorrentFile {
  path: string;
  size: number;
}

export interface TorrentMeta {
  name: string;
  files: TorrentFile[];
  total_size: number;
}

export interface ReleaseParsed {
  content_type: DetectedContentType;
  title: string;
  year: number | null;
  quality: string | null;
  video_codec: string | null;
  audio: string | null;
  language: string | null;
  group: string | null;
  season: number | null;
  episode: number | null;
}

export interface TorrentInfo {
  meta: TorrentMeta;
  parsed: ReleaseParsed;
  size_formatted: string;
}

export type AppState =
  | { step: "idle" }
  | { step: "searching" }
  | { step: "selecting"; results: SearchResult[]; contentType: ContentType }
  | { step: "game_extras"; game: Game; claudeDescription: string | null; torrentInfo?: TorrentInfo }
  | { step: "app_form" }
  | { step: "generating" }
  | { step: "done"; bbcode: string; html: string }
  | { step: "error"; message: string }
  | { step: "torrent_parsed"; torrentInfo: TorrentInfo }
  | { step: "torrent_selecting"; results: SearchResult[]; contentType: ContentType; torrentInfo: TorrentInfo };
