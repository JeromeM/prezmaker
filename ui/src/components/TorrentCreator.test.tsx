import { describe, it, expect, vi, beforeEach } from "vitest";
import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { save } from "@tauri-apps/plugin-dialog";
import { renderWithProviders } from "../test/render";
import TorrentCreator from "./TorrentCreator";

const mockSave = vi.mocked(save);

beforeEach(() => {
  vi.clearAllMocks();
  localStorage.clear();
});

describe("TorrentCreator", () => {
  const defaultProps = {
    initialPath: null as string | null,
    onCreateTorrent: vi.fn(),
    onCancel: vi.fn(),
  };

  it("renders the form with empty tracker by default", () => {
    renderWithProviders(<TorrentCreator {...defaultProps} />);
    expect(screen.getByText(/Créer un torrent/)).toBeInTheDocument();
    const trackerInput = screen.getByPlaceholderText(/tracker\.example/);
    expect(trackerInput).toHaveValue("");
  });

  it("pre-fills tracker from localStorage when saved", () => {
    localStorage.setItem("prezmaker_default_tracker", "https://my.tracker/announce");
    renderWithProviders(<TorrentCreator {...defaultProps} />);
    const trackerInput = screen.getByPlaceholderText(/tracker\.example/);
    expect(trackerInput).toHaveValue("https://my.tracker/announce");
  });

  it("sets private=true when tracker is remembered", () => {
    localStorage.setItem("prezmaker_default_tracker", "https://my.tracker/announce");
    renderWithProviders(<TorrentCreator {...defaultProps} />);
    const privateCheckbox = screen.getByLabelText(/Torrent privé/);
    expect(privateCheckbox).toBeChecked();
  });

  it("remember checkbox is checked when tracker exists", () => {
    localStorage.setItem("prezmaker_default_tracker", "https://my.tracker/announce");
    renderWithProviders(<TorrentCreator {...defaultProps} />);
    const rememberCheckbox = screen.getByLabelText(/Mémoriser ce tracker/);
    expect(rememberCheckbox).toBeChecked();
  });

  it("remember checkbox is unchecked by default", () => {
    renderWithProviders(<TorrentCreator {...defaultProps} />);
    const rememberCheckbox = screen.getByLabelText(/Mémoriser ce tracker/);
    expect(rememberCheckbox).not.toBeChecked();
  });

  it("saves tracker to localStorage on create when remember is checked", async () => {
    mockSave.mockResolvedValue("/output/file.torrent");
    const user = userEvent.setup();

    renderWithProviders(
      <TorrentCreator {...defaultProps} initialPath="/source/folder" />,
    );

    // Enter tracker URL
    const trackerInput = screen.getByPlaceholderText(/tracker\.example/);
    await user.clear(trackerInput);
    await user.type(trackerInput, "https://new.tracker/announce");

    // Check remember
    const rememberCheckbox = screen.getByLabelText(/Mémoriser ce tracker/);
    await user.click(rememberCheckbox);

    // Click create
    await user.click(screen.getByText(/Créer le torrent/));

    expect(localStorage.getItem("prezmaker_default_tracker")).toBe("https://new.tracker/announce");
  });

  it("removes tracker from localStorage when remember is unchecked", async () => {
    localStorage.setItem("prezmaker_default_tracker", "https://old.tracker/announce");
    mockSave.mockResolvedValue("/output/file.torrent");
    const user = userEvent.setup();

    renderWithProviders(
      <TorrentCreator {...defaultProps} initialPath="/source/folder" />,
    );

    // Uncheck remember
    const rememberCheckbox = screen.getByLabelText(/Mémoriser ce tracker/);
    await user.click(rememberCheckbox);

    // Click create
    await user.click(screen.getByText(/Créer le torrent/));

    expect(localStorage.getItem("prezmaker_default_tracker")).toBeNull();
  });

  it("adds a tracker row when clicking '+ Ajouter'", async () => {
    const user = userEvent.setup();
    renderWithProviders(<TorrentCreator {...defaultProps} />);

    const addBtn = screen.getByText(/Ajouter un tracker/);
    await user.click(addBtn);

    const inputs = screen.getAllByPlaceholderText(/tracker\.example/);
    expect(inputs).toHaveLength(2);
  });

  it("disables create button when no source path", () => {
    renderWithProviders(<TorrentCreator {...defaultProps} />);
    const createBtn = screen.getByText(/Créer le torrent/);
    expect(createBtn).toBeDisabled();
  });

  it("enables create button when source path is set", () => {
    renderWithProviders(
      <TorrentCreator {...defaultProps} initialPath="/some/path" />,
    );
    const createBtn = screen.getByText(/Créer le torrent/);
    expect(createBtn).not.toBeDisabled();
  });

  it("calls onCancel when cancel is clicked", async () => {
    const user = userEvent.setup();
    renderWithProviders(<TorrentCreator {...defaultProps} />);
    await user.click(screen.getByText(/Annuler/));
    expect(defaultProps.onCancel).toHaveBeenCalled();
  });
});
