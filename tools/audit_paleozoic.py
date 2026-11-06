"""Audit generated Paleozoic assets, merge provenance, and render period sheets."""
import json
import sys
from collections import Counter
from pathlib import Path
from PIL import Image, ImageDraw

ROOT=Path(__file__).resolve().parents[1]
OUTPUT=ROOT/'output/paleozoic'

def read(path):
    return json.loads(path.read_text(encoding='utf-8'))

def main():
    requests=read(ROOT/'assets/requests.json')
    batch=(read(ROOT/sys.argv[sys.argv.index('--batch')+1]) if '--batch' in sys.argv else None)
    selected={entry['slug'] for entry in batch} if batch is not None else None
    missing=[]
    invalid=[]
    checked=Counter()
    generated=[read(f) for f in sorted((OUTPUT/'generated').glob('*.json'))]
    for folder,kind in [('artwork','artwork'),('units','unit')]:
        manifest=ROOT/'assets'/folder/'manifest.json'
        entries=read(manifest)
        prior={e['slug']:e for e in entries}
        for e in generated:
            if e['kind']==kind:
                prior[e['slug']]={k:v for k,v in e.items() if k!='exists'}
        manifest.write_text(json.dumps(list(prior.values()),ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    for request in requests:
        for kind in ('artwork','unit'):
            if kind not in request:
                continue
            path=ROOT/request[kind]['path']
            if not path.exists():
                missing.append({'slug':request['slug'],'kind':kind,'path':str(path)})
                continue
            try:
                with Image.open(path) as image:
                    image.load()
                    if kind=='unit':
                        assert image.mode=='RGBA','expected RGBA'
                        lo,hi=image.getchannel('A').getextrema()
                        assert lo==0 and hi>220,'expected clear alpha background and visible specimen'
                        assert max(image.size)<=768,'oversized sprite'
                    else:
                        assert image.width>=512 and image.height>=480,'undersized guide image'
                    checked[kind]+=1
            except Exception as error:
                invalid.append({'slug':request['slug'],'kind':kind,'error':str(error)})
    for period in ['Ordovician','Silurian','Devonian','Carboniferous','Permian']:
        for kind in ('artwork','unit'):
            entries=[r for r in requests if r.get('period')==period and kind in r and (ROOT/r[kind]['path']).exists()
                     and (selected is None or r['slug'] in selected)]
            width,height=(240,190) if kind=='artwork' else (240,190)
            columns=5
            sheet=Image.new('RGB',(columns*width,max(1,(len(entries)+columns-1)//columns)*height),'#123344')
            draw=ImageDraw.Draw(sheet)
            for i,e in enumerate(entries):
                with Image.open(ROOT/e[kind]['path']) as image:
                    image=image.copy()
                    image.thumbnail((width-10,height-30))
                    x,y=(i%columns)*width,(i//columns)*height
                    point=(x+(width-image.width)//2,y+(height-30-image.height)//2)
                    sheet.paste(image,point,image if image.mode=='RGBA' else None)
                    draw.text((x+5,y+height-22),e['slug'],fill='#e9f0ec')
            prefix='new-' if selected is not None else ''
            sheet.save(OUTPUT/f'{prefix}{period.lower()}-{kind}.jpg',quality=92)
    backdrops=[r for r in requests if r.get('kind')=='backdrop' and (ROOT/r['artwork']['path']).exists()]
    sheet=Image.new('RGB',(1280,max(1,(len(backdrops)+3)//4)*205),'#123344')
    draw=ImageDraw.Draw(sheet)
    for i,e in enumerate(backdrops):
        with Image.open(ROOT/e['artwork']['path']) as image:
            image=image.copy()
            image.thumbnail((310,175))
            x,y=(i%4)*320,(i//4)*205
            sheet.paste(image,(x+(320-image.width)//2,y+(175-image.height)//2))
            draw.text((x+5,y+182),e['slug'],fill='#e9f0ec')
    sheet.save(OUTPUT/'all-backdrops.jpg',quality=92)
    report={'checked':dict(checked),'generated':len(generated),'missing':missing,'invalid':invalid}
    (OUTPUT/'audit.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'checked':dict(checked),'generated':len(generated),'missing':len(missing),'invalid':invalid},ensure_ascii=True))
    if invalid or (missing and '--partial' not in sys.argv):
        raise SystemExit(1)

if __name__=='__main__':
    main()
