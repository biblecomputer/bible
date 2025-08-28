use crate::language::Language;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationMetaData {
    pub full_name: String,
    pub short_name: String,
    pub description: String,
    pub link: Url,
    pub release: Year,
    pub languages: Vec<Language>,
    pub equivalence_level: EquivalenceLevel,
    /// Describes the organisation or person who funded it.
    pub funded_by: Option<String>,
}

/// describes equivulance of a translation
/// 0 means extreamly formal - word for word
/// 255 means extreamly functional - meaning
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EquivalenceLevel(pub u8);

impl EquivalenceLevel {
    pub fn new(level: u8) -> Self {
        EquivalenceLevel(level)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Year(pub i32);

impl Year {
    pub fn new(year: i32) -> Self {
        Year(year)
    }
}
