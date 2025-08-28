use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationV0 {
    pub books: Vec<Book>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub name: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub chapter: u32,
    pub name: String,
    pub verses: Vec<Verse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verse {
    pub verse: u32,
    pub chapter: u32,
    pub name: String,
    pub text: String,
}

impl TryFrom<&str> for TranslationV0 {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}
