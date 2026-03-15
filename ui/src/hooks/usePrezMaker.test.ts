import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { invoke } from "@tauri-apps/api/core";
import { usePrezMaker } from "./usePrezMaker";
import { mockSettings, mockTorrentInfo } from "../test/mocks";

const mockInvoke = vi.mocked(invoke);

beforeEach(() => {
  vi.clearAllMocks();
  // Default: get_settings returns mock settings
  mockInvoke.mockImplementation(async (cmd: string) => {
    if (cmd === "get_settings") return mockSettings;
    return null;
  });
});

describe("usePrezMaker", () => {
  it("starts in idle state", () => {
    const { result } = renderHook(() => usePrezMaker());
    expect(result.current.state.step).toBe("idle");
  });

  it("torrentFilePath is null initially", () => {
    const { result } = renderHook(() => usePrezMaker());
    expect(result.current.torrentFilePath).toBeNull();
  });

  it("torrentInfo is null initially", () => {
    const { result } = renderHook(() => usePrezMaker());
    expect(result.current.torrentInfo).toBeNull();
  });

  describe("importTorrent", () => {
    it("sets torrentFilePath after parsing", async () => {
      mockInvoke.mockImplementation(async (cmd: string) => {
        if (cmd === "get_settings") return mockSettings;
        if (cmd === "parse_torrent") return mockTorrentInfo;
        if (cmd === "search") return [{ id: 123, label: "Movie Title", source: "tmdb" }];
        return null;
      });

      const { result } = renderHook(() => usePrezMaker());

      await act(async () => {
        await result.current.importTorrent("/path/to/movie.torrent");
      });

      expect(result.current.torrentFilePath).toBe("/path/to/movie.torrent");
      expect(result.current.torrentInfo).toEqual(mockTorrentInfo);
    });
  });

  describe("reset", () => {
    it("clears torrentFilePath and torrentInfo", async () => {
      mockInvoke.mockImplementation(async (cmd: string) => {
        if (cmd === "get_settings") return mockSettings;
        if (cmd === "parse_torrent") return mockTorrentInfo;
        if (cmd === "search") return [{ id: 123, label: "Movie Title" }];
        return null;
      });

      const { result } = renderHook(() => usePrezMaker());

      await act(async () => {
        await result.current.importTorrent("/path/to/movie.torrent");
      });
      expect(result.current.torrentFilePath).toBe("/path/to/movie.torrent");

      act(() => {
        result.current.reset();
      });

      expect(result.current.state.step).toBe("idle");
      expect(result.current.torrentFilePath).toBeNull();
      expect(result.current.torrentInfo).toBeNull();
    });
  });

  describe("loadPresentation", () => {
    it("restores torrentFilePath from collection", async () => {
      const { result } = renderHook(() => usePrezMaker());

      await act(async () => {
        result.current.loadPresentation(
          "[b]Test[/b]",
          "<b>Test</b>",
          { title: "Test", contentType: "film", posterUrl: null },
          "/saved/path.torrent",
          "NFO content here",
        );
      });

      expect(result.current.state.step).toBe("done");
      expect(result.current.torrentFilePath).toBe("/saved/path.torrent");
      if (result.current.state.step === "done") {
        expect(result.current.state.nfoText).toBe("NFO content here");
      }
    });

    it("handles null torrentPath and nfoText", async () => {
      const { result } = renderHook(() => usePrezMaker());

      await act(async () => {
        result.current.loadPresentation(
          "[b]Test[/b]",
          "<b>Test</b>",
          { title: "Test", contentType: "film", posterUrl: null },
          null,
          null,
        );
      });

      expect(result.current.torrentFilePath).toBeNull();
      if (result.current.state.step === "done") {
        expect(result.current.state.nfoText).toBeNull();
      }
    });
  });

  describe("createTorrent", () => {
    it("sets torrentFilePath from output_path after creation", async () => {
      mockInvoke.mockImplementation(async (cmd: string) => {
        if (cmd === "get_settings") return mockSettings;
        if (cmd === "create_torrent") return mockTorrentInfo;
        if (cmd === "search") return [{ id: 123, label: "Movie Title" }];
        return null;
      });

      const { result } = renderHook(() => usePrezMaker());

      await act(async () => {
        await result.current.createTorrent({
          source_path: "/source/movie",
          output_path: "/output/movie.torrent",
          piece_size: null,
          private: true,
          trackers: ["https://tracker.example.com"],
          comment: null,
        });
      });

      expect(result.current.torrentFilePath).toBe("/output/movie.torrent");
      expect(result.current.torrentInfo).toEqual(mockTorrentInfo);
    });
  });
});
