use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bible {
    pub books: Vec<Book>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Book {
    pub name: String,
    pub chapters: Vec<Chapter>,
}

impl Book {
    pub fn get_book_name(&self) -> Option<BookName> {
        BookName::from_kjv_name(&self.name)
    }
    
    pub fn is_standard_book(&self) -> bool {
        self.get_book_name().is_some()
    }
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

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    BookAmount(u32),
    ChapterAmount(u32),
    VerseAmount(u32),
    SuspiciousVerseLength(Verse),
    SuspiciousChapterLength(Chapter),
    InvalidBookName(String),
}

impl Bible {
    pub fn is_valid(&self) -> Result<(), ValidationError> {
        // Standard Bible has 66 books
        if self.books.len() != 66 {
            return Err(ValidationError::BookAmount(self.books.len() as u32));
        }

        // Verify book order and names match KJV standard
        let expected_books = BookName::all_books_in_order();
        for (i, (expected_book, actual_book)) in expected_books.iter().zip(self.books.iter()).enumerate() {
            let expected_name = expected_book.to_kjv_name();
            if expected_name != actual_book.name {
                // Check if it's an invalid book name or wrong order
                if BookName::from_kjv_name(&actual_book.name).is_none() {
                    return Err(ValidationError::InvalidBookName(actual_book.name.clone()));
                } else {
                    return Err(ValidationError::BookAmount(i as u32 + 1));
                }
            }
        }

        // Check for any invalid book names that might be in wrong positions
        for book in &self.books {
            if BookName::from_kjv_name(&book.name).is_none() {
                return Err(ValidationError::InvalidBookName(book.name.clone()));
            }
        }

        // Check each book
        for book in &self.books {
            for chapter in &book.chapters {
                // Check for suspicious chapter length
                if chapter.suspicious_verse_amount() {
                    return Err(ValidationError::SuspiciousChapterLength(chapter.clone()));
                }

                // Check each verse
                for verse in &chapter.verses {
                    if verse.suspicious() {
                        return Err(ValidationError::SuspiciousVerseLength(verse.clone()));
                    }
                }
            }
        }

        Ok(())
    }

    pub fn verify_book_order(&self) -> bool {
        if self.books.len() != 66 {
            return false;
        }
        
        let expected_books = BookName::all_books_in_order();
        for (expected_book, actual_book) in expected_books.iter().zip(self.books.iter()) {
            if expected_book.to_kjv_name() != actual_book.name {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BookName {
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
    Joshua,
    Judges,
    Ruth,
    ISamuel,
    IISamuel,
    IKings,
    IIKings,
    IChronicles,
    IIChronicles,
    Ezra,
    Nehemiah,
    Esther,
    Job,
    Psalms,
    Proverbs,
    Ecclesiastes,
    SongOfSolomon,
    Isaiah,
    Jeremiah,
    Lamentations,
    Ezekiel,
    Daniel,
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
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    ICorinthians,
    IICorinthians,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    IThessalonians,
    IIThessalonians,
    ITimothy,
    IITimothy,
    Titus,
    Philemon,
    Hebrews,
    James,
    IPeter,
    IIPeter,
    IJohn,
    IIJohn,
    IIIJohn,
    Jude,
    RevelationOfJohn,
}

impl BookName {
    pub fn from_kjv_name(name: &str) -> Option<Self> {
        match name {
            "Genesis" => Some(Self::Genesis),
            "Exodus" => Some(Self::Exodus),
            "Leviticus" => Some(Self::Leviticus),
            "Numbers" => Some(Self::Numbers),
            "Deuteronomy" => Some(Self::Deuteronomy),
            "Joshua" => Some(Self::Joshua),
            "Judges" => Some(Self::Judges),
            "Ruth" => Some(Self::Ruth),
            "I Samuel" => Some(Self::ISamuel),
            "II Samuel" => Some(Self::IISamuel),
            "I Kings" => Some(Self::IKings),
            "II Kings" => Some(Self::IIKings),
            "I Chronicles" => Some(Self::IChronicles),
            "II Chronicles" => Some(Self::IIChronicles),
            "Ezra" => Some(Self::Ezra),
            "Nehemiah" => Some(Self::Nehemiah),
            "Esther" => Some(Self::Esther),
            "Job" => Some(Self::Job),
            "Psalms" => Some(Self::Psalms),
            "Proverbs" => Some(Self::Proverbs),
            "Ecclesiastes" => Some(Self::Ecclesiastes),
            "Song of Solomon" => Some(Self::SongOfSolomon),
            "Isaiah" => Some(Self::Isaiah),
            "Jeremiah" => Some(Self::Jeremiah),
            "Lamentations" => Some(Self::Lamentations),
            "Ezekiel" => Some(Self::Ezekiel),
            "Daniel" => Some(Self::Daniel),
            "Hosea" => Some(Self::Hosea),
            "Joel" => Some(Self::Joel),
            "Amos" => Some(Self::Amos),
            "Obadiah" => Some(Self::Obadiah),
            "Jonah" => Some(Self::Jonah),
            "Micah" => Some(Self::Micah),
            "Nahum" => Some(Self::Nahum),
            "Habakkuk" => Some(Self::Habakkuk),
            "Zephaniah" => Some(Self::Zephaniah),
            "Haggai" => Some(Self::Haggai),
            "Zechariah" => Some(Self::Zechariah),
            "Malachi" => Some(Self::Malachi),
            "Matthew" => Some(Self::Matthew),
            "Mark" => Some(Self::Mark),
            "Luke" => Some(Self::Luke),
            "John" => Some(Self::John),
            "Acts" => Some(Self::Acts),
            "Romans" => Some(Self::Romans),
            "I Corinthians" => Some(Self::ICorinthians),
            "II Corinthians" => Some(Self::IICorinthians),
            "Galatians" => Some(Self::Galatians),
            "Ephesians" => Some(Self::Ephesians),
            "Philippians" => Some(Self::Philippians),
            "Colossians" => Some(Self::Colossians),
            "I Thessalonians" => Some(Self::IThessalonians),
            "II Thessalonians" => Some(Self::IIThessalonians),
            "I Timothy" => Some(Self::ITimothy),
            "II Timothy" => Some(Self::IITimothy),
            "Titus" => Some(Self::Titus),
            "Philemon" => Some(Self::Philemon),
            "Hebrews" => Some(Self::Hebrews),
            "James" => Some(Self::James),
            "I Peter" => Some(Self::IPeter),
            "II Peter" => Some(Self::IIPeter),
            "I John" => Some(Self::IJohn),
            "II John" => Some(Self::IIJohn),
            "III John" => Some(Self::IIIJohn),
            "Jude" => Some(Self::Jude),
            "Revelation of John" => Some(Self::RevelationOfJohn),
            _ => None,
        }
    }

    pub fn to_kjv_name(&self) -> &'static str {
        match self {
            Self::Genesis => "Genesis",
            Self::Exodus => "Exodus",
            Self::Leviticus => "Leviticus",
            Self::Numbers => "Numbers",
            Self::Deuteronomy => "Deuteronomy",
            Self::Joshua => "Joshua",
            Self::Judges => "Judges",
            Self::Ruth => "Ruth",
            Self::ISamuel => "I Samuel",
            Self::IISamuel => "II Samuel",
            Self::IKings => "I Kings",
            Self::IIKings => "II Kings",
            Self::IChronicles => "I Chronicles",
            Self::IIChronicles => "II Chronicles",
            Self::Ezra => "Ezra",
            Self::Nehemiah => "Nehemiah",
            Self::Esther => "Esther",
            Self::Job => "Job",
            Self::Psalms => "Psalms",
            Self::Proverbs => "Proverbs",
            Self::Ecclesiastes => "Ecclesiastes",
            Self::SongOfSolomon => "Song of Solomon",
            Self::Isaiah => "Isaiah",
            Self::Jeremiah => "Jeremiah",
            Self::Lamentations => "Lamentations",
            Self::Ezekiel => "Ezekiel",
            Self::Daniel => "Daniel",
            Self::Hosea => "Hosea",
            Self::Joel => "Joel",
            Self::Amos => "Amos",
            Self::Obadiah => "Obadiah",
            Self::Jonah => "Jonah",
            Self::Micah => "Micah",
            Self::Nahum => "Nahum",
            Self::Habakkuk => "Habakkuk",
            Self::Zephaniah => "Zephaniah",
            Self::Haggai => "Haggai",
            Self::Zechariah => "Zechariah",
            Self::Malachi => "Malachi",
            Self::Matthew => "Matthew",
            Self::Mark => "Mark",
            Self::Luke => "Luke",
            Self::John => "John",
            Self::Acts => "Acts",
            Self::Romans => "Romans",
            Self::ICorinthians => "I Corinthians",
            Self::IICorinthians => "II Corinthians",
            Self::Galatians => "Galatians",
            Self::Ephesians => "Ephesians",
            Self::Philippians => "Philippians",
            Self::Colossians => "Colossians",
            Self::IThessalonians => "I Thessalonians",
            Self::IIThessalonians => "II Thessalonians",
            Self::ITimothy => "I Timothy",
            Self::IITimothy => "II Timothy",
            Self::Titus => "Titus",
            Self::Philemon => "Philemon",
            Self::Hebrews => "Hebrews",
            Self::James => "James",
            Self::IPeter => "I Peter",
            Self::IIPeter => "II Peter",
            Self::IJohn => "I John",
            Self::IIJohn => "II John",
            Self::IIIJohn => "III John",
            Self::Jude => "Jude",
            Self::RevelationOfJohn => "Revelation of John",
        }
    }

    pub fn all_books_in_order() -> [Self; 66] {
        [
            Self::Genesis, Self::Exodus, Self::Leviticus, Self::Numbers, Self::Deuteronomy,
            Self::Joshua, Self::Judges, Self::Ruth, Self::ISamuel, Self::IISamuel,
            Self::IKings, Self::IIKings, Self::IChronicles, Self::IIChronicles, Self::Ezra,
            Self::Nehemiah, Self::Esther, Self::Job, Self::Psalms, Self::Proverbs,
            Self::Ecclesiastes, Self::SongOfSolomon, Self::Isaiah, Self::Jeremiah, Self::Lamentations,
            Self::Ezekiel, Self::Daniel, Self::Hosea, Self::Joel, Self::Amos,
            Self::Obadiah, Self::Jonah, Self::Micah, Self::Nahum, Self::Habakkuk,
            Self::Zephaniah, Self::Haggai, Self::Zechariah, Self::Malachi, Self::Matthew,
            Self::Mark, Self::Luke, Self::John, Self::Acts, Self::Romans,
            Self::ICorinthians, Self::IICorinthians, Self::Galatians, Self::Ephesians, Self::Philippians,
            Self::Colossians, Self::IThessalonians, Self::IIThessalonians, Self::ITimothy, Self::IITimothy,
            Self::Titus, Self::Philemon, Self::Hebrews, Self::James, Self::IPeter,
            Self::IIPeter, Self::IJohn, Self::IIJohn, Self::IIIJohn, Self::Jude,
            Self::RevelationOfJohn,
        ]
    }
}

impl Chapter {
    pub fn suspicious_verse_amount(&self) -> bool {
        let verse_count = self.verses.len();
        verse_count < 3 || verse_count > 200
    }
}

impl Verse {
    pub fn suspicious(&self) -> bool {
        let words_count = self.text.split_whitespace().count();
        words_count < 3 || words_count > 150
    }
}
