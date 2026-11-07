"""Validate requested assets and produce period contact sheets for visual review."""
import json
import sys
from collections import Counter
from pathlib import Path
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / 'output/mesozoic'


def main():
    OUTPUT.mkdir(parents=True, exist_ok=True)
    requests = json.loads((ROOT/'assets/requests.json').read_text(encoding='utf-8'))
    missing, invalid, checked = [], [], Counter()
    for request in requests:
        for kind in ('artwork', 'unit'):
            if kind not in request:
                continue
            path = ROOT/request[kind]['path']
            if not path.exists():
                missing.append({'slug': request['slug'], 'kind': kind})
                continue
            try:
                with Image.open(path) as image:
                    image.load()
                    if kind == 'unit':
                        assert image.mode == 'RGBA', 'expected RGBA'
                        lo, hi = image.getchannel('A').getextrema()
                        assert lo == 0 and hi > 220, 'expected transparent background and visible subject'
                        assert max(image.size) <= 768, 'oversized sprite'
                    else:
                        assert image.width >= 512 and image.height >= 480, 'undersized guide image'
                    checked[kind] += 1
            except Exception as error:
                invalid.append({'slug': request['slug'], 'kind': kind, 'error': str(error)})
    for period in ('Triassic', 'Jurassic', 'Cretaceous'):
        for kind in ('artwork', 'unit'):
            entries = [e for e in requests if e.get('period') == period and kind in e and (ROOT/e[kind]['path']).exists()]
            sheet = Image.new('RGB', (1200, max(1, (len(entries)+4)//5)*190), '#123344')
            draw = ImageDraw.Draw(sheet)
            for i, entry in enumerate(entries):
                with Image.open(ROOT/entry[kind]['path']) as image:
                    image = image.copy()
                    image.thumbnail((230, 160))
                    x, y = (i % 5)*240, (i // 5)*190
                    sheet.paste(image, (x+(240-image.width)//2, y+(160-image.height)//2), image if image.mode == 'RGBA' else None)
                    draw.text((x+5, y+167), entry['slug'], fill='#e9f0ec')
            sheet.save(OUTPUT/f'{period.lower()}-{kind}.jpg', quality=92)
    backdrops = [e for e in requests if e.get('kind') == 'backdrop' and e.get('period') in ('Triassic', 'Jurassic', 'Cretaceous') and (ROOT/e['artwork']['path']).exists()]
    sheet = Image.new('RGB', (1200, max(1, (len(backdrops)+2)//3)*280), '#123344')
    draw = ImageDraw.Draw(sheet)
    for i, entry in enumerate(backdrops):
        with Image.open(ROOT/entry['artwork']['path']) as image:
            image = image.copy()
            image.thumbnail((390, 250))
            x, y = (i % 3)*400, (i // 3)*280
            sheet.paste(image, (x+(400-image.width)//2, y+(250-image.height)//2))
            draw.text((x+5, y+257), entry['slug'], fill='#e9f0ec')
    sheet.save(OUTPUT/'backdrops.jpg', quality=92)
    report = {'checked': dict(checked), 'missing': missing, 'invalid': invalid}
    (OUTPUT/'audit.json').write_text(json.dumps(report, indent=2)+'\n', encoding='utf-8')
    print(json.dumps({'checked': dict(checked), 'missing': len(missing), 'invalid': invalid}))
    if invalid or (missing and '--partial' not in sys.argv):
        raise SystemExit(1)


if __name__ == '__main__':
    main()
