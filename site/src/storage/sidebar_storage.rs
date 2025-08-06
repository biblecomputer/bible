use gloo_storage::{LocalStorage, Storage};

const SIDEBAR_OPEN_KEY: &str = "sidebar_open";
const REFERENCES_SIDEBAR_OPEN_KEY: &str = "references_sidebar_open";
const VERSE_VISIBILITY_KEY: &str = "verse_visibility";
const SELECTED_THEME_KEY: &str = "selected_theme";

pub fn get_sidebar_open() -> bool {
    LocalStorage::get(SIDEBAR_OPEN_KEY).unwrap_or(true)
}

pub fn save_sidebar_open(open: bool) {
    let _ = LocalStorage::set(SIDEBAR_OPEN_KEY, open);
}

pub fn get_references_sidebar_open() -> bool {
    LocalStorage::get(REFERENCES_SIDEBAR_OPEN_KEY).unwrap_or(false)
}

pub fn save_references_sidebar_open(open: bool) {
    let _ = LocalStorage::set(REFERENCES_SIDEBAR_OPEN_KEY, open);
}

pub fn get_verse_visibility() -> bool {
    LocalStorage::get(VERSE_VISIBILITY_KEY).unwrap_or(true)
}

pub fn save_verse_visibility(visible: bool) {
    let _ = LocalStorage::set(VERSE_VISIBILITY_KEY, visible);
}

pub fn get_selected_theme() -> String {
    LocalStorage::get(SELECTED_THEME_KEY).unwrap_or_else(|_| "light".to_string())
}

pub fn save_selected_theme(theme_id: &str) {
    let _ = LocalStorage::set(SELECTED_THEME_KEY, theme_id);
}