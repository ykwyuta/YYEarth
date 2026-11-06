"""Prepare one generated missing Paleozoic asset without changing existing images."""
import base64
import json
import sys
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]

def ingest(entry):
    target = ROOT / entry['path']
    target.parent.mkdir(parents=True, exist_ok=True)
    if not target.exists() or entry.get('replace', False):
        image = Image.open(entry['source']).convert('RGBA' if entry['kind'] == 'unit' else 'RGB')
        if entry['kind'] == 'unit':
            alpha = image.getchannel('A')
            if alpha.getextrema()[0] != 0 or not alpha.getbbox():
                raise ValueError(f"{entry['slug']}: missing genuine transparency")
            box = alpha.point(lambda a:255 if a>8 else 0).getbbox()
            if not box:
                raise ValueError(f"{entry['slug']}: empty sprite")
            x0,y0,x1,y1=box
            image=image.crop((max(0,x0-12),max(0,y0-12),min(image.width,x1+12),min(image.height,y1+12)))
            image.thumbnail((768,640),Image.Resampling.LANCZOS)
            image.save(target,optimize=True)
        else:
            image.thumbnail((1536,1024) if entry['slug'].startswith('backdrop-') else (960,640),Image.Resampling.LANCZOS)
            image.save(target,quality=88,optimize=True,progressive=False)
    metadata=ROOT/'output/paleozoic/generated'/f"{entry['kind']}-{entry['slug']}.json"
    metadata.parent.mkdir(parents=True,exist_ok=True)
    metadata.write_text(json.dumps(entry,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    return {'slug':entry['slug'],'kind':entry['kind'],'bytes':target.stat().st_size}

if __name__=='__main__':
    entry=(json.loads(Path(sys.argv[2]).read_text(encoding='utf-8')) if sys.argv[1]=='--metadata'
           else json.loads(base64.b64decode(sys.argv[1]).decode('utf-8')))
    print(json.dumps(ingest(entry)))
