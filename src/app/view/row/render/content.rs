// clip-paste — a fork of clippy-land (https://github.com/k33wee/clippy-land)
// Copyright (C) 2026 ajithakdev <ajithkumarks2579@gmail.com>
// Copyright (C) k33wee <https://github.com/k33wee>
// SPDX-License-Identifier: GPL-3.0-or-later
// Modified 2026-07-22: see below.
// Bounded image thumbnail widget to prevent an oversized renderer pixmap allocation.

use super::super::RowContent;
use super::super::state::RowRenderState;
use crate::app::Message;
use cosmic::iced::Length;
use cosmic::iced::widget::image::Handle as ImageHandle;
use cosmic::prelude::*;
use cosmic::widget;

const IMAGE_PREVIEW_HEIGHT: f32 = 200.0;
/// Hard cap on the on-screen width of a row thumbnail. Bounds the pixmap the
/// software renderer (tiny_skia) must allocate so an extreme-aspect-ratio source
/// image can never force an oversized surface allocation and panic the applet.
const IMAGE_PREVIEW_MAX_WIDTH: f32 = 360.0;

pub(super) fn row_label(state: &RowRenderState, row_expanded: bool) -> Element<'static, Message> {
    match &state.content {
        RowContent::Text {
            collapsed_summary,
            expanded_summary: _,
            overlay_available: _,
        } => widget::text::body(collapsed_summary.clone()).into(),
        RowContent::Image {
            mime,
            bytes_len,
            thumbnail_handle,
            ..
        } => {
            let thumb: Option<Element<'_, Message>> = thumbnail_handle.clone().map(|handle| {
                // Both axes are fixed-size and the container clips, so the software
                // renderer (tiny_skia) always allocates a small, bounded pixmap for
                // the thumbnail regardless of the source image's dimensions or
                // aspect ratio. An unbounded/intrinsic image size here can force an
                // oversized surface allocation that panics the applet.
                widget::container(
                    widget::image::<ImageHandle>(handle)
                        .width(Length::Fixed(IMAGE_PREVIEW_MAX_WIDTH))
                        .height(Length::Fixed(IMAGE_PREVIEW_HEIGHT))
                        .content_fit(cosmic::iced::ContentFit::Contain),
                )
                .width(Length::Fill)
                .max_width(IMAGE_PREVIEW_MAX_WIDTH)
                .height(Length::Fixed(IMAGE_PREVIEW_HEIGHT))
                .max_height(IMAGE_PREVIEW_HEIGHT)
                .align_x(cosmic::iced::Alignment::Center)
                .clip(true)
                .into()
            });

            let mut col = widget::column::Column::new()
                .width(Length::Fill)
                .align_x(cosmic::iced::Alignment::Center);
            if let Some(thumb) = thumb {
                col = col.push(thumb);
            }

            let details = if row_expanded {
                format!("{} ({} KB)", mime, (bytes_len.saturating_add(1023)) / 1024)
            } else {
                "\u{00A0}".to_string()
            };

            col.push(widget::text::caption(details).width(Length::Fill))
                .into()
        }
    }
}
