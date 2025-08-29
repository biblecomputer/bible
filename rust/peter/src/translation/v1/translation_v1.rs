use super::book_name::{BookName, BookNameParseError};
use super::meta::TranslationMetaData;
use crate::storage::Storage;
use crate::translation::translation_v0::TranslationV0;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationV1 {
    pub meta: TranslationMetaData,
    pub books: Storage<Books>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Books(#[serde(with = "btreemap_as_tuple_list")] BTreeMap<BookName, Book>);

// Serde helper module for BTreeMap serialization
mod btreemap_as_tuple_list {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::BTreeMap;

    pub fn serialize<S, K, V>(map: &BTreeMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        K: Serialize + Ord,
        V: Serialize,
    {
        let vec: Vec<(&K, &V)> = map.iter().collect();
        vec.serialize(serializer)
    }

    pub fn deserialize<'de, D, K, V>(deserializer: D) -> Result<BTreeMap<K, V>, D::Error>
    where
        D: Deserializer<'de>,
        K: Deserialize<'de> + Ord,
        V: Deserialize<'de>,
    {
        let vec: Vec<(K, V)> = Vec::deserialize(deserializer)?;
        Ok(vec.into_iter().collect())
    }
}

#[derive(Debug, Error)]
pub enum BooksConversionError {
    #[error("Failed to parse book name: {0}")]
    BookNameParse(#[from] BookNameParseError),

    #[error("Book '{0}' has no chapters")]
    EmptyBook(String),

    #[error("Chapter {chapter} in book '{book}' has no verses")]
    EmptyChapter { book: String, chapter: u32 },

    #[error("Invalid verse range: start ({start}) must be less than or equal to end ({end})")]
    InvalidVerseRange { start: u32, end: u32 },

    #[error("Duplicate book found: {0}")]
    DuplicateBook(String),

    #[error(
        "Verse {verse} in chapter {chapter} of book '{book}' has inconsistent chapter number: expected {expected}, found {found}"
    )]
    InconsistentChapterNumber {
        book: String,
        chapter: u32,
        verse: u32,
        expected: u32,
        found: u32,
    },
}

fn parse_verse_id(text: &str, verse_number: u32) -> Result<VerseID, BooksConversionError> {
    if text.contains('-') && text.starts_with(char::is_numeric) {
        text.find('-')
            .and_then(|dash_pos| {
                text[..dash_pos]
                    .trim()
                    .parse::<u32>()
                    .ok()
                    .and_then(|start| {
                        text[dash_pos + 1..]
                            .find(' ')
                            .and_then(|space_pos| {
                                text[dash_pos + 1..dash_pos + 1 + space_pos]
                                    .trim()
                                    .parse::<u32>()
                                    .ok()
                            })
                            .map(|end| (start, end))
                    })
            })
            .map(|(start, end)| {
                if start > end {
                    Err(BooksConversionError::InvalidVerseRange { start, end })
                } else {
                    Ok(VerseID::Range(start, end))
                }
            })
            .unwrap_or_else(|| Ok(VerseID::Single(verse_number)))
    } else {
        Ok(VerseID::Single(verse_number))
    }
}

impl TryFrom<TranslationV0> for Books {
    type Error = BooksConversionError;

    fn try_from(value: TranslationV0) -> Result<Self, Self::Error> {
        let books_with_names = value
            .books
            .into_iter()
            .map(|book| {
                BookName::try_from(book.name.as_str())
                    .map(|book_name| (book_name, book))
                    .map_err(Into::into)
            })
            .collect::<Result<Vec<_>, BooksConversionError>>()?;

        // Check for duplicates
        let book_names: Vec<_> = books_with_names.iter().map(|(name, _)| *name).collect();
        let unique_names: std::collections::HashSet<_> = book_names.iter().copied().collect();
        if book_names.len() != unique_names.len() {
            let duplicate = books_with_names
                .iter()
                .find(|(name, _)| book_names.iter().filter(|&&n| n == *name).count() > 1)
                .map(|(_, book)| book.name.clone())
                .unwrap_or_else(|| "Unknown".to_string());
            return Err(BooksConversionError::DuplicateBook(duplicate));
        }

        books_with_names
            .into_iter()
            .map(|(book_name, v0_book)| {
                if v0_book.chapters.is_empty() {
                    return Err(BooksConversionError::EmptyBook(v0_book.name.clone()));
                }

                let chapters = v0_book
                    .chapters
                    .into_iter()
                    .map(|v0_chapter| {
                        if v0_chapter.verses.is_empty() {
                            return Err(BooksConversionError::EmptyChapter {
                                book: v0_book.name.clone(),
                                chapter: v0_chapter.chapter,
                            });
                        }

                        let converted_verses = v0_chapter
                            .verses
                            .into_iter()
                            .map(|v0_verse| {
                                if v0_verse.chapter != v0_chapter.chapter {
                                    return Err(BooksConversionError::InconsistentChapterNumber {
                                        book: v0_book.name.clone(),
                                        chapter: v0_chapter.chapter,
                                        verse: v0_verse.verse,
                                        expected: v0_chapter.chapter,
                                        found: v0_verse.chapter,
                                    });
                                }

                                let verse_id = parse_verse_id(&v0_verse.text, v0_verse.verse)?;

                                Ok((v0_verse, verse_id))
                            })
                            .collect::<Result<Vec<_>, BooksConversionError>>()?;

                        let verses: Vec<Verse> = converted_verses
                            .iter()
                            .map(|(v0_verse, verse_id)| Verse {
                                verse_id: verse_id.clone(),
                                content: v0_verse.text.clone(),
                                footnotes: None,
                            })
                            .collect();

                        let verse_sections: std::collections::HashMap<VerseID, String> =
                            converted_verses
                                .into_iter()
                                .filter(|(v0_verse, _)| {
                                    !v0_verse.name.is_empty() && v0_verse.name != v0_verse.text
                                })
                                .map(|(v0_verse, verse_id)| (verse_id, v0_verse.name))
                                .collect();

                        Ok((
                            ChapterID(v0_chapter.chapter),
                            Chapter {
                                verses,
                                verse_sections,
                            },
                        ))
                    })
                    .collect::<Result<BTreeMap<_, _>, BooksConversionError>>()?;

                Ok((
                    book_name,
                    Book {
                        name: v0_book.name,
                        introduction: None,
                        chapters: Chapters(chapters),
                    },
                ))
            })
            .collect::<Result<BTreeMap<_, _>, BooksConversionError>>()
            .map(Books)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Book {
    /// The name in the language of the translation
    name: String,
    /// Some translations wrote their own introduction about an bible book.
    introduction: Option<String>,
    chapters: Chapters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Chapters(#[serde(with = "btreemap_as_tuple_list")] BTreeMap<ChapterID, Chapter>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
struct ChapterID(u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Chapter {
    verses: Vec<Verse>,
    #[serde(with = "hashmap_as_tuple_list")]
    verse_sections: std::collections::HashMap<VerseID, String>,
}

// Serde helper module for HashMap serialization
mod hashmap_as_tuple_list {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::HashMap;
    use std::hash::Hash;

    pub fn serialize<S, K, V>(map: &HashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        K: Serialize + Eq + Hash,
        V: Serialize,
    {
        let vec: Vec<(&K, &V)> = map.iter().collect();
        vec.serialize(serializer)
    }

    pub fn deserialize<'de, D, K, V>(deserializer: D) -> Result<HashMap<K, V>, D::Error>
    where
        D: Deserializer<'de>,
        K: Deserialize<'de> + Eq + Hash,
        V: Deserialize<'de>,
    {
        let vec: Vec<(K, V)> = Vec::deserialize(deserializer)?;
        Ok(vec.into_iter().collect())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Verse {
    verse_id: VerseID,
    content: String,
    footnotes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum VerseID {
    Single(u32),
    // right should be greater then left.
    Range(u32, u32),
}

impl Ord for VerseID {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (VerseID::Single(a), VerseID::Single(b)) => a.cmp(b),
            (VerseID::Single(a), VerseID::Range(b, _)) => {
                match a.cmp(b) {
                    Ordering::Equal => Ordering::Less, // Single < Range if same left bound
                    ord => ord,
                }
            }
            (VerseID::Range(a, _), VerseID::Single(b)) => {
                match a.cmp(b) {
                    Ordering::Equal => Ordering::Greater, // Range > Single if same left bound
                    ord => ord,
                }
            }
            (VerseID::Range(a1, b1), VerseID::Range(a2, b2)) => match a1.cmp(a2) {
                Ordering::Equal => b1.cmp(b2),
                ord => ord,
            },
        }
    }
}

impl PartialOrd for VerseID {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<TranslationV0> for TranslationV1 {
    type Error = BooksConversionError;

    fn try_from(value: TranslationV0) -> Result<Self, Self::Error> {
        let books = Books::try_from(value)?;

        // Create placeholder metadata - this will be replaced in the migration process
        let meta = TranslationMetaData {
            full_name: String::from("Placeholder"),
            short_name: String::from("PH"),
            description: String::from("Placeholder metadata - to be replaced during migration"),
            link: url::Url::parse("https://example.com").unwrap(),
            release: super::meta::Year::new(2000),
            languages: vec![crate::language::Language::English],
            equivalence_level: super::meta::EquivalenceLevel::new(128),
            funded_by: None,
        };

        Ok(TranslationV1 {
            meta,
            books: Storage::Local(books),
        })
    }
}

pub fn build_v1(books: Books, meta: TranslationMetaData) -> TranslationV1 {
    TranslationV1 {
        meta,
        books: Storage::Local(books),
    }
}
