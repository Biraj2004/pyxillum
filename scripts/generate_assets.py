#!/usr/bin/env python3
"""
Pyxillum Asset Generation Utility
Generates multi-resolution PNG icons, Windows .ico container, and WiX MSI installer bitmaps.
"""

import os
from PIL import Image, ImageDraw, ImageFont

def main():
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    assets_dir = os.path.join(base_dir, "assets")
    icons_dir = os.path.join(assets_dir, "icons")
    wix_dir = os.path.join(assets_dir, "wix")
    windows_dir = os.path.join(base_dir, "crates", "app", "windows")

    os.makedirs(icons_dir, exist_ok=True)
    os.makedirs(wix_dir, exist_ok=True)
    os.makedirs(windows_dir, exist_ok=True)

    master_path = os.path.join(icons_dir, "icon.png")
    if not os.path.exists(master_path):
        raise FileNotFoundError(f"Master icon not found: {master_path}")

    master_img = Image.open(master_path).convert("RGBA")
    print(f"Loaded master icon: {master_img.size}")

    # Standard icon sizes
    sizes = [16, 24, 32, 48, 64, 128, 256, 512]
    rendered_images = {}

    for sz in sizes:
        resized = master_img.resize((sz, sz), Image.Resampling.LANCZOS)
        out_png = os.path.join(icons_dir, f"icon-{sz}.png")
        resized.save(out_png, format="PNG", optimize=True)
        rendered_images[sz] = resized
        print(f"Wrote {out_png} ({sz}x{sz})")

    # Build Windows .ico containing multi-resolution layers
    ico_sizes = [(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    ico_destinations = [
        os.path.join(assets_dir, "pyxillum.ico"),
        os.path.join(windows_dir, "pyxillum.ico"),
        os.path.join(wix_dir, "pyxillum.ico"),
    ]

    for dest in ico_destinations:
        master_img.save(dest, format="ICO", sizes=ico_sizes)
        print(f"Generated multi-resolution .ico: {dest}")

    # Generate Rust source file with embedded 32x32 RGBA icon bytes
    icon_32 = master_img.resize((32, 32), Image.Resampling.LANCZOS)
    raw_rgba = icon_32.tobytes()
    rs_path = os.path.join(base_dir, "crates", "app", "src", "icon_data.rs")
    with open(rs_path, "w", encoding="utf-8") as f:
        f.write("//! Auto-generated application icon pixel data.\n\n")
        f.write("/// Raw RGBA byte array for 32x32 application icon.\n")
        f.write(f"pub const APP_ICON_RGBA_32: &[u8; {len(raw_rgba)}] = &[\n")
        for i in range(0, len(raw_rgba), 16):
            chunk = raw_rgba[i : i + 16]
            f.write("    " + ", ".join(f"0x{b:02X}" for b in chunk) + ",\n")
        f.write("];\n")
    print(f"Generated embedded icon data: {rs_path}")

    # Generate WiX MSI Installer Banner (493 x 58 px)
    banner_w, banner_h = 493, 58
    banner = Image.new("RGB", (banner_w, banner_h), color="#0B0F19")
    banner_draw = ImageDraw.Draw(banner)

    # Accent bottom line (Electric Cyan)
    banner_draw.line([(0, banner_h - 2), (banner_w, banner_h - 2)], fill="#00E5FF", width=2)

    # Place 44x44 icon on the right
    icon_44 = master_img.resize((44, 44), Image.Resampling.LANCZOS)
    banner.paste(icon_44, (banner_w - 44 - 10, 7), mask=icon_44.split()[3])

    # Text titles
    banner_draw.text((15, 11), "Pyxillum Setup", fill="#F8FAFC")
    banner_draw.text((15, 31), "Bluetooth Low Energy Pixel LED Controller", fill="#94A3B8")

    banner_bmp_path = os.path.join(wix_dir, "WixUIBannerBmp.bmp")
    banner_png_path = os.path.join(wix_dir, "WixUIBannerBmp.png")
    banner.save(banner_bmp_path, format="BMP")
    banner.save(banner_png_path, format="PNG")
    print(f"Generated MSI banner bitmap: {banner_bmp_path} (493x58)")

    # Generate WiX MSI Installer Dialog Welcome/Exit Banner (493 x 312 px)
    dialog_w, dialog_h = 493, 312
    dialog = Image.new("RGB", (dialog_w, dialog_h), color="#FFFFFF")
    dialog_draw = ImageDraw.Draw(dialog)

    # Left sidebar panel (164 px wide)
    sidebar_w = 164
    dialog_draw.rectangle([(0, 0), (sidebar_w, dialog_h)], fill="#0B0F19")
    dialog_draw.line([(sidebar_w - 1, 0), (sidebar_w - 1, dialog_h)], fill="#1E293B", width=1)

    # Place 96x96 icon on the sidebar
    icon_96 = master_img.resize((96, 96), Image.Resampling.LANCZOS)
    dialog.paste(icon_96, (34, 40), mask=icon_96.split()[3])

    # Sidebar branding text
    dialog_draw.text((32, 160), "Pyxillum", fill="#FFFFFF")
    dialog_draw.text((32, 185), "Native LED Manager", fill="#64748B")
    dialog_draw.line([(32, 215), (132, 215)], fill="#00E5FF", width=2)

    dialog_bmp_path = os.path.join(wix_dir, "WixUIDialogBmp.bmp")
    dialog_png_path = os.path.join(wix_dir, "WixUIDialogBmp.png")
    dialog.save(dialog_bmp_path, format="BMP")
    dialog.save(dialog_png_path, format="PNG")
    print(f"Generated MSI dialog bitmap: {dialog_bmp_path} (493x312)")

    print("All icon and installer assets generated successfully.")

if __name__ == "__main__":
    main()
