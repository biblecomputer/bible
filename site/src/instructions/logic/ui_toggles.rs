use leptos::prelude::*;
use crate::utils::is_mobile_screen;
use crate::view_state::ViewStateSignal;

/// Handle toggling the Bible palette
pub fn handle_toggle_bible_palette(
    view_state: ViewStateSignal,
) {
    let is_currently_open = view_state.with(|state| state.is_command_palette_open);
    view_state.update(|state| state.set_command_palette(!is_currently_open));
    // Close sidebar on mobile when command palette opens
    if !is_currently_open && is_mobile_screen() {
        view_state.update(|state| state.set_left_sidebar(false));
    }
}

/// Handle toggling the command palette with ">" prefix
pub fn handle_toggle_command_palette(
    view_state: ViewStateSignal,
) {
    // Open the command palette with ">" pre-filled
    view_state.update(|state| {
        state.set_initial_search_query(Some(">".to_string()));
        state.set_command_palette(true);
    });
    // Close sidebar on mobile when command palette opens
    if is_mobile_screen() {
        view_state.update(|state| state.set_left_sidebar(false));
    }
}

/// Handle toggling the verse palette with ":" prefix
pub fn handle_toggle_verse_palette(
    view_state: ViewStateSignal,
) {
    // Open the command palette with ":" pre-filled
    view_state.update(|state| {
        state.set_initial_search_query(Some(":".to_string()));
        state.set_command_palette(true);
    });
    // Close sidebar on mobile when command palette opens
    if is_mobile_screen() {
        view_state.update(|state| state.set_left_sidebar(false));
    }
}

/// Handle toggling the left sidebar
pub fn handle_toggle_sidebar(view_state: ViewStateSignal) {
    view_state.update(|state| state.toggle_left_sidebar());
}

/// Handle toggling the cross-references sidebar
pub fn handle_toggle_cross_references(view_state: ViewStateSignal) {
    view_state.update(|state| state.toggle_right_sidebar());
}

/// Handle toggling the theme sidebar
pub fn handle_toggle_theme_sidebar(view_state: ViewStateSignal) {
    view_state.update(|state| state.toggle_theme_sidebar());
}

/// Handle toggling the translation comparison panel
pub fn handle_toggle_translation_comparison(view_state: ViewStateSignal) {
    view_state.update(|state| {
        state.toggle_translation_comparison();
        // Close other right-side panels if opening comparison panel
        if state.is_translation_comparison_open {
            state.set_right_sidebar(false);
            state.set_theme_sidebar(false);
        }
    });
}

/// Handle toggling verse visibility
pub fn handle_toggle_verse_visibility(view_state: ViewStateSignal) {
    view_state.update(|state| state.toggle_verse_visibility());
}