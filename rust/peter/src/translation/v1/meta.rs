use crate::language::Language;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationMetaData {
    name: String,
    description: String,
    link: Url,
    release: Year,
    languages: Vec<Language>,
    equivalence_level: EquivalenceLevel,
    /// Describes the organisation or person who funded it.
    funded_by: Option<String>,
}

/// describes equivulance of a translation
/// 0 means extreamly formal - word for word
/// 255 means extreamly functional - meaning
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EquivalenceLevel(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Year(i32);
