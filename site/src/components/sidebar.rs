use crate::core::{get_bible, init_bible_signal};
use crate::core::*;
use crate::utils::is_mobile_screen;
use crate::storage::translations::get_current_translation;
use crate::storage::save_sidebar_open;
use crate::core::types::Language;
use crate::translation_map::translation::Translation;
use leptos::component;
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::NavigateOptions;
use leptos_router::location::Location;
use urlencoding::decode;

fn convert_language(storage_lang: &crate::storage::translation_storage::Language) -> Language {
    match storage_lang {
        crate::storage::translation_storage::Language::Dutch => Language::Dutch,
        crate::storage::translation_storage::Language::English => Language::English,
    }
}

fn get_translated_name(input: &str) -> String {
    if let Some(current_translation) = get_current_translation() {
        if let Some(first_language) = current_translation.languages.first() {
            let translation = Translation::from_language(convert_language(first_language));
            
            // Convert input to lowercase and replace spaces with underscores for lookup
            let lookup_key = input.to_lowercase().replace(' ', "_");
            
            if let Some(translated_name) = translation.get(&lookup_key) {
                return translated_name;
            }
        }
    }
    
    // Return original input if no translation found
    input.to_string()
}

fn is_name_translated(input: &str) -> bool {
    if let Some(current_translation) = get_current_translation() {
        if let Some(first_language) = current_translation.languages.first() {
            let translation = Translation::from_language(convert_language(first_language));
            
            // Convert input to lowercase and replace spaces with underscores for lookup
            let lookup_key = input.to_lowercase().replace(' ', "_");
            
            return translation.get(&lookup_key).is_some();
        }
    }
    false
}

fn get_ui_text(key: &str) -> String {
    if let Some(current_translation) = get_current_translation() {
        if let Some(first_language) = current_translation.languages.first() {
            match (key, first_language) {
                ("books", crate::storage::translation_storage::Language::Dutch) => "Boeken".to_string(),
                ("books", crate::storage::translation_storage::Language::English) => "Books".to_string(),
                _ => key.to_string(),
            }
        } else {
            key.to_string()
        }
    } else {
        // Default to English
        match key {
            "books" => "Books".to_string(),
            _ => key.to_string(),
        }
    }
}

#[component]
pub fn Sidebar(set_sidebar_open: WriteSignal<bool>) -> impl IntoView {
    let location = use_location();
    let bible_signal = init_bible_signal();
    
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

    // Create reactive books list
    let books = Memo::new(move |_| {
        if let Some(bible) = bible_signal.get() {
            bible.books
        } else {
            get_bible().books.clone() // Keep clone for now, optimize component later
        }
    });

    view! {
        <div class="sidebar">
            <h2 class="text-lg font-bold mb-4 text-black">{get_ui_text("books")}</h2>
            <ul class="space-y-2">
            {move || books.get().iter().map(|b| view! {
                <BookView
                    book=b.clone() // Required by component signature
                    current_book=current_book
                    selected_book=selected_book
                    set_selected_book=set_selected_book
                    location=location.clone()
                    set_sidebar_open=set_sidebar_open
                />
            }).collect::<Vec<_>>()}
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
                <span class={
                    let book_name = book.name.clone();
                    let is_translated = is_name_translated(&book_name);
                    if is_translated { "" } else { "font-bold" }
                }>
                    {
                        let book_name = book.name.clone();
                        get_translated_name(&book_name)
                    }
                </span>
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
                                navigate(&nav_path, NavigateOptions { scroll: false, ..Default::default() });
                                // Close sidebar on mobile when chapter is selected
                                if is_mobile_screen() {
                                    set_sidebar_open.set(false);
                                    save_sidebar_open(false);
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
