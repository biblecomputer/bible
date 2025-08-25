pub mod event_handlers;
pub mod export_handlers;
pub mod linked_markdown_export;
pub mod markdown_export;
pub mod navigation_handlers;
pub mod pdf_export;
pub mod ui_toggles;

// Re-export only the functions that are actually used
pub use navigation_handlers::update_view_state_from_url;

// Re-export business logic functions
pub use linked_markdown_export::{
    export_bible_to_linked_markdown, trigger_linked_markdown_download,
};
pub use markdown_export::{export_bible_to_markdown, trigger_markdown_download};
pub use pdf_export::{export_bible_to_pdf, trigger_pdf_download};
