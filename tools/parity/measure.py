import sys
from PIL import Image, ImageChops

a = Image.open(sys.argv[1]).convert("RGB")
b = Image.open(sys.argv[2]).convert("RGB")
if a.size != b.size:
    print(f"size mismatch {a.size} vs {b.size}")
    raise SystemExit
d = ImageChops.difference(a, b).convert("L").point(lambda v: 255 if v > 40 else 0)
total = sum(1 for p in d.getdata() if p)
regions = {"titlebar": (0, 0, 1440, 48), "rail": (0, 48, 64, 900), "content": (64, 48, 1140, 900), "sidebar": (1140, 48, 1440, 900)}
parts = []
for name, box in regions.items():
    crop = d.crop(box)
    parts.append(f"{name} {sum(1 for p in crop.getdata() if p)}")
print(f"{total} differing pixels (" + ", ".join(parts) + ")")
