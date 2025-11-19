"""
Icon Converter - Convert PNG to ICO with multiple sizes
This script converts the generated PNG icon to ICO format for Windows
"""

from PIL import Image
import sys

def create_ico(png_path, ico_path):
    """Convert PNG to ICO with multiple sizes"""
    try:
        # Open the PNG image
        img = Image.open(png_path)
        
        # Define icon sizes (Windows standard sizes)
        sizes = [(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
        
        # Create resized versions
        icon_images = []
        for size in sizes:
            resized = img.resize(size, Image.Resampling.LANCZOS)
            icon_images.append(resized)
        
        # Save as ICO
        icon_images[0].save(
            ico_path,
            format='ICO',
            sizes=[(img.width, img.height) for img in icon_images],
            append_images=icon_images[1:]
        )
        
        print(f"✓ Successfully created {ico_path}")
        print(f"  Sizes included: {', '.join([f'{s[0]}x{s[1]}' for s in sizes])}")
        return True
        
    except Exception as e:
        print(f"✗ Error: {e}")
        return False

if __name__ == "__main__":
    png_file = r"C:\Users\satya\.gemini\antigravity\brain\c38b16f0-eb62-4d0e-a059-a57ce63c5798\activity_logger_icon_1763573689380.png"
    ico_file = r"assets\icon.ico"
    
    print("Converting PNG to ICO...")
    if create_ico(png_file, ico_file):
        print("\nIcon created successfully!")
        print(f"Location: {ico_file}")
    else:
        print("\nFailed to create icon.")
        sys.exit(1)
