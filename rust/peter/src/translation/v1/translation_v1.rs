use super::book_name::BookName;
use super::meta::TranslationMetaData;
use crate::translation::translation_v0::TranslationV0;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    meta: TranslationMetaData,
    books: Books,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Books(BTreeMap<BookName, Book>);

impl TryFrom<TranslationV0> for Books {
    type Error = ();

    fn try_from(_value: TranslationV0) -> Result<Self, Self::Error> {
        todo!()
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
struct Chapters(BTreeMap<ChapterID, Chapter>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
struct ChapterID(u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Chapter {
    verses: Vec<Verse>,
    verse_sections: HashMap<VerseID, String>,
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

impl TryFrom<TranslationV0> for Translation {
    type Error = ();

    fn try_from(_value: TranslationV0) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub fn build_v1(books: Books, meta: TranslationMetaData) -> Translation {
    Translation { meta, books }
}
