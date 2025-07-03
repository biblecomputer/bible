use crate::Chapter;
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;

#[component]
pub fn ChapterDetail(chapter: Chapter) -> impl IntoView {
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
        </article>
    }
}
