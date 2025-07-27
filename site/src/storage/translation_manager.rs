use leptos::prelude::*;
use crate::storage::{
    BibleTranslation, get_selected_translation, set_selected_translation, 
    is_translation_downloaded, download_translation, switch_bible_translation, uninstall_translation,
    get_translations
};
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn TranslationManager() -> impl IntoView {
    let (selected_translation, set_selected_translation_signal) = signal(get_selected_translation().unwrap_or_else(|| "sv".to_string()));
    let (downloading_states, set_downloading_states) = signal::<Vec<(String, bool)>>(vec![]); // (translation_short_name, is_downloading)
    let (uninstalling_states, set_uninstalling_states) = signal::<Vec<(String, bool)>>(vec![]); // (translation_short_name, is_uninstalling)
    let (download_error, set_download_error) = signal::<Option<String>>(None);
    let (uninstall_error, set_uninstall_error) = signal::<Option<String>>(None);
    
    let translations = get_translations();
    
    let handle_translation_change = {
        let set_selected_translation_signal = set_selected_translation_signal.clone();
        move |translation_short_name: String| {
            let _ = set_selected_translation(&translation_short_name);
            set_selected_translation_signal.set(translation_short_name.clone());
            
            // Switch the Bible data to the selected translation
            spawn_local(async move {
                if let Err(e) = switch_bible_translation(&translation_short_name).await {
                    leptos::logging::error!("Failed to switch translation: {}", e);
                }
            });
        }
    };

    let handle_download = {
        let set_downloading_states = set_downloading_states.clone();
        let set_download_error = set_download_error.clone();
        move |translation: BibleTranslation| {
            let translation_short_name = translation.short_name.clone();
            
            // Set downloading state
            set_downloading_states.update(|states| {
                if let Some(pos) = states.iter().position(|(name, _)| name == &translation_short_name) {
                    states[pos].1 = true;
                } else {
                    states.push((translation_short_name.clone(), true));
                }
            });
            
            set_download_error.set(None);
            
            spawn_local(async move {
                match download_translation(&translation).await {
                    Ok(_) => {
                        // Download successful, remove from downloading states
                        set_downloading_states.update(|states| {
                            states.retain(|(name, _)| name != &translation_short_name);
                        });
                    }
                    Err(e) => {
                        // Download failed
                        set_downloading_states.update(|states| {
                            if let Some(pos) = states.iter().position(|(name, _)| name == &translation_short_name) {
                                states[pos].1 = false;
                            }
                        });
                        set_download_error.set(Some(format!("Failed to download {}: {}", translation.name, e)));
                    }
                }
            });
        }
    };

    let handle_uninstall = {
        let set_uninstalling_states = set_uninstalling_states.clone();
        let set_uninstall_error = set_uninstall_error.clone();
        let set_selected_translation_signal = set_selected_translation_signal.clone();
        move |translation_short_name: String| {
            // Set uninstalling state
            set_uninstalling_states.update(|states| {
                if let Some(pos) = states.iter().position(|(name, _)| name == &translation_short_name) {
                    states[pos].1 = true;
                } else {
                    states.push((translation_short_name.clone(), true));
                }
            });
            
            set_uninstall_error.set(None);
            
            spawn_local(async move {
                match uninstall_translation(&translation_short_name).await {
                    Ok(_) => {
                        // Uninstall successful, remove from uninstalling states
                        set_uninstalling_states.update(|states| {
                            states.retain(|(name, _)| name != &translation_short_name);
                        });
                        
                        // Update selected translation if it was uninstalled
                        let current_selected = get_selected_translation().unwrap_or_else(|| "sv".to_string());
                        set_selected_translation_signal.set(current_selected);
                    }
                    Err(e) => {
                        // Uninstall failed
                        set_uninstalling_states.update(|states| {
                            if let Some(pos) = states.iter().position(|(name, _)| name == &translation_short_name) {
                                states[pos].1 = false;
                            }
                        });
                        set_uninstall_error.set(Some(format!("Failed to uninstall {}: {}", translation_short_name, e)));
                    }
                }
            });
        }
    };

    view! {
        <div class="bg-white rounded-lg shadow-md p-6 mb-6">
            <h2 class="text-xl font-semibold text-gray-900 mb-4">"Bible Translations"</h2>
            
            <Show
                when=move || download_error.get().is_some()
                fallback=|| view! { <></> }
            >
                <div class="mb-4 p-3 bg-red-100 border border-red-400 text-red-700 rounded">
                    {move || download_error.get().unwrap_or_default()}
                </div>
            </Show>
            
            <Show
                when=move || uninstall_error.get().is_some()
                fallback=|| view! { <></> }
            >
                <div class="mb-4 p-3 bg-red-100 border border-red-400 text-red-700 rounded">
                    {move || uninstall_error.get().unwrap_or_default()}
                </div>
            </Show>
            
            <div class="space-y-4">
{
                    translations.into_iter().map(|translation| {
                        let is_downloaded = is_translation_downloaded(&translation.short_name);
                        
                        let handle_translation_change_clone = handle_translation_change.clone();
                        let handle_download_clone = handle_download.clone();
                        let handle_uninstall_clone = handle_uninstall.clone();
                        let translation_for_change = translation.clone();
                        let translation_for_download = translation.clone();
                        let translation_short_name_for_class = translation.short_name.clone();
                        let translation_short_name_for_radio = translation.short_name.clone();
                        let translation_short_name_for_download = translation.short_name.clone();
                        let translation_short_name_for_uninstall_condition = translation.short_name.clone();
                        let translation_short_name_for_uninstall_button = translation.short_name.clone();
                        
                        view! {
                            <div class=move || {
                                let is_selected = selected_translation.get() == translation_short_name_for_class;
                                if is_selected { 
                                    "border rounded-lg p-4 border-blue-500 bg-blue-50" 
                                } else { 
                                    "border rounded-lg p-4 border-gray-200" 
                                }
                            }>
                                <div class="flex items-start justify-between">
                                    <div class="flex-1">
                                        <div class="flex items-center mb-2">
                                            <input
                                                type="radio"
                                                name="translation"
                                                id=format!("translation-{}", translation.short_name)
                                                class="mr-3 text-blue-600"
                                                checked=move || selected_translation.get() == translation_short_name_for_radio
                                                disabled=!is_downloaded
                                                on:change=move |_| {
                                                    if is_downloaded {
                                                        handle_translation_change_clone(translation_for_change.short_name.clone());
                                                    }
                                                }
                                            />
                                            <label
                                                for=format!("translation-{}", translation.short_name)
                                                class=if is_downloaded { "font-medium text-gray-900" } else { "font-medium text-gray-500" }
                                            >
                                                {translation.name.clone()}
                                            </label>
                                            <span class="ml-2 text-sm text-gray-500">
                                                "(" {translation.release_year.to_string()} ")"
                                            </span>
                                        </div>
                                        <p class="text-sm text-gray-600 mb-2 ml-6">
                                            {translation.description.clone()}
                                        </p>
                                        <div class="ml-6 flex items-center space-x-4">
                                            <a
                                                href=translation.wikipedia.clone()
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="text-sm text-blue-600 hover:text-blue-800"
                                            >
                                                "Learn more"
                                            </a>
                                            <Show
                                                when=move || is_downloaded
                                                fallback=|| view! { <></> }
                                            >
                                                <span class="text-sm text-green-600 font-medium">"✓ Downloaded"</span>
                                            </Show>
                                        </div>
                                    </div>
                                    <div class="ml-4">
                                        <Show
                                            when=move || !is_downloaded
                                            fallback=|| view! { <></> }
                                        >
                                            {
                                                let translation_clone = translation_for_download.clone();
                                                let handle_download_clone = handle_download_clone.clone();
                                                let translation_short_name_for_download_state = translation_short_name_for_download.clone();
                                                
                                                view! {
                                                    <Show
                                                        when=move || {
                                                            downloading_states.get().iter()
                                                                .find(|(name, _)| name == &translation_short_name_for_download_state)
                                                                .map(|(_, downloading)| *downloading)
                                                                .unwrap_or(false)
                                                        }
                                                        fallback=move || {
                                                            let translation_clone = translation_clone.clone();
                                                            let handle_download_clone = handle_download_clone.clone();
                                                            view! {
                                                                <button 
                                                                    class="px-4 py-2 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
                                                                    on:click=move |_| {
                                                                        handle_download_clone(translation_clone.clone());
                                                                    }
                                                                >
                                                                    "Download"
                                                                </button>
                                                            }
                                                        }
                                                    >
                                                        <button 
                                                            class="px-4 py-2 text-sm bg-yellow-100 text-yellow-800 rounded border border-yellow-300 cursor-not-allowed"
                                                            disabled=true
                                                        >
                                                            <div class="flex items-center">
                                                                <svg class="animate-spin w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24">
                                                                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                                                    <path class="opacity-75" fill="currentColor" d="m12 2a10 10 0 0 1 10 10h-2a8 8 0 0 0-8-8v-2z"></path>
                                                                </svg>
                                                                "Downloading..."
                                                            </div>
                                                        </button>
                                                    </Show>
                                                }
                                            }
                                        </Show>
                                        
                                        // Uninstall button for downloaded translations (but not if it's the only one or currently selected)
                                        <Show
                                            when=move || {
                                                is_downloaded && 
                                                translation_short_name_for_uninstall_condition != "sv" && // Don't allow uninstalling Staten vertaling
                                                selected_translation.get() != translation_short_name_for_uninstall_condition // Don't allow uninstalling currently selected
                                            }
                                            fallback=|| view! { <></> }
                                        >
                                            {
                                                let handle_uninstall_clone = handle_uninstall_clone.clone();
                                                let translation_short_name_for_uninstall_state = translation_short_name_for_uninstall_button.clone();
                                                let translation_short_name_for_uninstall_click = translation_short_name_for_uninstall_button.clone();
                                                
                                                view! {
                                                    <Show
                                                        when=move || {
                                                            uninstalling_states.get().iter()
                                                                .find(|(name, _)| name == &translation_short_name_for_uninstall_state)
                                                                .map(|(_, uninstalling)| *uninstalling)
                                                                .unwrap_or(false)
                                                        }
                                                        fallback=move || {
                                                            let handle_uninstall_clone = handle_uninstall_clone.clone();
                                                            let translation_short_name_clone = translation_short_name_for_uninstall_click.clone();
                                                            view! {
                                                                <button 
                                                                    class="px-3 py-1 text-xs bg-red-100 text-red-700 rounded border border-red-300 hover:bg-red-200 transition-colors ml-2"
                                                                    on:click=move |_| {
                                                                        handle_uninstall_clone(translation_short_name_clone.clone());
                                                                    }
                                                                    title="Remove this translation from your device"
                                                                >
                                                                    "Uninstall"
                                                                </button>
                                                            }
                                                        }
                                                    >
                                                        <button 
                                                            class="px-3 py-1 text-xs bg-red-200 text-red-800 rounded border border-red-300 cursor-not-allowed ml-2"
                                                            disabled=true
                                                        >
                                                            <div class="flex items-center">
                                                                <svg class="animate-spin w-3 h-3 mr-1" fill="none" viewBox="0 0 24 24">
                                                                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                                                    <path class="opacity-75" fill="currentColor" d="m12 2a10 10 0 0 1 10 10h-2a8 8 0 0 0-8-8v-2z"></path>
                                                                </svg>
                                                                "Uninstalling..."
                                                            </div>
                                                        </button>
                                                    </Show>
                                                }
                                            }
                                        </Show>
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect_view()
                }
            </div>
            
            <div class="mt-4 text-sm text-gray-600">
                <p>"You need to download a translation before you can select and use it."</p>
                <p>"Downloaded translations are stored locally in your browser for offline access."</p>
            </div>
        </div>
    }
}
