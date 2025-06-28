use leptos::IntoView;
use leptos::prelude::*;
use leptos::view;
use leptos_router::hooks::use_params_map;
use crate::types::*;

#[component]
pub fn ChapterView(bible: Bible) -> impl IntoView {
    let params = use_params_map();
    let book = move || params.read().get("book").unwrap();
    let chapter = move || {
        params
            .read()
            .get("chapter")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(1) // fallback chapter number if parsing fails
    };

    let chapter: Chapter = bible.get_chapter(&book(), chapter()).unwrap();

    view! {
        <ChapterDetail chapter=chapter />
    }
}

#[component]
fn ChapterDetail(chapter: Chapter) -> impl IntoView  {
    view! {
        <h1>{chapter.name.as_str()}</h1>
        {chapter.verses.iter().map(|verse| {
            view! {
                <p>{verse.text.as_str()}</p>
            }
        }).collect_view()}
    }
}
