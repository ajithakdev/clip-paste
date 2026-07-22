# Changes in clip-paste vs. upstream clippy-land

clip-paste is a fork of [k33wee/clippy-land](https://github.com/k33wee/clippy-land),
licensed under GPL-3.0-or-later. Per GPL-3.0 section 5, the modifications made in
this fork are recorded here.

Copyright (C) k33wee — original clippy-land.
Copyright (C) 2026 ajithakdev <ajithkumarks2579@gmail.com> — clip-paste modifications.

## Modifications (2026-07-22)

- **Auto-paste on select** — selecting a history entry now copies it to the
  clipboard, dismisses the popup so focus returns to the previously focused
  window, then synthesizes a paste (Ctrl+V via `wtype`) after a short delay.
  Added a "Paste on select" setting (default on). Image entries are copied but
  not auto-pasted. (`src/app/handlers/update/shared.rs`, `src/services/paste.rs`,
  `src/settings.rs`, `src/app/view/popup/settings.rs`.)
- **Removed the layer-surface `--toggle` path** — the hotkey/IPC layer-shell
  surface tripped a fatal `cosmic_corner_radius_layer_v1` protocol error on
  cosmic-comp. Removed the file-based IPC toggle and the layer surface entirely;
  `--toggle` is now a clean no-op. The removed file `src/ipc.rs` implemented the
  old signal-file toggle. (`src/main.rs`, `src/app/mod.rs`, `src/app/surfaces.rs`,
  `src/app/handlers/update/popup.rs`, `src/app/handlers/subscription.rs`.)
- **Hardened image rendering** — bounded the list thumbnail widget so the
  software renderer cannot attempt an oversized pixmap allocation.
  (`src/app/view/row/render/content.rs`.)
- **Renamed** the application ID, binary references, config/state directory, and
  environment variables from clippy-land to clip-paste. Runtime deps documented:
  `wtype`, `wl-clipboard`.
