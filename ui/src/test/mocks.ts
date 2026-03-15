import type {
  TorrentInfo,
  SettingsPayload,
  GenerationResult,
  SavedPresentation,
  Collection,
  ContentTemplate,
  C411Category,
  C411OptionType,
} from "../types/api";

export const mockSettings: SettingsPayload = {
  tmdb_api_key: "test-tmdb-key",
  igdb_client_id: "test-igdb-id",
  igdb_client_secret: "test-igdb-secret",
  language: "fr-FR",
  title_color: "c0392b",
  default_templates: {},
  auto_clipboard: false,
  llm_provider: null,
  llm_api_key: null,
  groq_api_key: null,
  mistral_api_key: null,
  gemini_api_key: null,
  pseudo: "TestUser",
  c411_enabled: true,
  c411_api_key: "test-c411-key",
};

export const mockTorrentInfo: TorrentInfo = {
  meta: {
    name: "Movie.Title.2024.1080p.WEB-DL.MULTI-GRP",
    files: [{ path: "Movie.Title.2024.1080p.WEB-DL.MULTI-GRP.mkv", size: 4_000_000_000 }],
    total_size: 4_000_000_000,
  },
  parsed: {
    content_type: "Film",
    title: "Movie Title",
    year: 2024,
    quality: "WEB-DL 1080",
    video_codec: "x264",
    audio: "DTS",
    language: "MULTI",
    group: "GRP",
    season: null,
    episode: null,
  },
  size_formatted: "3.73 GiB",
};

export const mockGenerationResult: GenerationResult = {
  bbcode: "[center][b]Movie Title[/b][/center]\n[img]https://example.com/poster.jpg[/img]",
  nfo_text: "General\nComplete name: Movie.Title.mkv\nFormat: Matroska",
};

export const mockCollection: Collection = {
  id: "col-1",
  name: "Ma collection",
  created_at: "2026-01-01T00:00:00Z",
};

export const mockSavedPresentation: SavedPresentation = {
  id: "prez-1",
  collection_id: "col-1",
  title: "Movie Title",
  content_type: "film",
  bbcode: "[center][b]Movie Title[/b][/center]",
  poster_url: "https://example.com/poster.jpg",
  torrent_path: "/path/to/movie.torrent",
  nfo_text: "General\nComplete name: Movie.Title.mkv",
  saved_at: "2026-01-01T00:00:00Z",
  updated_at: "2026-01-01T00:00:00Z",
};

export const mockSavedGamePresentation: SavedPresentation = {
  id: "prez-2",
  collection_id: "col-1",
  title: "Cool Game",
  content_type: "jeu",
  bbcode: "[center][b]Cool Game[/b][/center]",
  poster_url: "https://example.com/game.jpg",
  torrent_path: "/path/to/game.torrent",
  nfo_text: null,
  saved_at: "2026-01-02T00:00:00Z",
  updated_at: "2026-01-02T00:00:00Z",
};

export const mockDefaultTemplate: ContentTemplate = {
  name: "default",
  content_type: "film",
  body: "{{title}}\n{{poster}}",
  is_default: true,
  title_color: null,
  order: 0,
};

export const mockC411Categories: C411Category[] = [
  {
    id: 1,
    name: "Films & Vidéos",
    subcategories: [
      { id: 6, name: "Film" },
      { id: 7, name: "Série TV" },
    ],
  },
  {
    id: 5,
    name: "Jeux",
    subcategories: [{ id: 36, name: "Jeu PC" }],
  },
];

export const mockC411Options: C411OptionType[] = [
  {
    id: 1,
    name: "Langue",
    slug: "langue",
    allowsMultiple: true,
    isRequired: true,
    sortOrder: 1,
    values: [
      { id: 1, value: "Anglais", slug: "anglais", sortOrder: 1 },
      { id: 2, value: "Français (VFF)", slug: "francais-vff", sortOrder: 2 },
      { id: 4, value: "Multi (FR inclus)", slug: "multi", sortOrder: 4 },
    ],
  },
  {
    id: 2,
    name: "Qualité",
    slug: "qualite",
    allowsMultiple: false,
    isRequired: true,
    sortOrder: 2,
    values: [
      { id: 25, value: "WEB-DL 1080", slug: "webdl-1080", sortOrder: 1 },
      { id: 413, value: "BluRay 1080", slug: "bluray-1080", sortOrder: 2 },
    ],
  },
];
