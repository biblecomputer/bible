use crate::translation::translation_v0::TranslationV0;
use crate::translation::v1::Translation as TranslationV1;

pub enum Translation {
    V0(TranslationV0),
    V1(TranslationV1),
}

impl From<TranslationV0> for Translation {
    fn from(value: TranslationV0) -> Self {
        Translation::V0(value)
    }
}
