use crate::types::{*, get_bible};
use leptos::component;
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::location::Location;
use leptos::web_sys::window;
use urlencoding::decode;

#[component]
pub fn Sidebar(set_sidebar_open: WriteSignal<bool>) -> impl IntoView {
    let location = use_location();
    
    
    // Extract book name from current URL and auto-expand it
    let current_book = Memo::new(move |_| {
        let pathname = location.pathname.get();
        let path_parts: Vec<&str> = pathname.trim_start_matches('/').split('/').collect();
        
        if path_parts.len() >= 2 {
            // Decode the URL-encoded book name and convert underscores back to spaces
            if let Ok(decoded) = decode(path_parts[0]) {
                return decoded.into_owned();
            }
        }
        
        String::new() // Return empty string if no valid book found
    });
    
    let (selected_book, set_selected_book) = signal(String::new());

    view! {
        <div class="sidebar">
            <h2 class="text-lg font-bold mb-4 text-black">Books</h2>
            <ul class="space-y-2">
            {get_bible().books.iter().map(|b| view! {
                <BookView
                    book=b.clone()
                    current_book=current_book
                    selected_book=selected_book
                    set_selected_book=set_selected_book
                    location=location.clone()
                    set_sidebar_open=set_sidebar_open
                />
            }).collect_view()}
            </ul>
        </div>
    }
}

#[component]
fn BookView(
    book: Book,
    current_book: Memo<String>,
    selected_book: ReadSignal<String>,
    set_selected_book: WriteSignal<String>,
    location: Location,
    set_sidebar_open: WriteSignal<bool>,
) -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <li>
            <button 
                class="w-full text-left px-3 py-2 rounded-md hover:bg-gray-100 transition-colors duration-150 font-medium text-black"
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
                    move || {
                        let current = current_book.get();
                        let selected = selected_book.get();
                        // Show if it's the current book from URL OR manually selected
                        book_name == current || book_name == selected
                    }
                }
                fallback=|| view! { <></> }
            >
            <div class="ml-4 mt-2 grid grid-cols-5 gap-1">
            {book.chapters.iter().cloned().map(|c| {
                let chapter_path = c.to_path();
                let chapter_path_for_class = chapter_path.clone();
                let location = location.clone();
                
                view! {
                    <button 
                        class={
                            move || {
                                let current_path = location.pathname.get();
                                if current_path == chapter_path_for_class {
                                    "w-full text-center px-3 py-2 text-xs bg-blue-500 text-white rounded transition-colors duration-150"
                                } else {
                                    "w-full text-center px-3 py-2 text-xs text-black hover:text-blue-600 hover:bg-blue-50 rounded transition-colors duration-150"
                                }
                            }
                        }
                        on:click={
                            let navigate = navigate.clone();
                            let nav_path = chapter_path.clone();
                            move |_| {
                                navigate(&nav_path, Default::default());
                                // Close sidebar on mobile when chapter is selected
                                if let Some(window) = window() {
                                    if let Ok(width) = window.inner_width() {
                                        if let Some(width_num) = width.as_f64() {
                                            if width_num < 768.0 {
                                                set_sidebar_open.set(false);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    >
                        {c.chapter}
                    </button>
                }
            }).collect_view()}
            </div>
            </Show>
        </li>
    }
}
