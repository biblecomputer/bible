use crate::core::{get_bible, init_bible_signal, Chapter, VerseRange};
use crate::storage::translations::get_current_translation;
use crate::core::types::Language;
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;
use leptos_router::components::A;
use wasm_bindgen_futures::spawn_local;
use leptos::wasm_bindgen::JsCast;

fn get_translated_chapter_name(chapter_name: &str) -> String {
    // Translation is now handled at the Bible data level, so chapter names are already translated
    chapter_name.to_string()
}

fn get_navigation_text(key: &str) -> String {
    if let Some(current_translation) = get_current_translation() {
        if let Some(first_language) = current_translation.languages.first() {
            match (key, first_language) {
                ("previous_chapter", Language::Dutch) => "Vorig Hoofdstuk".to_string(),
                ("next_chapter", Language::Dutch) => "Volgend Hoofdstuk".to_string(),
                ("previous_chapter", Language::English) => "Previous Chapter".to_string(),
                ("next_chapter", Language::English) => "Next Chapter".to_string(),
                _ => key.to_string(),
            }
        } else {
            key.to_string()
        }
    } else {
        // Default to English
        match key {
            "previous_chapter" => "Previous Chapter".to_string(),
            "next_chapter" => "Next Chapter".to_string(),
            _ => key.to_string(),
        }
    }
}

#[component]
pub fn ChapterDetail(chapter: Chapter, verse_visibility_enabled: ReadSignal<bool>) -> impl IntoView {
    let bible_signal = init_bible_signal();
    
    // Parse verse ranges from URL - track location explicitly for reactivity
    let location = leptos_router::hooks::use_location();
    let highlighted_verses = Memo::new(move |_| {
        let search_params = location.search.get(); // Explicitly track location.search
        // Parse verse ranges from the search params
        if let Some(verses_param) = search_params
            .split('&')
            .find_map(|param| {
                let mut parts = param.split('=');
                if parts.next()? == "verses" {
                    parts.next()
                } else {
                    None
                }
            })
        {
            verses_param
                .split(',')
                .filter_map(|range_str| VerseRange::from_string(range_str))
                .collect()
        } else {
            Vec::new()
        }
    });
    
    // Enable smooth scrolling globally
    Effect::new(move |_| {
        spawn_local(async move {
            // Small delay to ensure the DOM is ready
            gloo_timers::future::TimeoutFuture::new(50).await;
            
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    // Use CSS scroll-behavior for smooth scrolling
                    if let Some(body) = document.body() {
                        if let Ok(html_body) = body.dyn_into::<web_sys::HtmlElement>() {
                            let _ = html_body.style().set_property("scroll-behavior", "smooth");
                        }
                    }
                    if let Some(document_element) = document.document_element() {
                        if let Ok(html_element) = document_element.dyn_into::<web_sys::HtmlElement>() {
                            let _ = html_element.style().set_property("scroll-behavior", "smooth");
                        }
                    }
                }
            }
        });
    });

    // Auto-focus and scroll to highlighted verses whenever they change
    Effect::new(move |_| {
        let verse_ranges = highlighted_verses.get();
        
        
        spawn_local(async move {
            // Try multiple times with increasing delays to handle timing issues
            for delay in [50, 150, 300, 500] {
                gloo_timers::future::TimeoutFuture::new(delay).await;
                
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(first_range) = verse_ranges.first() {
                            // Focus on the first verse in the first range
                            let verse_id = format!("verse-{}", first_range.start);
                            
                            if let Some(verse_element) = document.get_element_by_id(&verse_id) {
                                // First scroll the element into view
                                verse_element.scroll_into_view();
                                
                                // Then try to focus and adjust position
                                if let Ok(html_element) = verse_element.dyn_into::<web_sys::HtmlElement>() {
                                    let _ = html_element.focus();
                                    
                                    // Try to adjust position to vim-like scrolloff (1/3 from top)
                                    if let Some(window) = web_sys::window() {
                                        if let Ok(window_height) = window.inner_height() {
                                            if let Some(window_height) = window_height.as_f64() {
                                                // Get current scroll position and adjust
                                                let current_scroll = window.scroll_y().unwrap_or(0.0);
                                                // Move the element down by 1/3 of screen height to achieve vim scrolloff
                                                let adjusted_scroll = current_scroll - (window_height / 3.0);
                                                window.scroll_to_with_x_and_y(0.0, adjusted_scroll);
                                            }
                                        }
                                    }
                                }
                                break; // Success, exit retry loop
                            }
                        } else {
                            // No verses selected, focus the chapter heading for accessibility
                            if let Some(heading_element) = document.get_element_by_id("chapter-heading") {
                                if let Ok(html_element) = heading_element.dyn_into::<web_sys::HtmlElement>() {
                                    let _ = html_element.focus();
                                }
                            }
                            break; // No verses to scroll to, exit loop
                        }
                    }
                }
            }
        });
    });
    
    // Clone the chapter for use in closures
    // Strategic cloning: clone once per memo instead of multiple times
    let chapter_for_prev = chapter.clone();
    let chapter_for_next = chapter.clone();
    let chapter_for_data = chapter.clone();
    
    // Create reactive computations for navigation chapters
    let prev_chapter = Memo::new(move |_| {
        if let Some(bible) = bible_signal.get() {
            bible.get_previous_chapter(&chapter_for_prev)
        } else {
            get_bible().get_previous_chapter(&chapter_for_prev)
        }
    });
    
    let next_chapter = Memo::new(move |_| {
        if let Some(bible) = bible_signal.get() {
            bible.get_next_chapter(&chapter_for_next)
        } else {
            get_bible().get_next_chapter(&chapter_for_next)
        }
    });
    
    let prev_path = Memo::new(move |_| prev_chapter.get().as_ref().map(|ch| ch.to_path()));
    let next_path = Memo::new(move |_| next_chapter.get().as_ref().map(|ch| ch.to_path()));
    
    // Create reactive chapter data - only update when bible translation changes, not on verse navigation
    let current_chapter_data = Memo::new(move |_| {
        if let Some(bible) = bible_signal.get() {
            // Try to get the equivalent chapter from the new Bible
            let book_name = chapter_for_data.name.split_whitespace().take(chapter_for_data.name.split_whitespace().count() - 1).collect::<Vec<_>>().join(" ");
            if let Ok(new_chapter) = bible.get_chapter(&book_name, chapter_for_data.chapter) {
                new_chapter
            } else {
                chapter_for_data.clone()
            }
        } else {
            chapter_for_data.clone()
        }
    });
    
    // Cache the chapter data to prevent unnecessary re-renders during verse navigation
    let stable_chapter_data = RwSignal::new(current_chapter_data.get_untracked());
    
    // Only update the stable data when the chapter actually changes (not just verse navigation)
    Effect::new(move |_| {
        let new_chapter = current_chapter_data.get();
        let current_stable = stable_chapter_data.get_untracked();
        
        // Only update if the chapter book/number changed, not just verse highlighting
        if new_chapter.name != current_stable.name || new_chapter.chapter != current_stable.chapter {
            stable_chapter_data.set(new_chapter);
        }
    });
    
    view! {
        <article class="chapter-detail max-w-2xl mx-auto px-4 pb-32">
            <header class="mb-8">
                <h1 id="chapter-heading" class="text-3xl font-bold" style="color: var(--theme-text-primary)" tabindex="-1">{move || get_translated_chapter_name(&stable_chapter_data.get().name)}</h1>
            </header>
            
            <div class="verses text-lg leading-8" style="color: var(--theme-text-primary)" role="main" aria-label="Chapter text">
                {move || {
                    let chapter_data = stable_chapter_data.get();
                    let verses = &chapter_data.verses;
                    let verse_ranges = highlighted_verses.get(); // Single reactive read
                    
                    // Pre-allocate vector with exact capacity for better memory efficiency
                    let mut verse_views = Vec::with_capacity(verses.len());
                    
                    for verse in verses {
                        let is_highlighted = verse_ranges.iter().any(|range| range.contains(verse.verse));
                        
                        // Use theme colors via CSS custom properties
                        let verse_number_class = if is_highlighted {
                            "text-xs font-semibold mr-1 align-sub"
                        } else {
                            "text-xs mr-1 align-sub"
                        };
                        
                        let verse_number_style = if is_highlighted {
                            "color: var(--theme-verse-number-highlighted)"
                        } else {
                            "color: var(--theme-verse-number)"
                        };
                        
                        let verse_text_class = if is_highlighted {
                            "font-bold px-1 rounded"
                        } else {
                            ""
                        };
                        
                        let verse_text_style = if is_highlighted {
                            "color: var(--theme-verse-text-highlighted); background-color: var(--theme-verse-background-highlighted)"
                        } else {
                            "color: var(--theme-text-primary)"
                        };
                        
                        let tabindex = if is_highlighted { "0" } else { "-1" };
                        // Clone verse text for view (required by Leptos)
                        let verse_text = verse.text.clone();
                        let verse_number = verse.verse;
                        
                        verse_views.push(view! {
                            <>
                                <Show 
                                    when=move || verse_visibility_enabled.get() && verse_number != 1
                                    fallback=|| view! { <></> }
                                >
                                    <span 
                                        class=verse_number_class
                                        style=verse_number_style
                                        role="text"
                                    >
                                        {verse_number}
                                    </span>
                                </Show>
                                <span 
                                    class=verse_text_class
                                    style=verse_text_style
                                    id=format!("verse-{}", verse_number)
                                    tabindex=tabindex
                                >
                                    {verse_text}
                                </span>
                            </>
                        });
                    }
                    
                    verse_views
                }}
            </div>
            
            <nav class="flex justify-between items-center mt-8 pt-6 border-t" style="border-color: var(--theme-sidebar-border)" role="navigation" aria-label="Chapter navigation">
                {move || if let Some(path) = prev_path.get() {
                    view! {
                        <A href=path attr:class="p-4 rounded-md transition-colors group navigation-button" attr:style="color: var(--theme-navigation-text); display: flex; align-items: center; justify-content: center;" attr:aria-label={get_navigation_text("previous_chapter")} attr:title={get_navigation_text("previous_chapter")}>
                            <svg class="w-8 h-8 group-hover:transform group-hover:-translate-x-1 transition-transform" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24" aria-hidden="true" style="min-width: 32px; min-height: 32px;">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7"></path>
                            </svg>
                        </A>
                    }.into_any()
                } else {
                    view! { <div class="invisible"></div> }.into_any()
                }}
                
                {move || if let Some(path) = next_path.get() {
                    view! {
                        <A href=path attr:class="p-4 rounded-md transition-colors group navigation-button" attr:style="color: var(--theme-navigation-text); display: flex; align-items: center; justify-content: center;" attr:aria-label={get_navigation_text("next_chapter")} attr:title={get_navigation_text("next_chapter")}>
                            <svg class="w-8 h-8 group-hover:transform group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24" aria-hidden="true" style="min-width: 32px; min-height: 32px;">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"></path>
                            </svg>
                        </A>
                    }.into_any()
                } else {
                    view! { <div class="invisible"></div> }.into_any()
                }}
            </nav>
        </article>
    }
}
