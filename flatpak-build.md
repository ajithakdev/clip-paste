# Prerequisites

- Install Flatpak and flatpak-builder (version >= 1.4.2)
- Install Flatpak SDKs: `org.freedesktop.Sdk` and `org.freedesktop.Platform`

## Quick start (fresh clone)

```bash
# 1. Validate the MetaInfo file
flatpak install -y --user flathub org.flatpak.Builder
flatpak run --command=flatpak-builder-lint org.flatpak.Builder \
  appstream resources/io.github.ajithakdev.clip-paste.metainfo.xml

# 2. Validate the desktop file
desktop-file-validate resources/io.github.ajithakdev.clip-paste.desktop

# 3. Install prerequisites
sudo apt-get install flatpak-builder
flatpak remote-add --if-not-exists --user flathub https://flathub.org/repo/flathub.flatpakrepo
flatpak remote-add --if-not-exists --user cosmic https://apt.pop-os.org/cosmic/cosmic.flatpakrepo
flatpak install -y --user flathub org.freedesktop.Sdk//25.08 org.freedesktop.Platform//25.08
flatpak install -y --user flathub org.freedesktop.Sdk.Extension.rust-stable//25.08
flatpak install -y --user cosmic com.system76.Cosmic.BaseApp//stable

# 4. Verify the generated Cargo sources match Cargo.lock
uv run scripts/check-cargo-sources.py
# If this fails after a dependency update, run:
# ./generate-cargo-sources.sh
# uv run scripts/check-cargo-sources.py
# Then commit the updated cargo-sources.json.

# 5. Build locally (from the clippy-land project root):
flatpak-builder \
  --user \
  --install \
  --force-clean \
  --install-deps-from=flathub \
  build-dir \
  io.github.ajithakdev.clip-paste.json

# 6. Run it
flatpak run --user io.github.ajithakdev.clip-paste

# 7. Run the manifest linter
flatpak run --command=flatpak-builder-lint org.flatpak.Builder \
  manifest io.github.ajithakdev.clip-paste.json
```
