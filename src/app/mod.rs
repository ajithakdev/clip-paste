// clip-paste — a fork of clippy-land (https://github.com/k33wee/clippy-land)
// Copyright (C) 2026 ajithakdev <ajithkumarks2579@gmail.com>
// Copyright (C) k33wee <https://github.com/k33wee>
// SPDX-License-Identifier: GPL-3.0-or-later
// Modified 2026-07-22: see below.
// Removed layer-surface geometry; startup opens the anchored popup; app-id renamed.

mod handlers;
mod icons;
mod messages;
mod model;
mod pinned_history;
mod surfaces;
mod view;

pub use messages::Message;
pub use model::AppModel;
#[derive(Debug, Clone, Copy, Default)]
pub struct AppFlags {
    pub open_popup_on_start: bool,
}

use crate::app::model::SettingsDraft;
use crate::services::clipboard;
use crate::settings::AppSettings;
use cosmic::iced::{Subscription, window::Id};
use cosmic::prelude::*;
use std::time::{Duration, Instant};

pub(super) fn history_scroll_id() -> cosmic::iced::core::widget::Id {
    cosmic::iced::core::widget::Id::new("history-scroll")
}

pub(super) fn text_overlay_scroll_id() -> cosmic::iced::core::widget::Id {
    cosmic::iced::core::widget::Id::new("text-overlay-scroll")
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;

    type Flags = AppFlags;

    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format
    const APP_ID: &'static str = "io.github.ajithakdev.clip-paste";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(core: cosmic::Core, flags: Self::Flags) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let init_started = Instant::now();

        let stage_started = Instant::now();
        let settings = AppSettings::load().normalized();
        init_timing_log(
            "settings loaded and normalized",
            init_started,
            stage_started.elapsed(),
        );

        let stage_started = Instant::now();
        clipboard::configure_limits(settings.max_image_bytes, settings.max_image_dimension_px);
        init_timing_log(
            "runtime image limits configured",
            init_started,
            stage_started.elapsed(),
        );

        let mut app = AppModel {
            core,
            settings_draft: SettingsDraft::from_settings(&settings),
            settings,
            ..Default::default()
        };

        let stage_started = Instant::now();
        app.history = pinned_history::load(&app.settings);
        init_timing_log(
            "pinned history loaded",
            init_started,
            stage_started.elapsed(),
        );

        let stage_started = Instant::now();
        icons::prewarm_popup_icons();
        init_timing_log(
            "popup icons prewarmed",
            init_started,
            stage_started.elapsed(),
        );

        let stage_started = Instant::now();
        handlers::prewarm_for_first_popup(&mut app);
        init_timing_log(
            "popup thumbnail handles prewarmed",
            init_started,
            stage_started.elapsed(),
        );

        let stage_started = Instant::now();
        app.recompute_filtered_indices();
        init_timing_log(
            "filtered indices recomputed",
            init_started,
            stage_started.elapsed(),
        );

        if flags.open_popup_on_start {
            app.begin_popup_open_trace("startup");
            let task = surfaces::open_anchored_popup(&mut app);

            (app, task)
        } else {
            (app, Task::none())
        }
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    /// Describes the interface based on the current state of the application model
    fn view(&self) -> Element<'_, Self::Message> {
        view::view(self)
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        view::view_window(self, _id)
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        handlers::subscription(self)
    }

    /// Handles messages emitted by the application and its widgets
    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        handlers::update(self, message)
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

fn init_timing_log(label: &'static str, init_started: Instant, stage_elapsed: Duration) {
    if std::env::var_os("CLIP_PASTE_DEBUG_TIMING").is_some() {
        eprintln!(
            "[clip-paste timing] init stage: {label} at {:.2}ms (stage={:.2}ms)",
            duration_ms(init_started.elapsed()),
            duration_ms(stage_elapsed)
        );
    }
}

fn duration_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}
