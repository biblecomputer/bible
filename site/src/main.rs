use crate::api::init_bible;
use crate::components::{CommandPalette, CrossReferencesSidebar, Sidebar};
use crate::core::{get_bible, parse_verse_ranges_from_url, Chapter};
use crate::instructions::{
    Instruction, InstructionContext, InstructionProcessor, VimKeyboardMapper,
};
use crate::storage::{
    get_references_sidebar_open, get_sidebar_open, save_references_sidebar_open, save_sidebar_open,
    add_recent_chapter,
};
use crate::utils::is_mobile_screen;
use crate::views::ChapterDetail;
use crate::views::HomeTranslationPicker;
use leptos::ev;
use leptos::prelude::*;
use leptos::web_sys::KeyboardEvent;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::path;
use leptos_router::NavigateOptions;
use urlencoding::decode;
use wasm_bindgen_futures::spawn_local;



// Helper function to create instruction context from URL
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

mod api;
mod components;
mod core;
mod instructions;
mod storage;
mod translation_map;
mod utils;
mod views;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    // Bible data loading state
    let (is_bible_loaded, set_is_bible_loaded) = signal(false);
    let (loading_error, set_loading_error) = signal::<Option<String>>(None);

    // Initialize Bible data on mount
    Effect::new(move |_| {
        spawn_local(async move {
            match init_bible().await {
                Ok(()) => set_is_bible_loaded.set(true),
                Err(err) => {
                    set_loading_error.set(Some(format!("Failed to load Bible data: {}", err)));
                }
            }
        });
    });

    view! {
        <Show
            when=move || is_bible_loaded.get()
            fallback=move || view! {
                <div class="flex items-center justify-center min-h-screen">
                    <Show
                        when=move || loading_error.get().is_none()
                        fallback=move || view! {
                            <div class="text-red-600">
                                {loading_error.get().unwrap_or_default()}
                            </div>
                        }
                    >
                        <div class="text-center">
                            <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
                            <p class="text-gray-600">"Loading Bible data..."</p>
                        </div>
                    </Show>
                </div>
            }
        >
            <BibleApp />
        </Show>
    }
}

#[component]
fn BibleApp() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=Home />
                <Route path=path!("/*any") view=BibleWithSidebar />
            </Routes>
        </Router>
    }
}

#[component]
fn BibleWithSidebar() -> impl IntoView {
    // Command palette state - ensure it starts closed
    let (is_palette_open, set_is_palette_open) = signal(false);
    // Command palette navigation signals
    let next_palette_result = RwSignal::new(false);
    let previous_palette_result = RwSignal::new(false);
    // Command palette initial search query signal
    let (initial_search_query, set_initial_search_query) = signal::<Option<String>>(None);
    
    // Clear initial search query after palette opens
    Effect::new(move |_| {
        if is_palette_open.get() && initial_search_query.get().is_some() {
            // Clear the initial search query after a short delay to allow it to be processed
            set_initial_search_query.set(None);
        }
    });
    // Left sidebar (books/chapters) visibility state - initialize from localStorage
    let (is_left_sidebar_open, set_is_left_sidebar_open) = signal(get_sidebar_open());
    // Right sidebar (cross-references) visibility state - load from storage
    let (is_right_sidebar_open, set_is_right_sidebar_open) = signal(get_references_sidebar_open());
    let location = use_location();

    // Detect if we have cross-references data to show
    let cross_references_data = Memo::new(move |_| {
        let pathname = location.pathname.get();
        let _search = location.search.get();

        // Parse URL to get book, chapter, and verse info
        let path_parts: Vec<&str> = pathname.trim_start_matches('/').split('/').collect();
        if path_parts.len() == 2 {
            let book_name = if let Ok(decoded) = decode(path_parts[0]) {
                decoded.into_owned()
            } else {
                path_parts[0].replace('_', " ")
            };

            if let Ok(chapter_num) = path_parts[1].parse::<u32>() {
                // Check if there are verse parameters and if it's exactly one verse
                let verse_ranges = parse_verse_ranges_from_url();
                if verse_ranges.len() == 1 {
                    let range = &verse_ranges[0];
                    if range.start == range.end {
                        // Single verse selected - return cross-references data
                        return Some((book_name, chapter_num, range.start));
                    }
                }
            }
        }

        None // No cross-references data available
    });

    // Track recent chapters when URL changes
    Effect::new(move |_| {
        let pathname = location.pathname.get();
        let path_parts: Vec<&str> = pathname.trim_start_matches('/').split('/').collect();
        
        if path_parts.len() == 2 {
            let book_name = if let Ok(decoded) = urlencoding::decode(path_parts[0]) {
                decoded.into_owned()
            } else {
                path_parts[0].replace('_', " ")
            };
            
            if let Ok(chapter_num) = path_parts[1].parse::<u32>() {
                if let Ok(_chapter) = get_bible().get_chapter(&book_name, chapter_num) {
                    let chapter_display = format!("{} {}", book_name, chapter_num);
                    add_recent_chapter(book_name, chapter_num, chapter_display, pathname);
                }
            }
        }
    });

    view! {
        <KeyboardNavigationHandler
            palette_open=is_palette_open
            set_palette_open=set_is_palette_open
            _left_sidebar_open=is_left_sidebar_open
            set_left_sidebar_open=set_is_left_sidebar_open
            _right_sidebar_open=is_right_sidebar_open
            set_right_sidebar_open=set_is_right_sidebar_open
            next_palette_result=next_palette_result
            previous_palette_result=previous_palette_result
            set_initial_search_query=set_initial_search_query
        />
        <SidebarAutoHide set_sidebar_open=set_is_left_sidebar_open />
        <CommandPalette
            is_open=is_palette_open
            set_is_open=set_is_palette_open
            next_palette_result=next_palette_result
            previous_palette_result=previous_palette_result
            initial_search_query=initial_search_query
        />
        <nav class="bg-white border-b border-gray-200 px-4 py-2">
            <div class="flex items-center justify-between">
                <div class="flex items-center space-x-2">
                    <button
                        class="p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded transition-colors"
                        on:click=move |_| {
                            set_is_left_sidebar_open.update(|open| {
                                *open = !*open;
                                save_sidebar_open(*open);
                            });
                        }
                        aria-label=move || if is_left_sidebar_open.get() { "Hide books sidebar" } else { "Show books sidebar" }
                        title=move || if is_left_sidebar_open.get() { "Hide books sidebar" } else { "Show books sidebar" }
                    >
                            <svg
                                width="24"
                                height="24"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                aria-hidden="true"
                            >
                                <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                                <line x1="9" y1="9" x2="21" y2="9"/>
                                <line x1="9" y1="15" x2="21" y2="15"/>
                                <line x1="3" y1="9" x2="7" y2="9"/>
                                <line x1="3" y1="15" x2="7" y2="15"/>
                            </svg>
                        </button>
                        <a
                            href="/?choose=true"
                            class="flex items-center px-3 py-2 text-sm text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded transition-colors"
                            aria-label="Kies vertaling"
                            title="Terug naar vertalingskeuze"
                        >
                            <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                            </svg>
                            "Kies vertaling"
                        </a>
                    </div>
                    <div class="flex items-center space-x-2">
                        <button
                            class=move || format!(
                                "p-2 rounded transition-colors {}",
                                if cross_references_data.get().is_some() {
                                    "text-gray-600 hover:text-gray-900 hover:bg-gray-100"
                                } else {
                                    "text-gray-400 hover:text-gray-600 hover:bg-gray-50"
                                }
                            )
                            on:click=move |_| {
                                if cross_references_data.get().is_some() {
                                    set_is_right_sidebar_open.update(|open| {
                                        *open = !*open;
                                        save_references_sidebar_open(*open);
                                    });
                                } else {
                                    // Show sidebar with helpful message
                                    set_is_right_sidebar_open.set(true);
                                    save_references_sidebar_open(true);
                                }
                            }
                            aria-label=move || {
                                if cross_references_data.get().is_some() {
                                    if is_right_sidebar_open.get() { "Hide cross-references" } else { "Show cross-references" }
                                } else {
                                    "Show references help"
                                }
                            }
                            title=move || {
                                if cross_references_data.get().is_some() {
                                    if is_right_sidebar_open.get() { "Hide cross-references" } else { "Show cross-references" }
                                } else {
                                    "References (select a verse first)"
                                }
                            }
                        >
                            <svg
                                width="24"
                                height="24"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                aria-hidden="true"
                            >
                                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                                <polyline points="14,2 14,8 20,8"/>
                                <line x1="16" y1="13" x2="8" y2="13"/>
                                <line x1="16" y1="17" x2="8" y2="17"/>
                                <polyline points="10,9 9,9 8,9"/>
                            </svg>
                        </button>
                        <a
                            href="https://github.com/sempruijs/bible"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-black hover:text-gray-600 transition-colors"
                            aria-label="GitHub repository"
                            title="View source on GitHub"
                        >
                        <svg
                            width="20"
                            height="20"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            class="text-black hover:text-gray-600"
                            aria-hidden="true"
                        >
                            <path d="M12 0C5.374 0 0 5.373 0 12 0 17.302 3.438 21.8 8.207 23.387c.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0112 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.30 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z"/>
                        </svg>
                    </a>
                </div>
            </div>
        </nav>
        <div class="flex h-screen relative">
            // Left sidebar (books/chapters)
            <Show
                    when=move || is_left_sidebar_open.get()
                    fallback=|| view! { <></> }
                >
                    <aside class="w-64 bg-white border-r border-black p-3 overflow-y-auto md:relative absolute inset-y-0 left-0 z-50 md:z-auto">
                        <Sidebar set_sidebar_open=set_is_left_sidebar_open />
                    </aside>
                </Show>

                // Left sidebar mobile overlay
                <Show
                    when=move || {
                        is_left_sidebar_open.get() && is_mobile_screen()
                    }
                    fallback=|| view! { <></> }
                >
                    <div
                        class="fixed inset-0 bg-black bg-opacity-50 z-30"
                        on:click=move |_| {
                            set_is_left_sidebar_open.set(false);
                            save_sidebar_open(false);
                        }
                    />
                </Show>

                // Main content area
                <main class="flex-1 p-4 md:p-6 overflow-y-auto">
                    <Routes fallback=|| "Not found.">
                        <Route
                            path=path!("/:book/:chapter")
                            view=move || {
                                let chapter = Chapter::from_url().unwrap();
                                view! {
                                    <ChapterDetail chapter=chapter />
                                }
                            }
                        />
                    </Routes>
                </main>

                // Right sidebar (cross-references)
                <Show
                    when=move || is_right_sidebar_open.get()
                    fallback=|| view! { <></> }
                >
                    <aside class="w-64 bg-white border-l border-black p-3 overflow-y-auto md:relative absolute inset-y-0 right-0 z-40 md:z-auto">
                        {move || {
                            if let Some((book_name, chapter, verse)) = cross_references_data.get() {
                                view! {
                                    <CrossReferencesSidebar
                                        book_name=book_name
                                        chapter=chapter
                                        verse=verse
                                        set_sidebar_open=set_is_right_sidebar_open
                                        palette_open=is_palette_open
                                    />
                                }.into_any()
                            } else {
                                view! {
                                    <div class="flex flex-col items-center justify-center h-full text-center p-6 text-gray-500">
                                        <svg
                                            width="48"
                                            height="48"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="1.5"
                                            class="mb-4 text-gray-400"
                                            aria-hidden="true"
                                        >
                                            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                                            <polyline points="14,2 14,8 20,8"/>
                                            <line x1="16" y1="13" x2="8" y2="13"/>
                                            <line x1="16" y1="17" x2="8" y2="17"/>
                                            <polyline points="10,9 9,9 8,9"/>
                                        </svg>
                                        <h3 class="text-lg font-medium text-gray-700 mb-2">References</h3>
                                        <p class="text-sm leading-relaxed">
                                            Please select a verse by navigating with arrow keys or
                                            <kbd class="px-1.5 py-0.5 bg-gray-100 border border-gray-300 rounded text-xs font-mono">j</kbd>
                                            /
                                            <kbd class="px-1.5 py-0.5 bg-gray-100 border border-gray-300 rounded text-xs font-mono">k</kbd>
                                            to see cross-references.
                                        </p>
                                        <button
                                            class="mt-4 px-3 py-1.5 text-sm text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded transition-colors"
                                            on:click=move |_| {
                                                set_is_right_sidebar_open.set(false);
                                                save_references_sidebar_open(false);
                                            }
                                        >
                                            Close
                                        </button>
                                    </div>
                                }.into_any()
                            }
                        }}
                    </aside>
                </Show>

                // Right sidebar mobile overlay
                <Show
                    when=move || {
                        is_right_sidebar_open.get() && is_mobile_screen()
                    }
                    fallback=|| view! { <></> }
                >
                    <div
                        class="fixed inset-0 bg-black bg-opacity-50 z-35"
                        on:click=move |_| {
                            set_is_right_sidebar_open.set(false);
                            save_references_sidebar_open(false);
                        }
                    />
                </Show>
            </div>
    }
}

#[component]
fn SidebarAutoHide(set_sidebar_open: WriteSignal<bool>) -> impl IntoView {
    let location = use_location();

    // Auto-hide sidebar on mobile when navigating to a chapter
    Effect::new(move |_| {
        let pathname = location.pathname.get();
        let path_parts: Vec<&str> = pathname.trim_start_matches('/').split('/').collect();

        // If we're on a chapter page and screen is mobile-sized, hide sidebar
        if path_parts.len() == 2 && !path_parts[0].is_empty() && !path_parts[1].is_empty() {
            if is_mobile_screen() {
                set_sidebar_open.set(false);
                save_sidebar_open(false);
            }
        }
    });

    view! {
        // This component renders nothing, it just handles auto-hide logic
        <></>
    }
}

#[component]
fn KeyboardNavigationHandler(
    palette_open: ReadSignal<bool>,
    set_palette_open: WriteSignal<bool>,
    _left_sidebar_open: ReadSignal<bool>,
    set_left_sidebar_open: WriteSignal<bool>,
    _right_sidebar_open: ReadSignal<bool>,
    set_right_sidebar_open: WriteSignal<bool>,
    next_palette_result: RwSignal<bool>,
    previous_palette_result: RwSignal<bool>,
    set_initial_search_query: WriteSignal<Option<String>>,
) -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();

    // Previous chapter tracking for "alt-tab" like switching
    let (previous_chapter_path, set_previous_chapter_path) = signal(Option::<String>::None);

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
        // Get instruction from vim-style keyboard mapper first
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

    view! {
        // Visual feedback for vim command buffer
        <Show when=move || vim_display.get().is_some()>
            <div class="fixed top-4 right-4 bg-black bg-opacity-75 text-white px-3 py-2 rounded-lg text-sm font-mono z-50">
                {move || vim_display.get().unwrap_or_default()}
            </div>
        </Show>
    }
}

#[component]
fn Home() -> impl IntoView {
    use crate::core::get_current_bible;
    use crate::storage::{get_selected_translation, is_translation_downloaded};
    use leptos_router::hooks::{use_location, use_navigate};
    use urlencoding::encode;

    let navigate = use_navigate();
    let location = use_location();

    // Check if user has a selected translation that's downloaded
    Effect::new(move |_| {
        // Check if user explicitly wants to choose a translation (bypass auto-redirect)
        let search_params = location.search.get();
        if search_params.contains("choose=true") {
            return; // Don't auto-redirect if user explicitly wants to choose
        }

        if let Some(selected_translation) = get_selected_translation() {
            if is_translation_downloaded(&selected_translation) {
                // Get the current Bible to find Genesis 1
                if let Some(bible) = get_current_bible() {
                    if let Some(genesis_book) = bible.books.first() {
                        if let Some(first_chapter) = genesis_book.chapters.first() {
                            let encoded_book = encode(&genesis_book.name);
                            let path = format!("/{}/{}", encoded_book, first_chapter.chapter);
                            navigate(
                                &path,
                                NavigateOptions {
                                    scroll: false,
                                    ..Default::default()
                                },
                            );
                            return;
                        }
                    }
                }

                // Fallback: try to navigate to a standard Genesis path
                navigate(
                    "/Genesis/1",
                    NavigateOptions {
                        scroll: false,
                        ..Default::default()
                    },
                );
            }
        }
    });

    view! {
        <div class="min-h-screen bg-gray-50">
            <HomeTranslationPicker />
        </div>
    }
}
