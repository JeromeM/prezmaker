import { describe, it, expect, vi, beforeEach } from "vitest";
import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { renderWithProviders } from "../test/render";
import BBCodePanel from "./BBCodePanel";

beforeEach(() => {
  vi.clearAllMocks();
});

describe("BBCodePanel", () => {
  const defaultProps = {
    bbcode: "[b]Hello World[/b]",
    onChange: vi.fn(),
  };

  it("renders bbcode content in textarea", () => {
    renderWithProviders(<BBCodePanel {...defaultProps} />);
    const textarea = screen.getByRole("textbox");
    expect(textarea).toHaveValue("[b]Hello World[/b]");
  });

  it("shows copy button", () => {
    renderWithProviders(<BBCodePanel {...defaultProps} />);
    expect(screen.getByText(/Copier/)).toBeInTheDocument();
  });

  it("calls onChange when editing", async () => {
    const user = userEvent.setup();
    renderWithProviders(<BBCodePanel {...defaultProps} />);
    const textarea = screen.getByRole("textbox");
    await user.clear(textarea);
    await user.type(textarea, "new content");
    expect(defaultProps.onChange).toHaveBeenCalled();
  });

  it("copies to clipboard and shows 'Copié !'", async () => {
    const spy = vi.spyOn(navigator.clipboard, "writeText").mockResolvedValue(undefined);
    const user = userEvent.setup();
    renderWithProviders(<BBCodePanel {...defaultProps} />);
    await user.click(screen.getByText(/Copier/));

    expect(spy).toHaveBeenCalledWith("[b]Hello World[/b]");
    expect(screen.getByText(/Copié/)).toBeInTheDocument();
  });

  it("renders headerActions when provided", () => {
    renderWithProviders(
      <BBCodePanel
        {...defaultProps}
        headerActions={<button>Custom Action</button>}
      />,
    );
    expect(screen.getByText("Custom Action")).toBeInTheDocument();
  });
});
