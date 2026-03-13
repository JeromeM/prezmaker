export type ContentType = "film" | "serie" | "jeu" | "app";

export interface SearchResult {
  id: number;
  label: string;
  source?: string;
  thumbnail?: string | null;
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
  default_templates: Record<string, string>;
  auto_clipboard: boolean;
  llm_provider: string | null;
  llm_api_key: string | null;
  groq_api_key: string | null;
  mistral_api_key: string | null;
  gemini_api_key: string | null;
  pseudo: string;
}

// Content Templates

export interface ContentTemplate {
  name: string;
  content_type: string;
  body: string;
  is_default: boolean;
  title_color?: string | null;
  order?: number | null;
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

export interface PresentationMeta {
  title: string;
  contentType: ContentType;
  posterUrl: string | null;
  savedRef?: { collectionId: string; entryId: string } | null;
}

export interface Collection {
  id: string;
  name: string;
  created_at: string;
}

export interface SavedPresentation {
  id: string;
  collection_id: string;
  title: string;
  content_type: string;
  bbcode: string;
  poster_url: string | null;
  saved_at: string;
}

export interface PendingGeneration {
  contentType: ContentType;
  tmdbId?: number;
  tech?: MediaTechInfo | null;
  gamePayload?: { game: Game; description: string | null; installation: string | null; tech_info: TechInfo };
  appPayload?: AppPayload;
  title?: string;
  posterUrl?: string | null;
}

export type AppState =
  | { step: "idle" }
  | { step: "searching" }
  | { step: "selecting"; results: SearchResult[]; contentType: ContentType }
  | { step: "game_extras"; game: Game; claudeDescription: string | null; torrentInfo?: TorrentInfo }
  | { step: "app_form" }
  | { step: "template_pick"; pending: PendingGeneration }
  | { step: "generating" }
  | { step: "done"; bbcode: string; html: string; meta: PresentationMeta }
  | { step: "error"; message: string }
  | { step: "torrent_parsed"; torrentInfo: TorrentInfo }
  | { step: "torrent_selecting"; results: SearchResult[]; contentType: ContentType; torrentInfo: TorrentInfo };
