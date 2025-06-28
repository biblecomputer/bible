use crate::Bible;
use leptos::component;
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;
use leptos_router::components::A;

#[component]
pub fn Sidebar<'a>(bible: &'a Bible) -> impl IntoView + 'a {
    view! {
        <ul>
            {bible.books.iter().flat_map(|b| b.chapters.iter().map(|c| {
                let path = c.to_path();
                let name = c.name.clone();
                view! {
                    <li>
                        <A href={path}>{name}</A>
                    </li>
                }
            }
            ).collect_view()).collect_view()}
        </ul>
    }
}
