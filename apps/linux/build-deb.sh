#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-1.0.5}"
ARCH="${2:-amd64}"
echo "=== Building Pyxillum Debian Package (.deb) v${VERSION} for ${ARCH} ==="

cargo build --release -p pyxillum-linux

DEB_ROOT="deb_build/pyxillum_${VERSION}_${ARCH}"
mkdir -p "${DEB_ROOT}/DEBIAN"
mkdir -p "${DEB_ROOT}/usr/bin"
mkdir -p "${DEB_ROOT}/usr/share/applications"
mkdir -p "${DEB_ROOT}/usr/share/icons/hicolor/512x512/apps"

cp target/release/pyxillum-linux "${DEB_ROOT}/usr/bin/pyxillum-linux"
chmod +x "${DEB_ROOT}/usr/bin/pyxillum-linux"
cp assets/icons/icon-512.png "${DEB_ROOT}/usr/share/icons/hicolor/512x512/apps/pyxillum.png"

cat <<EOF > "${DEB_ROOT}/usr/share/applications/pyxillum.desktop"
[Desktop Entry]
Name=Pyxillum
Comment=Native Bluetooth Low Energy pixel LED controller
Exec=pyxillum-linux
Icon=pyxillum
Terminal=false
Type=Application
Categories=Utility;HardwareSettings;
EOF

cat <<EOF > "${DEB_ROOT}/DEBIAN/control"
Package: pyxillum
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Maintainer: Pyxillum Contributors <pyxillum@github.com>
Depends: libgtk-4-1, libadwaita-1-0, libdbus-1-3
Description: Native Bluetooth Low Energy pixel LED controller
 Pyxillum is a lightweight, native cross-platform controller for BLE pixel LED strips
 of the SP621E/SP6xxE hardware family.
EOF

mkdir -p dist
dpkg-deb --build "${DEB_ROOT}" "dist/pyxillum_${VERSION}_${ARCH}.deb"
echo "=== Debian package created at dist/pyxillum_${VERSION}_${ARCH}.deb ==="
