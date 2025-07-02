use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use flate2::read::GzDecoder;
use std::io::Read as IoRead;

// Global static Bible instance - decompressed and parsed once, used everywhere
pub static BIBLE: LazyLock<Bible> = LazyLock::new(|| {
    // Include the compressed binary data
    let compressed_data = include_bytes!(concat!(env!("OUT_DIR"), "/stv_compressed.bin"));
    
    // Decompress the data
    let mut decoder = GzDecoder::new(&compressed_data[..]);
    let mut json_string = String::new();
    IoRead::read_to_string(&mut decoder, &mut json_string)
        .expect("Failed to decompress Bible data");
    
    // Parse the JSON
    serde_json::from_str(&json_string)
        .expect("Failed to parse Bible JSON")
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bible {
    pub books: Vec<Book>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        let name_parts: Vec<&str> = self.name.trim().split_whitespace().collect();
        
        // Remove the last part (chapter number) to get book name
        let book_name = if name_parts.len() > 1 {
            name_parts[..name_parts.len()-1].join(" ")
        } else {
            self.name.clone()
        };

        let book = book_name.replace(' ', "_");
        format!("/{}/{}", book, self.chapter)
    }

    pub fn from_url() -> Result<Self, ParamParseError> {
        let params = move || use_params_map();
        let book = move || params().read().get("book").unwrap();
        let chapter = move || {
            params()
                .read()
                .get("chapter")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(1) // fallback chapter number if parsing fails
        };

        let chapter: Chapter = BIBLE.get_chapter(&book(), chapter()).unwrap();
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
    pub fn get_chapter(&self, book: &str, chapter: u32) -> Result<Chapter, ParamParseError> {
        // Convert URL book name (with underscores) back to space-separated name
        let book_name = book.replace('_', " ");
        
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
            if let Some(chapter_idx) = book.chapters.iter().position(|c| c.chapter == current.chapter && c.name == current.name) {
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
            if let Some(chapter_idx) = book.chapters.iter().position(|c| c.chapter == current.chapter && c.name == current.name) {
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
