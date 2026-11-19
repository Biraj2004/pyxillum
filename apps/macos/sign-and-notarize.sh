#!/usr/bin/env bash
set -euo pipefail

# Code signing and notarization for Pyxillum macOS release
DEVELOPER_ID="${DEVELOPER_ID:-}"
APPLE_ID="${APPLE_ID:-}"
TEAM_ID="${TEAM_ID:-}"
APP_PASSWORD="${APP_PASSWORD:-}"

if [ -z "${DEVELOPER_ID}" ]; then
    echo "Developer ID certificate not specified. Skipping code signing."
    exit 0
fi

echo "=== Signing Pyxillum App Bundle ==="
codesign --deep --force --verify --verbose \
  --sign "${DEVELOPER_ID}" \
  --options runtime \
  --entitlements apps/macos/Pyxillum.entitlements \
  build/Pyxillum.app

if [ -n "${APPLE_ID}" ] && [ -n "${APP_PASSWORD}" ] && [ -n "${TEAM_ID}" ]; then
    echo "=== Submitting Disk Image for Notarization ==="
    xcrun notarytool submit "dist/Pyxillum-macOS-Universal.dmg" \
      --apple-id "${APPLE_ID}" \
      --password "${APP_PASSWORD}" \
      --team-id "${TEAM_ID}" \
      --wait
    xcrun stapler staple "dist/Pyxillum-macOS-Universal.dmg"
fi
