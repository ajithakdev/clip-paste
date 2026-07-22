use super::shared::{cache_thumbnail_handle, prune_thumbnail_handles};
use crate::app::model::HistoryItem;
use crate::app::{AppModel, Message, pinned_history};
use crate::services::clipboard::ClipboardEntry;

use super::super::history;

pub(super) fn handle(app: &mut AppModel, message: Message) -> bool {
    match message {
        Message::ClipboardChanged(entry) => {
            if app
                .history
                .front()
                .is_some_and(|it: &HistoryItem| &it.entry == &entry)
            {
                return true;
            }

            if let ClipboardEntry::Text(text) = &entry {
                if history::should_ignore_clipboard_entry(text) {
                    return true;
                }
            }

            cache_thumbnail_handle(app, &entry);

            let pinned = app
                .history
                .iter()
                .position(|it| &it.entry == &entry)
                .and_then(|idx| app.history.remove(idx))
                .is_some_and(|it| it.pinned);

            history::insert_after_pins(&mut app.history, HistoryItem { entry, pinned });
            history::trim_history(&mut app.history, &app.settings);
            prune_thumbnail_handles(app);
            app.text_overlay_index = None;
            app.recompute_filtered_indices();
            if pinned {
                pinned_history::save(&app.history);
            }
            true
        }
        Message::TogglePin(index) => {
            if history::toggle_pin(&mut app.history, index, &app.settings) {
                pinned_history::save(&app.history);
            }
            app.text_overlay_index = None;
            app.recompute_filtered_indices();
            true
        }
        Message::OpenTextOverlay(index) => {
            if app
                .history
                .get(index)
                .is_some_and(|item| matches!(item.entry, ClipboardEntry::Text(_)))
            {
                app.text_overlay_index = Some(index);
            }
            true
        }
        Message::CloseTextOverlay => {
            app.text_overlay_index = None;
            true
        }
        Message::RemoveHistory(index) => {
            let removed_pinned = app.history.remove(index).is_some_and(|item| item.pinned);
            prune_thumbnail_handles(app);
            app.text_overlay_index = None;
            app.recompute_filtered_indices();
            if removed_pinned {
                pinned_history::save(&app.history);
            }
            true
        }
        Message::ClearHistory => {
            app.history.retain(|item| item.pinned);
            prune_thumbnail_handles(app);
            app.text_overlay_index = None;
            app.recompute_filtered_indices();
            pinned_history::save(&app.history);
            true
        }
        Message::SearchChanged(query) => {
            app.search_query = query;
            app.recompute_filtered_indices();
            app.text_overlay_index = None;
            app.hovered_index = None;
            app.hovered_focus = None;
            app.keyboard_focus = None;
            true
        }
        _ => false,
    }
}
