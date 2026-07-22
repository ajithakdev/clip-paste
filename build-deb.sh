#!/bin/bash
set -e

# Get metadata from Cargo.toml
NAME=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].name')
VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
DESC=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].description')
# The crate/package is named clip-paste but the installed binary keeps the
# cosmic-applet-* name so COSMIC discovers it as a panel applet.
BIN=cosmic-applet-clip-paste
ARCH=amd64
APPID=io.github.ajithakdev.clip-paste

# Build the project
cargo build --release

# Prepare directories
rm -rf deb_build
mkdir -p deb_build/DEBIAN \
         deb_build/usr/bin \
         deb_build/usr/share/applications \
         deb_build/usr/share/icons/hicolor/scalable/apps \
         deb_build/usr/share/metainfo

# Copy files
cp target/release/$BIN deb_build/usr/bin/
chmod 755 deb_build/usr/bin/$BIN
sh scripts/render-debug-wrapper.sh \
    resources/$BIN-debug.sh.in \
    "/usr/bin/$BIN" \
    deb_build/usr/bin/$BIN-debug.sh
sh scripts/render-desktop-entry.sh \
    resources/$APPID.desktop \
    "/usr/bin/$BIN" \
    deb_build/usr/share/applications/$APPID.desktop
cp resources/icon.svg deb_build/usr/share/icons/hicolor/scalable/apps/$APPID.svg
cp resources/$APPID.metainfo.xml deb_build/usr/share/metainfo/$APPID.metainfo.xml

# Create control file
cat > deb_build/DEBIAN/control <<EOL
Package: $NAME
Version: $VERSION
Section: utils
Priority: optional
Architecture: $ARCH
Depends: wtype, wl-clipboard
Maintainer: ajithakdev <ajithkumarks2579@gmail.com>
Description: $DESC
EOL

# Build the .deb
DEB_NAME="${NAME}_${VERSION}_${ARCH}.deb"

# Export to release directory
mkdir -p release
dpkg-deb --build deb_build "release/$DEB_NAME"
echo "Created $DEB_NAME"
