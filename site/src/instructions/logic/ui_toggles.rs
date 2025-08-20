use leptos::prelude::*;
use crate::storage::{save_references_sidebar_open, save_sidebar_open, save_verse_visibility};
use crate::utils::is_mobile_screen;

/// Handle toggling the Bible palette
pub fn handle_toggle_bible_palette(
    palette_open: ReadSignal<bool>,
    set_palette_open: WriteSignal<bool>,
    set_left_sidebar_open: WriteSignal<bool>,
) {
    let is_currently_open = palette_open.get();
    set_palette_open.set(!is_currently_open);
    // Close sidebar on mobile when command palette opens
    if !is_currently_open && is_mobile_screen() {
        set_left_sidebar_open.set(false);
        save_sidebar_open(false);
    }
}

/// Handle toggling the command palette with ">" prefix
pub fn handle_toggle_command_palette(
    set_palette_open: WriteSignal<bool>,
    set_initial_search_query: WriteSignal<Option<String>>,
    set_left_sidebar_open: WriteSignal<bool>,
) {
    // Open the command palette with ">" pre-filled
    set_initial_search_query.set(Some(">".to_string()));
    set_palette_open.set(true);
    // Close sidebar on mobile when command palette opens
    if is_mobile_screen() {
        set_left_sidebar_open.set(false);
        save_sidebar_open(false);
    }
}

/// Handle toggling the verse palette with ":" prefix
pub fn handle_toggle_verse_palette(
    set_palette_open: WriteSignal<bool>,
    set_initial_search_query: WriteSignal<Option<String>>,
    set_left_sidebar_open: WriteSignal<bool>,
) {
    // Open the command palette with ":" pre-filled
    set_initial_search_query.set(Some(":".to_string()));
    set_palette_open.set(true);
    // Close sidebar on mobile when command palette opens
    if is_mobile_screen() {
        set_left_sidebar_open.set(false);
        save_sidebar_open(false);
    }
}

/// Handle toggling the left sidebar
pub fn handle_toggle_sidebar(set_left_sidebar_open: WriteSignal<bool>) {
    set_left_sidebar_open.update(|open| {
        *open = !*open;
        save_sidebar_open(*open);
    });
}

/// Handle toggling the cross-references sidebar
pub fn handle_toggle_cross_references(
    set_right_sidebar_open: WriteSignal<bool>,
    set_theme_sidebar_open: WriteSignal<bool>,
) {
    set_right_sidebar_open.update(|open| {
        *open = !*open;
        save_references_sidebar_open(*open);
        // Close theme sidebar if opening references sidebar
        if *open {
            set_theme_sidebar_open.set(false);
        }
    });
}

/// Handle toggling the theme sidebar
pub fn handle_toggle_theme_sidebar(
    set_theme_sidebar_open: WriteSignal<bool>,
    set_right_sidebar_open: WriteSignal<bool>,
) {
    set_theme_sidebar_open.update(|open| {
        *open = !*open;
        // Close references sidebar if opening theme sidebar
        if *open {
            set_right_sidebar_open.set(false);
            save_references_sidebar_open(false);
        }
    });
}

/// Handle toggling the translation comparison panel
pub fn handle_toggle_translation_comparison(
    set_translation_comparison_open: WriteSignal<bool>,
    set_right_sidebar_open: WriteSignal<bool>,
    set_theme_sidebar_open: WriteSignal<bool>,
) {
    set_translation_comparison_open.update(|open| {
        *open = !*open;
        // Close other right-side panels if opening comparison panel
        if *open {
            set_right_sidebar_open.set(false);
            set_theme_sidebar_open.set(false);
            save_references_sidebar_open(false);
        }
    });
}

/// Handle toggling verse visibility
pub fn handle_toggle_verse_visibility(set_verse_visibility_enabled: WriteSignal<bool>) {
    set_verse_visibility_enabled.update(|visible| {
        *visible = !*visible;
        save_verse_visibility(*visible);
    });
}