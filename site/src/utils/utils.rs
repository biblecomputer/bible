use crate::instructions::Instruction;
use crate::view_state::{InstructionResult, ViewStateSignal};
use leptos::prelude::Update;
use leptos_router::NavigateOptions;

const MOBILE_BREAKPOINT: f64 = 768.0;

/// Helper function to execute an instruction and handle navigation results
/// This provides a clean interface for components to execute instructions without handling the result manually
pub fn execute_with_navigation<F>(
    view_state: ViewStateSignal,
    navigate: &F,
    instruction: Instruction,
) where
    F: Fn(&str, NavigateOptions) + Clone,
{
    let mut instruction_result = InstructionResult::Failed("Not executed".to_string());
    
    view_state.update(|state| {
        instruction_result = state.execute(&instruction);
    });
    
    match instruction_result {
        InstructionResult::Navigate(path) => {
            navigate(&path, NavigateOptions { scroll: false, ..Default::default() });
        }
        InstructionResult::Handled => {
            // Instruction was handled, nothing more to do
        }
        InstructionResult::Failed(_error) => {
            #[cfg(target_arch = "wasm32")]
            leptos::web_sys::console::error_1(&format!("❌ Instruction failed: {}", _error).into());
        }
        InstructionResult::NotHandled => {
            #[cfg(target_arch = "wasm32")]
            leptos::web_sys::console::warn_1(&format!("⚠️ Instruction not handled: {:?}", instruction).into());
        }
    }
}

pub fn is_mobile_screen() -> bool {
    if let Some(window) = leptos::web_sys::window() {
        if let Ok(width) = window.inner_width() {
            if let Some(width_num) = width.as_f64() {
                return width_num < MOBILE_BREAKPOINT;
            }
        }
    }
    false
}