use super::book_name::BookName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Testament {
    Old,
    New,
    Deuterocanonical,
}

impl From<BookName> for Testament {
    fn from(_book: BookName) -> Self {
        todo!()
    }
}
