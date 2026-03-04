import sys
from PIL import Image

def remove_white(input_path, output_path, threshold=240):
    try:
        # Load the image and ensure it has an alpha channel
        img = Image.open(input_path).convert("RGBA")
        data = img.getdata()
        
        new_data = []
        for item in data:
            # item is (R, G, B, A)
            # If the pixel is close to pure white and isn't already transparent
            if item[0] > threshold and item[1] > threshold and item[2] > threshold:
                # Make it fully transparent
                new_data.append((255, 255, 255, 0))
            else:
                new_data.append(item)
                
        img.putdata(new_data)
        img.save(output_path, "PNG")
        print(f"Successfully processed image and saved to {output_path}")
    except Exception as e:
        print(f"Error processing image: {e}")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python remove_white.py <input> <output>")
        sys.exit(1)
        
    remove_white(sys.argv[1], sys.argv[2])
