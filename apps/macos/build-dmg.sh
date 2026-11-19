#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-1.0.5}"
echo "=== Packaging Pyxillum macOS Application Bundle and Disk Image v${VERSION} ==="

APP_DIR="build/Pyxillum.app"
mkdir -p "${APP_DIR}/Contents/MacOS"
mkdir -p "${APP_DIR}/Contents/Resources"

cp Info.plist "${APP_DIR}/Contents/"
cp assets/icons/icon.icns "${APP_DIR}/Contents/Resources/AppIcon.icns" 2>/dev/null || true

# Assemble DMG volume directory with Applications symlink
DMG_ROOT="dmg_root"
mkdir -p "${DMG_ROOT}"
cp -R "${APP_DIR}" "${DMG_ROOT}/"
ln -sf /Applications "${DMG_ROOT}/Applications"

hdiutil create -volname "Pyxillum" -srcfolder "${DMG_ROOT}" -ov -format UDZO "dist/Pyxillum-macOS-Universal-v${VERSION}.dmg"
echo "=== macOS Disk Image created at dist/Pyxillum-macOS-Universal-v${VERSION}.dmg ==="
