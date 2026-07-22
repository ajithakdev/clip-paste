use crate::app::{AppModel, Message};
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::widget;

pub(super) fn settings_panel(app: &AppModel) -> Element<'_, Message> {
    let mut col = widget::column::Column::new().spacing(8).width(Length::Fill);

    col = col
        .push(widget::text::heading("Settings"))
        .push(widget::text::caption("History limits"));

    let history_max = widget::column::Column::new()
        .spacing(4)
        .push(widget::text::body("Max history entries"))
        .push(
            widget::text_input("e.g. 200", &app.settings_draft.max_history)
                .on_input(Message::SettingsMaxHistoryChanged)
                .width(Length::Fill),
        )
        .push(widget::text::caption("Allowed range: 30–5000"));

    let pinned_max = widget::column::Column::new()
        .spacing(4)
        .push(widget::text::body("Max pinned entries"))
        .push(
            widget::text_input("e.g. 20", &app.settings_draft.max_pinned)
                .on_input(Message::SettingsMaxPinnedChanged)
                .width(Length::Fill),
        )
        .push(widget::text::caption(
            "Allowed range: 0–500 (and ≤ max history)",
        ));

    col = col.push(
        widget::row::Row::new()
            .spacing(8)
            .push(history_max)
            .push(pinned_max),
    );

    col = col.push(widget::divider::horizontal::light());
    col = col.push(widget::text::caption("Image limits"));

    let image_bytes = widget::column::Column::new()
        .spacing(4)
        .push(widget::text::body("Max image size (bytes)"))
        .push(
            widget::text_input("e.g. 8388608", &app.settings_draft.max_image_bytes)
                .on_input(Message::SettingsMaxImageBytesChanged)
                .width(Length::Fill),
        )
        .push(widget::text::caption("Allowed range: 262144–67108864"));

    let image_dimension = widget::column::Column::new()
        .spacing(4)
        .push(widget::text::body("Max image dimension (px)"))
        .push(
            widget::text_input("e.g. 8192", &app.settings_draft.max_image_dimension_px)
                .on_input(Message::SettingsMaxImageDimensionChanged)
                .width(Length::Fill),
        )
        .push(widget::text::caption("Allowed range: 512–16384"));

    col = col.push(
        widget::row::Row::new()
            .spacing(8)
            .push(image_bytes)
            .push(image_dimension),
    );

    col = col.push(widget::divider::horizontal::light());
    col = col.push(widget::text::caption("Behavior"));
    col = col.push(
        widget::row::Row::new()
            .spacing(8)
            .align_y(Alignment::Center)
            .push(widget::text::body("Paste on select").width(Length::Fill))
            .push(
                widget::toggler(app.settings_draft.paste_on_select)
                    .on_toggle(Message::SettingsPasteOnSelectToggled),
            ),
    );
    col = col.push(widget::text::caption(
        "Auto-paste (Ctrl+V) into the focused window after selecting. Requires wtype.",
    ));

    if let Some(err) = &app.settings_error {
        col = col.push(widget::text::body(err));
    }

    let apply_row = widget::row::Row::new()
        .width(Length::Fill)
        .push(widget::button::suggested("Apply").on_press(Message::ApplySettings));

    col = col.push(apply_row);

    widget::container(col)
        .class(cosmic::theme::Container::Card)
        .padding([8, 12])
        .width(Length::Fill)
        .into()
}
