"""Import completed built-in ImageGen results without replacing pre-existing assets."""
import json
from pathlib import Path
from ingest_mesozoic import ingest

ROOT=Path(__file__).resolve().parents[1]


def main():
    journal=ROOT/'output/cenozoic-generated.json'
    rows=json.loads(journal.read_text(encoding='utf-8')) if journal.exists() else []
    index={(r['kind'],r['slug']):r for r in rows}
    for path in sorted((ROOT/'output/cenozoic-generation/results').glob('*.json')):
        row=json.loads(path.read_text(encoding='utf-8'))
        index[(row['kind'],row['slug'])]=row
    rows=list(index.values())
    pending=[r for r in rows if not (ROOT/r['path']).exists()]
    ingest(pending)
    journal.write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(f'Imported {len(pending)} new images; {len(rows)}/276 generation records saved.')


if __name__=='__main__':
    main()
