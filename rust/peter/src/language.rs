use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    English,
    Dutch,
}

impl Default for Language {
    fn default() -> Self {
        Self::English
    }
}

// Improve this later for better  translated words.
// English -> English
// Dutch -> Nederlands
impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
