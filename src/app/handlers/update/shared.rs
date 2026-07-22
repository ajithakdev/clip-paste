// clip-paste — a fork of clippy-land (https://github.com/k33wee/clippy-land)
// Copyright (C) 2026 ajithakdev <ajithkumarks2579@gmail.com>
// Copyright (C) k33wee <https://github.com/k33wee>
// SPDX-License-Identifier: GPL-3.0-or-later
// Modified 2026-07-22: see below.
// Added paste-on-select flow (copy + dismiss popup + synthetic Ctrl+V via wtype; skip for images).

use crate::app::view::summary::text_overlay_available;
use crate::app::{AppModel, Message};
use crate::services::clipboard::ClipboardEntry;
use crate::services::paste;
use cosmic::iced::widget::image::Handle as ImageHandle;
use cosmic::prelude::*;

use super::super::history;

/// Copy the history entry at `index` to the clipboard, then — when "Paste on
/// select" is enabled — dismiss the popup so focus returns to the previously
/// focused window and synthesize a paste into it. Ordering is explicit: copy
/// first, then close, then (after a delay) paste.
///
/// Returns `Some(task)` when the popup should close (paste-on-select ON), or
/// `None` to keep the popup open (copy-only, legacy behavior).
pub(super) fn select_history_item(
    app: &mut AppModel,
    index: usize,
) -> Option<Task<cosmic::Action<Message>>> {
    let item = app.history.get(index)?;
    let is_image = matches!(item.entry, ClipboardEntry::Image { .. });
    history::copy_history_item(item);

    if !app.settings.paste_on_select {
        return None;
    }

    // Close the popup so the compositor restores focus to the prior window, then
    // fire the synthetic Ctrl+V after PASTE_DELAY_MS (see services::paste).
    let close = super::popup::close_popup(app, "closed for paste-on-select");

    // Only synthesize Ctrl+V for text. A Ctrl+V "paste" of an image is
    // meaningless for the many targets that cannot accept an image via the
    // keyboard, and firing it into such a target risks unwanted behavior. The
    // image is still on the clipboard for the user to paste manually where it
    // makes sense.
    if !is_image {
        paste::spawn_paste();
    }

    Some(close)
}

pub(super) fn cache_thumbnail_handle(app: &mut AppModel, entry: &ClipboardEntry) {
    let ClipboardEntry::Image {
        bytes,
        hash,
        thumbnail_png: Some(thumbnail_png),
        ..
    } = entry
    else {
        return;
    };

    app.thumbnail_handles
        .entry((*hash, bytes.len()))
        .or_insert_with(|| ImageHandle::from_bytes(thumbnail_png.clone()));
}

pub(super) fn prune_thumbnail_handles(app: &mut AppModel) {
    app.thumbnail_handles.retain(|key, _| {
        app.history.iter().any(|item| match &item.entry {
            ClipboardEntry::Image { bytes, hash, .. } => key == &(*hash, bytes.len()),
            ClipboardEntry::Text(_) => false,
        })
    });
}

pub(super) fn warm_thumbnail_handles(app: &mut AppModel) {
    for item in app.history.iter() {
        let ClipboardEntry::Image {
            bytes,
            hash,
            thumbnail_png: Some(thumbnail_png),
            ..
        } = &item.entry
        else {
            continue;
        };

        app.thumbnail_handles
            .entry((*hash, bytes.len()))
            .or_insert_with(|| ImageHandle::from_bytes(thumbnail_png.clone()));
    }
}

pub(super) fn row_has_preview(app: &AppModel, idx: usize) -> bool {
    app.history
        .get(idx)
        .and_then(|item| match &item.entry {
            ClipboardEntry::Text(text) => Some(text_overlay_available(text)),
            ClipboardEntry::Image { .. } => None,
        })
        .unwrap_or(false)
}

pub(super) fn parse_usize_field(input: &str) -> Result<usize, &'static str> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("value is required");
    }
    trimmed
        .parse::<usize>()
        .map_err(|_| "must be a valid positive integer")
}

pub(super) fn parse_u32_field(input: &str) -> Result<u32, &'static str> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("value is required");
    }
    trimmed
        .parse::<u32>()
        .map_err(|_| "must be a valid positive integer")
}
