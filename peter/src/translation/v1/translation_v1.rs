use super::book_name::BookName;
use super::meta::TranslationMetaData;
use crate::translation::translation_v0::TranslationV0;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    meta: TranslationMetaData,
    books: Books,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Books(BTreeMap<BookName, Book>);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Book {
    name: String,
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

impl TryFrom<TranslationV0> for Translation {
    type Error = ();

    fn try_from(value: TranslationV0) -> Result<Self, Self::Error> {
        todo!()
    }
}
