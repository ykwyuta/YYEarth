"""Preserve generated alpha while preparing embedded animated unit sprites.

The list of sprites comes from assets/requests.json (written from the game data by
`cargo test --lib write_asset_requests -- --ignored`). Sprites that are not ready yet are
simply skipped: build.rs embeds whatever PNGs exist and the game draws the rest procedurally."""
import json
from pathlib import Path
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'assets' / 'units'

def main():
    OUT.mkdir(parents=True, exist_ok=True)
    requests = json.loads((ROOT/'assets/requests.json').read_text(encoding='utf-8'))
    slugs = [e['slug'] for e in requests if 'unit' in e] + ['capsule', 'rock', 'seabed-sand']
    entries = []
    dimensions = []
    saved = OUT/'manifest.json'
    prior = {e['slug']:e for e in json.loads(saved.read_text(encoding='utf-8'))} if saved.exists() else {}
    for slug in slugs:
        metadata = ROOT/'output/unit-generation'/f'{slug}.json'
        if metadata.exists():
            entry = json.loads(metadata.read_text(encoding='utf-8'))
        elif slug in prior:
            entry = prior[slug]
        else:
            continue
        if not entry.get('source'):
            raise ValueError(f'Missing generated source: {slug}')
        target = OUT/f'{slug}.png'
        if not target.exists():
            image = Image.open(entry['source']).convert('RGBA')
            alpha = image.getchannel('A')
            if slug != 'seabed-sand' and (alpha.getextrema()[0] != 0 or not alpha.getbbox()):
                raise ValueError(f'{slug}: genuine transparency required')
            box = alpha.point(lambda a: 255 if a > 8 else 0).getbbox()
            if not box:
                raise ValueError(f'{slug}: empty silhouette')
            x0, y0, x1, y1 = box
            if slug != 'seabed-sand':
                image = image.crop((max(0,x0-12), max(0,y0-12), min(image.width,x1+12), min(image.height,y1+12)))
            image.thumbnail((768, 640), Image.Resampling.LANCZOS)
            image.save(target, optimize=True)
        with Image.open(target) as image:
            dimensions.append((slug, image.width, image.height))
        entries.append(entry)
    (OUT/'manifest.json').write_text(json.dumps(entries, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    # build.rs が assets/units/*.png を自動で組み込むので、Rust の表はもう書かない
    sheet = Image.new('RGB', (1000, max(1,(len(entries)+4)//5)*155), '#123344')
    draw = ImageDraw.Draw(sheet)
    for i,(slug,_,_) in enumerate(dimensions):
        image = Image.open(OUT/f'{slug}.png')
        image.thumbnail((192,125))
        x,y = (i%5)*200,(i//5)*155
        sheet.paste(image,(x+(196-image.width)//2,y+(125-image.height)//2),image)
        draw.text((x+5,y+132),slug,fill='#e1f2ec')
    preview = ROOT/'output/artwork'
    preview.mkdir(parents=True,exist_ok=True)
    sheet.save(preview/'unit-contact-sheet.jpg',quality=92)
    print(f'Prepared {len(entries)}/{len(slugs)} unit and scenery sprites; '+
          f'{sum(p.stat().st_size for p in OUT.glob("*.png"))/1048576:.1f} MiB')

if __name__ == '__main__':
    main()
