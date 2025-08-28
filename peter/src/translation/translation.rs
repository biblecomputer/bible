use crate::translation::translation_v0::TranslationV0;
use crate::translation::translation_v0::TranslationV1;

pub enum Translation {
    V0(TranslationV0),
    V1(TranslationV1),
}
