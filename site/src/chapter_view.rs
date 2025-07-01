use crate::Chapter;
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;

#[component]
pub fn ChapterDetail(chapter: Chapter) -> impl IntoView {
    view! {
        <div class="chapter-detail">
            <h1 class="text-3xl font-bold mb-6 text-gray-900">{chapter.name.as_str()}</h1>
            <div class="prose max-w-none">
                {chapter.verses.iter().map(|verse| {
                    view! {
                        <p class="mb-4 text-gray-800 leading-relaxed">{verse.text.as_str()}</p>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
