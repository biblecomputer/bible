use super::types::Instruction;
use leptos::web_sys::KeyboardEvent;

#[derive(Debug, Clone, PartialEq)]
pub struct KeyCombination {
    pub key: String,
    pub shift: bool,
    pub ctrl: bool,
    pub meta: bool,
    pub alt: bool,
}

impl KeyCombination {
    pub fn from_event(e: &KeyboardEvent) -> Self {
        Self {
            key: e.key(),
            shift: e.shift_key(),
            ctrl: e.ctrl_key(),
            meta: e.meta_key(),
            alt: e.alt_key(),
        }
    }
    
    pub fn simple(key: &str) -> Self {
        Self {
            key: key.to_string(),
            shift: false,
            ctrl: false,
            meta: false,
            alt: false,
        }
    }
    
    pub fn with_shift(key: &str) -> Self {
        Self {
            key: key.to_string(),
            shift: true,
            ctrl: false,
            meta: false,
            alt: false,
        }
    }
    
    pub fn with_ctrl(key: &str) -> Self {
        Self {
            key: key.to_string(),
            shift: false,
            ctrl: true,
            meta: false,
            alt: false,
        }
    }
    
    pub fn with_meta(key: &str) -> Self {
        Self {
            key: key.to_string(),
            shift: false,
            ctrl: false,
            meta: true,
            alt: false,
        }
    }
    
    pub fn with_ctrl_shift(key: &str) -> Self {
        Self {
            key: key.to_string(),
            shift: true,
            ctrl: true,
            meta: false,
            alt: false,
        }
    }
    
    pub fn with_meta_or_ctrl(key: &str) -> Self {
        // For cross-platform compatibility (Cmd on Mac, Ctrl on others)
        Self {
            key: key.to_string(),
            shift: false,
            ctrl: true, // This will be checked as (ctrl OR meta) in mapping
            meta: false,
            alt: false,
        }
    }
}

pub struct KeyboardMapper;

impl KeyboardMapper {
    pub fn map_to_instruction(combination: &KeyCombination) -> Instruction {
        // Handle numeric input for verse navigation
        if let Ok(verse_num) = combination.key.parse::<u32>() {
            if !combination.shift && !combination.ctrl && !combination.meta && !combination.alt {
                return Instruction::GoToVerse(verse_num);
            }
        }
        
        match (
            combination.key.as_str(),
            combination.shift,
            combination.ctrl,
            combination.meta,
            combination.alt,
        ) {
            // Basic navigation
            ("ArrowRight" | "l", false, false, false, false) => Instruction::NextChapter,
            ("ArrowLeft" | "h", false, false, false, false) => Instruction::PreviousChapter,
            ("ArrowDown" | "j", false, false, false, false) => Instruction::NextVerse,
            ("ArrowUp" | "k", false, false, false, false) => Instruction::PreviousVerse,
            
            // Book navigation
            ("H", true, false, false, false) => Instruction::PreviousBook,
            ("L", true, false, false, false) => Instruction::NextBook,
            
            // Chapter jumping
            ("g", false, false, false, false) => Instruction::PendingG,
            ("G", true, false, false, false) => Instruction::EndOfChapter,
            
            // Special navigation
            ("s", false, false, false, false) => Instruction::SwitchToPreviousChapter,
            
            // Copy operations
            ("c", false, false, false, false) => Instruction::CopyRawVerse,
            ("C", true, false, false, false) | ("c", true, false, false, false) => {
                Instruction::CopyVerseWithReference
            }
            
            // UI toggles
            ("b", false, true, false, false) => Instruction::ToggleSidebar,
            ("r", false, false, false, false) => Instruction::ToggleCrossReferences,
            ("R", true, true, false, false) => Instruction::ToggleCrossReferences,
            
            // Command palette (Cmd/Ctrl+K)
            ("k", false, true, false, false) | ("k", false, false, true, false) => {
                Instruction::OpenCommandPalette
            }
            
            _ => Instruction::NoOp,
        }
    }
    
    pub fn should_handle_key(combination: &KeyCombination) -> bool {
        !matches!(KeyboardMapper::map_to_instruction(combination), Instruction::NoOp)
    }
}