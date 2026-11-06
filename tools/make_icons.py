"""アプリのアイコンを作る：カンブリア紀の地球と観察カプセル（PIL が必要）
python3 tools/make_icons.py  → web/icon-192.png, web/icon-512.png, web/apple-touch-icon.png, assets/app_icon.ico
"""
import math, random
from PIL import Image, ImageDraw, ImageFilter

def icon(n):
    S = 4 * n  # 大きく描いて縮める
    im = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    # 角の丸い背景（深い海の色）
    d.rounded_rectangle([0, 0, S - 1, S - 1], radius=S * 0.22, fill=(8, 30, 52, 255))
    cx, cy, r = S * 0.5, S * 0.52, S * 0.36
    # 地球（海）
    globe = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    g = ImageDraw.Draw(globe)
    g.ellipse([cx - r, cy - r, cx + r, cy + r], fill=(22, 92, 140, 255))
    # 大陸（植物のない赤茶けた陸と、浅い海）
    rnd = random.Random(5)
    blobs = [(-0.35, 0.25, 0.42), (0.25, 0.35, 0.30), (0.05, -0.38, 0.26), (-0.45, -0.25, 0.16), (0.48, -0.1, 0.14)]
    for (bx, by, br) in blobs:
        for k in range(14):
            a = rnd.random() * math.tau
            rr = br * r * (0.3 + 0.6 * rnd.random())
            px = cx + bx * r + math.cos(a) * rr * 0.8
            py = cy + by * r + math.sin(a) * rr * 0.6
            q = br * r * (0.35 + 0.35 * rnd.random())
            g.ellipse([px - q * 1.25, py - q * 1.25, px + q * 1.25, py + q * 1.25], fill=(70, 190, 190, 255))
    for (bx, by, br) in blobs:
        for k in range(14):
            a = rnd.random() * math.tau
            rr = br * r * (0.3 + 0.5 * rnd.random())
            px = cx + bx * r + math.cos(a) * rr * 0.8
            py = cy + by * r + math.sin(a) * rr * 0.6
            q = br * r * (0.3 + 0.3 * rnd.random())
            g.ellipse([px - q, py - q, px + q, py + q], fill=(176, 120, 82, 255))
    mask = Image.new("L", (S, S), 0)
    ImageDraw.Draw(mask).ellipse([cx - r, cy - r, cx + r, cy + r], fill=255)
    im.paste(globe, (0, 0), Image.composite(globe, Image.new("RGBA", (S, S)), mask).split()[3])
    # 光と影（右下を暗く、左上を明るく）
    shade = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    sd = ImageDraw.Draw(shade)
    sd.ellipse([cx - r * 0.35, cy - r * 0.35, cx + r * 1.7, cy + r * 1.7], fill=(0, 8, 24, 150))
    sd.ellipse([cx - r * 0.95, cy - r * 0.95, cx - r * 0.05, cy - r * 0.05], fill=(255, 255, 255, 40))
    shade = shade.filter(ImageFilter.GaussianBlur(S * 0.06))
    shade = Image.composite(shade, Image.new("RGBA", (S, S)), mask)
    im.alpha_composite(shade)
    d = ImageDraw.Draw(im)
    d.ellipse([cx - r, cy - r, cx + r, cy + r], outline=(150, 230, 220, 255), width=int(S * 0.012))
    # 観察カプセル（右上）
    kx, ky, kr = S * 0.70, S * 0.28, S * 0.13
    d.ellipse([kx - kr * 1.3 + S * 0.01, ky - kr + S * 0.015, kx + kr * 1.3 + S * 0.01, ky + kr + S * 0.015], fill=(0, 0, 0, 90))
    d.polygon([(kx - kr * 1.05, ky - kr * 0.25), (kx - kr * 1.75, ky - kr * 0.75), (kx - kr * 0.95, ky - kr * 0.6)], fill=(250, 158, 52, 255))
    d.polygon([(kx - kr * 1.05, ky + kr * 0.25), (kx - kr * 1.7, ky + kr * 0.75), (kx - kr * 0.9, ky + kr * 0.6)], fill=(250, 158, 52, 255))
    d.ellipse([kx - kr * 1.3, ky - kr, kx + kr * 1.3, ky + kr], fill=(226, 230, 236, 255))
    d.chord([kx - kr * 1.3, ky - kr, kx + kr * 1.3, ky + kr], 20, 160, fill=(92, 108, 128, 255))
    d.rectangle([kx - kr * 1.25, ky - kr * 0.06, kx + kr * 1.1, ky + kr * 0.1], fill=(250, 158, 52, 255))
    d.ellipse([kx + kr * 0.05 - kr * 0.55, ky - kr * 0.6, kx + kr * 0.05 + kr * 0.55, ky + kr * 0.5], fill=(24, 60, 86, 255))
    d.ellipse([kx + kr * 0.05 - kr * 0.44, ky - kr * 0.5, kx + kr * 0.05 + kr * 0.44, ky + kr * 0.4], fill=(80, 186, 212, 255))
    d.ellipse([kx + kr * 0.15 - kr * 0.13, ky - kr * 0.38, kx + kr * 0.15 + kr * 0.13, ky - kr * 0.12], fill=(255, 255, 255, 220))
    return im.resize((n, n), Image.LANCZOS)

if __name__ == "__main__":
    icon(512).save("web/icon-512.png")
    icon(192).save("web/icon-192.png")
    icon(180).save("web/apple-touch-icon.png")
    big = icon(256)
    big.save("assets/app_icon.ico", sizes=[(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)])
    print("ok")
