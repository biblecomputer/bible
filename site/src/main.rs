/*!
 * Bible Application Main Module
 *
 */

// === External Dependencies ===
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::path;
use leptos_router::NavigateOptions;
use urlencoding::encode;
use wasm_bindgen_futures::spawn_local;

// === Internal Dependencies ===
use crate::api::init_bible;
use crate::components::{
    CommandPalette, CrossReferencesSidebar, Sidebar, ThemeSidebar, TranslationComparison,
};
use crate::core::{get_bible, parse_verse_ranges_from_url, Chapter};
use crate::instructions::types::Instruction;
use crate::keyboard_navigation::KeyboardNavigationHandler;
use crate::storage::{add_recent_chapter, get_selected_theme};
use crate::themes::{get_default_theme, get_theme_by_id, theme_to_css_vars, Theme};
use crate::utils::{is_mobile_screen, parse_book_chapter_from_url};
use crate::view_state::{create_view_state, ViewStateSignal};
use crate::views::{About, ChapterDetail, HomeTranslationPicker};

mod api;
mod components;
mod core;
mod instructions;
mod keyboard_navigation;
mod storage;
mod themes;
mod translation_map;
mod utils;
mod view_state;
mod views;

// === Application Entry Point ===

/// Main application entry point
///
/// Initializes the Bible data, mounts the app component to the DOM,
/// and starts the Leptos reactive system.
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

// === Main App Components ===

/// Root application component
///
/// Sets up routing and global context for the entire application.
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
    // Theme state - initialize from localStorage at app level
    let (current_theme, set_current_theme) =
        signal(get_theme_by_id(&get_selected_theme()).unwrap_or_else(get_default_theme));

    // Apply theme CSS variables to document at app level
    Effect::new(move |_| {
        let theme = current_theme.get();
        let css_vars = theme_to_css_vars(&theme);

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                // Apply to root element
                if let Some(root) = document.document_element() {
                    let _ = root.set_attribute("style", &css_vars);
                }

                // Also apply background to body element
                if let Some(body) = document.body() {
                    let _ = body
                        .style()
                        .set_property("background-color", &format!("var(--theme-background)"));
                    let _ = body.style().set_property("margin", "0");
                    let _ = body.style().set_property("padding", "0");
                }

                // Inject text selection CSS with direct color values
                let selection_css = format!(
                    "::selection {{ background-color: {} !important; color: {} !important; }} ::-moz-selection {{ background-color: {} !important; color: {} !important; }}",
                    theme.colors.verses.selected_background,
                    theme.colors.verses.selected,
                    theme.colors.verses.selected_background,
                    theme.colors.verses.selected
                );

                // Remove existing selection style if present
                if let Ok(Some(existing_style)) =
                    document.query_selector("style[data-selection-theme]")
                {
                    if let Some(parent) = existing_style.parent_node() {
                        let _ = parent.remove_child(&existing_style);
                    }
                }

                // Create and inject new selection style
                if let Ok(style_element) = document.create_element("style") {
                    style_element.set_text_content(Some(&selection_css));
                    let _ = style_element.set_attribute("data-selection-theme", "true");
                    if let Some(head) = document.head() {
                        let _ = head.append_child(&style_element);
                    }
                }
            }
        }
    });

    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=move || view! { <Home current_theme=current_theme set_current_theme=set_current_theme /> } />
                <Route path=path!("/*any") view=move || view! { <BibleWithSidebar current_theme=current_theme set_current_theme=set_current_theme /> } />
            </Routes>
        </Router>
    }
}

/// Main Bible application component with sidebar layout
///
/// Manages global state for sidebars, keyboard navigation, and UI panels.
/// Handles theming, cross-references, and translation comparison functionality.
#[component]
fn BibleWithSidebar(
    current_theme: ReadSignal<Theme>,
    set_current_theme: WriteSignal<Theme>,
) -> impl IntoView {
    // Centralized view state management
    let view_state = create_view_state();

    // Clear initial search query after palette opens
    Effect::new(move |_| {
        if view_state.with(|state| state.is_command_palette_open)
            && view_state.with(|state| state.initial_search_query.is_some())
        {
            // Clear the initial search query after a short delay to allow it to be processed
            view_state.update(|state| state.clear_initial_search_query());
        }
    });
    let location = use_location();

    // Detect if we have cross-references data to show
    let cross_references_data = Memo::new(move |_| {
        let pathname = location.pathname.get();
        let _search = location.search.get();

        // Parse URL to get book and chapter info
        if let Some((book_name, chapter_num)) = parse_book_chapter_from_url(&pathname) {
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

        None // No cross-references data available
    });

    // Current book and chapter data for translation comparison
    let current_book_chapter = Memo::new(move |_| {
        let pathname = location.pathname.get();
        parse_book_chapter_from_url(&pathname)
    });

    // Track recent chapters when URL changes
    Effect::new(move |_| {
        let pathname = location.pathname.get();

        if let Some((book_name, chapter_num)) = parse_book_chapter_from_url(&pathname) {
            if let Ok(_chapter) = get_bible().get_chapter(&book_name, chapter_num) {
                let chapter_display = format!("{} {}", book_name, chapter_num);
                add_recent_chapter(book_name, chapter_num, chapter_display, pathname);
            }
        }
    });

    view! {
        <KeyboardNavigationHandler view_state=view_state />
        <SidebarAutoHide view_state=view_state />
        <CommandPalette view_state=view_state />
        <nav class="border-b px-4 py-2" style="background-color: var(--theme-header-background); border-color: var(--theme-header-border)">
            <div class="flex items-center justify-between">
                <div class="flex items-center space-x-2">
                    <button
                        class="p-2 rounded transition-colors header-button"
                        on:click=move |_| {
                            view_state.update(|state| { state.execute(&Instruction::ToggleSidebar); });
                        }
                        aria-label=move || if view_state.with(|state| state.is_left_sidebar_open) { "Hide books sidebar" } else { "Show books sidebar" }
                        title=move || if view_state.with(|state| state.is_left_sidebar_open) { "Hide books sidebar" } else { "Show books sidebar" }
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
                            class="flex items-center px-3 py-2 text-sm rounded transition-colors header-button"
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
                            class=move || {
                                if cross_references_data.get().is_some() {
                                    "p-2 rounded transition-colors header-button"
                                } else {
                                    "p-2 rounded transition-colors header-button opacity-50"
                                }
                            }
                            on:click=move |_| {
                                view_state.update(|state| { state.execute(&Instruction::ToggleCrossReferences); });
                            }
                            aria-label=move || {
                                if view_state.with(|state| state.is_right_sidebar_open) { "Hide cross-references" } else { "Show cross-references" }
                            }
                            title=move || {
                                if view_state.with(|state| state.is_right_sidebar_open) { "Hide cross-references" } else { "Show cross-references" }
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
                        <button
                            class="p-2 rounded transition-colors header-button"
                            on:click=move |_| {
                                view_state.update(|state| { state.execute(&Instruction::ToggleThemeSidebar); });
                            }
                            aria-label="Theme options"
                            title="Theme options"
                        >
                            <svg
                                width="20"
                                height="20"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                aria-hidden="true"
                            >
                                <path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z"/>
                                <circle cx="6.5" cy="11.5" r=".5"/>
                                <circle cx="8.5" cy="7.5" r=".5"/>
                                <circle cx="12.5" cy="13.5" r=".5"/>
                                <circle cx="13.5" cy="6.5" r=".5"/>
                            </svg>
                        </button>
                        <a
                            href="/about"
                            class="p-2 ml-2 rounded transition-colors header-button"
                            aria-label="About page"
                            title="About this Bible website"
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
                            <circle cx="12" cy="12" r="10"/>
                            <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/>
                            <path d="M12 17h.01"/>
                        </svg>
                    </a>
                </div>
            </div>
        </nav>
        <div class="flex h-screen relative" style="background-color: var(--theme-background)">
            // Left sidebar (books/chapters)
            <Show
                    when=move || view_state.with(|state| state.is_left_sidebar_open)
                    fallback=|| view! { <></> }
                >
                    <aside class="w-64 border-r p-3 overflow-y-auto md:relative absolute inset-y-0 left-0 z-50 md:z-auto" style="background-color: var(--theme-sidebar-background); border-color: var(--theme-sidebar-border)">
                        <Sidebar view_state=view_state />
                    </aside>
                </Show>

                // Left sidebar mobile overlay
                <Show
                    when=move || {
                        view_state.with(|state| state.is_left_sidebar_open) && is_mobile_screen()
                    }
                    fallback=|| view! { <></> }
                >
                    <div
                        class="fixed inset-0 bg-black bg-opacity-50 z-30"
                        on:click=move |_| {
                            view_state.update(|state| { state.execute(&Instruction::CloseLeftSidebar); });
                        }
                    />
                </Show>

                // Main content area
                <main class="flex-1 p-4 md:p-6 overflow-y-auto">
                    <Routes fallback=|| "Not found.">
                        <Route path=path!("/about") view=About />
                        <Route
                            path=path!("/:book/:chapter")
                            view=move || {
                                view! {
                                    <ChapterWrapper view_state=view_state />
                                }
                            }
                        />
                    </Routes>
                </main>

                // Right sidebar (cross-references)
                <Show
                    when=move || view_state.with(|state| state.is_right_sidebar_open)
                    fallback=|| view! { <></> }
                >
                    <aside class="w-64 border-l p-3 overflow-y-auto md:relative absolute inset-y-0 right-0 z-40 md:z-auto" style="background-color: var(--theme-sidebar-background); border-color: var(--theme-sidebar-border)">
                        {move || {
                            if let Some((book_name, chapter, verse)) = cross_references_data.get() {
                                view! {
                                    <CrossReferencesSidebar
                                        book_name=book_name
                                        chapter=chapter
                                        verse=verse
                                        view_state=view_state
                                    />
                                }.into_any()
                            } else {
                                view! {
                                    <div class="flex flex-col items-center justify-center h-full text-center p-6" style="color: var(--theme-text-muted)">
                                        <svg
                                            width="48"
                                            height="48"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="1.5"
                                            class="mb-4"
                                            aria-hidden="true"
                                        >
                                            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                                            <polyline points="14,2 14,8 20,8"/>
                                            <line x1="16" y1="13" x2="8" y2="13"/>
                                            <line x1="16" y1="17" x2="8" y2="17"/>
                                            <polyline points="10,9 9,9 8,9"/>
                                        </svg>
                                        <h3 class="text-lg font-medium mb-2" style="color: var(--theme-text-primary)">References</h3>
                                        <p class="text-sm leading-relaxed">
                                            Please select a verse by navigating with arrow keys or
                                            <kbd class="px-1.5 py-0.5 border rounded text-xs font-mono" style="background-color: var(--theme-sidebar-background); border-color: var(--theme-sidebar-border); color: var(--theme-text-primary)">j</kbd>
                                            /
                                            <kbd class="px-1.5 py-0.5 border rounded text-xs font-mono" style="background-color: var(--theme-sidebar-background); border-color: var(--theme-sidebar-border); color: var(--theme-text-primary)">k</kbd>
                                            to see cross-references.
                                        </p>
                                        <button
                                            class="mt-4 px-3 py-1.5 text-sm rounded transition-colors hover:opacity-80"
                                            style="color: var(--theme-text-muted)"
                                            on:click=move |_| {
                                                view_state.update(|state| { state.execute(&Instruction::CloseRightSidebar); });
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
                        view_state.with(|state| state.is_right_sidebar_open) && is_mobile_screen()
                    }
                    fallback=|| view! { <></> }
                >
                    <div
                        class="fixed inset-0 bg-black bg-opacity-50 z-35"
                        on:click=move |_| {
                            view_state.update(|state| { state.execute(&Instruction::CloseRightSidebar); });
                        }
                    />
                </Show>

                // Theme sidebar
                <Show
                    when=move || view_state.with(|state| state.is_theme_sidebar_open)
                    fallback=|| view! { <></> }
                >
                    <aside class="w-64 border-r p-3 overflow-y-auto md:relative absolute inset-y-0 right-0 z-45 md:z-auto" style="background-color: var(--theme-sidebar-background); border-color: var(--theme-sidebar-border)">
                        <ThemeSidebar
                            current_theme=current_theme
                            set_current_theme=set_current_theme
                            view_state=view_state
                        />
                    </aside>
                </Show>

                // Theme sidebar mobile overlay
                <Show
                    when=move || {
                        view_state.with(|state| state.is_theme_sidebar_open) && is_mobile_screen()
                    }
                    fallback=|| view! { <></> }
                >
                    <div
                        class="fixed inset-0 bg-black bg-opacity-50 z-44"
                        on:click=move |_| {
                            view_state.update(|state| { state.execute(&Instruction::CloseThemeSidebar); });
                        }
                    />
                </Show>

                // Translation comparison panel
                {move || {
                    if let Some((book_name, chapter_num)) = current_book_chapter.get() {
                        let (current_book_signal, _) = signal(book_name.clone());
                        let (current_chapter_signal, _) = signal(chapter_num);

                        view! {
                            <TranslationComparison
                                current_book=current_book_signal
                                current_chapter=current_chapter_signal
                                view_state=view_state
                            />
                        }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }}
            </div>
    }
}

#[component]
fn SidebarAutoHide(view_state: ViewStateSignal) -> impl IntoView {
    let location = use_location();

    // Auto-hide sidebar on mobile when navigating to a chapter
    Effect::new(move |_| {
        let pathname = location.pathname.get();
        let path_parts: Vec<&str> = pathname.trim_start_matches('/').split('/').collect();

        // If we're on a chapter page and screen is mobile-sized, hide sidebar
        if path_parts.len() == 2 && !path_parts[0].is_empty() && !path_parts[1].is_empty() {
            if is_mobile_screen() {
                view_state.update(|state| {
                    state.execute(&Instruction::CloseLeftSidebar);
                });
            }
        }
    });

    view! {
        // This component renders nothing, it just handles auto-hide logic
        <></>
    }
}

#[component]
fn Home(current_theme: ReadSignal<Theme>, set_current_theme: WriteSignal<Theme>) -> impl IntoView {
    use crate::storage::{get_selected_translation, is_translation_downloaded};
    use leptos_router::hooks::{use_location, use_navigate};

    let navigate = use_navigate();
    let location = use_location();

    // Check if user has a selected translation that's downloaded
    Effect::new(move |_| {
        // Check if user explicitly wants to choose a translation (bypass auto-redirect)
        let search_params = location.search.get();
        if search_params.contains("choose=true") {
            return; // Don't auto-redirect if user explicitly wants to choose
        }

        // Also don't auto-redirect if there's a return_url (let the translation picker handle it)
        if search_params.contains("return_url=") {
            return;
        }

        if let Some(selected_translation) = get_selected_translation() {
            if is_translation_downloaded(&selected_translation) {
                // Navigate to Genesis 1 - predictable default chapter
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
        <div class="min-h-screen" style="background-color: var(--theme-background)">
            <HomeTranslationPicker current_theme=current_theme set_current_theme=set_current_theme />
        </div>
    }
}

#[component]
fn ChapterWrapper(view_state: ViewStateSignal) -> impl IntoView {
    use crate::storage::{get_selected_translation, is_translation_downloaded};
    use leptos_router::hooks::{use_location, use_navigate};

    let navigate = use_navigate();
    let location = use_location();

    // Check if user has a downloaded translation
    let (redirect_triggered, set_redirect_triggered) = signal(false);

    // Create effect to check translation and redirect if needed
    Effect::new(move |_| {
        // Prevent multiple redirects
        if redirect_triggered.get() {
            return;
        }

        // Check if user has a selected translation that's downloaded
        if let Some(selected_translation) = get_selected_translation() {
            if is_translation_downloaded(&selected_translation) {
                // Translation found, no redirect needed
                return;
            }
        }

        // No valid translation found - redirect to home with current URL as return path
        set_redirect_triggered.set(true);
        let current_path = format!("{}{}", location.pathname.get(), location.search.get());
        let encoded_return_url = encode(&current_path);
        let redirect_url = format!("/?choose=true&return_url={}", encoded_return_url);

        navigate(
            &redirect_url,
            NavigateOptions {
                scroll: false,
                replace: true, // Use replace to avoid adding to history
                ..Default::default()
            },
        );
    });

    // Simple check for rendering - if we have a translation, show the chapter
    let has_translation = move || {
        if let Some(selected_translation) = get_selected_translation() {
            is_translation_downloaded(&selected_translation)
        } else {
            false
        }
    };

    view! {
        <Show
            when=move || has_translation()
            fallback=move || view! {
                <div class="flex items-center justify-center min-h-[200px]">
                    <div class="text-center">
                        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto mb-4"></div>
                        <p style="color: var(--theme-text-muted)">Redirecting to translation picker...</p>
                    </div>
                </div>
            }
        >
            {move || {
                match Chapter::from_url() {
                    Ok(chapter) => {
                        let (verse_visibility_read, verse_visibility_write) = signal(false);
                        Effect::new(move |_| {
                            verse_visibility_write.set(view_state.with(|state| state.verse_visibility_enabled));
                        });
                        view! {
                            <ChapterDetail
                                chapter=chapter
                                verse_visibility_enabled=verse_visibility_read
                            />
                        }
                    }.into_any(),
                    Err(_) => view! {
                        <div class="text-center p-8">
                            <p style="color: var(--theme-text-primary)">Chapter not found</p>
                        </div>
                    }.into_any()
                }
            }}
        </Show>
    }
}
