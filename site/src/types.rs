use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use urlencoding::{encode, decode};
use gloo_net::http::Request;

// Global static Bible instance - fetched from API once, used everywhere
pub static BIBLE: OnceLock<Bible> = OnceLock::new();

// Function to initialize the Bible data
pub async fn init_bible() -> Result<(), Box<dyn std::error::Error>> {
    if BIBLE.get().is_some() {
        return Ok(());
    }

    // Try multiple CORS proxy services in case one is down
    let proxy_urls = [
        "https://corsproxy.io/?https://gw.iagon.com/api/v2/storage/shareable/link/Njg2ZDFjNDgwOGQ0M2UzNTUyNTdhYmRh:MTJjOTRlYTBmNzM2YWZiZDE2NzdkMzU3NzA3MjBmMTRmZGZkMWYzNWVkYWVlNTU1Y2RjYTA1NzYzZmE1YmEzNA",
        "https://api.allorigins.win/get?url=https://gw.iagon.com/api/v2/storage/shareable/link/Njg2ZDFjNDgwOGQ0M2UzNTUyNTdhYmRh:MTJjOTRlYTBmNzM2YWZiZDE2NzdkMzU3NzA3MjBmMTRmZGZkMWYzNWVkYWVlNTU1Y2RjYTA1NzYzZmE1YmEzNA",
    ];

    let mut last_error = None;

    for proxy_url in &proxy_urls {
        match try_fetch_bible(proxy_url).await {
            Ok(bible) => {
                BIBLE.set(bible).map_err(|_| "Failed to set Bible data")?;
                return Ok(());
            }
            Err(e) => {
                last_error = Some(e);
                continue;
            }
        }
    }

    Err(last_error.unwrap_or_else(|| "All proxy attempts failed".into()))
}

async fn try_fetch_bible(url: &str) -> Result<Bible, Box<dyn std::error::Error>> {
    let response = Request::get(url)
        .send()
        .await?;
    
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
    BIBLE.get().expect("Bible not initialized - call init_bible() first")
}

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
        let name_parts: Vec<&str> = self.name.split_whitespace().collect();
        
        // Remove the last part (chapter number) to get book name
        let book_name = if name_parts.len() > 1 {
            name_parts[..name_parts.len()-1].join(" ")
        } else {
            self.name.clone()
        };

        let encoded_book = encode(&book_name);
        format!("/{}/{}", encoded_book, self.chapter)
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
    pub fn get_chapter(&self, book: &str, chapter: u32) -> Result<Chapter, ParamParseError> {
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
