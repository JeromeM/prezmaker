#!/usr/bin/env python3
"""Generate PrezMaker app icons using Pillow."""

from PIL import Image, ImageDraw, ImageFont
import os

ICON_DIR = os.path.join(os.path.dirname(__file__), "..", "src-tauri", "icons")

# Colors matching the app dark theme
BG_TOP = (25, 35, 60)        # Slightly lighter top
BG_BOT = (16, 20, 42)        # Darker bottom
ACCENT = (52, 152, 219)      # #3498db
ACCENT_LIGHT = (100, 190, 240)
RED = (231, 76, 60)          # #e74c3c
WHITE = (235, 235, 245)
BRACKET_COLOR = (60, 80, 120, 70)


def draw_rounded_rect(draw, xy, radius, fill):
    """Draw a rounded rectangle."""
    x0, y0, x1, y1 = xy
    r = radius
    draw.ellipse([x0, y0, x0 + 2*r, y0 + 2*r], fill=fill)
    draw.ellipse([x1 - 2*r, y0, x1, y0 + 2*r], fill=fill)
    draw.ellipse([x0, y1 - 2*r, x0 + 2*r, y1], fill=fill)
    draw.ellipse([x1 - 2*r, y1 - 2*r, x1, y1], fill=fill)
    draw.rectangle([x0 + r, y0, x1 - r, y1], fill=fill)
    draw.rectangle([x0, y0 + r, x1, y1 - r], fill=fill)


def create_icon(size):
    """Create a PrezMaker icon at the given size."""
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    s = size

    # --- Background: rounded square with vertical gradient ---
    margin = int(s * 0.04)
    radius = int(s * 0.18)

    # Draw gradient background manually
    for y in range(margin, s - margin):
        t = (y - margin) / (s - 2 * margin)
        r = int(BG_TOP[0] * (1 - t) + BG_BOT[0] * t)
        g = int(BG_TOP[1] * (1 - t) + BG_BOT[1] * t)
        b = int(BG_TOP[2] * (1 - t) + BG_BOT[2] * t)
        draw.line([(margin, y), (s - margin, y)], fill=(r, g, b, 255))

    # Apply rounded mask
    mask = Image.new("L", (s, s), 0)
    mask_draw = ImageDraw.Draw(mask)
    draw_rounded_rect(mask_draw, (margin, margin, s - margin, s - margin), radius, 255)
    img.putalpha(mask)
    draw = ImageDraw.Draw(img)

    # --- Decorative BBCode brackets in background ---
    bracket_size = int(s * 0.20)
    try:
        bracket_font = ImageFont.truetype(
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf", bracket_size)
    except Exception:
        bracket_font = ImageFont.load_default()

    draw.text((int(s * 0.07), int(s * 0.10)), "[", fill=BRACKET_COLOR, font=bracket_font)
    draw.text((int(s * 0.73), int(s * 0.67)), "]", fill=BRACKET_COLOR, font=bracket_font)

    # --- Main "P" letter ---
    p_size = int(s * 0.50)
    try:
        p_font = ImageFont.truetype(
            "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", p_size)
    except Exception:
        p_font = ImageFont.load_default()

    bbox = draw.textbbox((0, 0), "P", font=p_font)
    tw = bbox[2] - bbox[0]
    th = bbox[3] - bbox[1]
    px = (s - tw) // 2 - int(s * 0.04)
    py = (s - th) // 2 - int(s * 0.08)

    # Drop shadow
    draw.text((px + max(1, s // 200), py + max(1, s // 200)), "P",
              fill=(0, 0, 0, 60), font=p_font)
    draw.text((px, py), "P", fill=WHITE, font=p_font)

    # --- "M" subscript in accent color ---
    m_size = int(s * 0.19)
    try:
        m_font = ImageFont.truetype(
            "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", m_size)
    except Exception:
        m_font = ImageFont.load_default()

    mx = px + tw - int(s * 0.01)
    my = py + th - int(s * 0.04)
    draw.text((mx + 1, my + 1), "M", fill=(0, 0, 0, 50), font=m_font)
    draw.text((mx, my), "M", fill=ACCENT_LIGHT, font=m_font)

    # --- Gradient accent bar (red -> blue) ---
    bar_y = py + th + int(s * 0.10)
    bar_h = max(int(s * 0.03), 2)
    bar_x1 = int(s * 0.22)
    bar_x2 = int(s * 0.78)

    for x in range(bar_x1, bar_x2):
        t = (x - bar_x1) / (bar_x2 - bar_x1)
        cr = int(RED[0] * (1 - t) + ACCENT[0] * t)
        cg = int(RED[1] * (1 - t) + ACCENT[1] * t)
        cb = int(RED[2] * (1 - t) + ACCENT[2] * t)
        draw.line([(x, bar_y), (x, bar_y + bar_h)], fill=(cr, cg, cb, 200))

    # --- "{{ }}" template hint at bottom ---
    tag_size = int(s * 0.085)
    try:
        tag_font = ImageFont.truetype(
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf", tag_size)
    except Exception:
        tag_font = ImageFont.load_default()

    tag_text = "{{ }}"
    bbox_tag = draw.textbbox((0, 0), tag_text, font=tag_font)
    tag_w = bbox_tag[2] - bbox_tag[0]
    tag_x = (s - tag_w) // 2
    tag_y = bar_y + bar_h + int(s * 0.025)
    draw.text((tag_x, tag_y), tag_text, fill=(100, 120, 160, 120), font=tag_font)

    return img


def main():
    os.makedirs(ICON_DIR, exist_ok=True)

    sizes = {
        "32x32.png": 32,
        "128x128.png": 128,
        "128x128@2x.png": 256,
        "icon.png": 512,
    }

    for filename, sz in sizes.items():
        icon = create_icon(sz)
        path = os.path.join(ICON_DIR, filename)
        icon.save(path, "PNG")
        print(f"Created {filename} ({sz}x{sz})")

    # ICO (multi-size)
    icon_512 = create_icon(512)
    ico_sizes = [16, 24, 32, 48, 64, 128, 256]
    ico_images = [icon_512.resize((sz, sz), Image.LANCZOS) for sz in ico_sizes]
    ico_path = os.path.join(ICON_DIR, "icon.ico")
    ico_images[0].save(ico_path, format="ICO",
                       sizes=[(sz, sz) for sz in ico_sizes],
                       append_images=ico_images[1:])
    print("Created icon.ico (multi-size)")

    # ICNS (macOS)
    icns_path = os.path.join(ICON_DIR, "icon.icns")
    icon_512.save(icns_path, format="ICNS")
    print("Created icon.icns")

    print("\nDone!")


if __name__ == "__main__":
    main()
