use super::book::Books;
use super::meta::TranslationMetaData;
use crate::storage::Storage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationV1 {
    pub meta: TranslationMetaData,
    pub books: Storage<Books>,
}