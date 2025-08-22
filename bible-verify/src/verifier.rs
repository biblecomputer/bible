use clap::Parser;
use miette::{Diagnostic, NamedSource, SourceSpan};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(name = "bible-verify")]
#[command(about = "A Bible JSON verifier that checks for correct verse counts", long_about = None)]
struct Args {
    /// Path to the Bible JSON file to verify
    file: PathBuf,
}

#[derive(Error, Debug, Diagnostic)]
enum VerificationError {
    #[error("Failed to read file")]
    #[diagnostic(code(bible_verify::io_error))]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse JSON")]
    #[diagnostic(code(bible_verify::json_error))]
    JsonError {
        #[source_code]
        src: NamedSource<String>,
        #[label("Invalid JSON here")]
        span: SourceSpan,
        #[source]
        error: serde_json::Error,
    },

    #[error("Invalid book count")]
    #[diagnostic(code(bible_verify::book_count))]
    InvalidBookCount {
        #[source_code]
        src: NamedSource<String>,
        #[label("Expected 66 books, found {found}")]
        span: SourceSpan,
        found: usize,
    },

    #[error("Suspicious chapter")]
    #[diagnostic(code(bible_verify::suspicious_chapter))]
    SuspiciousChapter {
        #[source_code]
        src: NamedSource<String>,
        #[label("{book} chapter {chapter} has {verse_count} verses")]
        span: SourceSpan,
        book: String,
        chapter: usize,
        verse_count: usize,
        #[help]
        help: String,
    },

    #[error("Suspicious verse")]
    #[diagnostic(code(bible_verify::suspicious_verse))]
    SuspiciousVerse {
        #[source_code]
        src: NamedSource<String>,
        #[label("{book} {chapter}:{verse} has {word_count} words")]
        span: SourceSpan,
        book: String,
        chapter: usize,
        verse: usize,
        word_count: usize,
        #[help]
        help: String,
    },

    #[error("Missing verse")]
    #[diagnostic(code(bible_verify::missing_verse))]
    MissingVerse {
        #[source_code]
        src: NamedSource<String>,
        #[label("Missing verse {verse} in {book} chapter {chapter}")]
        span: SourceSpan,
        book: String,
        chapter: usize,
        verse: usize,
    },

    #[error("Duplicate verse")]
    #[diagnostic(code(bible_verify::duplicate_verse))]
    DuplicateVerse {
        #[source_code]
        src: NamedSource<String>,
        #[label("Duplicate verse {verse} in {book} chapter {chapter}")]
        span: SourceSpan,
        book: String,
        chapter: usize,
        verse: usize,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Bible {
    books: Vec<Book>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    name: String,
    chapters: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Chapter {
    chapter: usize,
    name: String,
    verses: Vec<Verse>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Verse {
    verse: usize,
    chapter: usize,
    name: String,
    text: String,
}

fn find_json_span(content: &str, book_idx: usize, chapter_idx: Option<usize>, verse_idx: Option<usize>) -> Option<SourceSpan> {
    // If we're looking for a specific verse, use a more precise approach
    if let Some(verse_idx) = verse_idx {
        return find_verse_text_span(content, book_idx, chapter_idx.unwrap_or(0), verse_idx);
    }
    
    let mut current_pos = 0;
    let mut book_count = 0;
    let mut in_books = false;
    let mut depth = 0;
    
    for (i, ch) in content.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => depth -= 1,
            '"' if depth > 0 => {
                if content[i..].starts_with("\"books\"") && !in_books {
                    in_books = true;
                } else if content[i..].starts_with("\"name\"") && in_books && book_count == book_idx {
                    if chapter_idx.is_none() {
                        return Some((i, 6).into());
                    }
                }
            }
            '[' if in_books => {
                if book_count == book_idx {
                    current_pos = i;
                }
            }
            _ => {}
        }
        
        if in_books && ch == '{' && depth == 3 {
            if book_count == book_idx {
                return Some((current_pos, 10).into());
            }
            book_count += 1;
        }
    }
    
    Some((0, content.len().min(100)).into())
}

fn find_verse_text_span(content: &str, book_idx: usize, chapter_idx: usize, verse_idx: usize) -> Option<SourceSpan> {
    let mut book_count = 0;
    let mut chapter_count = 0;
    let mut verse_count = 0;
    let mut in_books = false;
    let mut in_chapters = false;
    let mut in_verses = false;
    let mut in_target_book = false;
    let mut in_target_chapter = false;
    
    let lines: Vec<&str> = content.lines().collect();
    
    for (line_idx, line) in lines.iter().enumerate() {
        
        if line.contains("\"books\"") && !in_books {
            in_books = true;
        }
        
        if in_books && line.contains("\"name\"") && line.contains("\"") {
            if book_count == book_idx {
                in_target_book = true;
            } else {
                in_target_book = false;
            }
            if !in_target_book {
                book_count += 1;
            }
        }
        
        if in_target_book && line.contains("\"chapters\"") {
            in_chapters = true;
            chapter_count = 0;
        }
        
        if in_chapters && in_target_book && line.contains("\"chapter\"") && line.contains(":") {
            if chapter_count == chapter_idx {
                in_target_chapter = true;
            } else {
                in_target_chapter = false;
            }
            chapter_count += 1;
        }
        
        if in_target_chapter && line.contains("\"verses\"") {
            in_verses = true;
            verse_count = 0;
        }
        
        if in_verses && in_target_chapter && line.contains("\"verse\"") && line.contains(":") {
            if verse_count == verse_idx {
                // Look for the text field in the next few lines
                for i in line_idx..lines.len().min(line_idx + 5) {
                    if lines[i].contains("\"text\"") {
                        // Calculate the offset to the start of this line
                        let offset = lines.iter()
                            .take(i)
                            .map(|l| l.len() + 1)
                            .sum::<usize>();
                        
                        // Find the position of "text" in the line
                        if let Some(text_pos) = lines[i].find("\"text\"") {
                            let line_offset = offset + text_pos;
                            let line_len = lines[i].len() - text_pos;
                            return Some((line_offset, line_len).into());
                        }
                    }
                }
            }
            verse_count += 1;
        }
    }
    
    None
}

fn verify_bible(path: &PathBuf) -> Result<(), VerificationError> {
    let content = fs::read_to_string(path)?;
    let filename = path.display().to_string();
    
    let bible: Bible = match serde_json::from_str(&content) {
        Ok(bible) => bible,
        Err(e) => {
            let line = e.line();
            let column = e.column();
            let offset = content
                .lines()
                .take(line - 1)
                .map(|l| l.len() + 1)
                .sum::<usize>()
                + column - 1;
            
            return Err(VerificationError::JsonError {
                src: NamedSource::new(&filename, content.clone()),
                span: (offset, 1).into(),
                error: e,
            });
        }
    };

    if bible.books.len() != 66 {
        return Err(VerificationError::InvalidBookCount {
            src: NamedSource::new(&filename, content.clone()),
            span: find_json_span(&content, 0, None, None).unwrap_or((0, 10).into()),
            found: bible.books.len(),
        });
    }

    for (book_idx, book) in bible.books.iter().enumerate() {
        for (chapter_idx, chapter) in book.chapters.iter().enumerate() {
            let verse_count = chapter.verses.len();
            
            // Special case: Psalm 117 has only 2 verses
            let is_psalm_117 = book.name == "Psalms" && chapter.chapter == 117;
            
            if (verse_count < 3 && !is_psalm_117) || verse_count > 200 {
                let help = if verse_count < 3 {
                    "Most Bible chapters have at least 3 verses (except Psalm 117)".to_string()
                } else {
                    "No Bible chapter has more than 200 verses (Psalm 119 has 176)".to_string()
                };
                
                return Err(VerificationError::SuspiciousChapter {
                    src: NamedSource::new(&filename, content.clone()),
                    span: find_json_span(&content, book_idx, Some(chapter_idx), None)
                        .unwrap_or((0, 10).into()),
                    book: book.name.clone(),
                    chapter: chapter.chapter,
                    verse_count,
                    help,
                });
            }

            let mut seen_verses = std::collections::HashSet::new();
            for i in 1..=verse_count {
                if !chapter.verses.iter().any(|v| v.verse == i) {
                    return Err(VerificationError::MissingVerse {
                        src: NamedSource::new(&filename, content.clone()),
                        span: find_json_span(&content, book_idx, Some(chapter_idx), None)
                            .unwrap_or((0, 10).into()),
                        book: book.name.clone(),
                        chapter: chapter.chapter,
                        verse: i,
                    });
                }
            }

            for (verse_idx, verse) in chapter.verses.iter().enumerate() {
                if !seen_verses.insert(verse.verse) {
                    return Err(VerificationError::DuplicateVerse {
                        src: NamedSource::new(&filename, content.clone()),
                        span: find_json_span(&content, book_idx, Some(chapter_idx), Some(verse_idx))
                            .unwrap_or((0, 10).into()),
                        book: book.name.clone(),
                        chapter: chapter.chapter,
                        verse: verse.verse,
                    });
                }

                let word_count = verse.text.split_whitespace().count();
                // Allow 2-word verses as there are a few in the Bible
                if word_count < 2 || word_count > 150 {
                    let help = if word_count < 2 {
                        "Bible verses should have at least 2 words".to_string()
                    } else {
                        "Very few Bible verses exceed 150 words".to_string()
                    };
                    
                    return Err(VerificationError::SuspiciousVerse {
                        src: NamedSource::new(&filename, content.clone()),
                        span: find_json_span(&content, book_idx, Some(chapter_idx), Some(verse_idx))
                            .unwrap_or((0, 10).into()),
                        book: book.name.clone(),
                        chapter: chapter.chapter,
                        verse: verse.verse,
                        word_count,
                        help,
                    });
                }
            }
        }
    }

    Ok(())
}

fn main() -> miette::Result<()> {
    let args = Args::parse();
    
    match verify_bible(&args.file) {
        Ok(()) => {
            println!("✓ Bible JSON file is valid");
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}