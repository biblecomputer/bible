use crate::types::*;
use leptos::component;
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;
use leptos_router::components::A;

#[component]
pub fn Sidebar<'a>(bible: &'a Bible) -> impl IntoView + 'a {
    let (selected_book, set_selected_book) = signal(String::from("Genesis"));

    view! {
        <ul>
        {bible.books.iter().map(|b| view! {
            <BookView
                book=b.clone()
                selected_book=selected_book
                set_selected_book=set_selected_book
            />
        }).collect_view()}
        </ul>
    }
}

#[component]
fn BookView(
    book: Book,
    selected_book: ReadSignal<String>,
    set_selected_book: WriteSignal<String>,
) -> impl IntoView {
    let book_name = book.name.clone();
    let book_name_2 = book.name.clone();

    view! {
        <li>
            <button on:click=move |_| {
                set_selected_book.update(|b| if *b == book_name {
                    // When you want to collapse the chapters
                    *b = String::new();
                } else {
                    *b = book_name.clone();
                })
            }>
                {book_name.clone()}
            </button>
            <Show
                when=move || selected_book.get() == book_name_2.clone()
                fallback=|| view! { <></> }
            >
            <ul>
            {book.chapters.iter().cloned().map(|c| view! {
                <li>
                    <A href=c.to_path() >{c.chapter}</A>
                </li>
            }).collect_view()}
            </ul>
            </Show>
        </li>
    }
}
