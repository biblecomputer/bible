use crate::views::ChapterDetail;
use crate::components::{CommandPalette, Sidebar, ShortcutsHelp};
use crate::views::HomeTranslationPicker;
use crate::api::init_bible;
use crate::core::{get_bible, Chapter, VerseRange};
use crate::utils::is_mobile_screen;
use leptos::prelude::*;
use leptos::ev;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::path;
use leptos::web_sys::KeyboardEvent;
use wasm_bindgen_futures::spawn_local;

mod api;
mod components;
mod core;
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
    // Command palette state
    let (is_palette_open, set_is_palette_open) = signal(false);
    // Sidebar visibility state
    let (is_sidebar_open, set_is_sidebar_open) = signal(true);
        
        view! {
            <KeyboardNavigationHandler 
                palette_open=is_palette_open 
                set_palette_open=set_is_palette_open 
                _sidebar_open=is_sidebar_open
                set_sidebar_open=set_is_sidebar_open
            />
            <SidebarAutoHide set_sidebar_open=set_is_sidebar_open />
            <CommandPalette is_open=is_palette_open set_is_open=set_is_palette_open />
                <nav class="bg-white border-b border-gray-200 px-4 py-2">
                    <div class="flex items-center justify-between">
                        <div class="flex items-center space-x-2">
                            <button
                                class="p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded transition-colors"
                                on:click=move |_| set_is_sidebar_open.update(|open| *open = !*open)
                                aria-label=move || if is_sidebar_open.get() { "Hide sidebar" } else { "Show sidebar" }
                                title=move || if is_sidebar_open.get() { "Hide sidebar" } else { "Show sidebar" }
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
                </nav>
                <div class="flex h-screen relative">
                    <Show
                        when=move || is_sidebar_open.get()
                        fallback=|| view! { <></> }
                    >
                        <aside class="w-64 bg-white border-r border-black p-3 overflow-y-auto md:relative absolute inset-y-0 left-0 z-50 md:z-auto">
                            <Sidebar set_sidebar_open=set_is_sidebar_open />
                        </aside>
                    </Show>
                    
                    <Show
                        when=move || {
                            is_sidebar_open.get() && is_mobile_screen()
                        }
                        fallback=|| view! { <></> }
                    >
                        <div 
                            class="fixed inset-0 bg-black bg-opacity-50 z-30"
                            on:click=move |_| set_is_sidebar_open.set(false)
                        />
                    </Show>
                    
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
                </div>
                <ShortcutsHelp />
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
    _sidebar_open: ReadSignal<bool>,
    set_sidebar_open: WriteSignal<bool>,
) -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();
    
    // Set up keyboard event handler
    let handle_keydown = move |e: KeyboardEvent| {
        // Handle Cmd/Ctrl+K to open command palette
        if e.key() == "k" && (e.meta_key() || e.ctrl_key()) {
            e.prevent_default();
            set_palette_open.set(true);
            // Close sidebar on mobile when command palette opens
            if is_mobile_screen() {
                set_sidebar_open.set(false);
            }
            return;
        }
        
        // Handle Ctrl+B to toggle sidebar
        if e.key() == "b" && e.ctrl_key() {
            e.prevent_default();
            set_sidebar_open.update(|open| *open = !*open);
            return;
        }
        
        // Skip arrow key navigation if command palette is open
        if palette_open.get() {
            return;
        }
        
        let pathname = location.pathname.get();
        let search = location.search.get();
        
        // Parse current path to get book and chapter
        let path_parts: Vec<&str> = pathname.trim_start_matches('/').split('/').collect();
        if path_parts.len() == 2 {
            let book_name = path_parts[0].replace('_', " ");
            if let Ok(chapter_num) = path_parts[1].parse::<u32>() {
                if let Ok(current_chapter) = get_bible().get_chapter(&book_name, chapter_num) {
                    match e.key().as_str() {
                        "ArrowRight" => {
                            if let Some(next_chapter) = get_bible().get_next_chapter(&current_chapter) {
                                navigate(&next_chapter.to_path(), Default::default());
                            }
                        }
                        "ArrowLeft" => {
                            if let Some(prev_chapter) = get_bible().get_previous_chapter(&current_chapter) {
                                navigate(&prev_chapter.to_path(), Default::default());
                            }
                        }
                        "ArrowDown" => {
                            e.prevent_default();
                            // Get current verse from URL or default to 1
                            let current_verse = if search.contains("verses=") {
                                let verse_param = search.split("verses=").nth(1).unwrap_or("1").split('&').next().unwrap_or("1");
                                // Handle single verse from comma-separated list
                                verse_param.split(',').next().unwrap_or("1").split('-').next().unwrap_or("1").parse().unwrap_or(1)
                            } else {
                                1
                            };
                            
                            if let Some(next_verse) = current_chapter.get_next_verse(current_verse) {
                                // Navigate to next verse in current chapter
                                let verse_range = VerseRange { start: next_verse, end: next_verse };
                                let new_path = current_chapter.to_path_with_verses(&[verse_range]);
                                navigate(&new_path, Default::default());
                            } else if let Some(next_chapter) = get_bible().get_next_chapter(&current_chapter) {
                                // Navigate to first verse of next chapter
                                let verse_range = VerseRange { start: 1, end: 1 };
                                let new_path = next_chapter.to_path_with_verses(&[verse_range]);
                                navigate(&new_path, Default::default());
                            }
                        }
                        "ArrowUp" => {
                            e.prevent_default();
                            // Get current verse from URL or default to first verse with content
                            let current_verse = if search.contains("verses=") {
                                let verse_param = search.split("verses=").nth(1).unwrap_or("1").split('&').next().unwrap_or("1");
                                // Handle single verse from comma-separated list
                                verse_param.split(',').next().unwrap_or("1").split('-').next().unwrap_or("1").parse().unwrap_or(1)
                            } else {
                                // If no verse is selected, start from the last verse to go up
                                current_chapter.verses.len() as u32
                            };
                            
                            if let Some(prev_verse) = current_chapter.get_previous_verse(current_verse) {
                                // Navigate to previous verse in current chapter
                                let verse_range = VerseRange { start: prev_verse, end: prev_verse };
                                let new_path = current_chapter.to_path_with_verses(&[verse_range]);
                                navigate(&new_path, Default::default());
                            } else if let Some(prev_chapter) = get_bible().get_previous_chapter(&current_chapter) {
                                // Navigate to last verse of previous chapter
                                let last_verse = prev_chapter.verses.len() as u32;
                                if last_verse > 0 {
                                    let verse_range = VerseRange { start: last_verse, end: last_verse };
                                    let new_path = prev_chapter.to_path_with_verses(&[verse_range]);
                                    navigate(&new_path, Default::default());
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    };

    // Add global keydown listener - this runs only once when the App mounts
    window_event_listener(ev::keydown, handle_keydown);

    view! {
        // This component renders nothing, it just handles keyboard events
        <></>
    }
}

#[component]
fn Home() -> impl IntoView {
    use crate::core::get_current_bible;
    use crate::storage::{get_selected_translation, is_translation_downloaded};
    use leptos_router::hooks::{use_navigate, use_location};
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
                            navigate(&path, Default::default());
                            return;
                        }
                    }
                }
                
                // Fallback: try to navigate to a standard Genesis path
                navigate("/Genesis/1", Default::default());
            }
        }
    });
    
    view! {
        <div class="min-h-screen bg-gray-50">
            <HomeTranslationPicker />
        </div>
    }
}

