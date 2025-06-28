use crate::Chapter;
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;

#[component]
pub fn ChapterDetail(chapter: Chapter) -> impl IntoView {
    view! {
        <h1>{chapter.name.as_str()}</h1>
        {chapter.verses.iter().map(|verse| {
            view! {
                <p>{verse.text.as_str()}</p>
            }
        }).collect_view()}
    }
}
