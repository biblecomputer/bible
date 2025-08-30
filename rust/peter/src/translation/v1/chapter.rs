use super::book_name::BookName;
use super::verse::{Verse, VerseNumber};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChapterNumber(pub u32);

impl std::fmt::Display for ChapterNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub verses: Vec<Verse>,
    #[serde(with = "super::serde_helpers::hashmap_as_tuple_list")]
    pub verse_sections: HashMap<VerseNumber, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapters(#[serde(with = "super::serde_helpers::btreemap_as_tuple_list")] pub BTreeMap<ChapterNumber, Chapter>);

#[derive(Debug, Clone)]
pub struct ChapterID {
    pub book_name: BookName,
    pub number: ChapterNumber,
}
