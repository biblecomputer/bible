use crate::{Chapter, BIBLE};
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;
use leptos_router::components::A;

#[component]
pub fn ChapterDetail(chapter: Chapter) -> impl IntoView {
    let prev_chapter = BIBLE.get_previous_chapter(&chapter);
    let next_chapter = BIBLE.get_next_chapter(&chapter);
    let prev_path = prev_chapter.as_ref().map(|ch| ch.to_path());
    let next_path = next_chapter.as_ref().map(|ch| ch.to_path());
    
    view! {
        <article class="chapter-detail max-w-2xl mx-auto px-4">
            <header class="mb-8">
                <h1 class="text-3xl font-bold text-gray-900">{chapter.name.clone()}</h1>
            </header>
            
            <div class="verses text-lg leading-8 text-gray-800" role="main" aria-label="Chapter text">
                {chapter.verses.iter().cloned().map(|verse| {
                    view! {
                        <>
                            <Show 
                                when=move || verse.verse != 1
                                fallback=|| view! { <></> }
                            >
                                <span 
                                    class="verse-number text-sm text-gray-500 font-medium mr-1 select-none align-super"
                                    tabindex="0"
                                    role="text"
                                >
                                    {verse.verse}
                                </span>
                            </Show>
                            <span 
                                class="verse-text focus:outline-none focus:ring-2 focus:ring-blue-200 focus:bg-blue-50 rounded-sm" 
                                id=format!("verse-{}", verse.verse)
                                tabindex="0"
                                role="text"
                            >
                                {verse.text.clone()}
                            </span>
                        </>
                    }
                }).collect_view()}
            </div>
            
            <div class="flex justify-between items-center mt-8 pt-6 border-t border-gray-200">
                {if let Some(path) = prev_path {
                    view! {
                        <div class="flex items-center px-4 py-2 text-sm font-medium text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-md transition-colors">
                            <A href=path>
                                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                                </svg>
                                Previous
                            </A>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
                
                {if let Some(path) = next_path {
                    view! {
                        <div class="flex items-center px-4 py-2 text-sm font-medium text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-md transition-colors">
                            <A href=path>
                                Next
                                <svg class="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                </svg>
                            </A>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>
        </article>
    }
}
