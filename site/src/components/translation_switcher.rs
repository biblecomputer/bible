use leptos::prelude::*;
use crate::storage::{
    get_selected_translation, get_downloaded_translations,
    switch_bible_translation, set_selected_translation,
    get_translations
};
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn TranslationSwitcher() -> impl IntoView {
    let downloaded_translations = get_downloaded_translations();
    
    // Only show if there are multiple downloaded translations
    if downloaded_translations.len() <= 1 {
        return view! { <></> }.into_any();
    }
    
    let (is_open, set_is_open) = signal(false);
    let (is_switching, set_is_switching) = signal(false);
    let current_translation = signal(get_selected_translation().unwrap_or_else(|| "sv".to_string())).0;
    
    // Get translation details
    let all_translations = get_translations();
    let available_translations = signal(all_translations
        .into_iter()
        .filter(|t| downloaded_translations.contains(&t.short_name))
        .collect::<Vec<_>>()).0;
    
    let current_translation_name = available_translations.get()
        .iter()
        .find(|t| t.short_name == current_translation.get())
        .map(|t| t.name.clone())
        .unwrap_or_else(|| "Unknown".to_string());
    
    let handle_translation_switch = move |translation_short_name: String| {
        set_is_switching.set(true);
        set_is_open.set(false);
        
        let _ = set_selected_translation(&translation_short_name);
        
        spawn_local(async move {
            if let Err(e) = switch_bible_translation(&translation_short_name).await {
                leptos::logging::error!("Failed to switch translation: {}", e);
            }
            set_is_switching.set(false);
        });
    };
    
    view! {
        <div class="relative">
            <button
                class="flex items-center px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
                disabled=is_switching.get()
                on:click=move |_| set_is_open.update(|open| *open = !*open)
            >
                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.746 0 3.332.477 4.5 1.253v13C19.832 18.477 18.246 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path>
                </svg>
                <span class="max-w-32 truncate">
                    {if is_switching.get() { "Switching..." } else { &current_translation_name }}
                </span>
                <svg class="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                </svg>
            </button>
            
            <Show
                when=move || is_open.get()
                fallback=|| view! { <></> }
            >
                <div class="absolute right-0 mt-2 w-56 bg-white border border-gray-200 rounded-md shadow-lg z-50">
                    <div class="py-1">
                        <div class="px-3 py-2 text-xs font-medium text-gray-500 uppercase tracking-wide border-b border-gray-100">
                            "Switch Translation"
                        </div>
                        <For
                            each=move || available_translations.get()
                            key=|translation| translation.short_name.clone()
                            children=move |translation| {
                                let is_current = translation.short_name == current_translation.get();
                                let short_name = translation.short_name.clone();
                                
                                view! {
                                    <button
                                        class=if is_current { "w-full text-left px-3 py-2 text-sm bg-blue-50 text-blue-700" } else { "w-full text-left px-3 py-2 text-sm hover:bg-gray-100 transition-colors text-gray-700" }
                                        disabled=is_current
                                        on:click=move |_| {
                                            handle_translation_switch(short_name.clone());
                                        }
                                    >
                                        <div class="flex items-center justify-between">
                                            <div>
                                                <div class="font-medium">{translation.name.clone()}</div>
                                                <div class="text-xs text-gray-500">{translation.release_year.to_string()}</div>
                                            </div>
                                            {if is_current {
                                                view! {
                                                    <svg class="w-4 h-4 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                                                        <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
                                                    </svg>
                                                }.into_any()
                                            } else {
                                                view! { <></> }.into_any()
                                            }}
                                        </div>
                                    </button>
                                }
                            }
                        />
                        <div class="border-t border-gray-100 mt-1">
                            <a
                                href="/translations"
                                class="block w-full text-left px-3 py-2 text-sm text-blue-600 hover:bg-gray-100 transition-colors"
                                on:click=move |_| set_is_open.set(false)
                            >
                                "Manage Translations..."
                            </a>
                        </div>
                    </div>
                </div>
            </Show>
            
            // Click outside to close
            <Show
                when=move || is_open.get()
                fallback=|| view! { <></> }
            >
                <div 
                    class="fixed inset-0 z-40"
                    on:click=move |_| set_is_open.set(false)
                />
            </Show>
        </div>
    }.into_any()
}