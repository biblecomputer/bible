use leptos::IntoView;
use leptos::prelude::*;
use leptos::view;
use leptos_router::hooks::use_params_map;
use crate::types::*;

#[component]
pub fn ChapterDetail(chapter: Chapter) -> impl IntoView  {
    view! {
        <h1>{chapter.name.as_str()}</h1>
        {chapter.verses.iter().map(|verse| {
            view! {
                <p>{verse.text.as_str()}</p>
            }
        }).collect_view()}
    }
}
