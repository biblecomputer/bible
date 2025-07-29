use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use urlencoding::encode;
use crate::core::get_current_bible;
use crate::storage::{
    get_selected_translation, set_selected_translation, 
    is_translation_downloaded, download_translation, switch_bible_translation, uninstall_translation,
    get_translations
};
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn HomeTranslationPicker() -> impl IntoView {
    let (selected_translation, set_selected_translation_signal) = signal(get_selected_translation().unwrap_or_else(|| "sv".to_string()));
    let (downloading_translation, set_downloading_translation) = signal::<Option<String>>(None);
    let (download_error, set_download_error) = signal::<Option<String>>(None);
    let (is_switching, set_is_switching) = signal(false);
    let (uninstalling_translation, set_uninstalling_translation) = signal::<Option<String>>(None);
    let (uninstall_error, set_uninstall_error) = signal::<Option<String>>(None);
    
    let navigate = use_navigate();
    let translations = get_translations();
    
    let navigate_to_first_chapter = move || {
        if let Some(bible) = get_current_bible() {
            if let Some(first_book) = bible.books.first() {
                if let Some(first_chapter) = first_book.chapters.first() {
                    let encoded_book = encode(&first_book.name);
                    navigate(&format!("/{}/{}", encoded_book, first_chapter.chapter), NavigateOptions { scroll: false, ..Default::default() });
                }
            }
        }
    };
    
    view! {
        <div class="max-w-2xl mx-auto">
            <div class="text-center mb-8">
                <h1 class="text-4xl font-bold text-gray-900 mb-4">"Bijbel"</h1>
                <p class="text-lg text-gray-600 mb-8">"Kies een vertaling om te beginnen met lezen"</p>
            </div>
            
            <div class="space-y-4">
                {
                    translations.into_iter().map(|translation| {
                        let is_downloaded = is_translation_downloaded(&translation.short_name);
                        let translation_short_name = translation.short_name.clone();
                        let translation_for_download = translation.clone();
                        
                        {
                            let navigate_clone_for_read = navigate_to_first_chapter.clone();
                            let navigate_clone_for_download = navigate_to_first_chapter.clone();
                            let translation_short_name_for_read = translation_short_name.clone();
                            let translation_short_name_for_uninstall = translation_short_name.clone();
                            let translation_short_name_for_check = translation_short_name.clone();
                            
                            view! {
                                <div class="border rounded-lg p-6 bg-white shadow-sm hover:shadow-md transition-shadow">
                                    <div class="flex items-center justify-between">
                                        <div class="flex-1">
                                            <h3 class="text-xl font-semibold text-gray-900 mb-1">
                                                {translation.name.clone()}
                                            </h3>
                                            <p class="text-sm text-gray-600">
                                                "Uitgegeven in " {translation.release_year.to_string()}
                                            </p>
                                        </div>
                                        <div class="ml-6 flex gap-2">
                                            {
                                                let is_downloading = downloading_translation.get().as_ref() == Some(&translation_short_name);
                                                let is_uninstalling = uninstalling_translation.get().as_ref() == Some(&translation_short_name);
                                                
                                                if is_downloaded {
                                                    view! {
                                                        <div class="flex gap-2">
                                                            <button
                                                                class="px-6 py-2 bg-blue-600 text-black rounded-md hover:bg-blue-700 transition-colors font-medium"
                                                                disabled=is_switching.get() || is_uninstalling
                                                                on:click=move |_| {
                                                                    if !is_switching.get() && !is_uninstalling {
                                                                        set_is_switching.set(true);
                                                                        let _ = set_selected_translation(&translation_short_name_for_read);
                                                                        set_selected_translation_signal.set(translation_short_name_for_read.clone());
                                                                        
                                                                        let translation_short_name_clone = translation_short_name_for_read.clone();
                                                                        let navigate_clone = navigate_clone_for_read.clone();
                                                                        spawn_local(async move {
                                                                            if let Err(e) = switch_bible_translation(&translation_short_name_clone).await {
                                                                                leptos::logging::error!("Failed to switch translation: {}", e);
                                                                            }
                                                                            set_is_switching.set(false);
                                                                            navigate_clone();
                                                                        });
                                                                    }
                                                                }
                                                            >
                                                                {
                                                                    if is_switching.get() && selected_translation.get() == translation_short_name_for_read {
                                                                        "Laden..."
                                                                    } else {
                                                                        "Lezen"
                                                                    }
                                                                }
                                                            </button>
                                                            
                                                            {
                                                                // Don't show uninstall button for Staten vertaling (sv) - it's the default
                                                                if translation_short_name_for_check != "sv" {
                                                                    view! {
                                                                        <button
                                                                            class="px-3 py-2 bg-red-600 text-black  rounded-md hover:bg-red-700 transition-colors font-medium text-sm"
                                                                            disabled=is_switching.get() || is_uninstalling
                                                                            on:click=move |_| {
                                                                                if !is_switching.get() && !is_uninstalling {
                                                                                    set_uninstalling_translation.set(Some(translation_short_name_for_uninstall.clone()));
                                                                                    set_uninstall_error.set(None);
                                                                                    
                                                                                    let translation_short_name_clone = translation_short_name_for_uninstall.clone();
                                                                                    spawn_local(async move {
                                                                                        match uninstall_translation(&translation_short_name_clone).await {
                                                                                            Ok(_) => {
                                                                                                set_uninstalling_translation.set(None);
                                                                                                // Update selected translation signal if this was the selected one
                                                                                                let current_selected = get_selected_translation().unwrap_or_else(|| "sv".to_string());
                                                                                                set_selected_translation_signal.set(current_selected);
                                                                                            }
                                                                                            Err(e) => {
                                                                                                set_uninstall_error.set(Some(format!("Verwijderen mislukt: {}", e)));
                                                                                                set_uninstalling_translation.set(None);
                                                                                            }
                                                                                        }
                                                                                    });
                                                                                }
                                                                            }
                                                                        >
                                                                            {
                                                                                if is_uninstalling {
                                                                                    "Verwijderen..."
                                                                                } else {
                                                                                    "Verwijderen"
                                                                                }
                                                                            }
                                                                        </button>
                                                                    }.into_any()
                                                                } else {
                                                                    view! { <></> }.into_any()
                                                                }
                                                            }
                                                        </div>
                                                    }.into_any()
                                                } else if is_downloading {
                                                    view! {
                                                        <div class="flex gap-2">
                                                            <button
                                                                class="px-6 py-2 bg-gray-400 text-black rounded-md cursor-not-allowed font-medium"
                                                                disabled=true
                                                            >
                                                                <div class="flex items-center">
                                                                    <svg class="animate-spin w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24">
                                                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                                                        <path class="opacity-75" fill="currentColor" d="m12 2a10 10 0 0 1 10 10h-2a8 8 0 0 0-8-8v-2z"></path>
                                                                    </svg>
                                                                    "Downloaden..."
                                                                </div>
                                                            </button>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <div class="flex gap-2">
                                                            <button
                                                                class="px-6 py-2 bg-green-600 text-black rounded-md hover:bg-green-700 transition-colors font-medium"
                                                                on:click=move |_| {
                                                                    set_downloading_translation.set(Some(translation_short_name.clone()));
                                                                    set_download_error.set(None);
                                                                    
                                                                    let translation_clone = translation_for_download.clone();
                                                                    let translation_short_name_clone = translation_short_name.clone();
                                                                    let navigate_clone = navigate_clone_for_download.clone();
                                                                    spawn_local(async move {
                                                                        match download_translation(&translation_clone).await {
                                                                            Ok(_) => {
                                                                                let _ = set_selected_translation(&translation_short_name_clone);
                                                                                set_selected_translation_signal.set(translation_short_name_clone.clone());
                                                                                
                                                                                if let Err(e) = switch_bible_translation(&translation_short_name_clone).await {
                                                                                    leptos::logging::error!("Failed to switch translation: {}", e);
                                                                                }
                                                                                
                                                                                set_downloading_translation.set(None);
                                                                                navigate_clone();
                                                                            }
                                                                            Err(e) => {
                                                                                set_download_error.set(Some(format!("Download mislukt: {}", e)));
                                                                                set_downloading_translation.set(None);
                                                                            }
                                                                        }
                                                                    });
                                                                }
                                                            >
                                                                "Download"
                                                            </button>
                                                        </div>
                                                    }.into_any()
                                                }
                                            }
                                        </div>
                                    </div>
                                    
                                    <div class="mt-4">
                                        <a
                                            href=translation.wikipedia.clone()
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="text-sm text-blue-600 hover:text-blue-800 hover:underline"
                                        >
                                            "Meer informatie →"
                                        </a>
                                    </div>
                                </div>
                            }
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
                            <h4 class="font-medium">"Fout"</h4>
                            <p class="text-sm mt-1">{move || download_error.get().unwrap_or_default()}</p>
                        </div>
                    </div>
                </div>
            </Show>
            
            <Show
                when=move || uninstall_error.get().is_some()
                fallback=|| view! { <></> }
            >
                <div class="mt-6 p-4 bg-red-100 border border-red-400 text-red-700 rounded-lg">
                    <div class="flex">
                        <svg class="w-5 h-5 mr-2 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"></path>
                        </svg>
                        <div>
                            <h4 class="font-medium">"Fout"</h4>
                            <p class="text-sm mt-1">{move || uninstall_error.get().unwrap_or_default()}</p>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}