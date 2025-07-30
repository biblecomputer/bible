use super::types::Instruction;
use leptos::web_sys::KeyboardEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct VimKey {
    pub key: String,
    pub shift: bool,
    pub ctrl: bool,
    pub meta: bool,
    pub alt: bool,
}

impl VimKey {
    pub fn from_vim_syntax(vim_key: &str) -> Option<Self> {
        let mut key = String::new();
        let mut shift = false;
        let mut ctrl = false;
        let mut meta = false;
        let mut alt = false;
        
        // Handle special key syntax like <C-k>, <S-G>, <M-k>
        if vim_key.starts_with('<') && vim_key.ends_with('>') {
            let inner = &vim_key[1..vim_key.len()-1];
            let parts: Vec<&str> = inner.split('-').collect();
            
            if parts.len() == 1 {
                // Simple special keys like <Up>, <Down>, <Left>, <Right>
                key = match parts[0] {
                    "Up" => "ArrowUp".to_string(),
                    "Down" => "ArrowDown".to_string(),
                    "Left" => "ArrowLeft".to_string(),
                    "Right" => "ArrowRight".to_string(),
                    other => other.to_string(),
                };
            } else if parts.len() == 2 {
                // Modified keys like <C-k>, <S-G>
                let modifier = parts[0];
                let base_key = parts[1];
                
                match modifier {
                    "C" => ctrl = true,
                    "S" => shift = true,
                    "M" => meta = true,
                    "A" => alt = true,
                    _ => return None,
                }
                
                key = base_key.to_string();
            } else if parts.len() == 3 {
                // Double modified keys like <C-S-R>
                let mod1 = parts[0];
                let mod2 = parts[1];
                let base_key = parts[2];
                
                for modifier in [mod1, mod2] {
                    match modifier {
                        "C" => ctrl = true,
                        "S" => shift = true,
                        "M" => meta = true,
                        "A" => alt = true,
                        _ => return None,
                    }
                }
                
                key = base_key.to_string();
            } else {
                return None;
            }
        } else {
            // Regular keys or multi-character sequences like "gg"
            key = vim_key.to_string();
        }
        
        Some(VimKey {
            key,
            shift,
            ctrl,
            meta,
            alt,
        })
    }
    
    pub fn matches_event(&self, e: &KeyboardEvent) -> bool {
        // For multi-character sequences like "gg", we need special handling
        if self.key.len() > 1 && !self.key.starts_with("Arrow") {
            return false; // Multi-char sequences handled separately
        }
        
        e.key() == self.key &&
        e.shift_key() == self.shift &&
        e.ctrl_key() == self.ctrl &&
        e.meta_key() == self.meta &&
        e.alt_key() == self.alt
    }
    
    pub fn is_multi_char_sequence(&self) -> bool {
        self.key.len() > 1 && !self.key.starts_with("Arrow")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KeyboardMappings {
    pub mappings: HashMap<String, String>,
    pub digit_mappings: HashMap<String, String>,
}

impl KeyboardMappings {
    pub fn load() -> Self {
        // In a real implementation, you'd load this from the JSON file
        // For now, we'll embed it directly
        let json_str = include_str!("keyboard_mappings.json");
        serde_json::from_str(json_str).expect("Failed to parse keyboard mappings")
    }
    
    pub fn get_instruction(&self, vim_key: &str) -> Option<Instruction> {
        if let Some(instruction_name) = self.mappings.get(vim_key) {
            self.parse_instruction(instruction_name)
        } else if let Some(digit) = vim_key.chars().next() {
            if digit.is_ascii_digit() {
                if let Some(instruction_name) = self.digit_mappings.get(&digit.to_string()) {
                    if instruction_name == "GoToVerse" {
                        if let Ok(verse_num) = vim_key.parse::<u32>() {
                            return Some(Instruction::GoToVerse(verse_num));
                        }
                    }
                }
            }
            None
        } else {
            None
        }
    }
    
    fn parse_instruction(&self, instruction_name: &str) -> Option<Instruction> {
        match instruction_name {
            "NextVerse" => Some(Instruction::NextVerse),
            "PreviousVerse" => Some(Instruction::PreviousVerse),
            "NextChapter" => Some(Instruction::NextChapter),
            "PreviousChapter" => Some(Instruction::PreviousChapter),
            "NextBook" => Some(Instruction::NextBook),
            "PreviousBook" => Some(Instruction::PreviousBook),
            "BeginningOfChapter" => Some(Instruction::BeginningOfChapter),
            "EndOfChapter" => Some(Instruction::EndOfChapter),
            "SwitchToPreviousChapter" => Some(Instruction::SwitchToPreviousChapter),
            "CopyRawVerse" => Some(Instruction::CopyRawVerse),
            "CopyVerseWithReference" => Some(Instruction::CopyVerseWithReference),
            "ToggleSidebar" => Some(Instruction::ToggleSidebar),
            "ToggleCrossReferences" => Some(Instruction::ToggleCrossReferences),
            "OpenCommandPalette" => Some(Instruction::OpenCommandPalette),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct VimKeyboardMapper {
    mappings: KeyboardMappings,
    sequence_buffer: String,
}

impl VimKeyboardMapper {
    pub fn new() -> Self {
        Self {
            mappings: KeyboardMappings::load(),
            sequence_buffer: String::new(),
        }
    }
    
    pub fn map_to_instruction(&mut self, e: &KeyboardEvent) -> Option<Instruction> {
        // Handle single digit input for verse navigation
        if let Some(digit) = e.key().chars().next() {
            if digit.is_ascii_digit() && !e.ctrl_key() && !e.meta_key() && !e.alt_key() && !e.shift_key() {
                if let Ok(verse_num) = digit.to_string().parse::<u32>() {
                    return Some(Instruction::GoToVerse(verse_num));
                }
            }
        }
        
        // Try to match single-key mappings first
        for (vim_key_str, _) in &self.mappings.mappings {
            if let Some(vim_key) = VimKey::from_vim_syntax(vim_key_str) {
                if !vim_key.is_multi_char_sequence() && vim_key.matches_event(e) {
                    self.sequence_buffer.clear(); // Clear any partial sequences
                    return self.mappings.get_instruction(vim_key_str);
                }
            }
        }
        
        // Handle multi-character sequences like "gg"
        if !e.ctrl_key() && !e.meta_key() && !e.alt_key() {
            self.sequence_buffer.push_str(&e.key());
            
            // Check if current buffer matches any multi-char sequence
            for (vim_key_str, _) in &self.mappings.mappings {
                if let Some(vim_key) = VimKey::from_vim_syntax(vim_key_str) {
                    if vim_key.is_multi_char_sequence() && vim_key.key == self.sequence_buffer {
                        let instruction = self.mappings.get_instruction(vim_key_str);
                        self.sequence_buffer.clear();
                        return instruction;
                    }
                }
            }
            
            // Check if current buffer is a prefix of any multi-char sequence
            let is_prefix = self.mappings.mappings.keys().any(|key| {
                if let Some(vim_key) = VimKey::from_vim_syntax(key) {
                    vim_key.is_multi_char_sequence() && vim_key.key.starts_with(&self.sequence_buffer)
                } else {
                    false
                }
            });
            
            if !is_prefix {
                // No potential matches, clear the buffer
                self.sequence_buffer.clear();
            }
        }
        
        None
    }
    
    pub fn clear_sequence_buffer(&mut self) {
        self.sequence_buffer.clear();
    }
    
    pub fn has_pending_sequence(&self) -> bool {
        !self.sequence_buffer.is_empty()
    }
}