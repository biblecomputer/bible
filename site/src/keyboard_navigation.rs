use leptos::ev;
use leptos::prelude::*;
use leptos::web_sys::KeyboardEvent;
use leptos_router::hooks::{use_location, use_navigate};

use crate::instructions::{
    Instruction, InstructionProcessor, VimKeyboardMapper,
    // Import all handler functions from the logic modules
    handle_export_to_pdf, handle_export_to_markdown, handle_export_linked_markdown,
    handle_toggle_bible_palette, handle_toggle_command_palette, handle_toggle_verse_palette,
    handle_toggle_sidebar, handle_toggle_cross_references, handle_toggle_theme_sidebar,
    handle_toggle_translation_comparison, handle_toggle_verse_visibility,
    handle_open_github_repository, handle_switch_to_previous_chapter, handle_go_to_verse,
    handle_next_palette_result, handle_previous_palette_result, setup_export_event_listeners,
    create_instruction_context,
};

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
                    Instruction::ToggleSidebar
                    | Instruction::ToggleCrossReferences
                    | Instruction::ToggleThemeSidebar
                    | Instruction::ToggleVerseVisibility => {
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
                    handle_toggle_bible_palette(
                        palette_open,
                        set_palette_open,
                        set_left_sidebar_open,
                    );
                    return;
                }
                Instruction::ToggleCommandPallate => {
                    e.prevent_default();
                    handle_toggle_command_palette(
                        set_palette_open,
                        set_initial_search_query,
                        set_left_sidebar_open,
                    );
                    return;
                }
                Instruction::ToggleVersePallate => {
                    e.prevent_default();
                    handle_toggle_verse_palette(
                        set_palette_open,
                        set_initial_search_query,
                        set_left_sidebar_open,
                    );
                    return;
                }
                Instruction::OpenGithubRepository => {
                    e.prevent_default();
                    handle_open_github_repository();
                    return;
                }
                Instruction::ToggleSidebar => {
                    e.prevent_default();
                    handle_toggle_sidebar(set_left_sidebar_open);
                    return;
                }
                Instruction::ToggleCrossReferences => {
                    e.prevent_default();
                    handle_toggle_cross_references(set_right_sidebar_open, set_theme_sidebar_open);
                    return;
                }
                Instruction::ToggleThemeSidebar => {
                    e.prevent_default();
                    handle_toggle_theme_sidebar(set_theme_sidebar_open, set_right_sidebar_open);
                    return;
                }
                Instruction::ToggleTranslationComparison => {
                    e.prevent_default();
                    handle_toggle_translation_comparison(
                        set_translation_comparison_open,
                        set_right_sidebar_open,
                        set_theme_sidebar_open,
                    );
                    return;
                }
                Instruction::ToggleVerseVisibility => {
                    e.prevent_default();
                    handle_toggle_verse_visibility(set_verse_visibility_enabled);
                    return;
                }
                Instruction::ExportToPDF => {
                    e.prevent_default();
                    handle_export_to_pdf(set_is_pdf_exporting, set_pdf_progress, set_pdf_status);
                    return;
                }
                Instruction::ExportToMarkdown => {
                    e.prevent_default();
                    handle_export_to_markdown(
                        set_is_pdf_exporting,
                        set_pdf_progress,
                        set_pdf_status,
                    );
                    return;
                }
                Instruction::ExportLinkedMarkdown => {
                    e.prevent_default();
                    handle_export_linked_markdown(
                        set_is_pdf_exporting,
                        set_pdf_progress,
                        set_pdf_status,
                    );
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
                    handle_next_palette_result(palette_open, next_palette_result);
                    return;
                }
                Instruction::PreviousPaletteResult => {
                    e.prevent_default();
                    handle_previous_palette_result(palette_open, previous_palette_result);
                    return;
                }
                Instruction::SwitchToPreviousChapter => {
                    e.prevent_default();
                    handle_switch_to_previous_chapter(
                        previous_chapter_path,
                        set_previous_chapter_path,
                        location.clone(),
                        navigate.clone(),
                    );
                    return;
                }
                Instruction::GoToVerse(verse_num) => {
                    e.prevent_default();
                    handle_go_to_verse(verse_num, location.clone(), &processor);
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

    // Set up export event listeners
    setup_export_event_listeners(set_is_pdf_exporting, set_pdf_progress, set_pdf_status);

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