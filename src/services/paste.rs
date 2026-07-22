//! Synthetic paste into the previously-focused window via `wtype`.
//!
//! Used by the "Paste on select" feature: after a history entry is copied to the
//! Wayland clipboard and the popup is dismissed, we spawn a detached thread that
//! waits for the compositor to restore focus to the prior window, then issues a
//! synthetic Ctrl+V.

use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

/// Delay before firing the synthetic paste, in milliseconds.
///
/// The Ctrl+V must land only AFTER the popup has released keyboard focus and the
/// compositor has restored the previously-focused window. If paste lands nowhere
/// or in the wrong window, this is the value to tune.
pub const PASTE_DELAY_MS: u64 = 120;

/// Spawn a detached background thread that waits `PASTE_DELAY_MS` then synthesizes
/// Ctrl+V using `wtype`. Never blocks the caller (UI thread). Non-zero exit and
/// spawn failures are ignored quietly (e.g. `wtype` not installed).
pub fn spawn_paste() {
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(PASTE_DELAY_MS));
        // wtype -M ctrl -k v -m ctrl  => press Ctrl, tap v, release Ctrl.
        let _ = Command::new("wtype")
            .args(["-M", "ctrl", "-k", "v", "-m", "ctrl"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    });
}
