use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use rexie::{ObjectStore, Rexie, TransactionMode};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use urlencoding::{decode, encode};

// Constants
const MOBILE_BREAKPOINT: f64 = 768.0;

// Global static Bible instance - fetched from API once, used everywhere
pub static BIBLE: OnceLock<Bible> = OnceLock::new();
static CURRENT_BIBLE_SIGNAL: OnceLock<RwSignal<Option<Bible>>> = OnceLock::new();

// Helper function to check if screen is mobile-sized
pub fn is_mobile_screen() -> bool {
    if let Some(window) = leptos::web_sys::window() {
        if let Ok(width) = window.inner_width() {
            if let Some(width_num) = width.as_f64() {
                return width_num < MOBILE_BREAKPOINT;
            }
        }
    }
    false
}

// Function to initialize the Bible data
pub async fn init_bible() -> std::result::Result<(), Box<dyn std::error::Error>> {
    if BIBLE.get().is_some() {
        return Ok(());
    }

    let bible = load_or_fetch_bible().await?;

    // Set both the static and signal versions
    BIBLE
        .set(bible.clone())
        .map_err(|_| "Failed to set Bible data")?;
    let bible_signal = init_bible_signal();
    bible_signal.set(Some(bible));

    Ok(())
}

pub fn init_bible_signal() -> RwSignal<Option<Bible>> {
    *CURRENT_BIBLE_SIGNAL.get_or_init(|| RwSignal::new(None))
}

// Function to switch to a different translation
pub async fn switch_bible_translation(
    translation_short_name: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let bible = if is_translation_downloaded(translation_short_name) {
        load_downloaded_translation(translation_short_name).await?
    } else {
        return Err("Translation not downloaded".into());
    };

    // Update the signal
    let bible_signal = init_bible_signal();
    bible_signal.set(Some(bible));

    Ok(())
}

// Consolidated function to load from cache or fetch from API
async fn load_or_fetch_bible() -> std::result::Result<Bible, Box<dyn std::error::Error>> {
    // Check if user has a selected translation that's downloaded
    if let Some(selected_translation) = get_selected_translation() {
        if is_translation_downloaded(&selected_translation) {
            if let Ok(bible) = load_downloaded_translation(&selected_translation).await {
                return Ok(bible);
            }
        }
    }

    // Fall back to default behavior (Staten vertaling)
    // Try to load from browser cache first
    match load_bible_from_cache().await {
        Ok(cached_bible) => return Ok(cached_bible),
        Err(_) => {
            // If not in cache, fetch from API
            let bible = fetch_bible_from_api().await?;

            // Save to cache for future use (ignore errors)
            let _ = save_bible_to_cache(&bible).await;

            Ok(bible)
        }
    }
}

// Load Bible data from IndexedDB
async fn load_bible_from_cache() -> std::result::Result<Bible, Box<dyn std::error::Error>> {
    const CACHE_VERSION: &str = "v1";

    // Open IndexedDB
    let rexie = Rexie::builder("BibleCache")
        .version(1)
        .add_object_store(ObjectStore::new("bible_data"))
        .build()
        .await
        .map_err(|e| format!("Failed to open IndexedDB: {:?}", e))?;

    let transaction = rexie
        .transaction(&["bible_data"], TransactionMode::ReadOnly)
        .map_err(|e| format!("Failed to create transaction: {:?}", e))?;
    let store = transaction
        .store("bible_data")
        .map_err(|e| format!("Failed to get store: {:?}", e))?;

    // Check cache version first
    let version_result = store.get("cache_version".into()).await;
    match version_result {
        Ok(Some(version_value)) => {
            if let Some(version_str) = version_value.as_string() {
                if version_str != CACHE_VERSION {
                    // Version mismatch, clear cache
                    drop(transaction);
                    clear_bible_cache()
                        .await
                        .map_err(|e| format!("Failed to clear cache: {:?}", e))?;
                    return Err("Cache version mismatch".into());
                }
            } else {
                return Err("Invalid cache version format".into());
            }
        }
        Ok(None) => return Err("No cache version found".into()),
        Err(_) => return Err("Failed to read cache version".into()),
    }

    // Get the cached Bible data
    let data_result = store.get("bible_json".into()).await;
    match data_result {
        Ok(Some(data_value)) => {
            if let Some(json_str) = data_value.as_string() {
                let bible: Bible = serde_json::from_str(&json_str)
                    .map_err(|e| format!("Failed to parse cached data: {:?}", e))?;
                Ok(bible)
            } else {
                Err("Invalid cached data format".into())
            }
        }
        Ok(None) => Err("No cached Bible data found".into()),
        Err(_) => Err("Failed to read cached data".into()),
    }
}

// Save Bible data to IndexedDB
async fn save_bible_to_cache(bible: &Bible) -> std::result::Result<(), Box<dyn std::error::Error>> {
    const CACHE_VERSION: &str = "v1";

    // Open IndexedDB
    let rexie = Rexie::builder("BibleCache")
        .version(1)
        .add_object_store(ObjectStore::new("bible_data"))
        .build()
        .await
        .map_err(|e| format!("Failed to open IndexedDB: {:?}", e))?;

    let transaction = rexie
        .transaction(&["bible_data"], TransactionMode::ReadWrite)
        .map_err(|e| format!("Failed to create transaction: {:?}", e))?;
    let store = transaction
        .store("bible_data")
        .map_err(|e| format!("Failed to get store: {:?}", e))?;

    // Serialize the Bible data
    let json_data = serde_json::to_string(bible)
        .map_err(|e| format!("Failed to serialize Bible data: {:?}", e))?;

    // Save both the data and version
    store
        .put(&json_data.into(), Some(&"bible_json".into()))
        .await
        .map_err(|e| format!("Failed to save Bible data: {:?}", e))?;
    store
        .put(&CACHE_VERSION.into(), Some(&"cache_version".into()))
        .await
        .map_err(|e| format!("Failed to save cache version: {:?}", e))?;

    transaction
        .commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {:?}", e))?;
    Ok(())
}

// Clear Bible data from cache (useful for debugging or forcing refresh)
#[allow(dead_code)]
pub async fn clear_bible_cache() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Open IndexedDB
    let rexie = Rexie::builder("BibleCache")
        .version(1)
        .add_object_store(ObjectStore::new("bible_data"))
        .build()
        .await
        .map_err(|e| format!("Failed to open IndexedDB: {:?}", e))?;

    let transaction = rexie
        .transaction(&["bible_data"], TransactionMode::ReadWrite)
        .map_err(|e| format!("Failed to create transaction: {:?}", e))?;
    let store = transaction
        .store("bible_data")
        .map_err(|e| format!("Failed to get store: {:?}", e))?;

    // Delete both the data and version
    store
        .delete("bible_json".into())
        .await
        .map_err(|e| format!("Failed to delete Bible data: {:?}", e))?;
    store
        .delete("cache_version".into())
        .await
        .map_err(|e| format!("Failed to delete cache version: {:?}", e))?;

    transaction
        .commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {:?}", e))?;
    Ok(())
}

// Fetch Bible data from API with fallback proxies
async fn fetch_bible_from_api() -> std::result::Result<Bible, Box<dyn std::error::Error>> {
    // Try multiple CORS proxy services in case one is down
    let proxy_urls = [
        "https://corsproxy.io/?https://gw.iagon.com/api/v2/storage/shareable/link/Njg2ZDFjNDgwOGQ0M2UzNTUyNTdhYmRh:MTJjOTRlYTBmNzM2YWZiZDE2NzdkMzU3NzA3MjBmMTRmZGZkMWYzNWVkYWVlNTU1Y2RjYTA1NzYzZmE1YmEzNA",
        "https://api.allorigins.win/get?url=https://gw.iagon.com/api/v2/storage/shareable/link/Njg2ZDFjNDgwOGQ0M2UzNTUyNTdhYmRh:MTJjOTRlYTBmNzM2YWZiZDE2NzdkMzU3NzA3MjBmMTRmZGZkMWYzNWVkYWVlNTU1Y2RjYTA1NzYzZmE1YmEzNA",
    ];

    let mut last_error = None;

    for proxy_url in &proxy_urls {
        match try_fetch_bible(proxy_url).await {
            Ok(bible) => return Ok(bible),
            Err(e) => {
                last_error = Some(e);
                continue;
            }
        }
    }

    Err(last_error.unwrap_or_else(|| "All proxy attempts failed".into()))
}

async fn try_fetch_bible(url: &str) -> std::result::Result<Bible, Box<dyn std::error::Error>> {
    let response = Request::get(url).send().await?;

    let json_string = if url.contains("allorigins.win") {
        // allorigins.win wraps the response in a JSON object
        let wrapped: serde_json::Value = response.json().await?;
        wrapped["contents"]
            .as_str()
            .ok_or("Failed to extract contents from allorigins response")?
            .to_string()
    } else {
        response.text().await?
    };

    let bible: Bible = serde_json::from_str(&json_string)?;
    Ok(bible)
}

// Helper function to get Bible data (panics if not initialized)
pub fn get_bible() -> &'static Bible {
    BIBLE
        .get()
        .expect("Bible not initialized - call init_bible() first")
}

// Helper function to get the current Bible from signal (for reactivity)
pub fn get_current_bible() -> Option<Bible> {
    let bible_signal = init_bible_signal();
    bible_signal.get()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bible {
    pub books: Vec<Book>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Book {
    pub name: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Chapter {
    pub chapter: u32,
    pub name: String,
    pub verses: Vec<Verse>,
}

impl Chapter {
    pub fn to_path(&self) -> String {
        // Extract book name by removing the chapter number from the end
        // Format is "Book Name X" where X is the chapter number
        let name_parts: Vec<&str> = self.name.split_whitespace().collect();

        // Remove the last part (chapter number) to get book name
        let book_name = if name_parts.len() > 1 {
            name_parts[..name_parts.len() - 1].join(" ")
        } else {
            self.name.clone()
        };

        let encoded_book = encode(&book_name);
        format!("/{}/{}", encoded_book, self.chapter)
    }

    pub fn from_url() -> std::result::Result<Self, ParamParseError> {
        let params = move || use_params_map();
        let book = move || params().read().get("book").unwrap();
        let chapter = move || {
            params()
                .read()
                .get("chapter")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(1) // fallback chapter number if parsing fails
        };

        let chapter: Chapter = get_bible().get_chapter(&book(), chapter()).unwrap();
        Ok(chapter)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Verse {
    pub verse: u32,
    pub chapter: u32,
    pub name: String,
    pub text: String,
}

#[derive(Debug)]
pub enum ParamParseError {
    ChapterNotFound,
    BookNotFound,
}

impl Bible {
    pub fn get_chapter(
        &self,
        book: &str,
        chapter: u32,
    ) -> std::result::Result<Chapter, ParamParseError> {
        // Decode URL-encoded book name back to original name with special characters
        let book_name = decode(book)
            .map_err(|_| ParamParseError::BookNotFound)?
            .into_owned();

        let book = self
            .books
            .iter()
            .find(|b| b.name.to_lowercase() == book_name.to_lowercase())
            .ok_or(ParamParseError::BookNotFound)?;

        let chapter = book
            .chapters
            .iter()
            .find(|c| c.chapter == chapter)
            .ok_or(ParamParseError::ChapterNotFound)?;

        Ok(chapter.clone())
    }

    pub fn get_next_chapter(&self, current: &Chapter) -> Option<Chapter> {
        // Find the current book and chapter
        for (book_idx, book) in self.books.iter().enumerate() {
            if let Some(chapter_idx) = book
                .chapters
                .iter()
                .position(|c| c.chapter == current.chapter && c.name == current.name)
            {
                // Try next chapter in same book
                if chapter_idx + 1 < book.chapters.len() {
                    return Some(book.chapters[chapter_idx + 1].clone());
                }
                // Try first chapter of next book
                if book_idx + 1 < self.books.len() {
                    return self.books[book_idx + 1].chapters.first().cloned();
                }
                // No next chapter
                return None;
            }
        }
        None
    }

    pub fn get_previous_chapter(&self, current: &Chapter) -> Option<Chapter> {
        // Find the current book and chapter
        for (book_idx, book) in self.books.iter().enumerate() {
            if let Some(chapter_idx) = book
                .chapters
                .iter()
                .position(|c| c.chapter == current.chapter && c.name == current.name)
            {
                // Try previous chapter in same book
                if chapter_idx > 0 {
                    return Some(book.chapters[chapter_idx - 1].clone());
                }
                // Try last chapter of previous book
                if book_idx > 0 {
                    return self.books[book_idx - 1].chapters.last().cloned();
                }
                // No previous chapter
                return None;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_chapter_to_path_roundtrip(
            chapter_num in 1u32..150,
            book_name in "[a-zA-Z ]{1,50}"
        ) {
            let chapter = Chapter {
                chapter: chapter_num,
                name: format!("{} {}", book_name.trim(), chapter_num),
                verses: vec![],
            };

            let path = chapter.to_path();

            // Path should start with encoded book name and end with chapter number
            prop_assert!(path.starts_with("/"));
            let expected_suffix = format!("/{}", chapter_num);
            prop_assert!(path.ends_with(&expected_suffix));
        }

        #[test]
        fn test_chapter_to_path_handles_special_chars(
            chapter_num in 1u32..150,
            book_name in "[a-zA-Z0-9 àáâãäéêëíîïóôõöúûüç]{1,30}"
        ) {
            let chapter = Chapter {
                chapter: chapter_num,
                name: format!("{} {}", book_name.trim(), chapter_num),
                verses: vec![],
            };

            let path = chapter.to_path();

            // Path should be URL-safe
            prop_assert!(path.starts_with("/"));
            prop_assert!(!path.contains(" "));
            let expected_suffix = format!("/{}", chapter_num);
            prop_assert!(path.ends_with(&expected_suffix));
        }

        #[test]
        fn test_get_chapter_book_case_insensitive(
            chapter_num in 1u32..10,
            book_name in "[a-zA-Z]{3,20}"
        ) {
            // Create a mock Bible with the test book
            let test_chapter = Chapter {
                chapter: chapter_num,
                name: format!("{} {}", book_name, chapter_num),
                verses: vec![],
            };

            let test_book = Book {
                name: book_name.clone(),
                chapters: vec![test_chapter.clone()],
            };

            let bible = Bible {
                books: vec![test_book],
            };

            // Test case insensitive matching
            let upper_result = bible.get_chapter(&book_name.to_uppercase(), chapter_num);
            let lower_result = bible.get_chapter(&book_name.to_lowercase(), chapter_num);

            prop_assert!(upper_result.is_ok());
            prop_assert!(lower_result.is_ok());

            if let (Ok(upper_chapter), Ok(lower_chapter)) = (upper_result, lower_result) {
                prop_assert_eq!(upper_chapter.chapter, chapter_num);
                prop_assert_eq!(lower_chapter.chapter, chapter_num);
            }
        }

        #[test]
        fn test_get_chapter_url_decoding(
            chapter_num in 1u32..10,
            book_name in "[a-zA-Z ]{3,20}"
        ) {
            let clean_book_name = book_name.trim();
            let test_chapter = Chapter {
                chapter: chapter_num,
                name: format!("{} {}", clean_book_name, chapter_num),
                verses: vec![],
            };

            let test_book = Book {
                name: clean_book_name.to_string(),
                chapters: vec![test_chapter.clone()],
            };

            let bible = Bible {
                books: vec![test_book],
            };

            // Test URL-encoded book name
            let encoded_book = urlencoding::encode(clean_book_name);
            let result = bible.get_chapter(&encoded_book, chapter_num);

            prop_assert!(result.is_ok());
            if let Ok(found_chapter) = result {
                prop_assert_eq!(found_chapter.chapter, chapter_num);
            }
        }

        #[test]
        fn test_navigation_consistency(
            num_chapters in 2usize..20,
            start_chapter in 1u32..10
        ) {
            // Create a book with multiple chapters
            let chapters: Vec<Chapter> = (start_chapter..start_chapter + num_chapters as u32)
                .map(|i| Chapter {
                    chapter: i,
                    name: format!("Test Book {}", i),
                    verses: vec![],
                })
                .collect();

            let book = Book {
                name: "Test Book".to_string(),
                chapters,
            };

            let bible = Bible {
                books: vec![book],
            };

            // Test that next/previous navigation is consistent
            for i in 1..num_chapters - 1 {
                let current_chapter = &bible.books[0].chapters[i];

                if let Some(next_chapter) = bible.get_next_chapter(current_chapter) {
                    if let Some(prev_of_next) = bible.get_previous_chapter(&next_chapter) {
                        prop_assert_eq!(prev_of_next.chapter, current_chapter.chapter);
                        prop_assert_eq!(prev_of_next.name, current_chapter.name.clone());
                    }
                }
            }
        }

        #[test]
        fn test_navigation_boundaries(
            num_chapters in 1usize..10,
            start_chapter in 1u32..5
        ) {
            // Create a book with chapters
            let chapters: Vec<Chapter> = (start_chapter..start_chapter + num_chapters as u32)
                .map(|i| Chapter {
                    chapter: i,
                    name: format!("Test Book {}", i),
                    verses: vec![],
                })
                .collect();

            let book = Book {
                name: "Test Book".to_string(),
                chapters,
            };

            let bible = Bible {
                books: vec![book],
            };

            // First chapter should have no previous
            let first_chapter = &bible.books[0].chapters[0];
            prop_assert!(bible.get_previous_chapter(first_chapter).is_none());

            // Last chapter should have no next
            let last_chapter = &bible.books[0].chapters[num_chapters - 1];
            prop_assert!(bible.get_next_chapter(last_chapter).is_none());
        }

        #[test]
        fn test_cross_book_navigation(
            num_books in 2usize..5,
            chapters_per_book in 1usize..5
        ) {
            // Create multiple books
            let books: Vec<Book> = (0..num_books)
                .map(|book_idx| {
                    let chapters: Vec<Chapter> = (1..=chapters_per_book)
                        .map(|chapter_idx| Chapter {
                            chapter: chapter_idx as u32,
                            name: format!("Book {} Chapter {}", book_idx, chapter_idx),
                            verses: vec![],
                        })
                        .collect();

                    Book {
                        name: format!("Book {}", book_idx),
                        chapters,
                    }
                })
                .collect();

            let bible = Bible { books };

            // Test navigation from last chapter of first book to first chapter of second book
            let last_chapter_book1 = &bible.books[0].chapters[chapters_per_book - 1];
            let first_chapter_book2 = &bible.books[1].chapters[0];

            if let Some(next_chapter) = bible.get_next_chapter(last_chapter_book1) {
                prop_assert_eq!(next_chapter.chapter, first_chapter_book2.chapter);
                prop_assert_eq!(next_chapter.name, first_chapter_book2.name.clone());
            }

            // Test navigation from first chapter of second book to last chapter of first book
            if let Some(prev_chapter) = bible.get_previous_chapter(first_chapter_book2) {
                prop_assert_eq!(prev_chapter.chapter, last_chapter_book1.chapter);
                prop_assert_eq!(prev_chapter.name, last_chapter_book1.name.clone());
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BibleTranslation {
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub release_year: u16,
    pub iagon: String,
    pub languages: Vec<Language>,
    pub wikipedia: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Language {
    Dutch,
    English,
}

const SELECTED_TRANSLATION_KEY: &str = "selected_translation";
const DOWNLOADED_TRANSLATIONS_KEY: &str = "downloaded_translations";

pub fn get_selected_translation() -> Option<String> {
    LocalStorage::get(SELECTED_TRANSLATION_KEY).ok()
}

pub fn set_selected_translation(
    translation_short_name: &str,
) -> Result<(), gloo_storage::errors::StorageError> {
    LocalStorage::set(SELECTED_TRANSLATION_KEY, translation_short_name)
}

pub fn get_downloaded_translations() -> Vec<String> {
    LocalStorage::get::<Vec<String>>(DOWNLOADED_TRANSLATIONS_KEY).unwrap_or_default()
}

pub fn add_downloaded_translation(
    translation_short_name: &str,
) -> Result<(), gloo_storage::errors::StorageError> {
    let mut downloaded = get_downloaded_translations();
    if !downloaded.contains(&translation_short_name.to_string()) {
        downloaded.push(translation_short_name.to_string());
        LocalStorage::set(DOWNLOADED_TRANSLATIONS_KEY, &downloaded)?;
    }
    Ok(())
}

pub fn is_translation_downloaded(translation_short_name: &str) -> bool {
    get_downloaded_translations().contains(&translation_short_name.to_string())
}

pub fn remove_downloaded_translation(
    translation_short_name: &str,
) -> Result<(), gloo_storage::errors::StorageError> {
    let mut downloaded = get_downloaded_translations();
    downloaded.retain(|name| name != translation_short_name);
    LocalStorage::set(DOWNLOADED_TRANSLATIONS_KEY, &downloaded)
}

pub async fn uninstall_translation(
    translation_short_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Remove from downloaded translations list
    remove_downloaded_translation(translation_short_name)?;

    // Remove from IndexedDB cache
    let translation_cache_key = format!("translation_{}", translation_short_name);
    remove_translation_from_cache(&translation_cache_key).await?;

    // If this was the selected translation, reset to default
    if let Some(selected) = get_selected_translation() {
        if selected == translation_short_name {
            // Reset to Staten vertaling (sv) as default
            let _ = set_selected_translation("sv");
        }
    }

    Ok(())
}

pub async fn download_translation(
    translation: &BibleTranslation,
) -> Result<Bible, Box<dyn std::error::Error>> {
    let bible = fetch_translation_from_url(&translation.iagon).await?;

    let translation_cache_key = format!("translation_{}", translation.short_name);
    save_translation_to_cache(&translation_cache_key, &bible).await?;

    add_downloaded_translation(&translation.short_name)?;

    Ok(bible)
}

pub async fn load_downloaded_translation(
    translation_short_name: &str,
) -> Result<Bible, Box<dyn std::error::Error>> {
    let translation_cache_key = format!("translation_{}", translation_short_name);
    load_translation_from_cache(&translation_cache_key).await
}

async fn fetch_translation_from_url(url: &str) -> Result<Bible, Box<dyn std::error::Error>> {
    let proxy_urls = [
        format!("https://corsproxy.io/?{}", url),
        format!("https://api.allorigins.win/get?url={}", url),
    ];

    let mut last_error = None;

    for proxy_url in &proxy_urls {
        match try_fetch_bible(proxy_url).await {
            Ok(bible) => return Ok(bible),
            Err(e) => {
                last_error = Some(e);
                continue;
            }
        }
    }

    Err(last_error.unwrap_or_else(|| "All proxy attempts failed".into()))
}

async fn save_translation_to_cache(
    cache_key: &str,
    bible: &Bible,
) -> Result<(), Box<dyn std::error::Error>> {
    let rexie = Rexie::builder("TranslationCache")
        .version(1)
        .add_object_store(ObjectStore::new("translations"))
        .build()
        .await
        .map_err(|e| format!("Failed to open IndexedDB: {:?}", e))?;

    let transaction = rexie
        .transaction(&["translations"], TransactionMode::ReadWrite)
        .map_err(|e| format!("Failed to create transaction: {:?}", e))?;
    let store = transaction
        .store("translations")
        .map_err(|e| format!("Failed to get store: {:?}", e))?;

    let json_data = serde_json::to_string(bible)
        .map_err(|e| format!("Failed to serialize Bible data: {:?}", e))?;

    store
        .put(&json_data.into(), Some(&cache_key.into()))
        .await
        .map_err(|e| format!("Failed to save translation data: {:?}", e))?;

    transaction
        .commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {:?}", e))?;
    Ok(())
}

async fn load_translation_from_cache(cache_key: &str) -> Result<Bible, Box<dyn std::error::Error>> {
    let rexie = Rexie::builder("TranslationCache")
        .version(1)
        .add_object_store(ObjectStore::new("translations"))
        .build()
        .await
        .map_err(|e| format!("Failed to open IndexedDB: {:?}", e))?;

    let transaction = rexie
        .transaction(&["translations"], TransactionMode::ReadOnly)
        .map_err(|e| format!("Failed to create transaction: {:?}", e))?;
    let store = transaction
        .store("translations")
        .map_err(|e| format!("Failed to get store: {:?}", e))?;

    let data_result = store.get(cache_key.into()).await;
    match data_result {
        Ok(Some(data_value)) => {
            if let Some(json_str) = data_value.as_string() {
                let bible: Bible = serde_json::from_str(&json_str)
                    .map_err(|e| format!("Failed to parse cached translation: {:?}", e))?;
                Ok(bible)
            } else {
                Err("Invalid cached translation format".into())
            }
        }
        Ok(None) => Err("Translation not found in cache".into()),
        Err(_) => Err("Failed to read cached translation".into()),
    }
}

async fn remove_translation_from_cache(cache_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let rexie = Rexie::builder("TranslationCache")
        .version(1)
        .add_object_store(ObjectStore::new("translations"))
        .build()
        .await
        .map_err(|e| format!("Failed to open IndexedDB: {:?}", e))?;

    let transaction = rexie
        .transaction(&["translations"], TransactionMode::ReadWrite)
        .map_err(|e| format!("Failed to create transaction: {:?}", e))?;
    let store = transaction
        .store("translations")
        .map_err(|e| format!("Failed to get store: {:?}", e))?;

    store
        .delete(cache_key.into())
        .await
        .map_err(|e| format!("Failed to delete translation from cache: {:?}", e))?;

    transaction
        .commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {:?}", e))?;
    Ok(())
}
