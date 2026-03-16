import { describe, it, expect, vi, beforeEach } from "vitest";
import { screen, waitFor } from "@testing-library/react";
import { invoke } from "@tauri-apps/api/core";
import { renderWithProviders } from "../test/render";

const mockInvoke = vi.mocked(invoke);

// Must import after mocks are set up
import TemplatePicker from "./TemplatePicker";

beforeEach(() => {
  vi.clearAllMocks();
});

describe("TemplatePicker", () => {
  const defaultProps = {
    contentType: "film" as const,
    onSelect: vi.fn(),
    onCancel: vi.fn(),
    onEditTemplates: vi.fn(),
  };

  it("shows (par défaut) on the user favorite, not the built-in default", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "list_content_templates") {
        return [
          { name: "default", content_type: "film", body: "...", is_default: true, title_color: null, order: 0 },
          { name: "Mon Template", content_type: "film", body: "...", is_default: false, title_color: null, order: 1 },
          { name: "Autre", content_type: "film", body: "...", is_default: false, title_color: null, order: 2 },
        ];
      }
      if (cmd === "get_default_template") return "Mon Template"; // User favorite
      return null;
    });

    renderWithProviders(<TemplatePicker {...defaultProps} />);

    await waitFor(() => {
      expect(screen.getByText("Mon Template")).toBeInTheDocument();
    });

    // The "(par défaut)" label should be on "Mon Template", not "default"
    const monTemplateButton = screen.getByText("Mon Template").closest("button");
    expect(monTemplateButton?.textContent).toContain("(par défaut)");

    const defaultButton = screen.getByText("default").closest("button");
    expect(defaultButton?.textContent).not.toContain("(par défaut)");
  });

  it("shows (par défaut) on built-in default when no user favorite", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "list_content_templates") {
        return [
          { name: "default", content_type: "film", body: "...", is_default: true, title_color: null, order: 0 },
          { name: "Custom", content_type: "film", body: "...", is_default: false, title_color: null, order: 1 },
        ];
      }
      if (cmd === "get_default_template") return null; // No favorite
      return null;
    });

    renderWithProviders(<TemplatePicker {...defaultProps} />);

    await waitFor(() => {
      expect(screen.getByText("default")).toBeInTheDocument();
    });

    const defaultButton = screen.getByText("default").closest("button");
    expect(defaultButton?.textContent).toContain("(par défaut)");
  });

  it("auto-selects when only one template", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "list_content_templates") {
        return [
          { name: "default", content_type: "film", body: "...", is_default: true, title_color: null, order: 0 },
        ];
      }
      if (cmd === "get_default_template") return null;
      return null;
    });

    renderWithProviders(<TemplatePicker {...defaultProps} />);

    await waitFor(() => {
      expect(defaultProps.onSelect).toHaveBeenCalledWith("default", "bbcode");
    });
  });
});
