# clip-paste: resources and guide

Every link for the clip-paste applet, what it is for, and how to run and release it. Passwords and keys are never written here.

## TL;DR

- clip-paste is a clipboard manager panel applet for the COSMIC desktop.
- Selecting an entry copies it and auto-pastes it at your cursor via wtype.
- Pinned clipboard items persist across reboots, while unpinned history is kept in memory.
- Host requirements are the COSMIC desktop, wtype, and wl-clipboard.
- Pushing a version tag like v1.0.0 triggers GitHub Actions to build .deb and Flatpak releases.

## Links and what each one is for

### Public

| Link | Use it for |
| --- | --- |
| https://github.com/ajithakdev/clip-paste | Source code and project documentation |
| https://github.com/ajithakdev/clip-paste/releases | Download .deb and Flatpak releases |
| https://github.com/ajithakdev/clip-paste/issues | Issue tracker and bug reports |
| https://github.com/k33wee/clippy-land | Upstream repository |

### Behind the scenes

| Link | Use it for |
| --- | --- |
| https://github.com/ajithakdev/clip-paste | GitHub repository settings and CI workflows |
| https://github.com/pop-os/libcosmic.git | COSMIC GUI library dependency |
| https://github.com/flatpak/flatpak-builder-tools.git | Flatpak cargo generator tool repository |

## Accounts

| Service | Account | Role |
| --- | --- | --- |
| GitHub | `ajithakdev` | Owner |

## User guide

Install runtime dependencies:
`sudo apt install wtype wl-clipboard`

Install from the release deb package:
`sudo apt install ./clip-paste_*_amd64.deb`

Or build and install from source:
`just build`
`sudo just install`

Add the applet to the panel in Settings -> Desktop -> Panel -> Applets.
Click the clip-paste panel icon to open history. Click an entry or press Enter to paste it into the active window. Move through entries with arrow keys or k/j, and close the popup with Esc.

## Run and release

| Job | How |
| --- | --- |
| Build | `just build` or `cargo build --release` |
| Deb | `bash build-deb.sh` |
| Flatpak | Verify with `python3 scripts/check-cargo-sources.py`; CI builds bundle |
| Tagging | `git tag v1.0.0` and `git push origin v1.0.0` |

## When something goes wrong

| Problem | Fix |
| --- | --- |
| Clipboard access fails on COSMIC | Add `COSMIC_DATA_CONTROL_ENABLED=1` to `/etc/environment.d/` and log in again |
| Auto-paste does not type at cursor | Install wtype and wl-clipboard with `sudo apt install wtype wl-clipboard` |
| Popup does not open with `--toggle` | The layer-surface toggle was removed; click the panel icon instead |
| Cargo sources do not match `Cargo.lock` | Run `./generate-cargo-sources.sh` and commit updated `cargo-sources.json` |

## More detail

- `README.md`: feature overview, requirements, installation, and config paths
- `Cargo.toml`: package metadata, dependencies, and deb packaging layout
- `justfile`: command recipes for building, testing, and installing
- `CHANGES.md`: modifications made in this fork compared to upstream clippy-land
- `.github/workflows/release.yml`: release pipeline for deb and Flatpak packages
- `io.github.ajithakdev.clip-paste.json`: Flatpak application manifest
