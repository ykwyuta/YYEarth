"""日本語の表示文から、読み辞書で未補正の熟語を一覧にする（標準ライブラリのみ）。

L / t! / tf! / ja の第1引数の通常文字列を対象にする。単漢字、動的に組み立てた
文字列、実際の音声の発音・アクセントまでは検査しない。
"""
import argparse
from collections import Counter
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
JAPANESE = re.compile(r'(?:\bL\(|\bt!\(|\btf!\(|\bja\()\s*"((?:[^"\\]|\\.)*)"')
COMPOUND = re.compile(r"[一-龯々]{2,}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="未補正の熟語があれば失敗する")
    args = parser.parse_args()
    entries = []
    for line in (ROOT / "src/speech_ja.tsv").read_text(encoding="utf-8").splitlines():
        if line and not line.startswith("#"):
            entries.append(line.split("\t"))
    entries.sort(key=lambda pair: len(pair[0]), reverse=True)
    by_first = {}
    for written, reading in entries:
        by_first.setdefault(written[0], []).append((written, reading))

    def spoken(text):
        result = []
        cursor = 0
        while cursor < len(text):
            for written, reading in by_first.get(text[cursor], []):
                if text.startswith(written, cursor):
                    result.append(reading)
                    cursor += len(written)
                    break
            else:
                result.append(text[cursor])
                cursor += 1
        return "".join(result)

    counts = Counter()
    examples = {}
    total = 0
    for path in sorted((ROOT / "src").rglob("*.rs")):
        if path.name == "speech.rs":
            continue
        source = path.read_text(encoding="utf-8")
        for match in JAPANESE.finditer(source):
            total += 1
            for word in COMPOUND.findall(spoken(match[1])):
                counts[word] += 1
                examples.setdefault(word, f"{path.relative_to(ROOT)}:{source.count(chr(10), 0, match.start()) + 1}")
    print(f"{len(entries)} readings; {total} Japanese strings; {len(counts)} remaining compounds")
    for word, count in counts.most_common():
        print(f"{word}\t{count}\t{examples[word]}")
    return int(args.check and bool(counts))


if __name__ == "__main__":
    # PowerShell/Windows の既定コードページに依存しない。
    sys.stdout.reconfigure(encoding="utf-8")
    raise SystemExit(main())
