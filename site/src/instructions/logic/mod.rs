pub mod export_handlers;
pub mod ui_toggles;
pub mod navigation_handlers;
pub mod event_handlers;
pub mod pdf_export;
pub mod markdown_export;
pub mod linked_markdown_export;

// Re-export commonly used functions for easier access
pub use export_handlers::{handle_export_to_pdf, handle_export_to_markdown, handle_export_linked_markdown};
pub use ui_toggles::{
    handle_toggle_bible_palette, handle_toggle_command_palette, handle_toggle_verse_palette,
    handle_toggle_sidebar, handle_toggle_cross_references, handle_toggle_theme_sidebar,
    handle_toggle_translation_comparison, handle_toggle_verse_visibility,
};
pub use navigation_handlers::{
    handle_open_github_repository, handle_switch_to_previous_chapter, handle_go_to_verse,
    handle_next_palette_result, handle_previous_palette_result, create_instruction_context,
};
pub use event_handlers::setup_export_event_listeners;

// Re-export business logic functions
pub use pdf_export::{export_bible_to_pdf, trigger_pdf_download};
pub use markdown_export::{export_bible_to_markdown, trigger_markdown_download};
pub use linked_markdown_export::{export_bible_to_linked_markdown, trigger_linked_markdown_download};