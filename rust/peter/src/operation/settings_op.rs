use crate::{language::Language, translation::Translation};

pub enum SettingsOp {
    SetLanguage(Language),
    SetDefaultTranslation(Translation),
}
