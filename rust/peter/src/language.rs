use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    English,
    Dutch,
}

impl Language {
    fn to_short_name(&self) -> String {
        match self {
            Language::English => String::from("en"),
            Language::Dutch => String::from("nl"),
        }
    }
}
