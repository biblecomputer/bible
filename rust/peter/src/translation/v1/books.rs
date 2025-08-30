use super::book::Book;
use super::book_name::BookName;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Books(
    #[serde(with = "super::serde_helpers::btreemap_as_tuple_list")] pub BTreeMap<BookName, Book>,
);
