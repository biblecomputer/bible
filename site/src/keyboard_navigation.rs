use leptos::ev;
use leptos::prelude::*;
use leptos::web_sys::KeyboardEvent;
use leptos_router::hooks::{use_location, use_navigate};

use crate::instructions::{
    Instruction, InstructionProcessor,
    // Import handler functions that are still needed
    handle_export_to_pdf, handle_export_to_markdown, handle_export_linked_markdown,
    handle_toggle_bible_palette, handle_toggle_verse_palette,
    handle_open_github_repository,
    update_view_state_from_url,
};
use crate::view_state::ViewStateSignal;

#[component]
pub fn KeyboardNavigationHandler(
    view_state: ViewStateSignal,
) -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();

    // Note: Previous chapter tracking and export progress are now managed in ViewState

    // Reactive effect to track path changes and update ViewState
    {
        let mut last_path = String::new();
        Effect::new(move |_| {
            let current_path = location.pathname.get();
            let current_search = location.search.get();
            
            // Update ViewState with current navigation context
            update_view_state_from_url(view_state, &current_path, &current_search);
            
            // Track previous chapter path for alt-tab switching
            if !last_path.is_empty() && last_path != current_path {
                view_state.update(|state| {
                    state.set_previous_chapter_path(Some(last_path.clone()));
                });
            }
            last_path = current_path;
        });
    }

    // Note: VimKeyboardMapper is now managed in ViewState
    // Keep a local processor only for instructions that need external handling
    let processor = InstructionProcessor::new(navigate.clone());

    // Visual display for vim command buffer from ViewState
    let vim_display = Memo::new(move |_| {
        view_state.with(|state| {
            let display = state.vim_mapper().get_current_input_display();
            if display.is_empty() {
                None
            } else {
                Some(display)
            }
        })
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
        if view_state.with(|state| state.is_command_palette_open) && is_typing_in_input {
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

        // Get instruction from vim-style keyboard mapper in ViewState
        let instruction_result = view_state.with_untracked(|state| {
            // We need to get mutable access to the mapper
            // Since we can't get mutable access in with(), we'll clone, update, and store back
            let mut mapper = state.vim_mapper().clone();
            let result = mapper.map_to_instruction(&e);
            
            // Store the updated mapper back - we need to do this with update
            view_state.update_untracked(|s| {
                *s.vim_mapper_mut() = mapper;
            });
            
            result
        });

        // Handle palette navigation priority when palette is open
        if view_state.with(|state| state.is_command_palette_open) {
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

        // Note: VimKeyboardMapper state is now automatically managed in ViewState

        // Handle instruction if we got one
        if let Some((instruction, multiplier)) = instruction_result {
            e.prevent_default();
            
            // First try to apply instruction to ViewState
            let mut instruction_result = crate::view_state::InstructionResult::NotHandled;
            view_state.update(|state| {
                instruction_result = if multiplier > 1 {
                    state.apply_instruction_with_multiplier(instruction.clone(), multiplier)
                } else {
                    state.apply_instruction(instruction.clone())
                };
            });
            
            // Handle the result
            match instruction_result {
                crate::view_state::InstructionResult::Handled => {
                    // Instruction was handled by ViewState, we're done
                    return;
                }
                crate::view_state::InstructionResult::Navigate(path) => {
                    // ViewState wants us to navigate to a path
                    navigate(&path, leptos_router::NavigateOptions {
                        scroll: false,
                        ..Default::default()
                    });
                    return;
                }
                crate::view_state::InstructionResult::Failed(error) => {
                    // Instruction failed, log and continue to external handlers
                    #[cfg(target_arch = "wasm32")]
                    leptos::web_sys::console::log_1(&format!("Instruction failed: {}", error).into());
                    // Fall through to external handlers
                }
                crate::view_state::InstructionResult::NotHandled => {
                    // Instruction not handled by ViewState, try external handlers
                    // Fall through to external handlers
                }
            }
            
            // Handle instructions that need external processing
            match instruction {
                Instruction::ToggleBiblePallate => {
                    handle_toggle_bible_palette(view_state);
                    return;
                }
                Instruction::ToggleVersePallate => {
                    handle_toggle_verse_palette(view_state);
                    return;
                }
                Instruction::OpenGithubRepository => {
                    handle_open_github_repository();
                    return;
                }
                Instruction::ExportToPDF => {
                    // Create temporary signals for export handlers
                    let (is_exporting, set_is_exporting) = signal(false);
                    let (export_progress, set_export_progress) = signal(0.0f32);
                    let (export_status, set_export_status) = signal("Preparing export...".to_string());
                    
                    // Sync with ViewState
                    Effect::new(move |_| {
                        view_state.update(|state| {
                            state.set_exporting(is_exporting.get());
                            state.set_export_progress(export_progress.get(), export_status.get());
                        });
                    });
                    
                    handle_export_to_pdf(set_is_exporting, set_export_progress, set_export_status);
                    return;
                }
                Instruction::ExportToMarkdown => {
                    // Create temporary signals for export handlers
                    let (is_exporting, set_is_exporting) = signal(false);
                    let (export_progress, set_export_progress) = signal(0.0f32);
                    let (export_status, set_export_status) = signal("Preparing export...".to_string());
                    
                    // Sync with ViewState
                    Effect::new(move |_| {
                        view_state.update(|state| {
                            state.set_exporting(is_exporting.get());
                            state.set_export_progress(export_progress.get(), export_status.get());
                        });
                    });
                    
                    handle_export_to_markdown(set_is_exporting, set_export_progress, set_export_status);
                    return;
                }
                Instruction::ExportLinkedMarkdown => {
                    // Create temporary signals for export handlers
                    let (is_exporting, set_is_exporting) = signal(false);
                    let (export_progress, set_export_progress) = signal(0.0f32);
                    let (export_status, set_export_status) = signal("Preparing export...".to_string());
                    
                    // Sync with ViewState
                    Effect::new(move |_| {
                        view_state.update(|state| {
                            state.set_exporting(is_exporting.get());
                            state.set_export_progress(export_progress.get(), export_status.get());
                        });
                    });
                    
                    handle_export_linked_markdown(set_is_exporting, set_export_progress, set_export_status);
                    return;
                }
                Instruction::NextReference => {
                    // Cross-references will handle this via keyboard events
                    // This should only be reached when palette is NOT open
                    return;
                }
                Instruction::PreviousReference => {
                    // Cross-references will handle this via keyboard events
                    // This should only be reached when palette is NOT open
                    return;
                }
                // Handle copy instructions that need the processor
                Instruction::CopyRawVerse | Instruction::CopyVerseWithReference => {
                    view_state.with(|state| {
                        processor.process_with_multiplier(instruction, state, multiplier);
                    });
                    return;
                }
                
                // Handle random instructions that need the processor
                Instruction::RandomVerse | Instruction::RandomChapter => {
                    view_state.with(|state| {
                        processor.process_with_multiplier(instruction, state, multiplier);
                    });
                    return;
                }
                
                // Handle page navigation instructions
                Instruction::OpenAboutPage | Instruction::ShowTranslations => {
                    view_state.with(|state| {
                        processor.process_with_multiplier(instruction, state, multiplier);
                    });
                    return;
                }
                
                _ => {
                    // All other instructions should have been handled by ViewState
                    #[cfg(target_arch = "wasm32")]
                    leptos::web_sys::console::log_1(&format!("Unhandled instruction: {:?}", instruction).into());
                }
            }
        }
    };

    // Add global keydown listener - this runs only once when the App mounts
    window_event_listener(ev::keydown, handle_keydown);

    // Note: Export event listeners would need to be set up with ViewState signals

    view! {
        // Visual feedback for vim command buffer
        <Show when=move || vim_display.get().is_some()>
            <div class="fixed top-4 right-4 bg-black bg-opacity-75 text-white px-3 py-2 rounded-lg text-sm font-mono z-50">
                {move || vim_display.get().unwrap_or_default()}
            </div>
        </Show>

        // PDF export progress component from ViewState  
        {
            // Create reactive signals from ViewState
            let (export_progress, set_export_progress) = signal(0.0f32);
            let (export_status, set_export_status) = signal(String::new());
            let (is_exporting, set_is_exporting) = signal(false);
            
            // Keep them in sync with ViewState
            Effect::new(move |_| {
                view_state.with(|state| {
                    set_export_progress.set(state.export_progress);
                    set_export_status.set(state.export_status.clone());
                    set_is_exporting.set(state.is_exporting);
                });
            });
            
            view! {
                <crate::components::PdfLoadingProgress
                    progress=export_progress
                    status_message=export_status
                    is_visible=is_exporting
                />
            }
        }
    }
}