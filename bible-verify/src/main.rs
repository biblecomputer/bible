mod types;

use serde_json::Value;
use std::fs;
use types::{Bible, Book, Chapter, ValidationError, Verse};

fn main() {
    let s = include_str!("./kjv.json");
    let bible = parse(s);
}

/// Parse a JSON string into a Bible structure
/// This is a simple example - users can implement their own parsers
fn parse(s: &str) -> Bible {
    serde_json::from_str(s).expect("Failed to parse JSON to Bible")
}

/// Test the validation functionality
fn test_bible_validation(bible: &Bible) {
    println!("\n=== Testing Bible Validation ===");

    match bible.is_valid() {
        Ok(()) => println!("✓ Bible passed validation"),
        Err(e) => {
            println!("✗ Bible validation failed: {:?}", e);
            match e {
                ValidationError::BookAmount(count) => {
                    println!("  Expected 66 books, found {}", count);
                }
                ValidationError::ChapterAmount(count) => {
                    println!("  Invalid chapter count: {}", count);
                }
                ValidationError::VerseAmount(count) => {
                    println!("  Invalid verse count: {}", count);
                }
                ValidationError::SuspiciousVerseLength => {
                    println!("  Found verse with less than 3 words");
                }
                ValidationError::SuspiciousChapterLength => {
                    println!("  Found chapter with less than 3 verses");
                }
            }
        }
    }
}

/// Test with sample data
fn test_with_sample_data() {
    println!("\n=== Testing with Sample Data ===");

    // Create a small sample Bible
    let sample_bible = Bible {
        books: vec![
            Book {
                name: "Genesis".to_string(),
                chapters: vec![
                    Chapter {
                        chapter: 1,
                        name: "Genesis 1".to_string(),
                        verses: vec![
                            Verse {
                                verse: 1,
                                chapter: 1,
                                name: "Genesis 1:1".to_string(),
                                text: "In the beginning God created the heaven and the earth.".to_string(),
                            },
                            Verse {
                                verse: 2,
                                chapter: 1,
                                name: "Genesis 1:2".to_string(),
                                text: "And the earth was without form, and void; and darkness was upon the face of the deep.".to_string(),
                            },
                            Verse {
                                verse: 3,
                                chapter: 1,
                                name: "Genesis 1:3".to_string(),
                                text: "And God said, Let there be light: and there was light.".to_string(),
                            },
                        ],
                    },
                ],
            },
            Book {
                name: "Exodus".to_string(),
                chapters: vec![
                    Chapter {
                        chapter: 1,
                        name: "Exodus 1".to_string(),
                        verses: vec![
                            Verse {
                                verse: 1,
                                chapter: 1,
                                name: "Exodus 1:1".to_string(),
                                text: "Now these are the names of the children of Israel.".to_string(),
                            },
                        ],
                    },
                ],
            },
        ],
    };

    // Test validation
    test_bible_validation(&sample_bible);

    // Test suspicious content detection
    println!("\n=== Testing Suspicious Content Detection ===");

    let suspicious_chapter = Chapter {
        chapter: 1,
        name: "Test Chapter".to_string(),
        verses: vec![Verse {
            verse: 1,
            chapter: 1,
            name: "Test 1:1".to_string(),
            text: "Only two words".to_string(),
        }],
    };

    println!(
        "Chapter with {} verses is suspicious: {}",
        suspicious_chapter.verses.len(),
        suspicious_chapter.suspicious_verse_amount()
    );

    let short_verse = Verse {
        verse: 1,
        chapter: 1,
        name: "Test 1:1".to_string(),
        text: "Two words".to_string(),
    };

    println!(
        "Verse with text '{}' is suspicious: {}",
        short_verse.text,
        short_verse.suspicious()
    );

    // Test serialization
    println!("\n=== Testing Serialization ===");
    match serde_json::to_string_pretty(&sample_bible) {
        Ok(json) => {
            println!("Successfully serialized to JSON:");
            println!("{}", &json[..200.min(json.len())]); // Print first 200 chars
            println!("...");
        }
        Err(e) => println!("Failed to serialize: {}", e),
    }

    // Test custom parser
    println!("\n=== Testing Custom Parser ===");
    let custom_data = "Genesis|1|1|In the beginning God created the heaven and the earth.\n\
                       Genesis|1|2|And the earth was without form, and void.\n\
                       Genesis|1|3|And God said, Let there be light.\n\
                       Exodus|1|1|Now these are the names of the children of Israel.";

    match parse_custom_format(custom_data) {
        Ok(bible) => {
            println!("Successfully parsed custom format:");
            println!("  Books: {}", bible.books.len());
            for book in &bible.books {
                println!("  - {} ({} chapters)", book.name, book.chapters.len());
            }
        }
        Err(e) => println!("Failed to parse custom format: {}", e),
    }
}

/// Example utility functions that users might find helpful
pub mod utils {
    use crate::types::{Bible, Book, Chapter};

    /// Count total verses in the Bible
    pub fn count_verses(bible: &Bible) -> usize {
        bible
            .books
            .iter()
            .flat_map(|book| &book.chapters)
            .map(|chapter| chapter.verses.len())
            .sum()
    }

    /// Find a book by name
    pub fn find_book<'a>(bible: &'a Bible, name: &str) -> Option<&'a Book> {
        bible
            .books
            .iter()
            .find(|book| book.name.eq_ignore_ascii_case(name))
    }

    /// Get chapter by book name and chapter number
    pub fn get_chapter<'a>(
        bible: &'a Bible,
        book_name: &str,
        chapter_num: u32,
    ) -> Option<&'a Chapter> {
        find_book(bible, book_name)?
            .chapters
            .iter()
            .find(|ch| ch.chapter == chapter_num)
    }
}
