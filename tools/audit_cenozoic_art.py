"""Validate Cenozoic generated assets and write review contact sheets."""
import json
from pathlib import Path
from PIL import Image, ImageDraw

ROOT=Path(__file__).resolve().parents[1]


def main():
    requests=json.loads((ROOT/'assets/requests.json').read_text(encoding='utf-8'))
    requested=[]
    for row in requests:
        if row['period'] not in ['Paleogene','Neogene','Quaternary']:
            continue
        for kind in ['artwork','unit']:
            if kind in row:
                requested.append((row['slug'],kind,ROOT/row[kind]['path']))
    missing=[]
    checked=[]
    for slug,kind,path in requested:
        if not path.exists():
            missing.append(f'{kind}:{slug}')
            continue
        with Image.open(path) as im:
            im.load()
            assert im.width>64 and im.height>64, slug
            if kind=='unit':
                assert im.mode=='RGBA', slug
                alpha=im.getchannel('A')
                lo,hi=alpha.getextrema()
                assert lo==0 and hi>=240 and alpha.getbbox(), f'{slug}: invalid transparency'
                # Allow sub-2% alpha traces from generation/resampling.
                assert all(alpha.getpixel(p)<=4 for p in [(0,0),(im.width-1,0),(0,im.height-1),(im.width-1,im.height-1)]), f'{slug}: opaque corner'
            checked.append((slug,kind,path))
    folder=ROOT/'output/cenozoic-generation/review'
    folder.mkdir(parents=True,exist_ok=True)
    for kind in ['artwork','unit']:
        group=[r for r in checked if r[1]==kind]
        for page in range((len(group)+19)//20):
            sheet=Image.new('RGB',(1000,800),'#102a35')
            pen=ImageDraw.Draw(sheet)
            for i,(slug,_,path) in enumerate(group[page*20:(page+1)*20]):
                x,y=(i%4)*250,(i//4)*160
                if kind=='unit':
                    for yy in range(y,y+136,12):
                        for xx in range(x,x+246,12):
                            pen.rectangle((xx,yy,xx+11,yy+11),fill='#c4ced0' if ((xx-x)//12+(yy-y)//12)%2 else '#83969e')
                with Image.open(path) as im:
                    im=im.copy()
                    im.thumbnail((238,132))
                    at=(x+(246-im.width)//2,y+(136-im.height)//2)
                    sheet.paste(im,at,im.getchannel('A') if im.mode=='RGBA' else None)
                pen.text((x+4,y+140),slug,fill='white')
            sheet.save(folder/f'{kind}-{page+1:02}.jpg',quality=90)
    report={'checked':len(checked),'requested':len(requested),'missing':missing}
    (folder/'report.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(report,ensure_ascii=False))


if __name__=='__main__':
    main()
