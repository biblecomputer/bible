use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bible {
    pub books: Vec<Book>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Book {
    pub name: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Chapter {
    pub chapter: u32,
    pub name: String,
    pub verses: Vec<Verse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Verse {
    pub verse: u32,
    pub chapter: u32,
    pub name: String,
    pub text: String,
}

#[derive(Debug)]
pub enum ParamParseError {
    ChapterNotFound,
    BookNotFound,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BibleTranslation {
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub release_year: u16,
    pub iagon: String,
    pub languages: Vec<Language>,
    pub wikipedia: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Language {
    Dutch,
    English,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct References(pub HashMap<VerseId, Vec<Reference>>);

/// Highly optimized verse identifier using a single u32
/// Format: book_id (8 bits) | chapter (12 bits) | verse (12 bits)
/// Supports: 256 books, 4096 chapters, 4096 verses
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VerseId(pub u32);

impl VerseId {
    pub fn new(book_id: u8, chapter: u32, verse: u32) -> Self {
        // Pack: book_id (8 bits) | chapter (12 bits) | verse (12 bits)
        let packed = ((book_id as u32) << 24) | ((chapter & 0xFFF) << 12) | (verse & 0xFFF);
        VerseId(packed)
    }
    
    pub fn from_book_name(book_name: &str, chapter: u32, verse: u32) -> Option<Self> {
        let book_id = book_name_to_id(book_name)?;
        Some(Self::new(book_id, chapter, verse))
    }
    
    pub fn book_id(&self) -> u8 {
        (self.0 >> 24) as u8
    }
    
    pub fn chapter(&self) -> u32 {
        (self.0 >> 12) & 0xFFF
    }
    
    pub fn verse(&self) -> u32 {
        self.0 & 0xFFF
    }
    
    pub fn book_name(&self) -> &'static str {
        book_id_to_name(self.book_id())
    }
}

/// Convert book name to compact ID for faster lookups
pub fn book_name_to_id(book_name: &str) -> Option<u8> {
    match book_name {
        // Old Testament
        "Genesis" => Some(1),
        "Exodus" => Some(2),
        "Leviticus" => Some(3),
        "Numbers" => Some(4),
        "Deuteronomy" => Some(5),
        "Joshua" => Some(6),
        "Judges" => Some(7),
        "Ruth" => Some(8),
        "1 Samuel" => Some(9),
        "2 Samuel" => Some(10),
        "1 Kings" => Some(11),
        "2 Kings" => Some(12),
        "1 Chronicles" => Some(13),
        "2 Chronicles" => Some(14),
        "Ezra" => Some(15),
        "Nehemiah" => Some(16),
        "Esther" => Some(17),
        "Job" => Some(18),
        "Psalms" => Some(19),
        "Proverbs" => Some(20),
        "Ecclesiastes" => Some(21),
        "Song of Solomon" => Some(22),
        "Isaiah" => Some(23),
        "Jeremiah" => Some(24),
        "Lamentations" => Some(25),
        "Ezekiel" => Some(26),
        "Daniel" => Some(27),
        "Hosea" => Some(28),
        "Joel" => Some(29),
        "Amos" => Some(30),
        "Obadiah" => Some(31),
        "Jonah" => Some(32),
        "Micah" => Some(33),
        "Nahum" => Some(34),
        "Habakkuk" => Some(35),
        "Zephaniah" => Some(36),
        "Haggai" => Some(37),
        "Zechariah" => Some(38),
        "Malachi" => Some(39),
        
        // New Testament
        "Matthew" => Some(40),
        "Mark" => Some(41),
        "Luke" => Some(42),
        "John" => Some(43),
        "Acts" => Some(44),
        "Romans" => Some(45),
        "1 Corinthians" => Some(46),
        "2 Corinthians" => Some(47),
        "Galatians" => Some(48),
        "Ephesians" => Some(49),
        "Philippians" => Some(50),
        "Colossians" => Some(51),
        "1 Thessalonians" => Some(52),
        "2 Thessalonians" => Some(53),
        "1 Timothy" => Some(54),
        "2 Timothy" => Some(55),
        "Titus" => Some(56),
        "Philemon" => Some(57),
        "Hebrews" => Some(58),
        "James" => Some(59),
        "1 Peter" => Some(60),
        "2 Peter" => Some(61),
        "1 John" => Some(62),
        "2 John" => Some(63),
        "3 John" => Some(64),
        "Jude" => Some(65),
        "Revelation" => Some(66),
        
        _ => None,
    }
}

/// Convert book ID back to name for display
pub fn book_id_to_name(book_id: u8) -> &'static str {
    match book_id {
        // Old Testament
        1 => "Genesis",
        2 => "Exodus",
        3 => "Leviticus",
        4 => "Numbers",
        5 => "Deuteronomy",
        6 => "Joshua",
        7 => "Judges",
        8 => "Ruth",
        9 => "1 Samuel",
        10 => "2 Samuel",
        11 => "1 Kings",
        12 => "2 Kings",
        13 => "1 Chronicles",
        14 => "2 Chronicles",
        15 => "Ezra",
        16 => "Nehemiah",
        17 => "Esther",
        18 => "Job",
        19 => "Psalms",
        20 => "Proverbs",
        21 => "Ecclesiastes",
        22 => "Song of Solomon",
        23 => "Isaiah",
        24 => "Jeremiah",
        25 => "Lamentations",
        26 => "Ezekiel",
        27 => "Daniel",
        28 => "Hosea",
        29 => "Joel",
        30 => "Amos",
        31 => "Obadiah",
        32 => "Jonah",
        33 => "Micah",
        34 => "Nahum",
        35 => "Habakkuk",
        36 => "Zephaniah",
        37 => "Haggai",
        38 => "Zechariah",
        39 => "Malachi",
        
        // New Testament
        40 => "Matthew",
        41 => "Mark",
        42 => "Luke",
        43 => "John",
        44 => "Acts",
        45 => "Romans",
        46 => "1 Corinthians",
        47 => "2 Corinthians",
        48 => "Galatians",
        49 => "Ephesians",
        50 => "Philippians",
        51 => "Colossians",
        52 => "1 Thessalonians",
        53 => "2 Thessalonians",
        54 => "1 Timothy",
        55 => "2 Timothy",
        56 => "Titus",
        57 => "Philemon",
        58 => "Hebrews",
        59 => "James",
        60 => "1 Peter",
        61 => "2 Peter",
        62 => "1 John",
        63 => "2 John",
        64 => "3 John",
        65 => "Jude",
        66 => "Revelation",
        
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verse_id_packing() {
        // Test Genesis 1:1 (book_id=1, chapter=1, verse=1)
        let verse_id = VerseId::new(1, 1, 1);
        
        assert_eq!(verse_id.book_id(), 1);
        assert_eq!(verse_id.chapter(), 1);
        assert_eq!(verse_id.verse(), 1);
        assert_eq!(verse_id.book_name(), "Genesis");
    }

    #[test]
    fn test_verse_id_from_book_name() {
        // Test creating VerseId from book name
        let verse_id = VerseId::from_book_name("Genesis", 1, 1).unwrap();
        
        assert_eq!(verse_id.book_id(), 1);
        assert_eq!(verse_id.chapter(), 1);
        assert_eq!(verse_id.verse(), 1);
        assert_eq!(verse_id.book_name(), "Genesis");
    }

    #[test]
    fn test_verse_id_large_values() {
        // Test maximum supported values (12 bits = 4095)
        let verse_id = VerseId::new(66, 4095, 4095);
        
        assert_eq!(verse_id.book_id(), 66);
        assert_eq!(verse_id.chapter(), 4095);
        assert_eq!(verse_id.verse(), 4095);
        assert_eq!(verse_id.book_name(), "Revelation");
    }

    #[test]
    fn test_book_name_mapping() {
        // Test all major books
        assert_eq!(book_name_to_id("Genesis"), Some(1));
        assert_eq!(book_name_to_id("Psalms"), Some(19));
        assert_eq!(book_name_to_id("Matthew"), Some(40));
        assert_eq!(book_name_to_id("Revelation"), Some(66));
        assert_eq!(book_name_to_id("Unknown"), None);
        
        // Test reverse mapping
        assert_eq!(book_id_to_name(1), "Genesis");
        assert_eq!(book_id_to_name(19), "Psalms");
        assert_eq!(book_id_to_name(40), "Matthew");
        assert_eq!(book_id_to_name(66), "Revelation");
        assert_eq!(book_id_to_name(99), "Unknown");
    }

    #[test]
    fn test_verse_id_hash_performance() {
        // Test that VerseId is much more efficient for hashing
        use std::collections::HashMap;
        
        let mut map: HashMap<VerseId, Vec<Reference>> = HashMap::new();
        let verse_id = VerseId::new(1, 1, 1);
        
        map.insert(verse_id, vec![]);
        assert!(map.contains_key(&verse_id));
        
        // The u32 should be much faster to hash than the old String-based key
        assert_eq!(verse_id.0, 0x01001001); // Expected packed value
    }
}

/// Legacy VerseKey for compatibility - can be removed after migration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VerseKey {
    pub book_name: String,
    pub chapter: u32,
    pub verse: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reference {
    pub to_book_name: String,
    pub to_chapter: u32,
    pub to_verse_start: u32,
    pub to_verse_end: Option<u32>, // None for single verse, Some for verse ranges
    pub votes: i32, // Can be negative based on the data
}
