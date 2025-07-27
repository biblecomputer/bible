use crate::core::{Chapter, get_bible, init_bible_signal};
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;
use leptos_router::components::A;

#[component]
pub fn ChapterDetail(chapter: Chapter) -> impl IntoView {
    let bible_signal = init_bible_signal();
    
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
                <h1 class="text-3xl font-bold text-black">{move || current_chapter_data.get().name.clone()}</h1>
            </header>
            
            <div class="verses text-lg leading-8 text-black" role="main" aria-label="Chapter text">
                {move || current_chapter_data.get().verses.iter().cloned().map(|verse| {
                    view! {
                        <>
                            <Show 
                                when=move || verse.verse != 1
                                fallback=|| view! { <></> }
                            >
                                <span 
                                    class="verse-number text-sm text-black font-medium mr-1 select-none align-super"
                                    role="text"
                                >
                                    {verse.verse}
                                </span>
                            </Show>
                            <span 
                                class="verse-text" 
                                id=format!("verse-{}", verse.verse)
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
                                "Previous Chapter"
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
                                "Next Chapter"
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
