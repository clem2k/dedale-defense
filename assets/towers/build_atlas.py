"""Extract the 48 tower views from the supplied reference (requires Pillow)."""

import json
from collections import deque
from pathlib import Path

from PIL import Image


def transparent_frame(source, center, size, vertical_bounds):
    x, y = center
    size -= 8
    scale_x, scale_y = source.width / 2000, source.height / 1493
    bounds = (round((x - size // 2) * scale_x), round((y - size // 2) * scale_y),
              round((x + size // 2) * scale_x), round((y + size // 2) * scale_y))
    frame = source.crop(bounds).resize((size, size), Image.Resampling.LANCZOS)
    pixels = frame.load()
    for py in range(size):
        world_y = center[1] - size // 2 + py
        if not vertical_bounds[0] <= world_y < vertical_bounds[1]:
            for px in range(size):
                pixels[px, py] = (0, 0, 0, 0)
    for y in range(size // 2 + 85, size):
        for x in range(size // 2 + 45, size):
            pixels[x, y] = (0, 0, 0, 0)
    background = set()
    pending = deque()
    for i in range(size):
        pending.extend(((i, 0), (i, size - 1), (0, i), (size - 1, i)))
    while pending:
        x, y = pending.popleft()
        if (x, y) in background or not (0 <= x < size and 0 <= y < size):
            continue
        if max(pixels[x, y][:3]) > 36:
            continue
        background.add((x, y))
        pending.extend(((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)))
    for x, y in background:
        pixels[x, y] = (0, 0, 0, 0)
    seen = set()
    for y in range(size):
        for x in range(size):
            if (x, y) in seen or not pixels[x, y][3]:
                continue
            component = []
            pending = deque([(x, y)])
            while pending:
                px, py = pending.popleft()
                if (px, py) in seen or not (0 <= px < size and 0 <= py < size):
                    continue
                if not pixels[px, py][3]:
                    continue
                seen.add((px, py))
                component.append((px, py))
                pending.extend(((px - 1, py), (px + 1, py), (px, py - 1), (px, py + 1)))
            if len(component) < 20:
                for px, py in component:
                    pixels[px, py] = (0, 0, 0, 0)
    # Suppress the dark JPEG fringe only at exposed edges, not inside the artwork.
    for y in range(size):
        for x in range(size):
            if (x, y) in background:
                continue
            if any(p in background for p in ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1))):
                r, g, b, _ = pixels[x, y]
                alpha = min(255, max(0, (max(r, g, b) - 36) * 13))
                pixels[x, y] = (r, g, b, alpha)
    padded = Image.new("RGBA", (size + 8, size + 8))
    padded.paste(frame, (4, 4))
    return padded


def main():
    root = Path(__file__).resolve().parent
    metadata = json.loads((root / "atlas.json").read_text())
    source = Image.open(root.parent / "sprite_towers.jpeg").convert("RGBA")
    if source.size != (2400, 1792):
        raise ValueError("Reference dimensions changed; update the extraction coordinates.")
    columns = (184, 426, 668, 909, 1152, 1394, 1636, 1878)
    rows = (170, 407, 644, 884, 1127, 1371)
    vertical_bounds = ((74, 250), (311, 496), (550, 738), (790, 971), (1034, 1211), (1275, 1472))
    size = metadata["frameSize"]
    if len(columns) != len(metadata["directions"]) or len(rows) != len(metadata["towers"]):
        raise ValueError("Atlas metadata does not match the reference layout.")
    atlas = Image.new("RGBA", (len(columns) * size, len(rows) * size))
    for row, y in enumerate(rows):
        for column, x in enumerate(columns):
            atlas.paste(transparent_frame(source, (x, y), size, vertical_bounds[row]),
                        (column * size, row * size))
    atlas.save(root / metadata["image"], optimize=True)
    print(f"Created {atlas.width}x{atlas.height} RGBA atlas: 6 towers x 8 directions")


if __name__ == "__main__":
    main()
