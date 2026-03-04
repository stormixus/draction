from PIL import Image
import sys

img = Image.open(sys.argv[1]).convert("RGBA")
data = img.getdata()

pure_white = 0
almost_white = 0
transparent = 0
opaque = 0

for r, g, b, a in data:
    if a == 0:
        transparent += 1
    elif a > 0:
        opaque += 1
        if r > 240 and g > 240 and b > 240:
            almost_white += 1
        if r == 255 and g == 255 and b == 255:
            pure_white += 1

print(f"Total: {len(data)}")
print(f"Transparent: {transparent}")
print(f"Opaque: {opaque}")
print(f"Almost White: {almost_white}")
print(f"Pure White: {pure_white}")
