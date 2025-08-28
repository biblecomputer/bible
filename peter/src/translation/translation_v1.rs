use crate::language::Language;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    meta: TranslationMetaData,
    books: Books,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Books(BTreeMap<BookName, Book>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BookName {
    // Old Testament - Torah
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,

    // Historical books
    Joshua,
    Judges,
    Ruth,
    FirstSamuel,
    SecondSamuel,
    FirstKings,
    SecondKings,
    FirstChronicles,
    SecondChronicles,
    Ezra,
    Nehemiah,

    // Catholic deuterocanonical historical additions
    Tobit,        // deuterocanonical
    Judith,       // deuterocanonical

    Esther,
    AdditionsToEsther, // deuterocanonical (additions/chapters in some editions)

    FirstMaccabees,    // deuterocanonical
    SecondMaccabees,   // deuterocanonical

    // Wisdom / Poetry
    Job,
    Psalms,
    Proverbs,
    Ecclesiastes,
    SongOfSongs,
    Wisdom,       // Wisdom of Solomon (deuterocanonical)
    Sirach,       // Ecclesiasticus (deuterocanonical)

    // Major Prophets
    Isaiah,
    Jeremiah,
    Lamentations,
    Baruch,               // deuterocanonical (often with Letter of Jeremiah)
    LetterOfJeremiah,     // sometimes treated as part of Baruch

    Ezekiel,
    Daniel,
    // Daniel additions (deuterocanonical)
    PrayerOfAzariah,      // "Song of the Three Holy Children" / Prayer of Azariah
    Susanna,
    BelAndTheDragon,

    // Minor Prophets
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,

    // New Testament - Gospels
    Matthew,
    Mark,
    Luke,
    John,

    // History
    Acts,

    // Pauline Epistles
    Romans,
    FirstCorinthians,
    SecondCorinthians,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    FirstThessalonians,
    SecondThessalonians,
    FirstTimothy,
    SecondTimothy,
    Titus,
    Philemon,

    // General Epistles
    Hebrews,
    James,
    FirstPeter,
    SecondPeter,
    FirstJohn,
    SecondJohn,
    ThirdJohn,
    Jude,

    // Prophecy
    Revelation,
}

impl From<BookName> for Testament {
    fn from(value: BookName) -> Self {
        todo!()
    }
}

impl From<BookName> for Genre {
    fn from(value: BookName) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Testament {
    Old,
    New,
    Deuterocanonical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Genre {
    Torah,
    History,
    Wisdom,
    Prophets,
    Gospel,
    Epistle,
    Apocalypse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TranslationMetaData {
    name: String,
    description: String,
    link: Url,
    release: Year,
    languages: Vec<Language>,
    equivalence_level: EquivalenceLevel,
    /// Describes the organisation or person who funded it.
    funded_by: Option<String>,
}

/// describes equivulance of a translation
/// 0 means extreamly formal - word for word
/// 255 means extreamly functional - meaning
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct EquivalenceLevel(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Year(u16);

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
