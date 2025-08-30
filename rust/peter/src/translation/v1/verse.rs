use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use super::chapter::ChapterID;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verse {
    pub number: VerseNumber,
    pub content: String,
    pub footnotes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerseNumber {
    Single(u32),
    // right should be greater then left.
    Range(u32, u32),
}

impl Ord for VerseNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (VerseNumber::Single(a), VerseNumber::Single(b)) => a.cmp(b),
            (VerseNumber::Single(a), VerseNumber::Range(b, _)) => {
                match a.cmp(b) {
                    Ordering::Equal => Ordering::Less, // Single < Range if same left bound
                    ord => ord,
                }
            }
            (VerseNumber::Range(a, _), VerseNumber::Single(b)) => {
                match a.cmp(b) {
                    Ordering::Equal => Ordering::Greater, // Range > Single if same left bound
                    ord => ord,
                }
            }
            (VerseNumber::Range(a1, b1), VerseNumber::Range(a2, b2)) => match a1.cmp(a2) {
                Ordering::Equal => b1.cmp(b2),
                ord => ord,
            },
        }
    }
}

impl PartialOrd for VerseNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct VerseID {
    pub chapter_id: ChapterID,
    pub verse: VerseNumber,
}

impl std::fmt::Display for VerseID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "verse {:?} in {:?} chapter {}", self.verse, self.chapter_id.book_name, self.chapter_id.number)
    }
}
