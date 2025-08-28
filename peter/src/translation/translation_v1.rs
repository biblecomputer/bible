use crate::language::Language;
use std::collections::BTreeMap;

struct Translation {
    meta: TranslationMetaData,
    books: Books,
}


struct Books(BTreeMap<BookName, Book>)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

enum Testament {
    Old,
    New,
    Deuterocanonical,
}

enum Genre {
    Torah,
    History,
    Wisdom,
    Prophets,
    Gospel,
    Epistle,
    Apocalypse,
}

struct TranslationMetaData {
    name: String,
    description: String,
    link: Url,
    release: Year,
    languages: Vec<Language>,
    equivalence_level: EquivalenceLevel,
    /// Describes the organisation or person who funded it.
    funded_by: String,
}

struct Url(String);

/// describes equivulance of a translation
/// 0 means extreamly formal - word for word
/// 255 means extreamly functional - meaning
struct EquivalenceLevel(u8);

struct Year(u16);

struct Book {
    name: String,
    chapters: Chapters,
}

struct Chapters(BTreeMap<ChapterID, Chapter>)

struct ChapterID(u32);

struct Chapter {
    verses: Vec<Verse>,
    verse_sections: HashMap<VerseID, String>,
}

struct Verse {
    verse_id: VerseID,
    content: String,
    footnotes: Option<String>,
}

enum VerseID {
    Single(u32),
    // right should be greater then left.
    Range(u32, u32),
}
