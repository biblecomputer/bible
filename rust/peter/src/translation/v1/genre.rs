use super::book_name::BookName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Genre {
    Torah,
    History,
    Wisdom,
    Prophets,
    Gospel,
    Epistle,
    Apocalypse,
}

impl From<BookName> for Genre {
    fn from(_book: BookName) -> Self {
        todo!()
    }
}
