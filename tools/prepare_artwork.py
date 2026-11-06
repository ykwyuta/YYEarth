"""Compress generated originals for the embedded, offline field guide."""
import json
from pathlib import Path
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "assets" / "artwork"

def main():
    entries = json.loads((OUT / "manifest.json").read_text(encoding="utf-8"))
    for entry in entries:
        source = entry.get("source")
        if not source:
            continue
        target = OUT / (entry["slug"] + ".jpg")
        if not target.exists():
            image = Image.open(source).convert("RGB")
            size = (768, 768) if entry["slug"] == "ms-toki" else (
                (1536, 1024) if entry["slug"] in ("title", "shallow-sea", "deep-sea") else (960, 640))
            image.thumbnail(size,
                            Image.Resampling.LANCZOS)
            image.save(target, quality=88, optimize=True, progressive=False)
    images = list(OUT.glob("*.jpg"))
    sheet = Image.new("RGB", (1000, ((len(images) + 4) // 5) * 150), "#07151e")
    pen = ImageDraw.Draw(sheet)
    for i, path in enumerate(images):
        image = Image.open(path)
        image.thumbnail((194, 124))
        x, y = (i % 5) * 200, (i // 5) * 150
        sheet.paste(image, (x + (194 - image.width) // 2, y))
        pen.text((x + 4, y + 127), path.stem, fill="#dceae9")
    preview = ROOT / "output" / "artwork"
    preview.mkdir(parents=True, exist_ok=True)
    sheet.save(preview / "contact-sheet.jpg", quality=90)
    print(f"Prepared {len(images)} images, {sum(p.stat().st_size for p in images) / 1048576:.1f} MiB")

if __name__ == "__main__":
    main()
