import { describe, it, expect, vi, beforeEach } from "vitest";
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { invoke } from "@tauri-apps/api/core";
import { renderWithProviders } from "../test/render";
import { mockSettings } from "../test/mocks";
import SplitPreview from "./SplitPreview";

const mockInvoke = vi.mocked(invoke);

beforeEach(() => {
  vi.clearAllMocks();
  mockInvoke.mockImplementation(async (cmd: string) => {
    if (cmd === "get_settings") return mockSettings;
    if (cmd === "convert_bbcode") return "<b>Test</b>";
    return null;
  });
});

const defaultProps = {
  bbcode: "[b]Test BBCode[/b]",
  html: "<b>Test BBCode</b>",
  onConvert: vi.fn().mockResolvedValue("<b>Test</b>"),
  meta: { title: "Test Film", contentType: "film" as const, posterUrl: null },
  nfoText: "General\nComplete name: Test.mkv",
};

describe("SplitPreview", () => {
  it("renders bbcode and html panels", () => {
    renderWithProviders(<SplitPreview {...defaultProps} />);
    expect(screen.getByText("BBCode")).toBeInTheDocument();
  });

  it("shows Upload button when c411 enabled and torrent path exists", async () => {
    renderWithProviders(
      <SplitPreview
        {...defaultProps}
        torrentFilePath="/path/to/file.torrent"
        torrentInfo={null}
      />,
    );

    await waitFor(() => {
      expect(screen.getByText("Upload")).toBeInTheDocument();
    });
  });

  it("hides Upload button when no torrent path", async () => {
    renderWithProviders(
      <SplitPreview {...defaultProps} torrentFilePath={null} />,
    );

    // Wait for settings to load, then verify no upload button
    await waitFor(() => {
      expect(screen.queryByText("Upload")).not.toBeInTheDocument();
    });
  });

  it("hides Upload button when c411 disabled", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "get_settings") return { ...mockSettings, c411_enabled: false };
      return null;
    });

    renderWithProviders(
      <SplitPreview
        {...defaultProps}
        torrentFilePath="/path/to/file.torrent"
      />,
    );

    // Small wait for the settings to load
    await new Promise((r) => setTimeout(r, 50));
    expect(screen.queryByText("Upload")).not.toBeInTheDocument();
  });

  it("NFO button shows content when nfoText is available", async () => {
    const user = userEvent.setup();
    renderWithProviders(<SplitPreview {...defaultProps} />);

    await user.click(screen.getByText("NFO"));

    await waitFor(() => {
      expect(screen.getByText(/General/)).toBeInTheDocument();
    });
  });

  it("NFO button for games without NFO: confirm → Yes calls onReset", async () => {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const mockOpen = vi.mocked(open);
    const confirmSpy = vi.spyOn(window, "confirm").mockReturnValue(true);
    const onReset = vi.fn();

    renderWithProviders(
      <SplitPreview
        {...defaultProps}
        meta={{ title: "Cool Game", contentType: "jeu", posterUrl: null }}
        nfoText={null}
        mediaAnalysis={null}
        onReset={onReset}
      />,
    );

    const user = userEvent.setup();
    await user.click(screen.getByText("NFO"));

    expect(mockOpen).not.toHaveBeenCalled();
    expect(confirmSpy).toHaveBeenCalledWith(
      expect.stringContaining("relancer la génération"),
    );
    expect(onReset).toHaveBeenCalled();
  });

  it("NFO button for games without NFO: confirm → No does nothing", async () => {
    vi.spyOn(window, "confirm").mockReturnValue(false);
    const onReset = vi.fn();

    renderWithProviders(
      <SplitPreview
        {...defaultProps}
        meta={{ title: "Cool Game", contentType: "jeu", posterUrl: null }}
        nfoText={null}
        mediaAnalysis={null}
        onReset={onReset}
      />,
    );

    const user = userEvent.setup();
    await user.click(screen.getByText("NFO"));

    expect(onReset).not.toHaveBeenCalled();
  });

  it("NFO button shows confirm for films without NFO", async () => {
    const confirmSpy = vi.spyOn(window, "confirm").mockReturnValue(false);

    renderWithProviders(
      <SplitPreview
        {...defaultProps}
        meta={{ title: "Film", contentType: "film", posterUrl: null }}
        nfoText={null}
        mediaAnalysis={null}
      />,
    );

    const user = userEvent.setup();
    await user.click(screen.getByText("NFO"));

    expect(confirmSpy).toHaveBeenCalledWith(
      expect.stringContaining("fichier média"),
    );
  });

  it("passes torrentPath and nfoText to save_to_collection via savedRef", async () => {
    // Test with a savedRef already set (re-save to existing entry)
    const savedResult = {
      id: "existing-id",
      collection_id: "col-1",
      title: "Test Film",
      content_type: "film",
      bbcode: "[b]Test BBCode[/b]",
      poster_url: null,
      torrent_path: "/path/to/file.torrent",
      nfo_text: "General\nComplete name: Test.mkv",
      saved_at: "2026-01-01",
      updated_at: "2026-01-01",
    };

    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "get_settings") return mockSettings;
      if (cmd === "save_to_collection") return savedResult;
      return null;
    });

    const user = userEvent.setup();
    renderWithProviders(
      <SplitPreview
        {...defaultProps}
        meta={{
          title: "Test Film",
          contentType: "film",
          posterUrl: null,
          savedRef: { collectionId: "col-1", entryId: "existing-id" },
        }}
        torrentFilePath="/path/to/file.torrent"
      />,
    );

    // With savedRef, clicking "Sauvegarder" saves directly (no dialog)
    const saveBtn = await screen.findByText(/Sauvegarder/i);
    await user.click(saveBtn);

    await waitFor(() => {
      const call = mockInvoke.mock.calls.find((c) => c[0] === "save_to_collection");
      expect(call).toBeDefined();
      const args = call![1] as Record<string, unknown>;
      expect(args.torrentPath).toBe("/path/to/file.torrent");
      expect(args.nfoText).toBe("General\nComplete name: Test.mkv");
    });
  });
});
