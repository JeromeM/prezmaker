import { describe, it, expect, vi, beforeEach } from "vitest";
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { invoke } from "@tauri-apps/api/core";
import { renderWithProviders } from "../test/render";
import { mockC411Categories, mockC411Options, mockTorrentInfo } from "../test/mocks";
import UploadDialog from "./UploadDialog";

const mockInvoke = vi.mocked(invoke);

beforeEach(() => {
  vi.clearAllMocks();
});

describe("UploadDialog", () => {
  const defaultProps = {
    torrentPath: "/path/to/movie.torrent",
    nfoContent: "General\nComplete name: Movie.mkv",
    bbcode: "[b]Movie Title[/b]",
    meta: { title: "Movie Title", contentType: "film" as const, posterUrl: null },
    torrentInfo: mockTorrentInfo,
    onClose: vi.fn(),
  };

  it("shows loading state while fetching categories", () => {
    mockInvoke.mockImplementation(() => new Promise(() => {})); // never resolves
    renderWithProviders(<UploadDialog {...defaultProps} />);
    expect(screen.getByText(/Chargement des catégories/)).toBeInTheDocument();
  });

  it("displays categories after loading", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "c411_fetch_categories") return mockC411Categories;
      if (cmd === "c411_auto_map")
        return { categoryId: 1, subcategoryId: 6, options: { "1": [4], "2": 25 } };
      if (cmd === "c411_fetch_options") return mockC411Options;
      return null;
    });

    renderWithProviders(<UploadDialog {...defaultProps} />);

    await waitFor(() => {
      expect(screen.getByText(/Titre/)).toBeInTheDocument();
    });
  });

  it("shows torrent file in files summary", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "c411_fetch_categories") return mockC411Categories;
      if (cmd === "c411_auto_map")
        return { categoryId: 1, subcategoryId: 6, options: {} };
      if (cmd === "c411_fetch_options") return [];
      return null;
    });

    renderWithProviders(<UploadDialog {...defaultProps} />);

    await waitFor(() => {
      expect(screen.getByText("movie.torrent")).toBeInTheDocument();
    });
  });

  it("shows NFO as auto-generated when provided", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "c411_fetch_categories") return mockC411Categories;
      if (cmd === "c411_auto_map")
        return { categoryId: 1, subcategoryId: 6, options: {} };
      if (cmd === "c411_fetch_options") return [];
      return null;
    });

    renderWithProviders(<UploadDialog {...defaultProps} />);

    await waitFor(() => {
      expect(screen.getByText(/auto-généré/)).toBeInTheDocument();
    });
  });

  it("shows warning when no NFO", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "c411_fetch_categories") return mockC411Categories;
      if (cmd === "c411_auto_map")
        return { categoryId: 1, subcategoryId: 6, options: {} };
      if (cmd === "c411_fetch_options") return [];
      return null;
    });

    renderWithProviders(<UploadDialog {...defaultProps} nfoContent={null} />);

    await waitFor(() => {
      expect(screen.getByText(/Aucun NFO généré/)).toBeInTheDocument();
    });
  });

  it("displays dynamic options from API", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "c411_fetch_categories") return mockC411Categories;
      if (cmd === "c411_auto_map")
        return { categoryId: 1, subcategoryId: 6, options: { "1": [4], "2": 25 } };
      if (cmd === "c411_fetch_options") return mockC411Options;
      return null;
    });

    renderWithProviders(<UploadDialog {...defaultProps} />);

    await waitFor(() => {
      expect(screen.getByText("Langue")).toBeInTheDocument();
      expect(screen.getByText("Qualité")).toBeInTheDocument();
    });
  });

  it("shows success message after upload", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "c411_fetch_categories") return mockC411Categories;
      if (cmd === "c411_auto_map")
        return { categoryId: 1, subcategoryId: 6, options: {} };
      if (cmd === "c411_fetch_options") return [];
      if (cmd === "c411_upload")
        return { success: true, message: "Torrent uploadé avec succès" };
      return null;
    });

    const user = userEvent.setup();
    renderWithProviders(<UploadDialog {...defaultProps} />);

    await waitFor(() => {
      expect(screen.getByText(/Uploader sur C411/)).toBeInTheDocument();
    });

    await user.click(screen.getByText(/Uploader sur C411/));

    await waitFor(() => {
      expect(screen.getByText(/Upload réussi/)).toBeInTheDocument();
    });
  });

  it("shows error message on upload failure", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "c411_fetch_categories") return mockC411Categories;
      if (cmd === "c411_auto_map")
        return { categoryId: 1, subcategoryId: 6, options: {} };
      if (cmd === "c411_fetch_options") return [];
      if (cmd === "c411_upload") throw new Error("Network error");
      return null;
    });

    const user = userEvent.setup();
    renderWithProviders(<UploadDialog {...defaultProps} />);

    await waitFor(() => {
      expect(screen.getByText(/Uploader sur C411/)).toBeInTheDocument();
    });

    await user.click(screen.getByText(/Uploader sur C411/));

    await waitFor(() => {
      expect(screen.getByText(/Network error/)).toBeInTheDocument();
    });
  });

  it("closes when clicking close button", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "c411_fetch_categories") return mockC411Categories;
      if (cmd === "c411_auto_map")
        return { categoryId: 1, subcategoryId: 6, options: {} };
      if (cmd === "c411_fetch_options") return [];
      return null;
    });

    const user = userEvent.setup();
    renderWithProviders(<UploadDialog {...defaultProps} />);

    // Wait for content to load
    await waitFor(() => {
      expect(screen.getByText(/Titre/)).toBeInTheDocument();
    });

    await user.click(screen.getByText(/Annuler/));
    expect(defaultProps.onClose).toHaveBeenCalled();
  });

  it("pre-fills title from torrent name", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "c411_fetch_categories") return mockC411Categories;
      if (cmd === "c411_auto_map")
        return { categoryId: 1, subcategoryId: 6, options: {} };
      if (cmd === "c411_fetch_options") return [];
      return null;
    });

    renderWithProviders(<UploadDialog {...defaultProps} />);

    await waitFor(() => {
      const titleInput = screen.getByDisplayValue("Movie.Title.2024.1080p.WEB-DL.MULTI-GRP");
      expect(titleInput).toBeInTheDocument();
    });
  });
});
