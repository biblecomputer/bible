use gloo_storage::{LocalStorage, Storage};

const SIDEBAR_OPEN_KEY: &str = "sidebar_open";
const REFERENCES_SIDEBAR_OPEN_KEY: &str = "references_sidebar_open";

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