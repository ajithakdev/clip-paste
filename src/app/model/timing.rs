use std::time::{Duration, Instant};

use super::state::AppModel;

#[derive(Debug)]
pub(in crate::app) struct PopupOpenTrace {
    pub(in crate::app) source: &'static str,
    pub(in crate::app) started_at: Instant,
    pub(in crate::app) history_len_at_request: usize,
    pub(in crate::app) visible_len_at_request: usize,
    pub(in crate::app) first_view_logged: bool,
    pub(in crate::app) opened_logged: bool,
}

impl AppModel {
    pub(in crate::app) fn popup_open_trace_pending(&self) -> bool {
        self.popup_open_trace.borrow().is_some()
    }

    pub(in crate::app) fn begin_popup_open_trace(&self, source: &'static str) {
        let trace = PopupOpenTrace {
            source,
            started_at: Instant::now(),
            history_len_at_request: self.history.len(),
            visible_len_at_request: self.current_filtered_len(),
            first_view_logged: false,
            opened_logged: false,
        };

        popup_timing_log(format!(
            "popup requested via {}: history={} visible={} search_len={}",
            source,
            trace.history_len_at_request,
            trace.visible_len_at_request,
            self.search_query.len()
        ));

        *self.popup_open_trace.borrow_mut() = Some(trace);
    }

    pub(in crate::app) fn note_popup_view_built(
        &self,
        visible_len_now: usize,
        image_rows_now: usize,
        build_elapsed: Duration,
    ) {
        let mut trace_slot = self.popup_open_trace.borrow_mut();
        let Some(trace) = trace_slot.as_mut() else {
            return;
        };

        if trace.first_view_logged {
            return;
        }

        popup_timing_log(format!(
            "first popup view via {} after {:.2}ms (view_build={:.2}ms, history_at_request={}, visible_at_request={}, visible_now={}, image_rows_now={})",
            trace.source,
            duration_ms(trace.started_at.elapsed()),
            duration_ms(build_elapsed),
            trace.history_len_at_request,
            trace.visible_len_at_request,
            visible_len_now,
            image_rows_now
        ));

        trace.first_view_logged = true;
    }

    pub(in crate::app) fn note_popup_opened(&self) {
        let mut trace_slot = self.popup_open_trace.borrow_mut();
        let Some(trace) = trace_slot.as_mut() else {
            return;
        };

        if trace.opened_logged {
            return;
        }

        popup_timing_log(format!(
            "popup window opened via {} after {:.2}ms",
            trace.source,
            duration_ms(trace.started_at.elapsed())
        ));

        trace.opened_logged = true;
    }

    pub(in crate::app) fn note_popup_stage_marker(&self, label: &'static str) {
        let trace_slot = self.popup_open_trace.borrow();
        let Some(trace) = trace_slot.as_ref() else {
            return;
        };

        popup_timing_log(format!(
            "popup stage via {}: {} at {:.2}ms",
            trace.source,
            label,
            duration_ms(trace.started_at.elapsed())
        ));
    }

    pub(in crate::app) fn finish_popup_open_trace_on_redraw(&self) {
        let Some(trace) = self.popup_open_trace.borrow_mut().take() else {
            return;
        };

        popup_timing_log(format!(
            "first popup redraw via {} after {:.2}ms (opened_logged={}, first_view_logged={})",
            trace.source,
            duration_ms(trace.started_at.elapsed()),
            trace.opened_logged,
            trace.first_view_logged
        ));
    }

    pub(in crate::app) fn cancel_popup_open_trace(&self, reason: &'static str) {
        let Some(trace) = self.popup_open_trace.borrow_mut().take() else {
            return;
        };

        if trace.first_view_logged || trace.opened_logged {
            popup_timing_log(format!(
                "popup trace via {} cleared after {:.2}ms: {}",
                trace.source,
                duration_ms(trace.started_at.elapsed()),
                reason
            ));
            return;
        }

        popup_timing_log(format!(
            "popup timing via {} cancelled after {:.2}ms: {}",
            trace.source,
            duration_ms(trace.started_at.elapsed()),
            reason
        ));
    }

    #[cfg(test)]
    pub(crate) fn popup_open_trace_pending_for_test(&self) -> bool {
        self.popup_open_trace_pending()
    }
}

fn popup_timing_log(message: impl std::fmt::Display) {
    if std::env::var_os("CLIP_PASTE_DEBUG_TIMING").is_some() {
        eprintln!("[clip-paste timing] {message}");
    }
}

fn duration_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}
