use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationV0 {
    books: Vec<Book>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    name: String,
    chapters: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Chapter {
    chapter: u32,
    name: String,
    verses: Vec<Verse>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Verse {
    verse: u32,
    chapter: u32,
    name: String,
    text: String,
}
