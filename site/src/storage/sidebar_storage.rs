use gloo_storage::{LocalStorage, Storage};

const SIDEBAR_OPEN_KEY: &str = "sidebar_open";
const REFERENCES_SIDEBAR_OPEN_KEY: &str = "references_sidebar_open";
const VERSE_VISIBILITY_KEY: &str = "verse_visibility";

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