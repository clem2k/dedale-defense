"""Generate the Android launcher icon from the tower atlas (requires Pillow).

Run from the repository root: python3 apps/android/icon/build_icon.py
"""

from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
ATLAS = ROOT / "assets/towers/towers.png"
RES = ROOT / "apps/android/res"
CELL = 208
TOWER = (5, 2)  # column, row in the atlas: upgraded orange tower, diagonal
BG = (14, 22, 33, 255)
WALL = (32, 46, 64, 255)
GOLD = (250, 196, 20, 255)
DENSITIES = {"mdpi": 1, "hdpi": 1.5, "xhdpi": 2, "xxhdpi": 3, "xxxhdpi": 4}
# Fixed maze walls on an 8x8 grid: (x0, y0, x1, y1) in cells.
WALLS = [
    (0, 0, 8, 0), (0, 8, 8, 8), (0, 0, 0, 8), (8, 0, 8, 8),
    (1, 1, 4, 1), (5, 1, 7, 1), (1, 1, 1, 3), (7, 1, 7, 4),
    (1, 7, 3, 7), (4, 7, 7, 7), (1, 5, 1, 7), (7, 5, 7, 7),
    (2, 2, 2, 4), (6, 2, 6, 3), (2, 6, 5, 6), (6, 5, 6, 6),
    (3, 2, 5, 2), (5, 2, 5, 3), (3, 3, 3, 5), (4, 4, 6, 4),
]


def tower():
    x, y = TOWER
    sprite = Image.open(ATLAS).convert("RGBA").crop(
        (x * CELL, y * CELL, (x + 1) * CELL, (y + 1) * CELL))
    return sprite.crop(sprite.getbbox())


def background(size):
    image = Image.new("RGBA", (size, size), BG)
    draw = ImageDraw.Draw(image)
    step = size / 9
    for x0, y0, x1, y1 in WALLS:
        draw.line([((x0 + 0.5) * step, (y0 + 0.5) * step),
                   ((x1 + 0.5) * step, (y1 + 0.5) * step)],
                  fill=WALL, width=max(1, round(size / 40)))
    return image


def foreground(size, content):
    """Tower centered and fitted into `content` pixels of a `size` canvas."""
    sprite = tower()
    scale = content / max(sprite.size)
    sprite = sprite.resize((round(sprite.width * scale), round(sprite.height * scale)),
                           Image.Resampling.LANCZOS)
    image = Image.new("RGBA", (size, size))
    image.alpha_composite(sprite, ((size - sprite.width) // 2, (size - sprite.height) // 2))
    return image


def legacy(size):
    """Pre-adaptive square icon with rounded corners and a gold rim."""
    big = size * 4
    image = background(big)
    image.alpha_composite(foreground(big, round(big * 0.78)))
    radius = big // 5
    rim = Image.new("RGBA", (big, big))
    ImageDraw.Draw(rim).rounded_rectangle(
        (0, 0, big - 1, big - 1), radius, outline=GOLD, width=big // 32)
    image.alpha_composite(rim)
    mask = Image.new("L", (big, big))
    ImageDraw.Draw(mask).rounded_rectangle((0, 0, big - 1, big - 1), radius, fill=255)
    image.putalpha(mask)
    return image.resize((size, size), Image.Resampling.LANCZOS)


def main():
    for name, density in DENSITIES.items():
        folder = RES / f"mipmap-{name}"
        folder.mkdir(parents=True, exist_ok=True)
        legacy(round(48 * density)).save(folder / "ic_launcher.png", optimize=True)
        # Adaptive layers are 108dp; launchers may crop down to the central 66dp.
        layer = round(108 * density)
        background(layer).save(folder / "ic_launcher_background.png", optimize=True)
        foreground(layer, round(70 * density)).save(
            folder / "ic_launcher_foreground.png", optimize=True)
    adaptive = RES / "mipmap-anydpi-v26"
    adaptive.mkdir(parents=True, exist_ok=True)
    (adaptive / "ic_launcher.xml").write_text(
        '<?xml version="1.0" encoding="utf-8"?>\n'
        '<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">\n'
        '    <background android:drawable="@mipmap/ic_launcher_background" />\n'
        '    <foreground android:drawable="@mipmap/ic_launcher_foreground" />\n'
        '</adaptive-icon>\n')


if __name__ == "__main__":
    main()
