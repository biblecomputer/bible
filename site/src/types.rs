use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bible {
    pub books: Vec<Book>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub name: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub chapter: u32,
    pub name: String,
    pub verses: Vec<Verse>,
}

impl Chapter {
    pub fn to_path(&self) -> String {
        // Trim off the final " <number>" part to get the book name
        let mut name = self.name.trim_end().to_string();

        if let Some(pos) = name.rfind(' ') {
            name.truncate(pos); // remove everything after the last space
        }

        let book = name.replace(' ', "_");
        format!("{}/{}", book, self.chapter)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verse {
    pub verse: u32,
    pub chapter: u32,
    pub name: String,
    pub text: String,
}
