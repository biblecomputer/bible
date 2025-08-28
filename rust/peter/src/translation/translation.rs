use crate::translation::translation_v0::TranslationV0;
use crate::translation::v1::Translation as TranslationV1;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Translation {
    V0(TranslationV0),
    V1(TranslationV1),
}

impl Translation {
    /// Export the translation as a JSON string in `.btrl` format.
    pub fn export_as_btrl(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl From<TranslationV0> for Translation {
    fn from(value: TranslationV0) -> Self {
        Translation::V0(value)
    }
}

impl TryFrom<&str> for Translation {
    type Error = ();

    fn try_from(_value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}
