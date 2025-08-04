use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use urlencoding::encode;
use crate::core::get_current_bible;
use crate::storage::{
    get_selected_translation, set_selected_translation, 
    is_translation_downloaded, download_translation_with_progress, switch_bible_translation, uninstall_translation,
    get_translations, BibleTranslation
};
use wasm_bindgen_futures::spawn_local;

#[component]
fn TranslationItem(
    translation: BibleTranslation,
    downloading_translation: ReadSignal<Option<String>>,
    set_downloading_translation: WriteSignal<Option<String>>,
    download_progress: ReadSignal<f32>,
    set_download_progress: WriteSignal<f32>,
    download_status: ReadSignal<String>,
    set_download_status: WriteSignal<String>,
    download_error: ReadSignal<Option<String>>,
    set_download_error: WriteSignal<Option<String>>,
    uninstalling_translation: ReadSignal<Option<String>>,
    set_uninstalling_translation: WriteSignal<Option<String>>,
    selected_translation: ReadSignal<String>,
    set_selected_translation_signal: WriteSignal<String>,
    is_switching: ReadSignal<bool>,
    set_is_switching: WriteSignal<bool>,
    ui_refresh_trigger: ReadSignal<u32>,
    set_ui_refresh_trigger: WriteSignal<u32>,
    navigate_to_first_chapter: impl Fn() + Clone + Send + 'static,
) -> impl IntoView {
    let translation_short_name = translation.short_name.clone();
    let translation_name = translation.name.clone();
    let translation_release_year = translation.release_year;
    let translation_wikipedia = translation.wikipedia.clone();
    let translation_clone_for_download = translation.clone();
    
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&format!("TranslationItem rendered for: {}", translation_short_name).into());
    
    view! {
        <div class="border rounded-lg p-6 bg-white shadow-sm hover:shadow-md transition-shadow">
            <div class="flex items-center justify-between">
                <div class="flex-1">
                    <h3 class="text-xl font-semibold text-gray-900 mb-1">
                        {translation_name.clone()}
                    </h3>
                    <p class="text-sm text-gray-600">
                        "Uitgegeven in " {translation_release_year.to_string()}
                    </p>
                </div>
                <div class="ml-6">
                    {
                        let translation_short_name_clone = translation_short_name.clone();
                        move || {
                        let translation_short_name_ref = translation_short_name_clone.clone();
                        let is_downloading = downloading_translation.get().as_ref() == Some(&translation_short_name_ref);
                        let is_uninstalling = uninstalling_translation.get().as_ref() == Some(&translation_short_name_ref);
                        // Watch the refresh trigger and check download status
                        let _ = ui_refresh_trigger.get();
                        let is_downloaded = is_translation_downloaded(&translation_short_name_ref);
                        
                        #[cfg(target_arch = "wasm32")]
                        web_sys::console::log_1(&format!("Translation {} - downloading: {}, downloaded: {}, uninstalling: {}", 
                            translation_short_name_ref, is_downloading, is_downloaded, is_uninstalling).into());
                        
                        
                        if is_downloading {
                            view! {
                                <div class="w-64">
                                    <div class="flex items-center">
                                        <svg class="animate-spin w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24">
                                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                            <path class="opacity-75" fill="currentColor" d="m12 2a10 10 0 0 1 10 10h-2a8 8 0 0 0-8-8v-2z"></path>
                                        </svg>
                                        <span class="text-sm font-medium text-gray-700">"Downloading..."</span>
                                    </div>
                                    <div class="mt-2">
                                        <div class="flex items-center justify-between mb-1">
                                            <span class="text-xs text-gray-600">{download_status.get()}</span>
                                            <span class="text-xs text-gray-600">{format!("{}%", (download_progress.get() * 100.0) as u32)}</span>
                                        </div>
                                        <div class="w-full bg-gray-200 rounded-full h-2">
                                            <div 
                                                class="bg-blue-600 h-2 rounded-full transition-all duration-300 ease-out"
                                                style:width={format!("{}%", download_progress.get() * 100.0)}
                                            ></div>
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        } else if is_downloaded {
                            view! {
                                <div class="flex gap-2">
                                    <button
                                        class="px-6 py-2 bg-blue-600 text-black rounded-md hover:bg-blue-700 transition-colors font-medium"
                                        disabled=move || is_switching.get() || is_uninstalling
                                        on:click={
                                            let translation_short_name_clone3 = translation_short_name_ref.clone();
                                            let navigate_clone = navigate_to_first_chapter.clone();
                                            move |_| {
                                                if !is_switching.get_untracked() && !is_uninstalling {
                                                    set_is_switching.set(true);
                                                    let _ = set_selected_translation(&translation_short_name_clone3);
                                                    set_selected_translation_signal.set(translation_short_name_clone3.clone());
                                                    
                                                    let translation_short_name_clone2 = translation_short_name_clone3.clone();
                                                    let navigate_clone2 = navigate_clone.clone();
                                                    spawn_local(async move {
                                                        if let Err(e) = switch_bible_translation(&translation_short_name_clone2).await {
                                                            leptos::logging::error!("Failed to switch translation: {}", e);
                                                        }
                                                        set_is_switching.set(false);
                                                        navigate_clone2();
                                                    });
                                                }
                                            }
                                        }
                                    >
                                        {
                                            let translation_short_name_clone2 = translation_short_name_ref.clone();
                                            move || {
                                            if is_switching.get() && selected_translation.get() == translation_short_name_clone2 {
                                                "Laden..."
                                            } else {
                                                "Lezen"
                                            }
                                        }}
                                    </button>
                                    
                                    {
                                        // Don't show uninstall button for Staten vertaling (sv) - it's the default
                                        if translation_short_name_ref != "sv" {
                                            view! {
                                                <button
                                                    class="px-3 py-2 bg-red-600 text-black  rounded-md hover:bg-red-700 transition-colors font-medium text-sm"
                                                    disabled=move || is_switching.get() || is_uninstalling
                                                    on:click={
                                                        let translation_short_name_clone4 = translation_short_name_ref.clone();
                                                        move |_| {
                                                            if !is_switching.get_untracked() && !is_uninstalling {
                                                                #[cfg(target_arch = "wasm32")]
                                                                web_sys::console::log_1(&format!("Delete clicked for: {}", translation_short_name_clone4).into());
                                                                set_uninstalling_translation.set(Some(translation_short_name_clone4.clone()));
                                                                
                                                                let translation_short_name_clone2 = translation_short_name_clone4.clone();
                                                                spawn_local(async move {
                                                                    match uninstall_translation(&translation_short_name_clone2).await {
                                                                        Ok(_) => {
                                                                            set_uninstalling_translation.set(None);
                                                                            // Update selected translation signal if this was the selected one
                                                                            let current_selected = get_selected_translation().unwrap_or_else(|| "sv".to_string());
                                                                            set_selected_translation_signal.set(current_selected);
                                                                            // Trigger UI refresh to update download status
                                                                            set_ui_refresh_trigger.update(|n| *n += 1);
                                                                        }
                                                                        Err(e) => {
                                                                            leptos::logging::error!("Uninstall failed: {}", e);
                                                                            set_uninstalling_translation.set(None);
                                                                        }
                                                                    }
                                                                });  
                                                            }
                                                        }
                                                    }
                                                >
                                                    {move || {
                                                        if is_uninstalling {
                                                            "Verwijderen..."
                                                        } else {
                                                            "Verwijderen"
                                                        }
                                                    }}
                                                </button>
                                            }.into_any()
                                        } else {
                                            view! { <></> }.into_any()
                                        }
                                    }
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="flex gap-2">
                                    <button
                                        class="px-6 py-2 bg-green-600 text-black rounded-md hover:bg-green-700 transition-colors font-medium"
                                        on:click={
                                            let translation_clone = translation_clone_for_download.clone();
                                            let translation_short_name_clone = translation_short_name_ref.clone();
                                            let navigate_clone = navigate_to_first_chapter.clone();
                                            move |_| {
                                                #[cfg(target_arch = "wasm32")]
                                                web_sys::console::log_1(&format!("Download clicked for: {}", translation_short_name_clone).into());
                                                set_downloading_translation.set(Some(translation_short_name_clone.clone()));
                                                set_download_error.set(None);
                                                set_download_progress.set(0.0);
                                                set_download_status.set("Preparing download...".to_string());
                                                
                                                let translation_clone2 = translation_clone.clone();
                                                let translation_short_name_clone2 = translation_short_name_clone.clone();
                                                let navigate_clone2 = navigate_clone.clone();
                                                
                                                // Create progress callback
                                                let progress_callback = {
                                                    move |progress: f32, status: String| {
                                                        set_download_progress.set(progress);
                                                        set_download_status.set(status);
                                                    }
                                                };
                                                
                                                spawn_local(async move {
                                                    match download_translation_with_progress(&translation_clone2, progress_callback).await {
                                                        Ok(_) => {
                                                            let _ = set_selected_translation(&translation_short_name_clone2);
                                                            set_selected_translation_signal.set(translation_short_name_clone2.clone());
                                                            
                                                            if let Err(e) = switch_bible_translation(&translation_short_name_clone2).await {
                                                                leptos::logging::error!("Failed to switch translation: {}", e);
                                                            }
                                                            
                                                            set_downloading_translation.set(None);
                                                            set_download_progress.set(0.0);
                                                            // Trigger UI refresh to update download status
                                                            set_ui_refresh_trigger.update(|n| *n += 1);
                                                            navigate_clone2();
                                                        }
                                                        Err(e) => {
                                                            set_download_error.set(Some(format!("Download mislukt: {}", e)));
                                                            set_downloading_translation.set(None);
                                                            set_download_progress.set(0.0);
                                                        }
                                                    }
                                                });
                                            }
                                        }
                                    >
                                        "Download"
                                    </button>
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
                
                // Test button to verify event handling works
                <div class="mt-2">
                    <button 
                        class="px-2 py-1 bg-yellow-500 text-black text-xs rounded"
                        on:click={
                            let translation_short_name_test = translation_short_name.clone();
                            move |_| {
                                #[cfg(target_arch = "wasm32")]
                                web_sys::console::log_1(&format!("TEST BUTTON CLICKED for {}", translation_short_name_test).into());
                            }
                        }
                    >
                        "TEST"
                    </button>
                </div>
            </div>
            
            <div class="mt-4">
                <a
                    href=translation_wikipedia.clone()
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

#[component]
pub fn HomeTranslationPicker() -> impl IntoView {
    let (selected_translation, set_selected_translation_signal) = signal(get_selected_translation().unwrap_or_else(|| "sv".to_string()));
    let (downloading_translation, set_downloading_translation) = signal::<Option<String>>(None);
    
    // Debug: Watch downloading translation changes
    Effect::new(move |_| {
        let current = downloading_translation.get();
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("Downloading translation changed to: {:?}", current).into());
    });
    let (download_progress, set_download_progress) = signal::<f32>(0.0);
    let (download_status, set_download_status) = signal::<String>(String::new());
    let (download_error, set_download_error) = signal::<Option<String>>(None);
    let (is_switching, set_is_switching) = signal(false);
    let (uninstalling_translation, set_uninstalling_translation) = signal::<Option<String>>(None);
    
    // Debug: Watch uninstalling translation changes
    Effect::new(move |_| {
        let current = uninstalling_translation.get();
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("Uninstalling translation changed to: {:?}", current).into());
    });
    
    let (uninstall_error, set_uninstall_error) = signal::<Option<String>>(None);
    let (ui_refresh_trigger, set_ui_refresh_trigger) = signal::<u32>(0);
    
    // Debug: Watch UI refresh trigger changes
    Effect::new(move |_| {
        let current = ui_refresh_trigger.get();
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("UI refresh trigger changed to: {}", current).into());
    });
    
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
                {translations.into_iter().map(|translation| {
                    view! {
                        <TranslationItem
                            translation=translation
                            downloading_translation=downloading_translation
                            set_downloading_translation=set_downloading_translation
                            download_progress=download_progress
                            set_download_progress=set_download_progress
                            download_status=download_status
                            set_download_status=set_download_status
                            download_error=download_error
                            set_download_error=set_download_error
                            uninstalling_translation=uninstalling_translation
                            set_uninstalling_translation=set_uninstalling_translation
                            selected_translation=selected_translation
                            set_selected_translation_signal=set_selected_translation_signal
                            is_switching=is_switching
                            set_is_switching=set_is_switching
                            ui_refresh_trigger=ui_refresh_trigger
                            set_ui_refresh_trigger=set_ui_refresh_trigger
                            navigate_to_first_chapter=navigate_to_first_chapter.clone()
                        />
                    }
                }).collect_view()}
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