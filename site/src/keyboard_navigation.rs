use leptos::ev;
use leptos::prelude::*;
use leptos::web_sys::KeyboardEvent;
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::NavigateOptions;
use urlencoding::encode;
use wasm_bindgen_futures::spawn_local;

use crate::core::{get_bible, parse_verse_ranges_from_url, Chapter};
use crate::instructions::{
    Instruction, InstructionContext, InstructionProcessor, VimKeyboardMapper,
};
use crate::storage::{
    save_references_sidebar_open, save_sidebar_open, save_verse_visibility,
};
use crate::utils::is_mobile_screen;

/// Helper function to create instruction context from URL
fn create_instruction_context(pathname: &str, search: &str) -> Option<InstructionContext> {
    let path_parts: Vec<&str> = pathname.trim_start_matches('/').split('/').collect();
    if path_parts.len() == 2 {
        let book_name = path_parts[0].replace('_', " ");
        if let Ok(chapter_num) = path_parts[1].parse::<u32>() {
            if let Ok(current_chapter) = get_bible().get_chapter(&book_name, chapter_num) {
                return Some(InstructionContext::new(
                    current_chapter,
                    search.to_string(),
                ));
            }
        }
    }
    None
}

#[component]
pub fn KeyboardNavigationHandler(
    palette_open: ReadSignal<bool>,
    set_palette_open: WriteSignal<bool>,
    _left_sidebar_open: ReadSignal<bool>,
    set_left_sidebar_open: WriteSignal<bool>,
    _right_sidebar_open: ReadSignal<bool>,
    set_right_sidebar_open: WriteSignal<bool>,
    _theme_sidebar_open: ReadSignal<bool>,
    set_theme_sidebar_open: WriteSignal<bool>,
    _translation_comparison_open: ReadSignal<bool>,
    set_translation_comparison_open: WriteSignal<bool>,
    _verse_visibility_enabled: ReadSignal<bool>,
    set_verse_visibility_enabled: WriteSignal<bool>,
    next_palette_result: RwSignal<bool>,
    previous_palette_result: RwSignal<bool>,
    set_initial_search_query: WriteSignal<Option<String>>,
) -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();

    // Previous chapter tracking for "alt-tab" like switching
    let (previous_chapter_path, set_previous_chapter_path) = signal(Option::<String>::None);
    
    // PDF export progress state
    let (pdf_progress, set_pdf_progress) = signal(0.0f32);
    let (pdf_status, set_pdf_status) = signal("Preparing export...".to_string());
    let (is_pdf_exporting, set_is_pdf_exporting) = signal(false);

    // Reactive effect to track all path changes
    {
        let mut last_path = String::new();
        Effect::new(move |_| {
            let current_path = location.pathname.get();
            if !last_path.is_empty() && last_path != current_path {
                set_previous_chapter_path.set(Some(last_path.clone()));
            }
            last_path = current_path;
        });
    }

    // Create instruction processor and vim keyboard mapper
    let processor = InstructionProcessor::new(navigate.clone());
    let (vim_mapper, set_vim_mapper) = signal(VimKeyboardMapper::new());

    // Visual display for vim command buffer
    let vim_display = Memo::new(move |_| {
        let mapper = vim_mapper.get();
        let display = mapper.get_current_input_display();
        if display.is_empty() {
            None
        } else {
            Some(display)
        }
    });

    // Cache location reads to avoid repeated reactive access during rapid navigation
    let cached_pathname = Memo::new(move |_| location.pathname.get());
    let cached_search = Memo::new(move |_| location.search.get());

    // Set up keyboard event handler
    let handle_keydown = move |e: KeyboardEvent| {
        // Check if user is typing in an input field BEFORE processing vim mapper
        let is_typing_in_input = if let Some(window) = leptos::web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(active_element) = document.active_element() {
                    // Check if the active element is an input, textarea, or has contenteditable
                    let tag_name = active_element.tag_name().to_lowercase();
                    tag_name == "input" 
                        || tag_name == "textarea"
                        || active_element.get_attribute("contenteditable").is_some()
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        // If user is typing in input and palette is open, only intercept specific control keys
        if palette_open.get() && is_typing_in_input {
            // Only process Ctrl+J, Ctrl+K, Ctrl+O, Escape, Enter for palette navigation
            let key = e.key();
            let is_control_sequence = e.ctrl_key() && (key == "j" || key == "k" || key == "o")
                || key == "Escape" 
                || key == "Enter"
                || key == "ArrowUp" 
                || key == "ArrowDown";
                
            if !is_control_sequence {
                // Let normal typing behavior work
                return;
            }
        }

        // Get instruction from vim-style keyboard mapper
        let mut mapper = vim_mapper.get();
        let instruction_result = mapper.map_to_instruction(&e);

        // Handle palette navigation priority when palette is open
        if palette_open.get() {
            if let Some((ref instruction, _)) = instruction_result {
                match instruction {
                    Instruction::NextPaletteResult | Instruction::PreviousPaletteResult => {
                        // Let palette navigation instructions through to be processed below
                    }
                    Instruction::ToggleBiblePallate | Instruction::ToggleCommandPallate => {
                        // Let palette toggle instructions through to be processed below
                    }
                    Instruction::ToggleSidebar | Instruction::ToggleCrossReferences | Instruction::ToggleThemeSidebar | Instruction::ToggleVerseVisibility => {
                        // Let UI toggle instructions through to be processed below
                    }
                    Instruction::NextReference | Instruction::PreviousReference => {
                        // Block reference navigation when palette is open
                        e.prevent_default();
                        return;
                    }
                    Instruction::NextVerse | Instruction::PreviousVerse => {
                        // Block verse navigation when palette is open (arrow keys should navigate palette)
                        e.prevent_default();
                        return;
                    }
                    _ => {
                        // Skip all other keyboard processing when palette is open
                        return;
                    }
                }
            } else {
                // No instruction, let palette handle regular keyboard input
                return;
            }
        }

        // Update the mapper state if needed
        // This prevents unnecessary reactive updates during rapid navigation
        if mapper.has_pending_sequence() || instruction_result.is_some() {
            set_vim_mapper.set(mapper);
        }

        // Handle instruction if we got one
        if let Some((instruction, multiplier)) = instruction_result {
            // Handle UI-specific instructions that need direct component access
            match instruction {
                Instruction::ToggleBiblePallate => {
                    e.prevent_default();
                    let is_currently_open = palette_open.get();
                    set_palette_open.set(!is_currently_open);
                    // Close sidebar on mobile when command palette opens
                    if !is_currently_open && is_mobile_screen() {
                        set_left_sidebar_open.set(false);
                        save_sidebar_open(false);
                    }
                    return;
                }
                Instruction::ToggleCommandPallate => {
                    e.prevent_default();
                    // Open the command palette with ">" pre-filled
                    set_initial_search_query.set(Some(">".to_string()));
                    set_palette_open.set(true);
                    // Close sidebar on mobile when command palette opens
                    if is_mobile_screen() {
                        set_left_sidebar_open.set(false);
                        save_sidebar_open(false);
                    }
                    return;
                }
                Instruction::ToggleVersePallate => {
                    e.prevent_default();
                    // Open the command palette with ":" pre-filled
                    set_initial_search_query.set(Some(":".to_string()));
                    set_palette_open.set(true);
                    // Close sidebar on mobile when command palette opens
                    if is_mobile_screen() {
                        set_left_sidebar_open.set(false);
                        save_sidebar_open(false);
                    }
                    return;
                }
                Instruction::OpenGithubRepository => {
                    e.prevent_default();
                    if let Some(window) = leptos::web_sys::window() {
                        let _ = window.location().set_href("https://github.com/sempruijs/bible");
                    }
                    return;
                }
                Instruction::ToggleSidebar => {
                    e.prevent_default();
                    set_left_sidebar_open.update(|open| {
                        *open = !*open;
                        save_sidebar_open(*open);
                    });
                    return;
                }
                Instruction::ToggleCrossReferences => {
                    e.prevent_default();
                    set_right_sidebar_open.update(|open| {
                        *open = !*open;
                        save_references_sidebar_open(*open);
                        // Close theme sidebar if opening references sidebar
                        if *open {
                            set_theme_sidebar_open.set(false);
                        }
                    });
                    return;
                }
                Instruction::ToggleThemeSidebar => {
                    e.prevent_default();
                    set_theme_sidebar_open.update(|open| {
                        *open = !*open;
                        // Close references sidebar if opening theme sidebar
                        if *open {
                            set_right_sidebar_open.set(false);
                            save_references_sidebar_open(false);
                        }
                    });
                    return;
                }
                Instruction::ToggleTranslationComparison => {
                    e.prevent_default();
                    set_translation_comparison_open.update(|open| {
                        *open = !*open;
                        // Close other right-side panels if opening comparison panel
                        if *open {
                            set_right_sidebar_open.set(false);
                            set_theme_sidebar_open.set(false);
                            save_references_sidebar_open(false);
                        }
                    });
                    return;
                }
                Instruction::ToggleVerseVisibility => {
                    e.prevent_default();
                    set_verse_visibility_enabled.update(|visible| {
                        *visible = !*visible;
                        save_verse_visibility(*visible);
                    });
                    return;
                }
                Instruction::ExportToPDF => {
                    e.prevent_default();
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
                        web_sys::console::log_1(&format!("✅ Bible data obtained with {} books", bible.books.len()).into());
                        
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
                        match crate::utils::export_bible_to_pdf(&bible, Some(progress_callback)) {
                            Ok(pdf_bytes) => {
                                web_sys::console::log_1(&format!("✅ PDF generation successful! {} bytes", pdf_bytes.len()).into());
                                set_pdf_status.set("Preparing download...".to_string());
                                
                                let translation_info = crate::storage::translations::get_current_translation().unwrap_or_else(|| {
                                    web_sys::console::log_1(&"⚠️ No translation info found, using default".into());
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
                                crate::utils::trigger_pdf_download(pdf_bytes, &filename);
                            }
                            Err(e) => {
                                web_sys::console::log_1(&format!("❌ Failed to generate PDF: {:?}", e).into());
                                set_pdf_status.set("Export failed!".to_string());
                            }
                        }
                        set_is_pdf_exporting.set(false);
                    });
                    return;
                }
                Instruction::ExportToMarkdown => {
                    e.prevent_default();
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
                        web_sys::console::log_1(&format!("✅ Bible data obtained with {} books", bible.books.len()).into());
                        
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
                        match crate::utils::export_bible_to_markdown(&bible, Some(progress_callback)) {
                            Ok(markdown_content) => {
                                web_sys::console::log_1(&format!("✅ Markdown generation successful! {} characters", markdown_content.len()).into());
                                set_pdf_status.set("Preparing download...".to_string());
                                
                                let translation_info = crate::storage::translations::get_current_translation().unwrap_or_else(|| {
                                    web_sys::console::log_1(&"⚠️ No translation info found, using default".into());
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
                                crate::utils::trigger_markdown_download(markdown_content, &filename);
                            }
                            Err(e) => {
                                web_sys::console::log_1(&format!("❌ Failed to generate Markdown: {:?}", e).into());
                                set_pdf_status.set("Export failed!".to_string());
                            }
                        }
                        set_is_pdf_exporting.set(false);
                    });
                    return;
                }
                Instruction::ExportLinkedMarkdown => {
                    e.prevent_default();
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
                        web_sys::console::log_1(&format!("✅ Bible data obtained with {} books", bible.books.len()).into());
                        
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
                        match crate::utils::export_bible_to_linked_markdown(&bible, Some(progress_callback)) {
                            Ok(linked_export) => {
                                web_sys::console::log_1(&format!("✅ Linked Markdown generation successful! {} files created", linked_export.files.len()).into());
                                set_pdf_status.set("Preparing download...".to_string());
                                
                                let translation_info = crate::storage::translations::get_current_translation().unwrap_or_else(|| {
                                    web_sys::console::log_1(&"⚠️ No translation info found, using default".into());
                                    crate::storage::translation_storage::BibleTranslation {
                                        name: "Unknown_Bible".to_string(),
                                        short_name: "unknown".to_string(),
                                        release_year: 2024,
                                        languages: vec![],
                                        iagon: "".to_string(),
                                    }
                                });
                                let filename = format!("{}_Obsidian_Vault.zip", translation_info.name.replace(" ", "_"));
                                web_sys::console::log_1(&format!("📁 Generated filename: {}", filename).into());
                                
                                web_sys::console::log_1(&"🔽 Triggering Linked Markdown download...".into());
                                crate::utils::trigger_linked_markdown_download(linked_export, &filename);
                            }
                            Err(e) => {
                                web_sys::console::log_1(&format!("❌ Failed to generate Linked Markdown: {:?}", e).into());
                                set_pdf_status.set("Export failed!".to_string());
                            }
                        }
                        set_is_pdf_exporting.set(false);
                    });
                    return;
                }
                Instruction::NextReference => {
                    e.prevent_default();
                    // Cross-references will handle this via keyboard events
                    // This should only be reached when palette is NOT open
                    return;
                }
                Instruction::PreviousReference => {
                    e.prevent_default();
                    // Cross-references will handle this via keyboard events
                    // This should only be reached when palette is NOT open
                    return;
                }
                Instruction::NextPaletteResult => {
                    e.prevent_default();
                    if palette_open.get() {
                        // Command palette is open, trigger navigation in palette
                        next_palette_result.set(true);
                    }
                    return;
                }
                Instruction::PreviousPaletteResult => {
                    e.prevent_default();
                    if palette_open.get() {
                        // Command palette is open, trigger navigation in palette
                        previous_palette_result.set(true);
                    }
                    return;
                }
                Instruction::SwitchToPreviousChapter => {
                    e.prevent_default();
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
                    return;
                }
                Instruction::GoToVerse(verse_num) => {
                    // Handle go to verse navigation
                    e.prevent_default();

                    // Process the instruction if we have a valid context
                    let pathname = location.pathname.get();
                    let search = location.search.get();
                    if let Some(context) = create_instruction_context(&pathname, &search) {
                        processor.process(Instruction::GoToVerse(verse_num), &context);
                    }
                    return;
                }
                _ => {
                    // For all other instructions, create context and process
                    let pathname = cached_pathname.get();
                    let search = cached_search.get();

                    if let Some(context) = create_instruction_context(&pathname, &search) {
                        e.prevent_default();
                        processor.process_with_multiplier(instruction, &context, multiplier);
                    }
                }
            }
        }
    };

    // Add global keydown listener - this runs only once when the App mounts
    window_event_listener(ev::keydown, handle_keydown);
    
    // Add CustomEvent listener for command palette PDF export
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    
    let set_is_pdf_exporting_custom = set_is_pdf_exporting.clone();
    let set_pdf_progress_custom = set_pdf_progress.clone();
    let set_pdf_status_custom = set_pdf_status.clone();
    let custom_event_handler = Closure::wrap(Box::new(move |_event: web_sys::Event| {
        web_sys::console::log_1(&"🎯 CustomEvent received from command palette!".into());
        let set_is_pdf_exporting = set_is_pdf_exporting_custom.clone();
        let set_progress = set_pdf_progress_custom.clone();
        let set_status = set_pdf_status_custom.clone();
        spawn_local(async move {
            set_is_pdf_exporting.set(true);
            set_progress.set(0.0);
            set_status.set("Getting Bible data...".to_string());
            
            web_sys::console::log_1(&"🔄 Getting current Bible data via CustomEvent...".into());
            let bible = crate::core::get_current_bible().unwrap_or_else(|| {
                web_sys::console::log_1(&"⚠️ No current Bible found, using default".into());
                crate::core::get_bible().clone()
            });
            
            // Create progress callback
            let progress_callback = {
                let set_progress = set_progress.clone();
                let set_status = set_status.clone();
                move |progress: f32, status: String| {
                    set_progress.set(progress);
                    set_status.set(status);
                }
            };
            
            web_sys::console::log_1(&"🔄 Starting PDF generation via CustomEvent...".into());
            match crate::utils::export_bible_to_pdf(&bible, Some(progress_callback)) {
                Ok(pdf_bytes) => {
                    web_sys::console::log_1(&format!("✅ PDF generation successful! {} bytes", pdf_bytes.len()).into());
                    set_status.set("Preparing download...".to_string());
                    
                    let translation_info = crate::storage::translations::get_current_translation().unwrap_or_else(|| {
                        web_sys::console::log_1(&"⚠️ No translation info found, using default".into());
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
                    crate::utils::trigger_pdf_download(pdf_bytes, &filename);
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("❌ Failed to generate PDF: {:?}", e).into());
                    set_status.set("Export failed!".to_string());
                }
            }
            set_is_pdf_exporting.set(false);
        });
    }) as Box<dyn FnMut(_)>);
    
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            let _ = document.add_event_listener_with_callback(
                "palette-pdf-export",
                custom_event_handler.as_ref().unchecked_ref(),
            );
            // Keep the closure alive by forgetting it
            custom_event_handler.forget();
            
            // Add CustomEvent listener for command palette Markdown export
            let set_is_pdf_exporting_markdown = set_is_pdf_exporting.clone();
            let set_pdf_progress_markdown = set_pdf_progress.clone();
            let set_pdf_status_markdown = set_pdf_status.clone();
            let markdown_event_handler = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                web_sys::console::log_1(&"📝 Markdown CustomEvent received from command palette!".into());
                let set_is_pdf_exporting = set_is_pdf_exporting_markdown.clone();
                let set_progress = set_pdf_progress_markdown.clone();
                let set_status = set_pdf_status_markdown.clone();
                spawn_local(async move {
                    set_is_pdf_exporting.set(true);
                    set_progress.set(0.0);
                    set_status.set("Getting Bible data...".to_string());
                    
                    web_sys::console::log_1(&"🔄 Getting current Bible data via Markdown CustomEvent...".into());
                    let bible = crate::core::get_current_bible().unwrap_or_else(|| {
                        web_sys::console::log_1(&"⚠️ No current Bible found, using default".into());
                        crate::core::get_bible().clone()
                    });
                    
                    // Create progress callback
                    let progress_callback = {
                        let set_progress = set_progress.clone();
                        let set_status = set_status.clone();
                        move |progress: f32, status: String| {
                            set_progress.set(progress);
                            set_status.set(status);
                        }
                    };
                    
                    web_sys::console::log_1(&"🔄 Starting Markdown generation via CustomEvent...".into());
                    match crate::utils::export_bible_to_markdown(&bible, Some(progress_callback)) {
                        Ok(markdown_content) => {
                            web_sys::console::log_1(&format!("✅ Markdown generation successful! {} characters", markdown_content.len()).into());
                            set_status.set("Preparing download...".to_string());
                            
                            let translation_info = crate::storage::translations::get_current_translation().unwrap_or_else(|| {
                                web_sys::console::log_1(&"⚠️ No translation info found, using default".into());
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
                            crate::utils::trigger_markdown_download(markdown_content, &filename);
                        }
                        Err(e) => {
                            web_sys::console::log_1(&format!("❌ Failed to generate Markdown: {:?}", e).into());
                            set_status.set("Export failed!".to_string());
                        }
                    }
                    set_is_pdf_exporting.set(false);
                });
            }) as Box<dyn FnMut(_)>);
            
            let _ = document.add_event_listener_with_callback(
                "palette-markdown-export",
                markdown_event_handler.as_ref().unchecked_ref(),
            );
            // Keep the closure alive by forgetting it
            markdown_event_handler.forget();
            
            // Add CustomEvent listener for command palette Linked Markdown export
            let set_is_pdf_exporting_linked = set_is_pdf_exporting.clone();
            let set_pdf_progress_linked = set_pdf_progress.clone();
            let set_pdf_status_linked = set_pdf_status.clone();
            let linked_markdown_event_handler = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                web_sys::console::log_1(&"🔗 Linked Markdown CustomEvent received from command palette!".into());
                let set_is_pdf_exporting = set_is_pdf_exporting_linked.clone();
                let set_progress = set_pdf_progress_linked.clone();
                let set_status = set_pdf_status_linked.clone();
                spawn_local(async move {
                    set_is_pdf_exporting.set(true);
                    set_progress.set(0.0);
                    set_status.set("Getting Bible data...".to_string());
                    
                    web_sys::console::log_1(&"🔄 Getting current Bible data via Linked Markdown CustomEvent...".into());
                    let bible = crate::core::get_current_bible().unwrap_or_else(|| {
                        web_sys::console::log_1(&"⚠️ No current Bible found, using default".into());
                        crate::core::get_bible().clone()
                    });
                    
                    // Create progress callback
                    let progress_callback = {
                        let set_progress = set_progress.clone();
                        let set_status = set_status.clone();
                        move |progress: f32, status: String| {
                            set_progress.set(progress);
                            set_status.set(status);
                        }
                    };
                    
                    web_sys::console::log_1(&"🔄 Starting Linked Markdown generation via CustomEvent...".into());
                    match crate::utils::export_bible_to_linked_markdown(&bible, Some(progress_callback)) {
                        Ok(linked_export) => {
                            web_sys::console::log_1(&format!("✅ Linked Markdown generation successful! {} files created", linked_export.files.len()).into());
                            set_status.set("Preparing download...".to_string());
                            
                            let translation_info = crate::storage::translations::get_current_translation().unwrap_or_else(|| {
                                web_sys::console::log_1(&"⚠️ No translation info found, using default".into());
                                crate::storage::translation_storage::BibleTranslation {
                                    name: "Unknown_Bible".to_string(),
                                    short_name: "unknown".to_string(),
                                    release_year: 2024,
                                    languages: vec![],
                                    iagon: "".to_string(),
                                }
                            });
                            let filename = format!("{}_Obsidian_Vault.md", translation_info.name.replace(" ", "_"));
                            web_sys::console::log_1(&format!("📁 Generated filename: {}", filename).into());
                            
                            web_sys::console::log_1(&"🔽 Triggering Linked Markdown download...".into());
                            crate::utils::trigger_linked_markdown_download(linked_export, &filename);
                        }
                        Err(e) => {
                            web_sys::console::log_1(&format!("❌ Failed to generate Linked Markdown: {:?}", e).into());
                            set_status.set("Export failed!".to_string());
                        }
                    }
                    set_is_pdf_exporting.set(false);
                });
            }) as Box<dyn FnMut(_)>);
            
            let _ = document.add_event_listener_with_callback(
                "palette-linked-markdown-export",
                linked_markdown_event_handler.as_ref().unchecked_ref(),
            );
            // Keep the closure alive by forgetting it
            linked_markdown_event_handler.forget();
        }
    }

    view! {
        // Visual feedback for vim command buffer
        <Show when=move || vim_display.get().is_some()>
            <div class="fixed top-4 right-4 bg-black bg-opacity-75 text-white px-3 py-2 rounded-lg text-sm font-mono z-50">
                {move || vim_display.get().unwrap_or_default()}
            </div>
        </Show>
        
        // PDF export progress component
        <crate::components::PdfLoadingProgress 
            progress=pdf_progress
            status_message=pdf_status 
            is_visible=is_pdf_exporting 
        />
    }
}