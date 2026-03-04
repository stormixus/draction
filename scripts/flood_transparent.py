from PIL import Image
import sys
from collections import deque

def process_image(input_path, output_path):
    print("Loading image...")
    img = Image.open(input_path).convert("RGBA")
    width, height = img.size
    pixels = img.load()
    
    # 1) Collect all initially transparent pixels
    q = deque()
    # visited set to avoid double pushing
    visited = set()
    
    print("Finding initial transparent pixels...")
    for y in range(height):
        for x in range(width):
            r, g, b, a = pixels[x, y]
            if a == 0:
                q.append((x, y))
                visited.add((x, y))

    print(f"Initial transparent: {len(q)}")
    
    # 2) Flood fill to almost-white pixels
    print("Flood filling white fringes...")
    removed = 0
    
    directions = [(0,1), (0,-1), (1,0), (-1,0), (1,1), (-1,-1), (1,-1), (-1,1)]
    
    while q:
        cx, cy = q.popleft()
        
        for dx, dy in directions:
            nx, ny = cx + dx, cy + dy
            if 0 <= nx < width and 0 <= ny < height:
                if (nx, ny) not in visited:
                    visited.add((nx, ny))
                    r, g, b, a = pixels[nx, ny]
                    
                    # If this pixel is almost white
                    if r > 230 and g > 230 and b > 230:
                        # Make it transparent
                        pixels[nx, ny] = (255, 255, 255, 0)
                        removed += 1
                        # Push to queue to continue flood filling from here
                        q.append((nx, ny))

    print(f"Removed additional {removed} white pixels on the boundaries.")
    
    print("Saving output...")
    img.save(output_path, "PNG")
    print(f"Done! Saved to {output_path}")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python flood.py <in> <out>")
        sys.exit(1)
    process_image(sys.argv[1], sys.argv[2])
