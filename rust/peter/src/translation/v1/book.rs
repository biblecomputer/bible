use super::chapter::Chapters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    /// The name in the language of the translation
    pub name: String,
    /// Some translations wrote their own introduction about an bible book.
    pub introduction: Option<String>,
    pub chapters: Chapters,
}