#!/usr/bin/env bash
set -euo pipefail

# Build AppImage for Pyxillum Linux
echo "=== Building Pyxillum AppImage ==="
cargo build --release -p pyxillum-linux

APP_DIR="dist/AppDir"
mkdir -p "${APP_DIR}/usr/bin"
mkdir -p "${APP_DIR}/usr/share/applications"
mkdir -p "${APP_DIR}/usr/share/icons/hicolor/512x512/apps"

cp target/release/pyxillum-linux "${APP_DIR}/usr/bin/pyxillum-linux"
cp assets/icons/icon-512.png "${APP_DIR}/usr/share/icons/hicolor/512x512/apps/pyxillum.png"
cp assets/icons/icon-512.png "${APP_DIR}/pyxillum.png"

cat <<EOF > "${APP_DIR}/pyxillum.desktop"
[Desktop Entry]
Name=Pyxillum
Comment=Native Bluetooth Low Energy pixel LED controller
Exec=pyxillum-linux
Icon=pyxillum
Terminal=false
Type=Application
Categories=Utility;HardwareSettings;
EOF

cp "${APP_DIR}/pyxillum.desktop" "${APP_DIR}/usr/share/applications/"

# Download appimagetool if not present
if [ ! -f "appimagetool" ]; then
    wget -q "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage" -O appimagetool
    chmod +x appimagetool
fi

./appimagetool "${APP_DIR}" "dist/Pyxillum-x86_64.AppImage"
echo "=== AppImage successfully created at dist/Pyxillum-x86_64.AppImage ==="
