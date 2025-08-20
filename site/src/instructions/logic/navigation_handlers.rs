use leptos::prelude::*;
use leptos_router::location::Location;
use leptos_router::NavigateOptions;
use crate::instructions::{Instruction, InstructionProcessor};
use crate::view_state::ViewStateSignal;

/// Helper function to create instruction context from URL
pub fn create_instruction_context(pathname: &str, search: &str) -> Option<crate::instructions::InstructionContext> {
    let path_parts: Vec<&str> = pathname.trim_start_matches('/').split('/').collect();
    if path_parts.len() == 2 {
        let book_name = path_parts[0].replace('_', " ");
        if let Ok(chapter_num) = path_parts[1].parse::<u32>() {
            if let Ok(current_chapter) = crate::core::get_bible().get_chapter(&book_name, chapter_num) {
                return Some(crate::instructions::InstructionContext::new(current_chapter, search.to_string()));
            }
        }
    }
    None
}

/// Handle opening the GitHub repository
pub fn handle_open_github_repository() {
    if let Some(window) = leptos::web_sys::window() {
        let _ = window
            .location()
            .set_href("https://github.com/sempruijs/bible");
    }
}

/// Handle switching to the previous chapter
pub fn handle_switch_to_previous_chapter<F>(
    previous_chapter_path: ReadSignal<Option<String>>,
    set_previous_chapter_path: WriteSignal<Option<String>>,
    location: Location,
    navigate: F,
) where
    F: Fn(&str, NavigateOptions),
{
    if let Some(prev_path) = previous_chapter_path.get() {
        let current_path = location.pathname.get();
        set_previous_chapter_path.set(Some(current_path));
        navigate(
            &prev_path,
            NavigateOptions {
                scroll: false,
                ..Default::default()
            },
        );
    }
}

/// Handle going to a specific verse
pub fn handle_go_to_verse<F>(
    verse_num: u32,
    location: Location,
    processor: &InstructionProcessor<F>,
) where
    F: Fn(&str, NavigateOptions) + Clone,
{
    // Process the instruction if we have a valid context
    let pathname = location.pathname.get();
    let search = location.search.get();
    if let Some(context) = create_instruction_context(&pathname, &search) {
        processor.process(Instruction::GoToVerse(verse_num), &context);
    }
}

/// Handle navigating to the next palette result
pub fn handle_next_palette_result(view_state: ViewStateSignal) {
    if view_state.with(|state| state.is_command_palette_open) {
        // Command palette is open, trigger navigation in palette
        view_state.update(|state| state.trigger_next_palette_result());
    }
}

/// Handle navigating to the previous palette result
pub fn handle_previous_palette_result(view_state: ViewStateSignal) {
    if view_state.with(|state| state.is_command_palette_open) {
        // Command palette is open, trigger navigation in palette
        view_state.update(|state| state.trigger_previous_palette_result());
    }
}