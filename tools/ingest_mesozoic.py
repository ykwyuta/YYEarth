"""Prepare generated Mesozoic assets and record their prompts without replacing existing art."""
import json
import sys
import base64
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]


def ingest(entries):
    for entry in entries:
        target = ROOT / entry['path']
        if not target.exists() or entry.get('replace', False):
            image = Image.open(entry['source']).convert('RGBA' if entry['kind'] == 'unit' else 'RGB')
            if entry['kind'] == 'unit':
                alpha = image.getchannel('A')
                if alpha.getextrema()[0] != 0 or not alpha.getbbox():
                    raise ValueError(f"{entry['slug']}: genuine transparency required")
                box = alpha.point(lambda a: 255 if a > 8 else 0).getbbox()
                x0, y0, x1, y1 = box
                image = image.crop((max(0, x0-12), max(0, y0-12), min(image.width, x1+12), min(image.height, y1+12)))
                image.thumbnail((768, 640), Image.Resampling.LANCZOS)
                image.save(target, optimize=True)
            else:
                image.thumbnail((1536, 1024) if entry['slug'].startswith('backdrop-') else (960, 640), Image.Resampling.LANCZOS)
                image.save(target, quality=88, optimize=True, progressive=False)
        manifest_path = target.parent / 'manifest.json'
        manifest = json.loads(manifest_path.read_text(encoding='utf-8'))
        if not any(e['slug'] == entry['slug'] for e in manifest):
            manifest.append({k: entry[k] for k in ('slug', 'prompt', 'source')})
            manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
        elif entry.get('replace', False):
            prior = next(e for e in manifest if e['slug'] == entry['slug'])
            prior['previous_source'] = prior['source']
            prior.update({k: entry[k] for k in ('prompt', 'source')})
            manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
        print(f"{entry['kind']} {entry['slug']}: {target.stat().st_size} bytes")


if __name__ == '__main__':
    data = base64.b64decode(sys.argv[2]).decode('utf-8') if sys.argv[1] == '--base64' else Path(sys.argv[1]).read_text(encoding='utf-8')
    ingest(json.loads(data))
