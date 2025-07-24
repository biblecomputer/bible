use leptos::prelude::*;
use crate::types::{
    get_selected_translation, set_selected_translation, 
    is_translation_downloaded, download_translation, switch_bible_translation
};
use crate::translations::get_translations;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn HomeTranslationPicker() -> impl IntoView {
    let (selected_translation, set_selected_translation_signal) = signal(get_selected_translation().unwrap_or_else(|| "sv".to_string()));
    let (downloading_translation, set_downloading_translation) = signal::<Option<String>>(None);
    let (download_error, set_download_error) = signal::<Option<String>>(None);
    let (is_switching, set_is_switching) = signal(false);
    
    let translations = get_translations();
    
    view! {
        <div class="max-w-4xl mx-auto">
            <div class="text-center mb-8">
                <h1 class="text-4xl font-bold text-gray-900 mb-4">"Bijbel"</h1>
                <p class="text-lg text-gray-600 mb-8">"Kies een vertaling om te beginnen met lezen"</p>
            </div>
            
            <div class="grid gap-6 md:grid-cols-2">
                {
                    translations.into_iter().map(|translation| {
                        let is_downloaded = is_translation_downloaded(&translation.short_name);
                        let short_name_for_class = translation.short_name.clone();
                        let short_name_for_click = translation.short_name.clone();
                        let translation_for_download = translation.clone();
                        
                        view! {
                            <div 
                                class=move || {
                                    let is_selected = selected_translation.get() == short_name_for_class;
                                    format!(
                                        "border rounded-lg p-6 cursor-pointer transition-all hover:shadow-md {}",
                                        if is_selected { 
                                            "border-blue-500 bg-blue-50 shadow-md" 
                                        } else if is_downloaded {
                                            "border-green-300 bg-green-50"
                                        } else { 
                                            "border-gray-200 hover:border-gray-300" 
                                        }
                                    )
                                }
                                on:click=move |_| {
                                    let translation_short_name = short_name_for_click.clone();
                                    let is_downloading = downloading_translation.get().as_ref() == Some(&translation_short_name);
                                    if !is_downloading && !is_switching.get() {
                                        if is_downloaded {
                                            // Switch to existing translation
                                            set_is_switching.set(true);
                                            let _ = set_selected_translation(&translation_short_name);
                                            set_selected_translation_signal.set(translation_short_name.clone());
                                            
                                            spawn_local(async move {
                                                if let Err(e) = switch_bible_translation(&translation_short_name).await {
                                                    leptos::logging::error!("Failed to switch translation: {}", e);
                                                }
                                                set_is_switching.set(false);
                                            });
                                        } else {
                                            // Download translation
                                            set_downloading_translation.set(Some(translation_short_name.clone()));
                                            set_download_error.set(None);
                                            
                                            let translation_clone = translation_for_download.clone();
                                            spawn_local(async move {
                                                match download_translation(&translation_clone).await {
                                                    Ok(_) => {
                                                        let _ = set_selected_translation(&translation_short_name);
                                                        set_selected_translation_signal.set(translation_short_name.clone());
                                                        
                                                        if let Err(e) = switch_bible_translation(&translation_short_name).await {
                                                            leptos::logging::error!("Failed to switch translation: {}", e);
                                                        }
                                                        
                                                        set_downloading_translation.set(None);
                                                    }
                                                    Err(e) => {
                                                        set_download_error.set(Some(format!("Failed to download {}: {}", translation_clone.name, e)));
                                                        set_downloading_translation.set(None);
                                                    }
                                                }
                                            });
                                        }
                                    }
                                }
                            >
                                <div class="flex items-start justify-between mb-4">
                                    <div class="flex-1">
                                        <h3 class="text-xl font-semibold text-gray-900 mb-2">
                                            {translation.name.clone()}
                                        </h3>
                                        <p class="text-sm text-gray-600 mb-2">
                                            "Jaar: " {translation.release_year.to_string()}
                                        </p>
                                        <p class="text-sm text-gray-700 leading-relaxed">
                                            {translation.description.clone()}
                                        </p>
                                    </div>
                                    <div class="ml-4 flex flex-col items-center space-y-2">
                                        {
                                            let is_selected = selected_translation.get() == translation.short_name;
                                            let is_downloading = downloading_translation.get().as_ref() == Some(&translation.short_name);
                                            
                                            if is_selected {
                                                view! {
                                                    <div class="flex items-center px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm font-medium">
                                                        <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                                            <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
                                                        </svg>
                                                        "Actief"
                                                    </div>
                                                }
                                            } else if is_downloaded {
                                                view! {
                                                    <div class="flex items-center px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm font-medium">
                                                        <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                                            <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
                                                        </svg>
                                                        "Gedownload"
                                                    </div>
                                                }
                                            } else if is_downloading {
                                                view! {
                                                    <div class="flex items-center px-3 py-1 bg-yellow-100 text-yellow-800 rounded-full text-sm font-medium">
                                                        <svg class="animate-spin w-4 h-4 mr-1" fill="none" viewBox="0 0 24 24">
                                                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                                            <path class="opacity-75" fill="currentColor" d="m12 2a10 10 0 0 1 10 10h-2a8 8 0 0 0-8-8v-2z"></path>
                                                        </svg>
                                                        "Downloaden..."
                                                    </div>
                                                }
                                            } else {
                                                view! {
                                                    <div class="px-3 py-1 bg-gray-100 text-gray-600 rounded-full text-sm font-medium">
                                                        "Klik om te downloaden"
                                                    </div>
                                                }
                                            }
                                        }
                                    </div>
                                </div>
                                
                                <div class="flex items-center justify-between">
                                    <a
                                        href=translation.wikipedia.clone()
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="text-sm text-blue-600 hover:text-blue-800 hover:underline"
                                        on:click=|e| e.stop_propagation()
                                    >
                                        "Meer informatie →"
                                    </a>
                                    
                                    {
                                        let is_selected = selected_translation.get() == translation.short_name;
                                        if is_selected {
                                            view! { <span class="text-sm text-gray-500">"Deze vertaling is nu actief"</span> }
                                        } else if is_downloaded {
                                            view! { <span class="text-sm text-gray-500">"Klik om te wisselen"</span> }
                                        } else {
                                            view! { <span class="text-sm text-gray-500">"Wordt lokaal opgeslagen"</span> }
                                        }
                                    }
                                </div>
                            </div>
                        }
                    }).collect_view()
                }
            </div>
            
            <Show
                when=move || download_error.get().is_some()
                fallback=|| view! { <></> }
            >
                <div class="mt-6 p-4 bg-red-100 border border-red-400 text-red-700 rounded-lg">
                    <div class="flex">
                        <svg class="w-5 h-5 mr-2 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"></path>
                        </svg>
                        <div>
                            <h4 class="font-medium">"Download mislukt"</h4>
                            <p class="text-sm mt-1">{move || download_error.get().unwrap_or_default()}</p>
                        </div>
                    </div>
                </div>
            </Show>
            
            <Show
                when=move || is_switching.get()
                fallback=|| view! { <></> }
            >
                <div class="mt-6 p-4 bg-blue-100 border border-blue-400 text-blue-700 rounded-lg">
                    <div class="flex items-center">
                        <svg class="animate-spin w-5 h-5 mr-2" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="m12 2a10 10 0 0 1 10 10h-2a8 8 0 0 0-8-8v-2z"></path>
                        </svg>
                        <span>"Wisselen van vertaling..."</span>
                    </div>
                </div>
            </Show>
            
            <div class="mt-8 text-center">
                <p class="text-sm text-gray-600">
                    "Vertalingen worden lokaal in je browser opgeslagen voor snelle toegang."
                </p>
                <p class="text-sm text-gray-500 mt-2">
                    "Je kunt later nog meer vertalingen toevoegen via " 
                    <a href="/translations" class="text-blue-600 hover:underline">"instellingen"</a>
                    "."
                </p>
            </div>
        </div>
    }
}