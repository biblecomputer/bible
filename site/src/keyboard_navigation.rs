use leptos::ev;
use leptos::prelude::*;
use leptos::web_sys::KeyboardEvent;
use leptos_router::hooks::{use_location, use_navigate};

use crate::instructions::{update_view_state_from_url, Instruction, VimKeyboardMapper};
use crate::view_state::ViewStateSignal;

#[component]
pub fn KeyboardNavigationHandler(view_state: ViewStateSignal) -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();

    // Create VimKeyboardMapper for this component
    let vim_mapper = RwSignal::new(VimKeyboardMapper::new());

    // Reactive effect to track path changes and update ViewState
    {
        let mut last_path = String::new();
        Effect::new(move |_| {
            let current_path = location.pathname.get();
            let current_search = location.search.get();

            // Update ViewState with current navigation context
            update_view_state_from_url(view_state, &current_path, &current_search);

            // Track previous chapter path for navigation history
            if !last_path.is_empty() && last_path != current_path {
                let _ = view_state.try_update(|state| {
                    state.set_previous_chapter_path(Some(last_path.clone()));
                });
            }
            last_path = current_path;
        });
    }

    // Visual display for vim command buffer
    let vim_display = Memo::new(move |_| {
        vim_mapper.try_with(|mapper| {
            let display = mapper.get_current_input_display();
            if display.is_empty() {
                None
            } else {
                Some(display)
            }
        }).unwrap_or(None)
    });

    // Set up keyboard event handler
    let handle_keydown = move |e: KeyboardEvent| {
        // Check if user is typing in an input field
        let is_typing_in_input = if let Some(window) = leptos::web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(active_element) = document.active_element() {
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
        let palette_open = view_state.try_with(|state| state.is_command_palette_open).unwrap_or(false);
        if palette_open && is_typing_in_input {
            let key = e.key();
            let is_control_sequence = e.ctrl_key() && (key == "j" || key == "k" || key == "o")
                || key == "Escape"
                || key == "Enter"
                || key == "ArrowUp"
                || key == "ArrowDown";

            if !is_control_sequence {
                return;
            }
        }

        // Get instruction from vim-style keyboard mapper
        let instruction_result = {
            // Get the current mapper state
            let mut current_mapper = vim_mapper.try_with_untracked(|mapper| mapper.clone()).unwrap_or_else(|| VimKeyboardMapper::new());
            
            #[cfg(target_arch = "wasm32")]
            leptos::web_sys::console::log_1(&format!("🔤 Key pressed: '{}', current sequence: '{}', multiplier: '{}'", 
                e.key(), 
                current_mapper.get_sequence_buffer(), 
                current_mapper.get_multiplier_buffer()
            ).into());
            
            let result = current_mapper.map_to_instruction(&e);
            
            #[cfg(target_arch = "wasm32")]
            leptos::web_sys::console::log_1(&format!("🎯 Mapper result: {:?}, new sequence: '{}', new multiplier: '{}'", 
                result, 
                current_mapper.get_sequence_buffer(), 
                current_mapper.get_multiplier_buffer()
            ).into());

            // Store the updated mapper back
            let _ = vim_mapper.try_update_untracked(|m| *m = current_mapper);

            result
        };

        // Handle palette navigation priority when palette is open
        let palette_open_check = view_state.try_with(|state| state.is_command_palette_open).unwrap_or(false);
        if palette_open_check {
            if let Some((ref instruction, _)) = instruction_result {
                match instruction {
                    Instruction::NextPaletteResult | Instruction::PreviousPaletteResult => {
                        // Let palette navigation instructions through
                    }
                    Instruction::ToggleBiblePallate | Instruction::ToggleCommandPallate => {
                        // Let palette toggle instructions through
                    }
                    Instruction::ToggleSidebar
                    | Instruction::ToggleCrossReferences
                    | Instruction::ToggleThemeSidebar
                    | Instruction::ToggleVerseVisibility => {
                        // Let UI toggle instructions through
                    }
                    Instruction::NextReference | Instruction::PreviousReference => {
                        // Block reference navigation when palette is open
                        e.prevent_default();
                        return;
                    }
                    Instruction::NextVerse | Instruction::PreviousVerse => {
                        // Block verse navigation when palette is open
                        e.prevent_default();
                        return;
                    }
                    _ => {
                        // Skip all other keyboard processing when palette is open
                        return;
                    }
                }
            } else {
                return;
            }
        }

        // Handle instruction if we got one
        if let Some((instruction, multiplier)) = instruction_result {
            #[cfg(target_arch = "wasm32")]
            leptos::web_sys::console::log_1(
                &format!(
                    "⌨️  Got instruction: {:?} with multiplier: {}",
                    instruction, multiplier
                )
                .into(),
            );

            e.prevent_default();

            // Execute instruction in ViewState
            let instruction_result = view_state
                .try_update(|state| {
                    if multiplier > 1 {
                        state.execute_with_multiplier(&instruction, multiplier)
                    } else {
                        state.execute(&instruction)
                    }
                })
                .unwrap_or(crate::view_state::InstructionResult::Failed(
                    "Update failed".to_string(),
                ));

            // Handle the result
            match instruction_result {
                crate::view_state::InstructionResult::Handled => {
                    // Instruction was handled by ViewState, we're done
                }
                crate::view_state::InstructionResult::Navigate(path) => {
                    // ViewState wants us to navigate to a path
                    navigate(
                        &path,
                        leptos_router::NavigateOptions {
                            scroll: false,
                            ..Default::default()
                        },
                    );
                }
                crate::view_state::InstructionResult::Failed(_error) => {
                    // Instruction failed - ViewState logs should show details
                    #[cfg(target_arch = "wasm32")]
                    leptos::web_sys::console::log_1(
                        &format!("❌ Instruction failed: {:?}", instruction).into(),
                    );
                }
                crate::view_state::InstructionResult::NotHandled => {
                    // Instruction not handled by ViewState - this shouldn't happen much anymore
                    #[cfg(target_arch = "wasm32")]
                    leptos::web_sys::console::log_1(
                        &format!("🤷 Instruction not handled: {:?}", instruction).into(),
                    );
                }
            }
        }
    };

    // Add global keydown listener
    window_event_listener(ev::keydown, handle_keydown);

    view! {
        // Visual feedback for vim command buffer
        <Show when=move || vim_display.get().is_some()>
            <div class="fixed top-4 right-4 bg-black bg-opacity-75 text-white px-3 py-2 rounded-lg text-sm font-mono z-50">
                {move || vim_display.get().unwrap_or_default()}
            </div>
        </Show>

        // Export progress component - read from ViewState
        {
            let (export_progress, set_export_progress) = signal(0.0f32);
            let (export_status, set_export_status) = signal(String::new());
            let (is_exporting, set_is_exporting) = signal(false);

            // Keep them in sync with ViewState
            Effect::new(move |_| {
                if let Some(_) = view_state.try_with(|state| {
                    set_export_progress.set(state.export_progress);
                    set_export_status.set(state.export_status.clone());
                    set_is_exporting.set(state.is_exporting);
                }) {
                    // Successfully updated
                }
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
