"""Extract the ten enemy sprites from the supplied reference (requires Pillow)."""

from pathlib import Path

from PIL import Image


def transparent_frame(source, column, columns, size):
    frame = source.crop((column * source.width // columns, 0,
                         (column + 1) * source.width // columns, source.height // 2))
    frame = frame.convert("RGBA")
    pixels = frame.load()
    for y in range(frame.height):
        for x in range(frame.width):
            r, g, b, _ = pixels[x, y]
            alpha = min(255, max(0, (max(r, g, b) - 24) * 8))
            if alpha == 0:
                pixels[x, y] = (0, 0, 0, 0)
            else:
                # Remove the dark JPEG matte from the partially transparent glow.
                pixels[x, y] = tuple(min(255, channel * 255 // alpha)
                                     for channel in (r, g, b)) + (alpha,)
    bounds = frame.getbbox()
    if bounds is None:
        raise ValueError(f"Missing sprite in column {column + 1}.")
    x = (bounds[0] + bounds[2]) // 2
    y = (bounds[1] + bounds[3]) // 2
    extent = 384
    if bounds[2] - bounds[0] > extent or bounds[3] - bounds[1] > extent:
        raise ValueError(f"Sprite exceeds extraction bounds in column {column + 1}.")
    frame = frame.crop((x - extent // 2, y - extent // 2,
                        x + extent // 2, y + extent // 2))
    frame = frame.resize((size - 8, size - 8), Image.Resampling.LANCZOS)
    padded = Image.new("RGBA", (size, size))
    padded.paste(frame, (4, 4))
    return padded


def main():
    root = Path(__file__).resolve().parent
    source = Image.open(root.parent / "sprite ennemis.jpeg")
    if source.size != (4128, 1024):
        raise ValueError("Reference dimensions changed; update the extraction coordinates.")
    size = 208
    atlas = Image.new("RGBA", (10 * size, size))
    for column in range(10):
        atlas.paste(transparent_frame(source, column, 10, size), (column * size, 0))
    atlas.save(root / "enemies.png", optimize=True)
    print(f"Created {atlas.width}x{atlas.height} RGBA atlas: 10 enemies")


if __name__ == "__main__":
    main()
