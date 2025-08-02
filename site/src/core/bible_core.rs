use leptos::prelude::*;
use leptos_router::hooks::{use_params_map, use_location};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use urlencoding::{decode, encode};

pub static BIBLE: OnceLock<Bible> = OnceLock::new();
static CURRENT_BIBLE_SIGNAL: OnceLock<RwSignal<Option<Bible>>> = OnceLock::new();

pub fn init_bible_signal() -> RwSignal<Option<Bible>> {
    *CURRENT_BIBLE_SIGNAL.get_or_init(|| RwSignal::new(None))
}

pub fn get_bible() -> &'static Bible {
    BIBLE
        .get()
        .expect("Bible not initialized - call init_bible() first")
}

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

#[derive(Debug, Clone, PartialEq)]
pub struct VerseRange {
    pub start: u32,
    pub end: u32,
}

impl VerseRange {
    pub fn contains(&self, verse_number: u32) -> bool {
        verse_number >= self.start && verse_number <= self.end
    }
    
    pub fn from_string(s: &str) -> Option<Self> {
        if let Some((start_str, end_str)) = s.split_once('-') {
            if let (Ok(start), Ok(end)) = (start_str.trim().parse::<u32>(), end_str.trim().parse::<u32>()) {
                if start <= end {
                    return Some(VerseRange { start, end });
                }
            }
        } else if let Ok(single_verse) = s.trim().parse::<u32>() {
            return Some(VerseRange { start: single_verse, end: single_verse });
        }
        None
    }
}

pub fn parse_verse_ranges_from_url() -> Vec<VerseRange> {
    let location = use_location();
    let search_params = location.search.get();
    
    if let Some(verses_param) = search_params
        .split('&')
        .find_map(|param| {
            let mut parts = param.split('=');
            if parts.next()? == "verses" {
                parts.next()
            } else {
                None
            }
        })
    {
        verses_param
            .split(',')
            .filter_map(|range_str| VerseRange::from_string(range_str))
            .collect()
    } else {
        Vec::new()
    }
}

impl Chapter {
    pub fn to_path(&self) -> String {
        let name_parts: Vec<&str> = self.name.split_whitespace().collect();

        let book_name = if name_parts.len() > 1 {
            name_parts[..name_parts.len().saturating_sub(1)].join(" ")
        } else {
            self.name.clone()
        };

        let encoded_book = encode(&book_name);
        format!("/{}/{}", encoded_book, self.chapter)
    }
    
    pub fn to_path_with_verses(&self, verse_ranges: &[VerseRange]) -> String {
        let base_path = self.to_path();
        if verse_ranges.is_empty() {
            return base_path;
        }
        
        let verse_param = verse_ranges
            .iter()
            .map(|range| {
                if range.start == range.end {
                    range.start.to_string()
                } else {
                    format!("{}-{}", range.start, range.end)
                }
            })
            .collect::<Vec<_>>()
            .join(",");
            
        format!("{}?verses={}", base_path, verse_param)
    }

    pub fn from_url() -> std::result::Result<Self, ParamParseError> {
        let params = move || use_params_map();
        let book = move || params().read().get("book").unwrap();
        let chapter = move || {
            params()
                .read()
                .get("chapter")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(1)
        };

        let chapter: Chapter = get_bible().get_chapter(&book(), chapter()).unwrap();
        Ok(chapter)
    }

    pub fn get_next_verse(&self, current_verse: u32) -> Option<u32> {
        if current_verse < self.verses.len() as u32 {
            Some(current_verse + 1)
        } else {
            None
        }
    }

    pub fn get_previous_verse(&self, current_verse: u32) -> Option<u32> {
        if current_verse > 1 {
            Some(current_verse - 1)
        } else {
            None
        }
    }
}

impl Bible {
    pub fn get_chapter(
        &self,
        book: &str,
        chapter: u32,
    ) -> std::result::Result<Chapter, ParamParseError> {
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
        for (book_idx, book) in self.books.iter().enumerate() {
            if let Some(chapter_idx) = book
                .chapters
                .iter()
                .position(|c| c.chapter == current.chapter && c.name == current.name)
            {
                if let Some(next_chapter) = book.chapters.get(chapter_idx + 1) {
                    return Some(next_chapter.clone());
                }
                if let Some(next_book) = self.books.get(book_idx + 1) {
                    return next_book.chapters.first().cloned();
                }
                return None;
            }
        }
        None
    }

    pub fn get_previous_chapter(&self, current: &Chapter) -> Option<Chapter> {
        for (book_idx, book) in self.books.iter().enumerate() {
            if let Some(chapter_idx) = book
                .chapters
                .iter()
                .position(|c| c.chapter == current.chapter && c.name == current.name)
            {
                if chapter_idx > 0 {
                    if let Some(prev_chapter) = book.chapters.get(chapter_idx - 1) {
                        return Some(prev_chapter.clone());
                    }
                }
                if book_idx > 0 {
                    if let Some(prev_book) = self.books.get(book_idx - 1) {
                        return prev_book.chapters.last().cloned();
                    }
                }
                return None;
            }
        }
        None
    }

    pub fn get_next_book(&self, current: &Chapter) -> Option<Chapter> {
        // Find the current book
        for (book_idx, book) in self.books.iter().enumerate() {
            if book
                .chapters
                .iter()
                .any(|c| c.chapter == current.chapter && c.name == current.name)
            {
                // Found current book, get next book's first chapter
                if let Some(next_book) = self.books.get(book_idx + 1) {
                    return next_book.chapters.first().cloned();
                }
                return None; // Already at last book
            }
        }
        None
    }

    pub fn get_previous_book(&self, current: &Chapter) -> Option<Chapter> {
        // Find the current book
        for (book_idx, book) in self.books.iter().enumerate() {
            if book
                .chapters
                .iter()
                .any(|c| c.chapter == current.chapter && c.name == current.name)
            {
                // Found current book, get previous book's first chapter
                if book_idx > 0 {
                    if let Some(prev_book) = self.books.get(book_idx - 1) {
                        return prev_book.chapters.first().cloned();
                    }
                }
                return None; // Already at first book
            }
        }
        None
    }

    pub fn get_first_chapter(&self) -> Option<Chapter> {
        self.books.first()?.chapters.first().cloned()
    }

    pub fn get_last_chapter(&self) -> Option<Chapter> {
        self.books.last()?.chapters.last().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_verse_range_from_string() {
        // Test single verse
        let range = VerseRange::from_string("5").unwrap();
        assert_eq!(range.start, 5);
        assert_eq!(range.end, 5);
        assert!(range.contains(5));
        assert!(!range.contains(4));
        assert!(!range.contains(6));
        
        // Test verse range
        let range = VerseRange::from_string("1-3").unwrap();
        assert_eq!(range.start, 1);
        assert_eq!(range.end, 3);
        assert!(range.contains(1));
        assert!(range.contains(2));
        assert!(range.contains(3));
        assert!(!range.contains(4));
        
        // Test invalid ranges
        assert!(VerseRange::from_string("3-1").is_none()); // end < start
        assert!(VerseRange::from_string("abc").is_none()); // invalid number
        assert!(VerseRange::from_string("1-abc").is_none()); // invalid range
    }
    
    #[test]
    fn test_verse_navigation() {
        let chapter = Chapter {
            chapter: 1,
            name: "Genesis".to_string(),
            verses: vec![
                Verse { verse: 1, chapter: 1, name: "Genesis".to_string(), text: "In the beginning...".to_string() },
                Verse { verse: 2, chapter: 1, name: "Genesis".to_string(), text: "And the earth...".to_string() },
                Verse { verse: 3, chapter: 1, name: "Genesis".to_string(), text: "And God said...".to_string() },
            ]
        };

        // Test next verse navigation
        assert_eq!(chapter.get_next_verse(1), Some(2));
        assert_eq!(chapter.get_next_verse(2), Some(3));
        assert_eq!(chapter.get_next_verse(3), None); // Last verse

        // Test previous verse navigation
        assert_eq!(chapter.get_previous_verse(1), None); // First verse
        assert_eq!(chapter.get_previous_verse(2), Some(1));
        assert_eq!(chapter.get_previous_verse(3), Some(2));
        
        // Test edge cases
        assert_eq!(chapter.get_next_verse(0), Some(1)); // Invalid verse number should still work
        assert_eq!(chapter.get_previous_verse(0), None); // Can't go before first verse
    }

    #[test]
    fn test_cross_chapter_navigation_logic() {
        // Test that we properly handle chapter boundaries for cross-chapter navigation
        let genesis_1 = Chapter {
            chapter: 1,
            name: "Genesis 1".to_string(),
            verses: vec![
                Verse { verse: 1, chapter: 1, name: "Genesis 1".to_string(), text: "First verse".to_string() },
                Verse { verse: 2, chapter: 1, name: "Genesis 1".to_string(), text: "Second verse".to_string() },
            ]
        };

        let genesis_2 = Chapter {
            chapter: 2,
            name: "Genesis 2".to_string(),
            verses: vec![
                Verse { verse: 1, chapter: 2, name: "Genesis 2".to_string(), text: "First verse chapter 2".to_string() },
                Verse { verse: 2, chapter: 2, name: "Genesis 2".to_string(), text: "Second verse chapter 2".to_string() },
                Verse { verse: 3, chapter: 2, name: "Genesis 2".to_string(), text: "Third verse chapter 2".to_string() },
            ]
        };

        // Test verse navigation within chapters
        assert_eq!(genesis_1.get_next_verse(1), Some(2));
        assert_eq!(genesis_1.get_next_verse(2), None); // End of chapter - cross-chapter navigation handled in main.rs
        
        assert_eq!(genesis_2.get_previous_verse(1), None); // Beginning of chapter - cross-chapter navigation handled in main.rs
        assert_eq!(genesis_2.get_previous_verse(2), Some(1));
        
        // Test that we can identify when we're at chapter boundaries
        assert!(genesis_1.get_next_verse(genesis_1.verses.len() as u32).is_none()); // Last verse of chapter
        assert!(genesis_2.get_previous_verse(1).is_none()); // First verse of chapter
    }


    #[test]
    fn test_chapter_to_path_with_verses() {
        let chapter = Chapter {
            chapter: 1,
            name: "Genesis 1".to_string(),
            verses: vec![],
        };
        
        // Test without verses
        let path = chapter.to_path_with_verses(&[]);
        assert_eq!(path, "/Genesis/1");
        
        // Test with single verse
        let ranges = vec![VerseRange { start: 5, end: 5 }];
        let path = chapter.to_path_with_verses(&ranges);
        assert_eq!(path, "/Genesis/1?verses=5");
        
        // Test with verse range
        let ranges = vec![VerseRange { start: 1, end: 3 }];
        let path = chapter.to_path_with_verses(&ranges);
        assert_eq!(path, "/Genesis/1?verses=1-3");
        
        // Test with multiple ranges
        let ranges = vec![
            VerseRange { start: 1, end: 3 },
            VerseRange { start: 5, end: 5 },
            VerseRange { start: 10, end: 12 },
        ];
        let path = chapter.to_path_with_verses(&ranges);
        assert_eq!(path, "/Genesis/1?verses=1-3,5,10-12");
    }

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

            let first_chapter = &bible.books[0].chapters[0];
            prop_assert!(bible.get_previous_chapter(first_chapter).is_none());

            let last_chapter = &bible.books[0].chapters[num_chapters - 1];
            prop_assert!(bible.get_next_chapter(last_chapter).is_none());
        }

        #[test]
        fn test_cross_book_navigation(
            num_books in 2usize..5,
            chapters_per_book in 1usize..5
        ) {
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

            let last_chapter_book1 = &bible.books[0].chapters[chapters_per_book - 1];
            let first_chapter_book2 = &bible.books[1].chapters[0];

            if let Some(next_chapter) = bible.get_next_chapter(last_chapter_book1) {
                prop_assert_eq!(next_chapter.chapter, first_chapter_book2.chapter);
                prop_assert_eq!(next_chapter.name, first_chapter_book2.name.clone());
            }

            if let Some(prev_chapter) = bible.get_previous_chapter(first_chapter_book2) {
                prop_assert_eq!(prev_chapter.chapter, last_chapter_book1.chapter);
                prop_assert_eq!(prev_chapter.name, last_chapter_book1.name.clone());
            }
        }

        #[test]
        fn test_global_navigation(
            num_books in 2usize..5,
            chapters_per_book in 1usize..3
        ) {
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

            // Test get_first_chapter - should return first chapter of first book
            if let Some(first_chapter) = bible.get_first_chapter() {
                prop_assert_eq!(first_chapter.chapter, 1);
                prop_assert_eq!(first_chapter.name, "Book 0 Chapter 1".to_string());
            } else {
                prop_assert!(false, "get_first_chapter should return Some");
            }

            // Test get_last_chapter - should return last chapter of last book
            if let Some(last_chapter) = bible.get_last_chapter() {
                prop_assert_eq!(last_chapter.chapter, chapters_per_book as u32);
                prop_assert_eq!(last_chapter.name, format!("Book {} Chapter {}", num_books - 1, chapters_per_book));
            } else {
                prop_assert!(false, "get_last_chapter should return Some");
            }
        }
    }
}