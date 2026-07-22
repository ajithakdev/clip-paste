# clip-paste

Auto-pasting clipboard manager applet for the COSMIC desktop (Pop!_OS 24.04+).

clip-paste keeps a history of what you copy and, when you pick an entry, pastes it
straight at the cursor of the window you were just in — no manual Ctrl+V.

> **Scope:** clip-paste is a **COSMIC panel applet**, not a standalone app. It runs
> **only** on the COSMIC desktop environment. It is **not** for GNOME, KDE, or X11.

![clip-paste popup](resources/example.png)

## Features

- Clipboard history for **text and images**
- **Auto-paste at cursor on select** — selecting an entry copies it, dismisses the
  popup, and synthesizes a paste into the previously focused window *(clip-paste
  addition; toggleable, on by default; image entries are copied but not auto-pasted)*
- **Pinned entries persist** across restarts and reboots
- Unpinned history is **runtime-only** (never written to disk)
- **Keyboard navigation** — arrows or `k`/`j` to move rows, `h`/`l` between row
  actions (copy, preview, pin, delete), `Enter` to activate, `Esc` to close
- In-popup **Settings** — max history / max pinned / max image size / max image
  dimensions, plus the paste-on-select toggle
- Search across history

## Requirements

- **COSMIC desktop environment** (e.g. Pop!_OS 24.04+).
- The compositor must expose these Wayland protocols:
  - `wlr-data-control` (clipboard access) — on some COSMIC builds you may need to
    enable it with `COSMIC_DATA_CONTROL_ENABLED=1`, e.g. via a drop-in in
    `/etc/environment.d/` (then log out and back in).
  - `wlr-layer-shell`
  - `zwp_virtual_keyboard`
- **`wtype`** and **`wl-clipboard`** installed:

  ```bash
  sudo apt install wtype wl-clipboard
  ```

## Install

### From the .deb release

Download the `.deb` from the [Releases](https://github.com/ajithakdev/clip-paste/releases)
page and install it:

```bash
sudo apt install ./clip-paste_*_amd64.deb
```

Then add the applet to your panel: **Settings → Desktop → Panel → Applets** and
add **clip-paste**.

### Build from source

Needs a Rust toolchain and [`just`](https://github.com/casey/just).

```bash
git clone https://github.com/ajithakdev/clip-paste.git
cd clip-paste
just build
sudo just install
```

Add it to the panel as above.

## Usage

Click the clip-paste icon in the panel to open the history popup, then click (or
press `Enter` on) an entry. With **Paste on select** enabled, focus returns to
your previous window and the entry is pasted at the cursor.

## Configuration

Config is stored at `~/.config/clip-paste/config.toml` (or
`$XDG_CONFIG_HOME/clip-paste/config.toml`), overridable with `CLIP_PASTE_CONFIG`.

Pinned entries are stored as app state so they survive restarts and reboots; only
pinned entries are persisted. By default pinned history lives at
`$XDG_STATE_HOME/clip-paste/pinned-history.toml` (or
`~/.local/state/clip-paste/pinned-history.toml`), overridable with
`CLIP_PASTE_PINNED_HISTORY`.

## Attribution

clip-paste is a fork of [k33wee/clippy-land](https://github.com/k33wee/clippy-land),
licensed under **GPL-3.0-or-later**. All original work is © k33wee.

Changes made in this fork (© 2026 ajithakdev), documented in [CHANGES.md](CHANGES.md):

- Auto-paste at cursor on item-select (via `wtype`).
- Removed the broken layer-surface `--toggle` path (fixed a `cosmic_corner_radius`
  protocol crash); `--toggle` is now a no-op.
- Hardened the image thumbnail render path against a renderer pixmap-allocation
  crash.
- Renamed the app ID, binary references, and config/state paths to clip-paste.

## License

[GPL-3.0-or-later](LICENSE). Copyright © k33wee and © 2026 ajithakdev.
