use super::book_name::BookName;
use super::chapter::Chapters;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    /// The name in the language of the translation
    pub name: String,
    /// Some translations wrote their own introduction about an bible book.
    pub introduction: Option<String>,
    pub chapters: Chapters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Books(
    #[serde(with = "super::serde_helpers::btreemap_as_tuple_list")] pub BTreeMap<BookName, Book>,
);