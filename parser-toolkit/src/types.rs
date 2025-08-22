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
}

impl Bible {
    pub fn is_valid(&self) -> Result<(), ValidationError> {
        // Standard Bible has 66 books
        if self.books.len() != 66 {
            return Err(ValidationError::BookAmount(self.books.len() as u32));
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
}

enum Book {
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomium,
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
        word_count < 3 || words_count > 150
    }
}
