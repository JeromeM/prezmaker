import { describe, it, expect, vi, beforeEach } from "vitest";
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { invoke } from "@tauri-apps/api/core";
import { renderWithProviders } from "../test/render";
import CollectionSaveDialog from "./CollectionSaveDialog";

const mockInvoke = vi.mocked(invoke);

beforeEach(() => {
  vi.clearAllMocks();
});

describe("CollectionSaveDialog", () => {
  const defaultProps = {
    onSave: vi.fn(),
    onClose: vi.fn(),
  };

  it("loads and displays collections", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "list_collections")
        return [
          { id: "col-1", name: "Films", created_at: "2026-01-01" },
          { id: "col-2", name: "Séries", created_at: "2026-01-02" },
        ];
      return null;
    });

    renderWithProviders(<CollectionSaveDialog {...defaultProps} />);

    await waitFor(() => {
      expect(screen.getByText("Films")).toBeInTheDocument();
      expect(screen.getByText("Séries")).toBeInTheDocument();
    });
  });

  it("pre-selects first collection", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "list_collections")
        return [{ id: "col-1", name: "Films", created_at: "2026-01-01" }];
      return null;
    });

    renderWithProviders(<CollectionSaveDialog {...defaultProps} />);

    await waitFor(() => {
      const radio = screen.getByRole("radio");
      expect(radio).toBeChecked();
    });
  });

  it("calls onSave with selected collection id", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "list_collections")
        return [
          { id: "col-1", name: "Films", created_at: "2026-01-01" },
          { id: "col-2", name: "Séries", created_at: "2026-01-02" },
        ];
      return null;
    });

    const user = userEvent.setup();
    renderWithProviders(<CollectionSaveDialog {...defaultProps} />);

    await waitFor(() => {
      expect(screen.getByText("Séries")).toBeInTheDocument();
    });

    // Select second collection
    await user.click(screen.getByText("Séries"));

    // Click save
    await user.click(screen.getByText(/Sauvegarder/));

    expect(defaultProps.onSave).toHaveBeenCalledWith("col-2");
  });

  it("allows creating a new collection", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "list_collections") return [];
      if (cmd === "create_collection")
        return { id: "new-col", name: "Nouvelle", created_at: "2026-01-01" };
      return null;
    });

    const user = userEvent.setup();
    renderWithProviders(<CollectionSaveDialog {...defaultProps} />);

    await waitFor(() => {
      expect(screen.getByText(/Nouvelle collection/)).toBeInTheDocument();
    });

    await user.click(screen.getByText(/Nouvelle collection/));

    const input = screen.getByPlaceholderText(/Nom de la collection/);
    await user.type(input, "Nouvelle");
    await user.click(screen.getByText(/Créer/));

    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith("create_collection", { name: "Nouvelle" });
    });
  });

  it("calls onClose when cancel is clicked", async () => {
    mockInvoke.mockImplementation(async () => []);
    const user = userEvent.setup();
    renderWithProviders(<CollectionSaveDialog {...defaultProps} />);

    const cancelButtons = screen.getAllByText(/Annuler/);
    await user.click(cancelButtons[0]);
    expect(defaultProps.onClose).toHaveBeenCalled();
  });

  it("does not close when clicking inside the dialog", async () => {
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "list_collections")
        return [{ id: "col-1", name: "Films", created_at: "2026-01-01" }];
      return null;
    });

    renderWithProviders(<CollectionSaveDialog {...defaultProps} />);

    await waitFor(() => {
      expect(screen.getByText("Films")).toBeInTheDocument();
    });

    // Click inside the dialog content
    await userEvent.click(screen.getByText("Films"));
    expect(defaultProps.onClose).not.toHaveBeenCalled();
  });
});
