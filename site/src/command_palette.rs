use crate::{Bible, Chapter};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos::web_sys::KeyboardEvent;

#[component]
pub fn CommandPalette(
    bible: Bible,
    is_open: ReadSignal<bool>,
    set_is_open: WriteSignal<bool>,
) -> impl IntoView {
    let navigate = use_navigate();
    let (search_query, set_search_query) = signal(String::new());
    let (selected_index, set_selected_index) = signal(0usize);
    let (navigate_to, set_navigate_to) = signal::<Option<String>>(None);
    
    // Create a node ref for the input element
    let input_ref = NodeRef::<leptos::html::Input>::new();

    // Create a memo for filtered chapters
    let filtered_chapters = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        if query.is_empty() {
            return Vec::new();
        }

        bible
            .books
            .iter()
            .flat_map(|book| book.chapters.iter())
            .filter(|chapter| {
                chapter.name.to_lowercase().contains(&query)
            })
            .take(10)
            .cloned()
            .collect::<Vec<Chapter>>()
    });

    // Set up global keyboard handling when palette is open
    let nav = navigate.clone();
    Effect::new(move |_| {
        if is_open.get() {
            let _nav = nav.clone();
            let handle_keydown = move |e: KeyboardEvent| {
                match e.key().as_str() {
                    "Escape" => {
                        set_is_open.set(false);
                        set_search_query.set(String::new());
                        set_selected_index.set(0);
                    }
                    "ArrowDown" => {
                        e.prevent_default();
                        let chapters = filtered_chapters.get();
                        if !chapters.is_empty() {
                            set_selected_index.update(|i| {
                                if *i < chapters.len() - 1 {
                                    *i += 1;
                                } else {
                                    *i = 0; // wrap to first
                                }
                            });
                        }
                    }
                    "ArrowUp" => {
                        e.prevent_default();
                        let chapters = filtered_chapters.get();
                        if !chapters.is_empty() {
                            set_selected_index.update(|i| {
                                if *i > 0 {
                                    *i -= 1;
                                } else {
                                    *i = chapters.len() - 1; // wrap to last
                                }
                            });
                        }
                    }
                    "Enter" => {
                        e.prevent_default();
                        let chapters = filtered_chapters.get();
                        let selected = selected_index.get();
                        if let Some(chapter) = chapters.get(selected) {
                            set_navigate_to.set(Some(chapter.to_path()));
                            set_is_open.set(false);
                            set_search_query.set(String::new());
                            set_selected_index.set(0);
                        }
                    }
                    _ => {}
                }
            };
            
            let _cleanup = window_event_listener(leptos::ev::keydown, handle_keydown);
            // cleanup will happen when effect re-runs or component unmounts
        }
    });

    // Reset selected index when search changes
    Effect::new(move |_| {
        search_query.track();
        set_selected_index.set(0);
    });

    // Handle navigation
    Effect::new(move |_| {
        if let Some(path) = navigate_to.get() {
            navigate(&path, Default::default());
            set_navigate_to.set(None);
        }
    });

    // Focus input when palette opens
    Effect::new(move |_| {
        if is_open.get() {
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
            }
        }
    });


    view! {
        <Show when=move || is_open.get() fallback=|| ()>
            // Backdrop
            <div 
                class="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-start justify-center pt-20"
                on:click=move |_| set_is_open.set(false)
            >
                // Command Palette Modal
                <div 
                    class="bg-white rounded-lg shadow-xl w-full max-w-lg mx-4 max-h-96 flex flex-col"
                    on:click=move |e| e.stop_propagation()
                >
                    // Search Input
                    <div class="p-4 border-b border-gray-200">
                        <input
                            node_ref=input_ref
                            type="text"
                            placeholder="Search chapters... (e.g., 'Genesis 1', 'John 3:16')"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            prop:value=search_query
                            on:input=move |e| set_search_query.set(event_target_value(&e))
                        />
                    </div>

                    // Results List
                    <div class="flex-1 overflow-y-auto">
                        <div class="py-2">
                            <Show
                                when=move || !search_query.get().is_empty()
                                fallback=|| view! { <div class="px-4 py-2 text-gray-500">"Start typing to search chapters..."</div> }
                            >
                                <div class="max-h-64 overflow-y-auto">
                                    {move || {
                                        let chapters = filtered_chapters.get();
                                        let selected = selected_index.get();
                                        
                                        if chapters.is_empty() {
                                            view! { 
                                                <div class="px-4 py-2 text-gray-500 text-sm">
                                                    "No chapters found"
                                                </div> 
                                            }.into_any()
                                        } else {
                                            view! {
                                                <div>
                                                    {chapters.into_iter().enumerate().map(|(index, chapter)| {
                                                        let is_selected = index == selected;
                                                        let chapter_name = chapter.name.clone();
                                                        let chapter_path = chapter.to_path();
                                                        view! {
                                                            <div 
                                                                class=if is_selected { 
                                                                    "px-4 py-3 bg-blue-500 text-white cursor-pointer flex items-center border-b border-blue-400" 
                                                                } else { 
                                                                    "px-4 py-3 hover:bg-gray-100 cursor-pointer flex items-center border-b border-gray-100" 
                                                                }
                                                                on:click={
                                                                    let path = chapter_path.clone();
                                                                    move |_| {
                                                                        set_navigate_to.set(Some(path.clone()));
                                                                        set_is_open.set(false);
                                                                        set_search_query.set(String::new());
                                                                        set_selected_index.set(0);
                                                                    }
                                                                }
                                                            >
                                                                <div class="flex-1">
                                                                    <div class="font-medium">
                                                                        {chapter_name}
                                                                    </div>
                                                                </div>
                                                                {if is_selected {
                                                                    view! {
                                                                        <div class="text-xs opacity-75">
                                                                            "↵"
                                                                        </div>
                                                                    }.into_any()
                                                                } else {
                                                                    view! { <div></div> }.into_any()
                                                                }}
                                                            </div>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            }.into_any()
                                        }
                                    }}
                                </div>
                            </Show>
                        </div>
                    </div>

                    // Footer with hint
                    <div class="px-4 py-2 border-t border-gray-200 text-xs text-gray-500">
                        Use up/down arrows to navigate, Enter to select, Esc to close
                    </div>
                </div>
            </div>
        </Show>
    }
}

// Simple fuzzy search scoring - will be used when search is implemented
#[allow(dead_code)]
fn fuzzy_score(text: &str, query: &str) -> usize {
    if query.is_empty() {
        return 1;
    }
    
    let text_chars: Vec<char> = text.chars().collect();
    let query_chars: Vec<char> = query.chars().collect();
    
    let mut score = 0;
    let mut text_index = 0;
    let mut consecutive_matches = 0;
    
    for &query_char in &query_chars {
        let mut found = false;
        
        while text_index < text_chars.len() {
            if text_chars[text_index] == query_char {
                found = true;
                consecutive_matches += 1;
                score += 1 + consecutive_matches; // Bonus for consecutive matches
                text_index += 1;
                break;
            } else {
                consecutive_matches = 0;
                text_index += 1;
            }
        }
        
        if !found {
            return 0; // Query character not found
        }
    }
    
    // Bonus for exact matches or matches at word boundaries
    if text.contains(query) {
        score += query.len() * 10;
    }
    
    score
}