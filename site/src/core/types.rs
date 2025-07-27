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

// Commented out unused Translation struct to fix compilation
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct Translation {
//     // Old Testament
//     pub genesis: &str,
//     pub exodus: &str,
//     pub leviticus: &str,
//     pub numbers: &str,
//     pub deuteronomy: &str,
//     pub joshua: &str,
//     pub judges: &str,
//     pub ruth: &str,
//     pub first_samuel: &str,
//     pub second_samuel: &str,
//     pub first_kings: &str,
//     pub second_kings: &str,
//     pub first_chronicles: &str,
//     pub second_chronicles: &str,
//     pub ezra: &str,
//     pub nehemiah: &str,
//     pub esther: &str,
//     pub job: &str,
//     pub psalms: &str,
//     pub proverbs: &str,
//     pub ecclesiastes: &str,
//     pub song_of_solomon: &str,
//     pub isaiah: &str,
//     pub jeremiah: &str,
//     pub lamentations: &str,
//     pub ezekiel: &str,
//     pub daniel: &str,
//     pub hosea: &str,
//     pub joel: &str,
//     pub amos: &str,
//     pub obadiah: &str,
//     pub jonah: &str,
//     pub micah: &str,
//     pub nahum: &str,
//     pub habakkuk: &str,
//     pub zephaniah: &str,
//     pub haggai: &str,
//     pub zechariah: &str,
//     pub malachi: &str,
//     // New Testament
//     pub matthew: &str,
//     pub mark: &str,
//     pub luke: &str,
//     pub john: &str,
//     pub acts: &str,
//     pub romans: &str,
//     pub first_corinthians: &str,
//     pub second_corinthians: &str,
//     pub galatians: &str,
//     pub ephesians: &str,
//     pub philippians: &str,
//     pub colossians: &str,
//     pub first_thessalonians: &str,
//     pub second_thessalonians: &str,
//     pub first_timothy: &str,
//     pub second_timothy: &str,
//     pub titus: &str,
//     pub philemon: &str,
//     pub hebrews: &str,
//     pub james: &str,
//     pub first_peter: &str,
//     pub second_peter: &str,
//     pub first_john: &str,
//     pub second_john: &str,
//     pub third_john: &str,
//     pub jude: &str,
//     pub revelation: &str,
// }
