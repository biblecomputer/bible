use crate::types::{*, BIBLE};
use leptos::component;
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;
use leptos_router::components::A;
use leptos_router::hooks::use_location;
use leptos_router::location::Location;

#[component]
pub fn Sidebar() -> impl IntoView {
    let (selected_book, set_selected_book) = signal(String::from("Genesis"));
    let location = use_location();

    view! {
        <div class="sidebar">
            <h2 class="text-lg font-bold mb-4 text-gray-800">Books</h2>
            <ul class="space-y-2">
            {BIBLE.books.iter().map(|b| view! {
                <BookView
                    book=b.clone()
                    selected_book=selected_book
                    set_selected_book=set_selected_book
                    location=location.clone()
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
    location: Location,
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
            {book.chapters.iter().cloned().map(|c| {
                let current_path = location.pathname.get();
                let chapter_path = c.to_path();
                let is_current = current_path == chapter_path;
                
                view! {
                    <div class={
                        if is_current {
                            "text-center px-2 py-1 text-xs bg-blue-500 text-white rounded transition-colors duration-150"
                        } else {
                            "text-center px-2 py-1 text-xs text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded transition-colors duration-150"
                        }
                    }>
                        <A href=chapter_path>
                            {c.chapter}
                        </A>
                    </div>
                }
            }).collect_view()}
            </div>
            </Show>
        </li>
    }
}
