use leptos::web_sys;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecentChapter {
    pub book_name: String,
    pub chapter: u32,
    pub display_name: String,
    pub path: String,
    pub timestamp: u64, // Unix timestamp
}

const RECENT_CHAPTERS_KEY: &str = "bible_recent_chapters";
const MAX_RECENT_CHAPTERS: usize = 10;

pub fn get_recent_chapters() -> Vec<RecentChapter> {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(stored)) = storage.get_item(RECENT_CHAPTERS_KEY) {
                if let Ok(chapters) = serde_json::from_str::<Vec<RecentChapter>>(&stored) {
                    return chapters;
                }
            }
        }
    }
    Vec::new()
}

pub fn add_recent_chapter(book_name: String, chapter: u32, display_name: String, path: String) {
    let mut recent_chapters = get_recent_chapters();
    
    // Remove if already exists (to avoid duplicates and move to front)
    recent_chapters.retain(|ch| !(ch.book_name == book_name && ch.chapter == chapter));
    
    // Add to front (use simple incrementing timestamp)
    let timestamp = recent_chapters.len() as u64;
    recent_chapters.insert(0, RecentChapter {
        book_name,
        chapter,
        display_name,
        path,
        timestamp,
    });
    
    // Keep only the most recent ones
    recent_chapters.truncate(MAX_RECENT_CHAPTERS);
    
    // Save to localStorage
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(serialized) = serde_json::to_string(&recent_chapters) {
                let _ = storage.set_item(RECENT_CHAPTERS_KEY, &serialized);
            }
        }
    }
}

pub fn clear_recent_chapters() {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item(RECENT_CHAPTERS_KEY);
        }
    }
}