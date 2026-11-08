"""Report the actual Cenozoic catalogue by region, time and biological group."""
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
REGIONS = {
    '北アメリカ': ['LAURENTIA'], '南アメリカ': ['SOUTH_AMERICA'],
    'ヨーロッパ': ['BALTICA', 'AVALONIA'],
    'アジア': ['SIBERIA', 'SOUTH_CHINA', 'NORTH_CHINA', 'KAZAKH', 'TARIM', 'INDIA'],
    'アフリカ': ['GONDWANA'], 'オーストラリア周辺': ['AUSTRALIA'], '南極周辺': ['ANTARCTICA'],
}
PERIODS = [('古第三紀', 66, 23.03), ('新第三紀', 23.03, 2.58), ('第四紀', 2.58, .01)]
TIMES = [('暁新世', 63), ('始新世', 50), ('漸新世', 28), ('中新世', 15), ('鮮新世', 3),
         ('更新世前期', 1), ('更新世中期', .4), ('更新世後期', .05), ('完新世', .01)]


def entries():
    text = (ROOT / 'src/life.rs').read_text(encoding='utf-8')
    text = text.split('pub static SPECIES: &[Species] = &[', 1)[1].split('/// 陸に降りて撮る景色', 1)[0]
    rows = []
    for match in re.finditer(r'\bSpecies\s*\{(.*?)\n    \},', text, re.S):
        body = match[1]
        number = lambda field: float(re.search(r'\b' + field + r':\s*([\d.]+)', body)[1])
        index = len(rows)
        slug = re.search(r'slug:\s*"([^"]+)"', body)[1]
        if index < 295:
            rows.append(None)
            continue
        rows.append({
            'id': index, 'slug': slug, 'name': re.search(r'name:\s*L\("([^"]+)"', body)[1],
            't0': number('t0'), 't1': number('t1'),
            'group': re.search(r'group:\s*L\("([^"]+)"', body)[1],
            'plates': re.findall(r'Plate\((\w+)\)', body),
            'wide': bool(re.search(r'\b(Global|Tropics)\b', re.search(r'regions:\s*&\[(.*?)\]', body, re.S)[1])),
            'marine': not bool(re.search(r'hab:\s*[^,]*\bLAND\b', body)),
        })
    assert len(rows) == 427, f'Unexpected species count: {len(rows)}'
    result = rows[295:]
    assert len(result) == 132
    assert len({r['slug'] for r in result}) == 132
    for old, young in [(66,23.03),(23.03,2.58),(2.58,.01)]:
        added = [r for r in result if 331 <= r['id'] < 403 and young < (r['t0'] + max(r['t1'], r['t0'] - 10)) / 2 <= old]
        assert len(added) == 24, f'Expected 24 additions in {old}–{young}, got {len(added)}'
    return result


def main():
    rows = entries()
    lines = ['# 新生代の収録バランス', '',
             '`python tools/audit_cenozoic.py` で `src/life.rs` から再生成します。', '',
             '新生代は初期36項目＋各巻24項目ずつの72項目＋南方古第三紀24項目で、計132項目。ゲーム全体は生きもの427項目、現象22項目、図鑑449項目です。', '',
             '## 地域と紀', '',
             '以下は地理指定のある項目の数で、周辺の海にいるものも含みます。複数地域や複数の紀に出る項目は各欄に数えるため、合計は132と一致しません。世界中・暖かい海の広域項目は地域欄に水増しせず、別に数えています。', '',
             '| 紀 | ' + ' | '.join(REGIONS) + ' | 広域 | 海・水辺 |',
             '|---|' + '---:|' * (len(REGIONS) + 3)]
    for name,old,young in PERIODS:
        alive = [r for r in rows if r['t0'] > young + 1e-6 and r['t1'] < old - 1e-6]
        counts = [sum(bool(set(r['plates']) & set(plates)) for r in alive) for plates in REGIONS.values()]
        lines.append('| ' + name + ' | ' + ' | '.join(map(str, counts + [sum(r['wide'] for r in alive), sum(r['marine'] for r in alive)])) + ' |')
    lines += ['', '## 世ごとの観察時点', '',
              '各世・更新世の区分から代表時点を選び、その時点で生きている新生代の項目を集計します。地域列は地理指定のある項目だけです。0はその時点・地域の固有項目が未収録という意味で、その地域に生きものがいなかったという意味ではありません。', '',
              '| 世・区分 | 百万年前 | 全項目 | ' + ' | '.join(REGIONS) + ' |',
              '|---|' + '---:|' * (len(REGIONS) + 2)]
    for name,t in TIMES:
        alive = [r for r in rows if r['t0'] + 1e-6 >= t >= r['t1'] - 1e-6]
        counts = [sum(bool(set(r['plates']) & set(plates)) for r in alive) for plates in REGIONS.values()]
        lines.append('| ' + name + f' | {t:g} | {len(alive)} | ' + ' | '.join(map(str, counts)) + ' |')
    lines += ['', '## 追加72項目の生物群', '', '| 生物群 | 項目数 |', '|---|---:|']
    groups = sorted({r['group'] for r in rows if 331 <= r['id'] < 403})
    for group in groups:
        lines.append(f'| {group} | {sum(r["group"] == group for r in rows if 331 <= r["id"] < 403)} |')
    lines += ['', '## 南方古第三紀の補強24項目', '',
              '豪州13・南極6の動物と、南方の地域を指定した森林植物5項目を追加しました。植物は科・群の代表像で、単一の固有種を全期間に置いたものではありません。', '',
              '| 区間 | 豪州・全項目 | 豪州・動物 | 南極・全項目 | 南極・動物 |', '|---|---:|---:|---:|---:|']
    for label,old,young in [('暁新世',66,56),('始新世',56,33.9),('漸新世',33.9,23.03)]:
        counts=[]
        for plate in ['AUSTRALIA','ANTARCTICA']:
            subset=[r for r in rows if plate in r['plates'] and r['t0']>young+1e-6 and r['t1']<old-1e-6]
            counts.extend([len(subset),sum(r['group']!='植物' for r in subset)])
        lines.append('| '+label+' | '+' | '.join(map(str,counts))+' |')
    lines += ['', 'マルゴンの動物は約5460万〜5450万年前、クロスヴァリアは約5930万〜5910万年前、エタダナの哺乳類は約2600万〜2400万年前など、短い化石記録の年代を維持しています。上の代表時点の表に出なくても、世の区間には観察できる動物がいます。詳細は [南方古第三紀の補強](paleogene-southern-coverage.md) を参照してください。', '']
    lines += ['', '## 解釈上の限界' , '',
              '- 数の完全な均等化はしていません。大型哺乳類は新生代を特徴づけるため多めですが、鳥・魚・爬虫類・昆虫・貝・有孔虫・ウニ・植物も追加しています。',
              '- 豪州・南極は植物を含め暁新世・始新世・漸新世のいずれにも地域指定の項目があります。ただし豪州の暁新世など、名前を付けられる陸生動物の記録が乏しい区間は残ります。未確認の動物を作ったり、マルゴンの年代を始新世全体へ広げたりはしていません。',
              '- 年代は観察できる期間の概略です。現生の属などの古い端は登場年代ではなく、その巻で観察する窓の始まりです。現象や古生代から生き続ける既存項目は、この132項目の表に含めません。',
              '- 地図は大陸単位の簡略モデルです。川・湖のワニは水辺環境、ニュージーランドのペンギンはオーストラリア周辺で観察します。アデリーペンギン属は属全体の分布と種の交代を簡略化しています。',
              '- 1万年前には絶滅済みのものを除きます。ゾウ・ライオン・キリン・野生のウマ、海鳥・魚・昆虫・針葉樹・広葉樹などが残る構成です。', '',
              '## 追加一覧', '', '| 紀（掲載巻） | 生きもの | 主な地理指定 | 観察窓（百万年前） |', '|---|---|---|---|']
    for row in rows:
        if row['id'] < 331:
            continue
        t = (row['t0'] + max(row['t1'], row['t0'] - 10)) / 2
        period = next(name for name,old,young in PERIODS if old >= t > young)
        regions = [name for name,plates in REGIONS.items() if set(plates) & set(row['plates'])]
        lines.append(f'| {period} | {row["name"]} | {"・".join(regions) or "広域"} | {row["t0"]:g}〜{row["t1"]:g} |')
    (ROOT / 'docs/cenozoic-balance.md').write_text('\n'.join(lines) + '\n', encoding='utf-8')
    print(f'Audited {len(rows)} Cenozoic entries, 72 original additions plus 24 southern Paleogene entries.')


if __name__ == '__main__':
    main()
