use crate::core::{Chapter, get_bible, init_bible_signal, parse_verse_ranges_from_url};
use crate::storage::translations::get_current_translation;
use crate::core::types::Language;
use crate::translation_map::translation::Translation;
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;
use leptos_router::components::A;
use wasm_bindgen_futures::spawn_local;
use leptos::wasm_bindgen::JsCast;

fn convert_language(storage_lang: &crate::storage::translation_storage::Language) -> Language {
    match storage_lang {
        crate::storage::translation_storage::Language::Dutch => Language::Dutch,
        crate::storage::translation_storage::Language::English => Language::English,
    }
}

fn get_translated_chapter_name(chapter_name: &str) -> String {
    if let Some(current_translation) = get_current_translation() {
        if let Some(first_language) = current_translation.languages.first() {
            let translation = Translation::from_language(convert_language(first_language));
            
            // Use the Translation.get() method which handles both book names and chapter references
            if let Some(translated_name) = translation.get(chapter_name) {
                return translated_name;
            }
        }
    }
    
    // Return original name if no translation found
    chapter_name.to_string()
}

fn get_navigation_text(key: &str) -> String {
    if let Some(current_translation) = get_current_translation() {
        if let Some(first_language) = current_translation.languages.first() {
            match (key, first_language) {
                ("previous_chapter", crate::storage::translation_storage::Language::Dutch) => "Vorig Hoofdstuk".to_string(),
                ("next_chapter", crate::storage::translation_storage::Language::Dutch) => "Volgend Hoofdstuk".to_string(),
                ("previous_chapter", crate::storage::translation_storage::Language::English) => "Previous Chapter".to_string(),
                ("next_chapter", crate::storage::translation_storage::Language::English) => "Next Chapter".to_string(),
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
pub fn ChapterDetail(chapter: Chapter) -> impl IntoView {
    let bible_signal = init_bible_signal();
    
    // Parse verse ranges from URL
    let highlighted_verses = Memo::new(|_| parse_verse_ranges_from_url());
    
    // Auto-focus on the first highlighted verse for accessibility, or chapter heading if no verses selected
    Effect::new(move |_| {
        let verse_ranges = highlighted_verses.get();
        
        spawn_local(async move {
            // Small delay to ensure the DOM is ready
            gloo_timers::future::TimeoutFuture::new(150).await;
            
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(first_range) = verse_ranges.first() {
                        // Focus on the first verse in the first range
                        let verse_id = format!("verse-{}", first_range.start);
                        if let Some(verse_element) = document.get_element_by_id(&verse_id) {
                            if let Ok(html_element) = verse_element.dyn_into::<web_sys::HtmlElement>() {
                                let _ = html_element.focus();
                                let _ = html_element.scroll_into_view();
                            }
                        }
                    } else {
                        // No verses selected, focus the chapter heading for accessibility
                        if let Some(heading_element) = document.get_element_by_id("chapter-heading") {
                            if let Ok(html_element) = heading_element.dyn_into::<web_sys::HtmlElement>() {
                                let _ = html_element.focus();
                            }
                        }
                    }
                }
            }
        });
    });
    
    // Clone the chapter for use in closures
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
    
    // Create reactive chapter data
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
    
    view! {
        <article class="chapter-detail max-w-2xl mx-auto px-4">
            <header class="mb-8">
                <h1 id="chapter-heading" class="text-3xl font-bold text-black" tabindex="-1">{move || get_translated_chapter_name(&current_chapter_data.get().name)}</h1>
            </header>
            
            <div class="verses text-lg leading-8 text-black" role="main" aria-label="Chapter text">
                {move || current_chapter_data.get().verses.iter().cloned().map(|verse| {
                    let verse_ranges = highlighted_verses.get();
                    let is_highlighted = verse_ranges.iter().any(|range| range.contains(verse.verse));
                    
                    view! {
                        <>
                            <Show 
                                when=move || verse.verse != 1
                                fallback=|| view! { <></> }
                            >
                                <span 
                                    class=format!(
                                        "verse-number text-sm {} font-medium mr-1 select-none align-super",
                                        if is_highlighted { "text-blue-600 font-bold" } else { "text-black" }
                                    )
                                    role="text"
                                >
                                    {verse.verse}
                                </span>
                            </Show>
                            <span 
                                class=format!(
                                    "verse-text {}",
                                    if is_highlighted { "font-bold text-black bg-yellow-100 px-1 rounded" } else { "" }
                                )
                                id=format!("verse-{}", verse.verse)
                                tabindex=if is_highlighted { "0" } else { "-1" }
                            >
                                {verse.text.clone()}
                            </span>
                        </>
                    }
                }).collect::<Vec<_>>()}
            </div>
            
            <nav class="flex justify-between items-center mt-8 pt-6 border-t border-gray-200" role="navigation" aria-label="Chapter navigation">
                {move || if let Some(path) = prev_path.get() {
                    view! {
                        <div class="flex items-center px-4 py-2 text-sm font-medium text-black hover:text-blue-600 hover:bg-blue-50 rounded-md transition-colors group">
                            <A href=path>
                                <svg class="w-4 h-4 mr-2 group-hover:transform group-hover:-translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                                </svg>
                                {get_navigation_text("previous_chapter")}
                            </A>
                        </div>
                    }.into_any()
                } else {
                    view! { <div class="invisible"></div> }.into_any()
                }}
                
                {move || if let Some(path) = next_path.get() {
                    view! {
                        <div class="flex items-center px-4 py-2 text-sm font-medium text-black hover:text-blue-600 hover:bg-blue-50 rounded-md transition-colors group">
                            <A href=path>
                                {get_navigation_text("next_chapter")}
                                <svg class="w-4 h-4 ml-2 group-hover:transform group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                </svg>
                            </A>
                        </div>
                    }.into_any()
                } else {
                    view! { <div class="invisible"></div> }.into_any()
                }}
            </nav>
        </article>
    }
}
