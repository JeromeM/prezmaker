import { describe, it, expect, vi, beforeEach } from "vitest";
import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { renderWithProviders } from "../test/render";
import NfoModal from "./NfoModal";

beforeEach(() => {
  vi.clearAllMocks();
});

describe("NfoModal", () => {
  const nfoContent = "General\nComplete name: Movie.Title.mkv\nFormat: Matroska\nFile size: 4.00 GiB";
  const defaultProps = {
    content: nfoContent,
    title: "Movie Title",
    onClose: vi.fn(),
  };

  it("displays NFO content", () => {
    renderWithProviders(<NfoModal {...defaultProps} />);
    expect(screen.getByText(/Complete name/)).toBeInTheDocument();
    expect(screen.getByText(/Matroska/)).toBeInTheDocument();
  });

  it("shows copy and download buttons", () => {
    renderWithProviders(<NfoModal {...defaultProps} />);
    expect(screen.getByText(/Copier/)).toBeInTheDocument();
    expect(screen.getByText(/Télécharger/)).toBeInTheDocument();
  });

  it("closes when clicking close button", async () => {
    const user = userEvent.setup();
    renderWithProviders(<NfoModal {...defaultProps} />);
    await user.click(screen.getByText(/Fermer/));
    expect(defaultProps.onClose).toHaveBeenCalled();
  });

  it("closes when clicking backdrop", async () => {
    const user = userEvent.setup();
    const { container } = renderWithProviders(<NfoModal {...defaultProps} />);
    const backdrop = container.querySelector(".fixed.inset-0");
    if (backdrop) {
      await user.click(backdrop as HTMLElement);
      expect(defaultProps.onClose).toHaveBeenCalled();
    }
  });

  it("copies content to clipboard", async () => {
    const spy = vi.spyOn(navigator.clipboard, "writeText").mockResolvedValue(undefined);
    const user = userEvent.setup();
    renderWithProviders(<NfoModal {...defaultProps} />);
    await user.click(screen.getByText(/Copier/));

    expect(spy).toHaveBeenCalledWith(nfoContent);
  });
});
