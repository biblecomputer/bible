use super::bible_op::BibleOp;
use super::settings_op::SettingsOp;

/// Every operation flows through this
pub enum Op {
    Bible(BibleOp),
    Settings(SettingsOp),
}
