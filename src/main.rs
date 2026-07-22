// clip-paste — a fork of clippy-land (https://github.com/k33wee/clippy-land)
// Copyright (C) 2026 ajithakdev <ajithkumarks2579@gmail.com>
// Copyright (C) k33wee <https://github.com/k33wee>
// SPDX-License-Identifier: GPL-3.0-or-later
// Modified 2026-07-22: see below.
// Made --toggle a safe no-op after removing the layer-surface IPC toggle path.

mod app;
mod i18n;
mod services;
mod settings;

fn main() -> cosmic::iced::Result {
    let mut open_popup_on_start = std::env::var_os("COSMIC_PANEL_NAME").is_none();

    for arg in std::env::args().skip(1) {
        if arg == "--toggle" || arg == "-t" {
            // The layer-surface toggle was removed: cosmic-comp rejects the
            // constrained/standalone layer surface (cosmic_corner_radius_layer_v1
            // protocol error) and the applet cannot open its app_popup without a
            // pointer event. Exit cleanly instead of mapping a broken surface.
            eprintln!("toggle unsupported; click the panel icon");
            return Ok(());
        }

        if arg == "--standalone" {
            open_popup_on_start = true;
            continue;
        }

        if arg == "--no-standalone" {
            open_popup_on_start = false;
            continue;
        }

        if arg == "-h" || arg == "--help" {
            println!("clip-paste - Clipboard history applet for COSMIC");
            println!();
            println!("USAGE:");
            println!("    cosmic-applet-clip-paste [OPTIONS]");
            println!();
            println!("OPTIONS:");
            println!("    -t, --toggle    Toggle the clipboard popup via keyboard shortcut");
            println!("    --standalone    Open popup window immediately on startup");
            println!("    --no-standalone Do not auto-open popup on startup");
            println!("    -h, --help      Print this help message");
            println!();
            println!("KEYBOARD SHORTCUT SETUP:");
            println!("    1. Open COSMIC Settings > Keyboard > Custom Shortcuts");
            println!("    2. Click 'Add Custom Shortcut'");
            println!("    3. Name:    Clipboard History");
            println!("    4. Command: cosmic-applet-clip-paste --toggle");
            println!("    5. Shortcut: Press Super+V (or your preferred shortcut)");
            return Ok(());
        }
    }

    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
    i18n::init(&requested_languages);
    cosmic::applet::run::<app::AppModel>(app::AppFlags {
        open_popup_on_start,
    })
}
