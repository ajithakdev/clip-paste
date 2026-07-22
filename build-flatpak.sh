#!/bin/bash
set -e

# Get metadata from Cargo.toml
NAME=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].name')
VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
APPID=io.github.ajithakdev.clip-paste
MANIFEST="${APPID}.json"
LOCAL_MANIFEST=".${APPID}.local.json"

# Ensure cargo-sources.json is present
if [[ ! -f cargo-sources.json ]]; then
    echo "→ cargo-sources.json not found, generating..."
    ./generate-cargo-sources.sh
fi

echo "→ Checking cargo-sources.json against Cargo.lock..."
python3 scripts/check-cargo-sources.py

# Create a temporary local manifest:
# - uses the current directory as source (not the pinned git commit)
# - strips com.system76.Cosmic.BaseApp which is only available on pop-os
#   build infrastructure, not from any public flatpak remote
jq '
  del(.base) | del(."base-version") |
  .modules[0].sources = [
    {"type": "dir", "path": "."},
    "cargo-sources.json"
  ]
' "${MANIFEST}" > "${LOCAL_MANIFEST}"

# Uninstall old version
flatpak --user uninstall ${APPID} -y 2>/dev/null || true

# Build and install the new version
flatpak-builder --force-clean --repo=repo build-dir "${LOCAL_MANIFEST}"

# Bundle into a .flatpak file
flatpak build-bundle repo "clippy-land_${VERSION}.flatpak" ${APPID}

# Clean up temp manifest
rm -f "${LOCAL_MANIFEST}"

echo "✔ Created clippy-land_${VERSION}.flatpak"
