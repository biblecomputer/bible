use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::instructions::logic::export_handlers::{
    handle_export_to_pdf, handle_export_to_markdown, handle_export_linked_markdown
};

/// Set up custom event listeners for export functionality
pub fn setup_export_event_listeners(
    set_is_pdf_exporting: WriteSignal<bool>,
    set_pdf_progress: WriteSignal<f32>,
    set_pdf_status: WriteSignal<String>,
) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            // PDF export event listener
            let set_is_pdf_exporting_pdf = set_is_pdf_exporting.clone();
            let set_pdf_progress_pdf = set_pdf_progress.clone();
            let set_pdf_status_pdf = set_pdf_status.clone();
            let pdf_event_handler = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                web_sys::console::log_1(&"🎯 CustomEvent received from command palette!".into());
                handle_export_to_pdf(
                    set_is_pdf_exporting_pdf,
                    set_pdf_progress_pdf,
                    set_pdf_status_pdf,
                );
            }) as Box<dyn FnMut(_)>);

            let _ = document.add_event_listener_with_callback(
                "palette-pdf-export",
                pdf_event_handler.as_ref().unchecked_ref(),
            );
            pdf_event_handler.forget();

            // Markdown export event listener
            let set_is_pdf_exporting_md = set_is_pdf_exporting.clone();
            let set_pdf_progress_md = set_pdf_progress.clone();
            let set_pdf_status_md = set_pdf_status.clone();
            let markdown_event_handler = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                web_sys::console::log_1(
                    &"📝 Markdown CustomEvent received from command palette!".into(),
                );
                handle_export_to_markdown(
                    set_is_pdf_exporting_md,
                    set_pdf_progress_md,
                    set_pdf_status_md,
                );
            }) as Box<dyn FnMut(_)>);

            let _ = document.add_event_listener_with_callback(
                "palette-markdown-export",
                markdown_event_handler.as_ref().unchecked_ref(),
            );
            markdown_event_handler.forget();

            // Linked Markdown export event listener
            let linked_markdown_event_handler =
                Closure::wrap(Box::new(move |_event: web_sys::Event| {
                    web_sys::console::log_1(
                        &"🔗 Linked Markdown CustomEvent received from command palette!".into(),
                    );
                    handle_export_linked_markdown(
                        set_is_pdf_exporting,
                        set_pdf_progress,
                        set_pdf_status,
                    );
                }) as Box<dyn FnMut(_)>);

            let _ = document.add_event_listener_with_callback(
                "palette-linked-markdown-export",
                linked_markdown_event_handler.as_ref().unchecked_ref(),
            );
            linked_markdown_event_handler.forget();
        }
    }
}