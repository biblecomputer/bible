use crate::types::{*, get_bible, is_mobile_screen};
use leptos::component;
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::location::Location;
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
            <div class="mb-6">
                <a 
                    href="/translations"
                    class="flex items-center px-3 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 transition-colors"
                    on:click=move |_| {
                        if is_mobile_screen() {
                            set_sidebar_open.set(false);
                        }
                    }
                >
                    <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.746 0 3.332.477 4.5 1.253v13C19.832 18.477 18.246 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path>
                    </svg>
                    "Translations"
                </a>
            </div>
            
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
                                if is_mobile_screen() {
                                    set_sidebar_open.set(false);
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
