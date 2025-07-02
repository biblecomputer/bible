use crate::types::{*, BIBLE};
use leptos::component;
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;
use leptos_router::components::A;

#[component]
pub fn Sidebar() -> impl IntoView {
    let (selected_book, set_selected_book) = signal(String::from("Genesis"));

    view! {
        <div class="sidebar">
            <h2 class="text-lg font-bold mb-4 text-gray-800">Books</h2>
            <ul class="space-y-2">
            {BIBLE.books.iter().map(|b| view! {
                <BookView
                    book=b.clone()
                    selected_book=selected_book
                    set_selected_book=set_selected_book
                />
            }).collect_view()}
            </ul>
        </div>
    }
}

#[component]
fn BookView(
    book: Book,
    selected_book: ReadSignal<String>,
    set_selected_book: WriteSignal<String>,
) -> impl IntoView {

    view! {
        <li>
            <button 
                class="w-full text-left px-3 py-2 rounded-md hover:bg-gray-200 transition-colors duration-150 font-medium text-gray-700"
                on:click={
                    let book_name = book.name.clone();
                    move |_| {
                        set_selected_book.update(|b| if *b == book_name {
                            // When you want to collapse the chapters
                            *b = String::new();
                        } else {
                            *b = book_name.clone();
                        })
                    }
                }
            >
                {book.name.clone()}
            </button>
            <Show
                when={
                    let book_name = book.name.clone();
                    move || selected_book.get() == book_name
                }
                fallback=|| view! { <></> }
            >
            <div class="ml-4 mt-2 grid grid-cols-5 gap-1">
            {book.chapters.iter().cloned().map(|c| view! {
                <div class="text-center px-2 py-1 text-xs text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded transition-colors duration-150">
                    <A href=c.to_path()>
                        {c.chapter}
                    </A>
                </div>
            }).collect_view()}
            </div>
            </Show>
        </li>
    }
}
