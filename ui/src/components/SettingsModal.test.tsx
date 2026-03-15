import { describe, it, expect, vi, beforeEach } from "vitest";
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { invoke } from "@tauri-apps/api/core";
import { renderWithProviders } from "../test/render";
import { mockSettings } from "../test/mocks";
import SettingsModal from "./SettingsModal";

const mockInvoke = vi.mocked(invoke);

beforeEach(() => {
  vi.clearAllMocks();
  mockInvoke.mockImplementation(async (cmd: string) => {
    if (cmd === "get_settings") return mockSettings;
    return null;
  });
});

describe("SettingsModal", () => {
  const defaultProps = {
    onClose: vi.fn(),
    theme: "dark" as const,
    onSetTheme: vi.fn(),
  };

  it("renders with tabs including Modules", async () => {
    renderWithProviders(<SettingsModal {...defaultProps} />);

    await waitFor(() => {
      expect(screen.getByText(/Modules/i)).toBeInTheDocument();
    });
  });

  it("shows C411 config in Modules tab", async () => {
    const user = userEvent.setup();
    renderWithProviders(<SettingsModal {...defaultProps} />);

    await waitFor(() => {
      expect(screen.getByText(/Modules/i)).toBeInTheDocument();
    });

    await user.click(screen.getByText(/Modules/i));

    await waitFor(() => {
      expect(screen.getByText("C411")).toBeInTheDocument();
    });
  });

  it("saves c411_enabled and c411_api_key", async () => {
    const user = userEvent.setup();
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "get_settings") return { ...mockSettings, c411_enabled: false, c411_api_key: null };
      if (cmd === "save_settings") return null;
      return null;
    });

    renderWithProviders(<SettingsModal {...defaultProps} />);

    await waitFor(() => {
      expect(screen.getByText(/Modules/i)).toBeInTheDocument();
    });

    await user.click(screen.getByText(/Modules/i));

    await waitFor(() => {
      expect(screen.getByText("C411")).toBeInTheDocument();
    });

    // Enable C411
    const checkbox = screen.getByRole("checkbox");
    await user.click(checkbox);

    // Save
    await user.click(screen.getByText(/Sauvegarder/i));

    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith("save_settings", expect.objectContaining({
        settings: expect.objectContaining({
          c411_enabled: true,
        }),
      }));
    });
  });
});
