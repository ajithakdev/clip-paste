set dotenv-load := false

name    := 'cosmic-applet-clip-paste'
appid   := 'io.github.ajithakdev.clip-paste'
prefix  := '/usr'

bin_dir      := prefix + '/bin'
app_dir      := prefix + '/share/applications'
icon_dir     := prefix + '/share/icons/hicolor/scalable/apps'
metainfo_dir := prefix + '/share/metainfo'
license_dir  := prefix + '/share/licenses/' + appid
debug_wrapper := bin_dir + '/' + name + '-debug.sh'

# default recipe
_default:
    @just --list

# Build release binary
build *args:
    cargo build --release {{args}}

# Alias for Flatpak compatibility
build-release *args:
    just build {{args}}

# Install (supports `just prefix=/app install` for Flatpak builds)
install:
    install -Dm755 target/release/{{name}}          {{bin_dir}}/{{name}}
    install -Dm755 resources/{{name}}.sh            {{bin_dir}}/{{name}}.sh
    sh scripts/render-debug-wrapper.sh resources/{{name}}-debug.sh.in "{{bin_dir}}/{{name}}" "{{debug_wrapper}}"
    sh scripts/render-desktop-entry.sh resources/{{appid}}.desktop "{{bin_dir}}/{{name}}" "{{app_dir}}/{{appid}}.desktop"
    install -Dm644 resources/{{appid}}.metainfo.xml {{metainfo_dir}}/{{appid}}.metainfo.xml
    install -Dm644 resources/icon.svg               {{icon_dir}}/{{appid}}.svg
    install -Dm644 resources/icon.svg               {{icon_dir}}/{{appid}}-symbolic.svg
    install -Dm644 LICENSE                          {{license_dir}}/LICENSE
    update-desktop-database {{app_dir}} || true
    gtk-update-icon-cache -f {{prefix}}/share/icons/hicolor || true

# Uninstall from the configured prefix
uninstall:
    rm -f "{{bin_dir}}/{{name}}"
    rm -f "{{bin_dir}}/{{name}}.sh"
    rm -f "{{debug_wrapper}}"
    rm -f "{{app_dir}}/{{appid}}.desktop"
    rm -f "{{metainfo_dir}}/{{appid}}.metainfo.xml"
    rm -f "{{icon_dir}}/{{appid}}.svg"
    rm -f "{{icon_dir}}/{{appid}}-symbolic.svg"
    rm -f "{{license_dir}}/LICENSE"
    rmdir --ignore-fail-on-non-empty "{{license_dir}}" || true
    if command -v update-desktop-database >/dev/null 2>&1 && [ -d "{{app_dir}}" ]; then update-desktop-database "{{app_dir}}" || true; fi
    if command -v gtk-update-icon-cache >/dev/null 2>&1 && [ -d "{{prefix}}/share/icons/hicolor" ]; then gtk-update-icon-cache -f "{{prefix}}/share/icons/hicolor" || true; fi

# Uninstall the current user's ~/.local source install without sudo
uninstall-user:
    just prefix="$HOME/.local" uninstall

# Clean build artifacts
clean:
    cargo clean

# Run unit/integration tests
test:
    cargo test

# Run Wayland clipboard round-trip unit tests (ignored by default)
test-wayland:
    CLIP_PASTE_RUN_WAYLAND_TESTS=1 cargo test -- --ignored

# Run UI-level Wayland E2E clipboard tests via --toggle
e2e:
    ./tests/e2e/run.sh

# Point the installed desktop entry at the debug wrapper
enable-debug-wrapper:
    sh scripts/render-desktop-entry.sh resources/{{appid}}.desktop "{{debug_wrapper}}" "{{app_dir}}/{{appid}}.desktop"

# Restore the installed desktop entry to the normal binary
disable-debug-wrapper:
    sh scripts/render-desktop-entry.sh resources/{{appid}}.desktop "{{bin_dir}}/{{name}}" "{{app_dir}}/{{appid}}.desktop"
