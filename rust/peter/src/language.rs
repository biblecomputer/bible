use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    English,
    Dutch,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::English => write!(f, "English"),
            Language::Dutch => write!(f, "Dutch"),
        }
    }
}

impl Language {
    fn to_short_name(&self) -> String {
        match self {
            Language::English => String::from("en"),
            Language::Dutch => String::from("nl"),
        }
    }
}
