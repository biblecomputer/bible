/*!
 * Instructions Module
 * 
 * This module provides the complete instruction system for the Bible application.
 * It handles keyboard shortcuts, command processing, and user interface actions.
 * 
 * # Architecture
 * 
 * The instruction system follows a multi-layered architecture:
 * 
 * ## 1. Instruction Types (`types.rs`)
 * Defines all possible user instructions as an enum. Instructions are categorized
 * by functionality (navigation, UI toggles, copy operations, etc.)
 * 
 * ## 2. Keyboard Mapping System
 * - `keyboard_mappings.json`: JSON configuration for vim-style key bindings
 * - `keyboard_map.rs`: Hardcoded key combinations for complex mappings
 * - `vim_keys.rs`: Vim-style key parser and mapping engine
 * 
 * ## 3. Instruction Processing (`processor.rs`)
 * Handles the execution of instructions, including context creation and
 * business logic for each instruction type.
 * 
 * # Key Features
 * 
 * - **Vim-style navigation**: hjkl keys, gg/G shortcuts, numeric prefixes
 * - **Multiple input methods**: Keyboard shortcuts, command palette, programmatic
 * - **Configurable shortcuts**: JSON-based configuration with fallback to hardcoded
 * - **Multi-key sequences**: Support for sequences like "gg" for beginning of chapter
 * - **Modifier key support**: Ctrl, Alt, Meta, Shift combinations
 * - **Cross-platform compatibility**: Handles Mac Cmd vs Ctrl differences
 * 
 * # Usage Examples
 * 
 * ```rust
 * use crate::instructions::{Instruction, VimKeyboardMapper};
 * 
 * // Create keyboard mapper
 * let mut mapper = VimKeyboardMapper::new();
 * 
 * // Process keyboard event
 * if let Some((instruction, multiplier)) = mapper.map_to_instruction(&event) {
 *     // Execute the instruction
 *     match instruction {
 *         Instruction::NextVerse => { /* handle next verse */ },
 *         Instruction::GoToVerse(verse_num) => { /* go to specific verse */ },
 *         // ... other instructions
 *     }
 * }
 * ```
 */

// === Module Declarations ===

/// Instruction type definitions and categorization
pub mod types;

/// Keyboard mapping configurations and processors
pub mod keyboard_map;
pub mod vim_keys;

/// Instruction execution logic and context management
pub mod processor;

/// Instruction handler logic organized by functionality
pub mod logic;

// === Public Exports ===

pub use types::*;
pub use vim_keys::*;
pub use processor::*;
pub use logic::*;