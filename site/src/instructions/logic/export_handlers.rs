use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

/// Handle PDF export functionality
pub fn handle_export_to_pdf(
    set_is_pdf_exporting: WriteSignal<bool>,
    set_pdf_progress: WriteSignal<f32>,
    set_pdf_status: WriteSignal<String>,
) {
    web_sys::console::log_1(&"🎯 PDF Export instruction received!".into());
    let set_is_pdf_exporting = set_is_pdf_exporting.clone();
    let set_pdf_progress = set_pdf_progress.clone();
    let set_pdf_status = set_pdf_status.clone();
    spawn_local(async move {
        web_sys::console::log_1(&"🚀 Setting PDF export flags...".into());
        set_is_pdf_exporting.set(true);
        set_pdf_progress.set(0.0);
        set_pdf_status.set("Getting Bible data...".to_string());
        web_sys::console::log_1(&"✅ PDF export flags set".into());

        web_sys::console::log_1(&"🔄 Getting current Bible data...".into());
        let bible = crate::core::get_current_bible().unwrap_or_else(|| {
            web_sys::console::log_1(&"⚠️ No current Bible found, using default".into());
            crate::core::get_bible().clone()
        });
        web_sys::console::log_1(
            &format!("✅ Bible data obtained with {} books", bible.books.len()).into(),
        );

        // Create progress callback
        let progress_callback = {
            let set_progress = set_pdf_progress.clone();
            let set_status = set_pdf_status.clone();
            move |progress: f32, status: String| {
                set_progress.set(progress);
                set_status.set(status);
            }
        };

        web_sys::console::log_1(&"🔄 Starting PDF generation...".into());
        match crate::instructions::logic::export_bible_to_pdf(&bible, Some(progress_callback)) {
            Ok(pdf_bytes) => {
                web_sys::console::log_1(
                    &format!("✅ PDF generation successful! {} bytes", pdf_bytes.len()).into(),
                );
                set_pdf_status.set("Preparing download...".to_string());

                let translation_info = crate::storage::translations::get_current_translation()
                    .unwrap_or_else(|| {
                        web_sys::console::log_1(
                            &"⚠️ No translation info found, using default".into(),
                        );
                        crate::storage::translation_storage::BibleTranslation {
                            name: "Unknown_Bible".to_string(),
                            short_name: "unknown".to_string(),
                            release_year: 2024,
                            languages: vec![],
                            iagon: "".to_string(),
                        }
                    });
                let filename = format!("{}_Bible.pdf", translation_info.name.replace(" ", "_"));
                web_sys::console::log_1(&format!("📁 Generated filename: {}", filename).into());

                web_sys::console::log_1(&"🔽 Triggering PDF download...".into());
                crate::instructions::logic::trigger_pdf_download(pdf_bytes, &filename);
            }
            Err(e) => {
                web_sys::console::log_1(&format!("❌ Failed to generate PDF: {:?}", e).into());
                set_pdf_status.set("Export failed!".to_string());
            }
        }
        set_is_pdf_exporting.set(false);
    });
}

/// Handle Markdown export functionality
pub fn handle_export_to_markdown(
    set_is_pdf_exporting: WriteSignal<bool>,
    set_pdf_progress: WriteSignal<f32>,
    set_pdf_status: WriteSignal<String>,
) {
    web_sys::console::log_1(&"📝 Markdown Export instruction received!".into());
    let set_is_pdf_exporting = set_is_pdf_exporting.clone();
    let set_pdf_progress = set_pdf_progress.clone();
    let set_pdf_status = set_pdf_status.clone();
    spawn_local(async move {
        web_sys::console::log_1(&"🚀 Setting Markdown export flags...".into());
        set_is_pdf_exporting.set(true);
        set_pdf_progress.set(0.0);
        set_pdf_status.set("Getting Bible data...".to_string());
        web_sys::console::log_1(&"✅ Markdown export flags set".into());

        web_sys::console::log_1(&"🔄 Getting current Bible data...".into());
        let bible = crate::core::get_current_bible().unwrap_or_else(|| {
            web_sys::console::log_1(&"⚠️ No current Bible found, using default".into());
            crate::core::get_bible().clone()
        });
        web_sys::console::log_1(
            &format!("✅ Bible data obtained with {} books", bible.books.len()).into(),
        );

        // Create progress callback
        let progress_callback = {
            let set_progress = set_pdf_progress.clone();
            let set_status = set_pdf_status.clone();
            move |progress: f32, status: String| {
                set_progress.set(progress);
                set_status.set(status);
            }
        };

        web_sys::console::log_1(&"🔄 Starting Markdown generation...".into());
        match crate::instructions::logic::export_bible_to_markdown(&bible, Some(progress_callback))
        {
            Ok(markdown_content) => {
                web_sys::console::log_1(
                    &format!(
                        "✅ Markdown generation successful! {} characters",
                        markdown_content.len()
                    )
                    .into(),
                );
                set_pdf_status.set("Preparing download...".to_string());

                let translation_info = crate::storage::translations::get_current_translation()
                    .unwrap_or_else(|| {
                        web_sys::console::log_1(
                            &"⚠️ No translation info found, using default".into(),
                        );
                        crate::storage::translation_storage::BibleTranslation {
                            name: "Unknown_Bible".to_string(),
                            short_name: "unknown".to_string(),
                            release_year: 2024,
                            languages: vec![],
                            iagon: "".to_string(),
                        }
                    });
                let filename = format!("{}_Bible.md", translation_info.name.replace(" ", "_"));
                web_sys::console::log_1(&format!("📁 Generated filename: {}", filename).into());

                web_sys::console::log_1(&"🔽 Triggering Markdown download...".into());
                crate::instructions::logic::trigger_markdown_download(markdown_content, &filename);
            }
            Err(e) => {
                web_sys::console::log_1(&format!("❌ Failed to generate Markdown: {:?}", e).into());
                set_pdf_status.set("Export failed!".to_string());
            }
        }
        set_is_pdf_exporting.set(false);
    });
}

/// Handle Linked Markdown export functionality
pub fn handle_export_linked_markdown(
    set_is_pdf_exporting: WriteSignal<bool>,
    set_pdf_progress: WriteSignal<f32>,
    set_pdf_status: WriteSignal<String>,
) {
    web_sys::console::log_1(&"🔗 Linked Markdown Export instruction received!".into());
    let set_is_pdf_exporting = set_is_pdf_exporting.clone();
    let set_pdf_progress = set_pdf_progress.clone();
    let set_pdf_status = set_pdf_status.clone();
    spawn_local(async move {
        web_sys::console::log_1(&"🚀 Setting Linked Markdown export flags...".into());
        set_is_pdf_exporting.set(true);
        set_pdf_progress.set(0.0);
        set_pdf_status.set("Getting Bible data...".to_string());
        web_sys::console::log_1(&"✅ Linked Markdown export flags set".into());

        web_sys::console::log_1(&"🔄 Getting current Bible data...".into());
        let bible = crate::core::get_current_bible().unwrap_or_else(|| {
            web_sys::console::log_1(&"⚠️ No current Bible found, using default".into());
            crate::core::get_bible().clone()
        });
        web_sys::console::log_1(
            &format!("✅ Bible data obtained with {} books", bible.books.len()).into(),
        );

        // Create progress callback
        let progress_callback = {
            let set_progress = set_pdf_progress.clone();
            let set_status = set_pdf_status.clone();
            move |progress: f32, status: String| {
                set_progress.set(progress);
                set_status.set(status);
            }
        };

        web_sys::console::log_1(&"🔄 Starting Linked Markdown generation...".into());
        match crate::instructions::logic::export_bible_to_linked_markdown(
            &bible,
            Some(progress_callback),
        ) {
            Ok(linked_export) => {
                web_sys::console::log_1(
                    &format!(
                        "✅ Linked Markdown generation successful! {} files created",
                        linked_export.files.len()
                    )
                    .into(),
                );
                set_pdf_status.set("Preparing download...".to_string());

                let translation_info = crate::storage::translations::get_current_translation()
                    .unwrap_or_else(|| {
                        web_sys::console::log_1(
                            &"⚠️ No translation info found, using default".into(),
                        );
                        crate::storage::translation_storage::BibleTranslation {
                            name: "Unknown_Bible".to_string(),
                            short_name: "unknown".to_string(),
                            release_year: 2024,
                            languages: vec![],
                            iagon: "".to_string(),
                        }
                    });
                let filename = format!(
                    "{}_Obsidian_Vault.zip",
                    translation_info.name.replace(" ", "_")
                );
                web_sys::console::log_1(&format!("📁 Generated filename: {}", filename).into());

                web_sys::console::log_1(&"🔽 Triggering Linked Markdown download...".into());
                crate::instructions::logic::trigger_linked_markdown_download(
                    linked_export,
                    &filename,
                );
            }
            Err(e) => {
                web_sys::console::log_1(
                    &format!("❌ Failed to generate Linked Markdown: {:?}", e).into(),
                );
                set_pdf_status.set("Export failed!".to_string());
            }
        }
        set_is_pdf_exporting.set(false);
    });
}
