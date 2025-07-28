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
pub struct References(pub HashMap<VerseKey, Vec<Reference>>);

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
