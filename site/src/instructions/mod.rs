pub mod types;
pub mod processor;
pub mod vim_keys;

pub use types::*;
pub use processor::*;
pub use vim_keys::*;

use leptos::prelude::{ReadSignal, WriteSignal, create_signal, Set};
use std::sync::OnceLock;

// Global signal for dispatching instructions from command palette to main app
static GLOBAL_INSTRUCTION_SIGNAL: OnceLock<(ReadSignal<Option<Instruction>>, WriteSignal<Option<Instruction>>)> = OnceLock::new();

/// Initialize the global instruction signal (call this from main app)
pub fn init_global_instruction_signal() -> (ReadSignal<Option<Instruction>>, WriteSignal<Option<Instruction>>) {
    let signals = create_signal::<Option<Instruction>>(None);
    GLOBAL_INSTRUCTION_SIGNAL.set(signals).expect("Global instruction signal can only be initialized once");
    signals
}

/// Get the global instruction signal writer (for command palette)
pub fn get_global_instruction_writer() -> Option<WriteSignal<Option<Instruction>>> {
    GLOBAL_INSTRUCTION_SIGNAL.get().map(|(_, writer)| *writer)
}

/// Dispatch an instruction globally
pub fn dispatch_instruction(instruction: Instruction) {
    if let Some(writer) = get_global_instruction_writer() {
        writer.set(Some(instruction));
    }
}