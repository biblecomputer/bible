use super::book::Books;
use super::chapter::{ChapterNumber, Chapters};
use super::translation_v1::TranslationV1;
use super::verse::{Verse, VerseNumber, VerseID};
use crate::storage::Storage;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TranslationV1ValidationError {
    #[error("Missing verse {0}")]
    MissingVerse(VerseID),

    #[error("Verse gap detected in {book} chapter {chapter}: missing verses {missing:?}")]
    VerseGap {
        book: String,
        chapter: ChapterNumber,
        missing: Vec<u32>,
    },

    #[error("Missing chapter {chapter} in {book}")]
    MissingChapter {
        book: String,
        chapter: ChapterNumber,
    },

    #[error("Chapter gap detected in {book}: missing chapters {missing:?}")]
    ChapterGap {
        book: String,
        missing: Vec<ChapterNumber>,
    },

    #[error("Empty chapter detected: {book} chapter {chapter} has no verses")]
    EmptyChapter {
        book: String,
        chapter: ChapterNumber,
    },

    #[error("Empty book detected: {book} has no chapters")]
    EmptyBook { book: String },

    #[error("Duplicate verse {verse} in {book} chapter {chapter}")]
    DuplicateVerse {
        book: String,
        chapter: ChapterNumber,
        verse: u32,
    },

    #[error(
        "Invalid verse range in {book} chapter {chapter}: {start}-{end} (start must be <= end)"
    )]
    InvalidVerseRange {
        book: String,
        chapter: u32,
        start: u32,
        end: u32,
    },

    #[error("Overlapping verse ranges in {book} chapter {chapter}: {range1:?} and {range2:?}")]
    OverlappingRanges {
        book: String,
        chapter: u32,
        range1: (u32, u32),
        range2: (u32, u32),
    },

    #[error("Cannot validate remote storage - books data is not local")]
    RemoteStorage,

    #[error("Multiple validation errors found: {count} errors")]
    MultipleErrors {
        count: usize,
        errors: Vec<TranslationV1ValidationError>,
    },
}

/// Validation result containing all errors and warnings found
#[derive(Debug, Default)]
pub struct ValidationResult {
    pub errors: Vec<TranslationV1ValidationError>,
    pub warnings: Vec<String>,
    pub statistics: ValidationStatistics,
}

/// Statistics gathered during validation
#[derive(Debug, Default)]
pub struct ValidationStatistics {
    pub total_books: usize,
    pub total_chapters: usize,
    pub total_verses: usize,
    pub books_with_errors: usize,
    pub chapters_with_errors: usize,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn to_error(&self) -> Result<(), TranslationV1ValidationError> {
        if self.errors.is_empty() {
            Ok(())
        } else if self.errors.len() == 1 {
            Err(self.errors[0].clone())
        } else {
            Err(TranslationV1ValidationError::MultipleErrors {
                count: self.errors.len(),
                errors: self.errors.clone(),
            })
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "Validation Summary:\n\
             - Total books: {}\n\
             - Total chapters: {}\n\
             - Total verses: {}\n\
             - Errors found: {}\n\
             - Warnings: {}\n\
             - Books with errors: {}\n\
             - Chapters with errors: {}",
            self.statistics.total_books,
            self.statistics.total_chapters,
            self.statistics.total_verses,
            self.errors.len(),
            self.warnings.len(),
            self.statistics.books_with_errors,
            self.statistics.chapters_with_errors,
        )
    }
}

impl Clone for TranslationV1ValidationError {
    fn clone(&self) -> Self {
        match self {
            Self::MissingVerse(verse_id) => Self::MissingVerse(verse_id.clone()),
            Self::VerseGap {
                book,
                chapter,
                missing,
            } => Self::VerseGap {
                book: book.clone(),
                chapter: *chapter,
                missing: missing.clone(),
            },
            Self::MissingChapter { book, chapter } => Self::MissingChapter {
                book: book.clone(),
                chapter: *chapter,
            },
            Self::ChapterGap { book, missing } => Self::ChapterGap {
                book: book.clone(),
                missing: missing.clone(),
            },
            Self::EmptyChapter { book, chapter } => Self::EmptyChapter {
                book: book.clone(),
                chapter: *chapter,
            },
            Self::EmptyBook { book } => Self::EmptyBook { book: book.clone() },
            Self::DuplicateVerse {
                book,
                chapter,
                verse,
            } => Self::DuplicateVerse {
                book: book.clone(),
                chapter: *chapter,
                verse: *verse,
            },
            Self::InvalidVerseRange {
                book,
                chapter,
                start,
                end,
            } => Self::InvalidVerseRange {
                book: book.clone(),
                chapter: *chapter,
                start: *start,
                end: *end,
            },
            Self::OverlappingRanges {
                book,
                chapter,
                range1,
                range2,
            } => Self::OverlappingRanges {
                book: book.clone(),
                chapter: *chapter,
                range1: *range1,
                range2: *range2,
            },
            Self::RemoteStorage => Self::RemoteStorage,
            Self::MultipleErrors { count, errors } => Self::MultipleErrors {
                count: *count,
                errors: errors.clone(),
            },
        }
    }
}

impl TranslationV1 {
    /// Check the translation for completeness and correctness
    pub fn check(&self) -> Result<(), TranslationV1ValidationError> {
        match &self.books {
            Storage::Local(books) => {
                let result = validate_books(books);
                result.to_error()
            }
            Storage::Iagon(_) => Err(TranslationV1ValidationError::RemoteStorage),
        }
    }

    /// Get a detailed validation result with statistics and warnings
    pub fn validate(&self) -> Result<ValidationResult, TranslationV1ValidationError> {
        match &self.books {
            Storage::Local(books) => Ok(validate_books(books)),
            Storage::Iagon(_) => Err(TranslationV1ValidationError::RemoteStorage),
        }
    }
}

/// Validate a Books structure for completeness and correctness
pub fn validate_books(books: &Books) -> ValidationResult {
    let mut result = ValidationResult::default();
    let mut books_with_errors = HashSet::new();
    let mut chapters_with_errors = HashSet::new();

    // Access the inner BTreeMap through the Books newtype
    let books_map = &books.0;

    // Iterate through all books
    for (book_name, book) in books_map.iter() {
        result.statistics.total_books += 1;
        let book_str = format!("{:?}", book_name);

        // Check for empty book
        if book.chapters.0.is_empty() {
            result.errors.push(TranslationV1ValidationError::EmptyBook {
                book: book_name.to_string(),
            });
            books_with_errors.insert(book_str.clone());
            continue;
        }

        // Validate chapters in the book
        let chapter_errors = validate_book_chapters(&book_str, &book.chapters, &mut result);
        if !chapter_errors.is_empty() {
            books_with_errors.insert(book_str.clone());
            for (chapter_id, _) in chapter_errors {
                chapters_with_errors.insert(format!("{} ch{}", book_str, chapter_id));
            }
        }
    }

    result.statistics.books_with_errors = books_with_errors.len();
    result.statistics.chapters_with_errors = chapters_with_errors.len();

    result
}

/// Validate chapters within a book
fn validate_book_chapters(
    book_name: &str,
    chapters: &Chapters,
    result: &mut ValidationResult,
) -> Vec<(u32, Vec<TranslationV1ValidationError>)> {
    let mut chapter_errors = Vec::new();
    let chapters_map = &chapters.0;

    // Get all chapter numbers
    let chapter_nums: Vec<u32> = chapters_map.keys().map(|c| c.0).collect();

    if let Some(&min_chapter) = chapter_nums.first() {
        if let Some(&max_chapter) = chapter_nums.last() {
            // Check for missing chapters (gaps in sequence)
            let expected_chapters: HashSet<u32> = (min_chapter..=max_chapter).collect();
            let actual_chapters: HashSet<u32> = chapter_nums.iter().cloned().collect();
            let missing_chapters: Vec<u32> = expected_chapters
                .difference(&actual_chapters)
                .cloned()
                .collect::<Vec<_>>();

            if !missing_chapters.is_empty() {
                let mut missing_sorted = missing_chapters.clone();
                missing_sorted.sort_unstable();

                result
                    .errors
                    .push(TranslationV1ValidationError::ChapterGap {
                        book: book_name.to_string(),
                        missing: missing_sorted.iter().map(|&c| ChapterNumber(c)).collect(),
                    });

                // Add individual missing chapter errors for clarity
                for chapter in missing_sorted {
                    result
                        .errors
                        .push(TranslationV1ValidationError::MissingChapter {
                            book: book_name.to_string(),
                            chapter: ChapterNumber(chapter),
                        });
                }
            }

            // Warn if book doesn't start at chapter 1
            if min_chapter != 1 {
                result.warnings.push(format!(
                    "Book {} starts at chapter {} instead of chapter 1",
                    book_name, min_chapter
                ));
            }
        }
    }

    // Validate each chapter
    for (chapter_id, chapter) in chapters_map {
        result.statistics.total_chapters += 1;
        let mut errors = Vec::new();

        // Check for empty chapter
        if chapter.verses.is_empty() {
            errors.push(TranslationV1ValidationError::EmptyChapter {
                book: book_name.to_string(),
                chapter: *chapter_id,
            });
            result
                .errors
                .push(TranslationV1ValidationError::EmptyChapter {
                    book: book_name.to_string(),
                    chapter: *chapter_id,
                });
        } else {
            // Validate verses in the chapter
            validate_chapter_verses(
                book_name,
                chapter_id.0,
                &chapter.verses,
                result,
                &mut errors,
            );
        }

        if !errors.is_empty() {
            chapter_errors.push((chapter_id.0, errors));
        }
    }

    chapter_errors
}

/// Validate verses within a chapter
fn validate_chapter_verses(
    book_name: &str,
    chapter_num: u32,
    verses: &[Verse],
    result: &mut ValidationResult,
    errors: &mut Vec<TranslationV1ValidationError>,
) {
    let mut verse_numbers = HashSet::new();
    let mut all_verse_nums = Vec::new();
    let mut verse_ranges = Vec::new();

    for verse in verses {
        result.statistics.total_verses += 1;

        match &verse.number {
            VerseNumber::Single(num) => {
                // Check for duplicates
                if !verse_numbers.insert(*num) {
                    let error = TranslationV1ValidationError::DuplicateVerse {
                        book: book_name.to_string(),
                        chapter: ChapterNumber(chapter_num),
                        verse: *num,
                    };
                    errors.push(error.clone());
                    result.errors.push(error);
                }
                all_verse_nums.push(*num);
            }
            VerseNumber::Range(start, end) => {
                // Validate range
                if start > end {
                    let error = TranslationV1ValidationError::InvalidVerseRange {
                        book: book_name.to_string(),
                        chapter: chapter_num,
                        start: *start,
                        end: *end,
                    };
                    errors.push(error.clone());
                    result.errors.push(error);
                } else {
                    // Check for overlapping ranges
                    for (existing_start, existing_end) in &verse_ranges {
                        if ranges_overlap(*start, *end, *existing_start, *existing_end) {
                            let error = TranslationV1ValidationError::OverlappingRanges {
                                book: book_name.to_string(),
                                chapter: chapter_num,
                                range1: (*existing_start, *existing_end),
                                range2: (*start, *end),
                            };
                            errors.push(error.clone());
                            result.errors.push(error);
                        }
                    }

                    verse_ranges.push((*start, *end));

                    // Add all verses in range for gap checking
                    for v in *start..=*end {
                        if !verse_numbers.insert(v) {
                            let error = TranslationV1ValidationError::DuplicateVerse {
                                book: book_name.to_string(),
                                chapter: ChapterNumber(chapter_num),
                                verse: v,
                            };
                            errors.push(error.clone());
                            result.errors.push(error);
                        }
                        all_verse_nums.push(v);
                    }
                }
            }
        }
    }

    // Check for gaps in verse numbering
    if !all_verse_nums.is_empty() {
        all_verse_nums.sort_unstable();
        let min_verse = *all_verse_nums.first().unwrap();
        let max_verse = *all_verse_nums.last().unwrap();

        // Warn if chapter doesn't start at verse 1
        if min_verse != 1 {
            result.warnings.push(format!(
                "{} chapter {} starts at verse {} instead of verse 1",
                book_name, chapter_num, min_verse
            ));
        }

        // Check for missing verses (gaps)
        let expected_verses: HashSet<u32> = (min_verse..=max_verse).collect();
        let actual_verses: HashSet<u32> = all_verse_nums.iter().cloned().collect();
        let missing_verses: Vec<u32> = expected_verses
            .difference(&actual_verses)
            .cloned()
            .collect::<Vec<_>>();

        if !missing_verses.is_empty() {
            let mut missing_sorted = missing_verses;
            missing_sorted.sort_unstable();

            let error = TranslationV1ValidationError::VerseGap {
                book: book_name.to_string(),
                chapter: ChapterNumber(chapter_num),
                missing: missing_sorted,
            };
            errors.push(error.clone());
            result.errors.push(error);
        }
    }
}

/// Check if two ranges overlap
fn ranges_overlap(start1: u32, end1: u32, start2: u32, end2: u32) -> bool {
    !(end1 < start2 || end2 < start1)
}

/// Quick validation check that returns true if valid, false otherwise
pub fn is_valid(books: &Books) -> bool {
    validate_books(books).is_valid()
}

/// Get a detailed validation report as a string
pub fn validation_report(books: &Books) -> String {
    let result = validate_books(books);
    let mut report = String::new();

    report.push_str(&result.summary());
    report.push_str("\n\n");

    if !result.errors.is_empty() {
        report.push_str("ERRORS:\n");
        for (i, error) in result.errors.iter().enumerate() {
            report.push_str(&format!("  {}. {}\n", i + 1, error));
        }
        report.push_str("\n");
    }

    if !result.warnings.is_empty() {
        report.push_str("WARNINGS:\n");
        for (i, warning) in result.warnings.iter().enumerate() {
            report.push_str(&format!("  {}. {}\n", i + 1, warning));
        }
    }

    if result.is_valid() && !result.has_warnings() {
        report.push_str("✓ All validation checks passed successfully!");
    }

    report
}
