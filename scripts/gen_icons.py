#!/usr/bin/env python3
"""Generate the Notation10 application icon set (original artwork).

Clean-room policy: nothing here derives from vendor assets. The glyph is an
original composition evoking an interactive whiteboard: a blue tile, a white
board and three marker strokes.

Outputs into crates/notation-tauri/icons/:
    icon.ico (16..256), icon.png (512), 32.png, 128.png, 256.png
"""

from PIL import Image, ImageDraw

OUT = "crates/notation-tauri/icons"

BLUE_TOP = (15, 122, 196)
BLUE_BOTTOM = (9, 84, 143)
BOARD = (247, 246, 242)
STROKES = [(229, 62, 62), (54, 169, 78), (240, 165, 30)]


def draw_glyph(size: int) -> Image.Image:
    s = size / 256.0
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)

    # Tile background with a vertical gradient.
    grad = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    gd = ImageDraw.Draw(grad)
    for y in range(size):
        t = y / max(1, size - 1)
        col = tuple(int(BLUE_TOP[i] + (BLUE_BOTTOM[i] - BLUE_TOP[i]) * t) for i in range(3))
        gd.line([(0, y), (size, y)], fill=col + (255,))
    mask = Image.new("L", (size, size), 0)
    md = ImageDraw.Draw(mask)
    md.rounded_rectangle([0, 0, size - 1, size - 1], radius=int(52 * s), fill=255)
    img.paste(grad, (0, 0), mask)

    # White board.
    pad = int(40 * s)
    board_box = (pad, pad, size - pad, size - pad)
    d.rounded_rectangle(board_box, radius=int(14 * s), fill=BOARD + (255,))

    # Marker strokes (handwriting evocation).
    stroke_specs = [
        (((int(66 * s), int(96 * s)), (int(190 * s), int(84 * s))), STROKES[0]),
        (((int(70 * s), int(138 * s)), (int(186 * s), int(148 * s))), STROKES[1]),
        (((int(74 * s), int(180 * s)), (int(150 * s), int(178 * s))), STROKES[2]),
    ]
    for (a, b), col in stroke_specs:
        lw = max(2, int(18 * s))
        d.line([a, b], fill=col + (255,), width=lw)
        r = lw // 2
        for p in (a, b):
            d.ellipse([p[0] - r, p[1] - r, p[0] + r, p[1] + r], fill=col + (255,))

    # A pen resting at the board's corner.
    pen_body = (24, 30, 38)
    x1, y1 = int(176 * s), int(200 * s)
    x2, y2 = int(216 * s), int(112 * s)
    lw = max(2, int(16 * s))
    d.line([(x1, y1), (x2, y2)], fill=pen_body + (255,), width=lw)
    # Pen tip.
    tipx, tipy = int(x2 + 10 * s), int(y2 - 20 * s)
    d.line([(x2, y2), (tipx, tipy)], fill=(15, 122, 196) + (255,), width=lw)

    return img


def main() -> None:
    import os

    os.makedirs(OUT, exist_ok=True)
    base = draw_glyph(256)
    base.resize((512, 512), Image.LANCZOS).save(f"{OUT}/icon.png")
    for px in (32, 128, 256):
        base.resize((px, px), Image.LANCZOS).save(f"{OUT}/{px}.png")

    ico_sizes = [(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    frames = [base.resize(sz, Image.LANCZOS) for sz in ico_sizes]
    frames[-1].save(f"{OUT}/icon.ico", format="ICO", sizes=ico_sizes)
    print("icons written to", OUT)


if __name__ == "__main__":
    main()
