use super::*;
use crate::app::model::PopupSurface;

#[test]
fn popup_closed_clears_popup_and_search() {
    let mut app = AppModel::default();
    let id = cosmic::iced::window::Id::unique();
    app.popup = Some(id);
    app.popup_surface = Some(PopupSurface::AnchoredPopup);
    app.popup_controls_ready = true;
    app.search_query = "hello".into();
    app.hovered_index = Some(1);
    app.at_scroll_bottom = true;

    dispatch(&mut app, Message::PopupClosed(id));

    assert!(app.popup.is_none());
    assert!(app.popup_surface.is_none());
    assert!(!app.popup_controls_ready);
    assert!(app.search_query.is_empty());
    assert!(app.hovered_index.is_none());
    assert!(!app.at_scroll_bottom);
}

#[test]
fn popup_closed_ignores_mismatched_id() {
    let mut app = AppModel::default();
    let real_id = cosmic::iced::window::Id::unique();
    let other_id = cosmic::iced::window::Id::unique();
    app.popup = Some(real_id);
    app.popup_surface = Some(PopupSurface::AnchoredPopup);
    app.search_query = "query".into();

    dispatch(&mut app, Message::PopupClosed(other_id));

    assert_eq!(app.popup, Some(real_id));
    assert_eq!(app.popup_surface, Some(PopupSurface::AnchoredPopup));
    assert_eq!(app.search_query, "query");
}

#[test]
fn popup_open_trace_starts_on_open_and_clears_after_first_view() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("row", false));
    app.recompute_filtered_indices();
    let popup_id = cosmic::iced::window::Id::unique();
    app.popup = Some(popup_id);

    app.begin_popup_open_trace("test");
    assert!(app.popup_open_trace_pending_for_test());

    let _ = view::view_window(&app, popup_id);
    assert!(app.popup_open_trace_pending_for_test());

    dispatch(&mut app, Message::PopupRedraw(popup_id));
    assert!(!app.popup_open_trace_pending_for_test());
}

#[test]
fn popup_controls_are_deferred_until_popup_opened() {
    let mut app = AppModel::default();

    dispatch(&mut app, Message::TogglePopup);

    let popup_id = app.popup.expect("popup should be opened by toggle");
    assert!(!app.popup_controls_ready);

    dispatch(&mut app, Message::PopupOpened(popup_id));

    assert!(app.popup_controls_ready);
}

#[test]
fn popup_controls_ignore_open_for_other_window() {
    let mut app = AppModel::default();

    dispatch(&mut app, Message::TogglePopup);

    let popup_id = app.popup.expect("popup should be opened by toggle");
    let other_id = cosmic::iced::window::Id::unique();
    assert_ne!(popup_id, other_id);

    dispatch(&mut app, Message::PopupOpened(other_id));

    assert!(!app.popup_controls_ready);
}

#[test]
fn popup_closed_cancels_pending_popup_open_trace() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("row", false));
    app.recompute_filtered_indices();
    let popup_id = cosmic::iced::window::Id::unique();
    app.popup = Some(popup_id);
    app.popup_surface = Some(PopupSurface::AnchoredPopup);

    app.begin_popup_open_trace("test");
    assert!(app.popup_open_trace_pending_for_test());

    dispatch(&mut app, Message::PopupClosed(popup_id));
    assert!(!app.popup_open_trace_pending_for_test());
}

#[test]
fn toggle_popup_opens_anchored_popup_and_starts_trace() {
    let mut app = AppModel::default();

    dispatch(&mut app, Message::TogglePopup);

    assert!(app.popup.is_some());
    assert_eq!(app.popup_surface, Some(PopupSurface::AnchoredPopup));
    assert!(!app.popup_controls_ready);
    assert!(app.popup_open_trace_pending_for_test());
}

#[test]
fn prewarm_for_first_popup_caches_existing_image_thumbnails() {
    let mut app = AppModel::default();
    app.history.push_back(HistoryItem {
        entry: image_entry(17),
        pinned: false,
    });

    prewarm_for_first_popup(&mut app);

    assert_eq!(app.thumbnail_handles.len(), 1);
}

#[test]
fn window_unfocused_leaves_anchored_popup_untouched() {
    // The layer surface was removed; the anchored icon-click popup manages its
    // own dismiss-on-focus-loss via the compositor, so WindowUnfocused is a no-op.
    let mut app = AppModel::default();
    let id = cosmic::iced::window::Id::unique();
    app.popup = Some(id);
    app.popup_surface = Some(PopupSurface::AnchoredPopup);
    app.search_query = "query".into();

    dispatch(&mut app, Message::WindowUnfocused(id));

    assert_eq!(app.popup, Some(id));
    assert_eq!(app.popup_surface, Some(PopupSurface::AnchoredPopup));
    assert_eq!(app.search_query, "query");
}
