use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use crate::storage::{
    get_selected_translation, set_selected_translation, 
    is_translation_downloaded, download_translation_with_progress, switch_bible_translation, uninstall_translation,
    get_available_languages, get_translations_by_language, BibleTranslation, Language
};
use crate::components::{theme_switcher::ThemeSwitcher, CustomTranslationImport};
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, PartialEq)]
enum ViewState {
    LanguageSelection,
    TranslationSelection(Language),
}

#[component]
fn TranslationItem(
    translation: BibleTranslation,
    downloading_translation: ReadSignal<Option<String>>,
    set_downloading_translation: WriteSignal<Option<String>>,
    download_progress: ReadSignal<f32>,
    set_download_progress: WriteSignal<f32>,
    _download_status: ReadSignal<String>,
    set_download_status: WriteSignal<String>,
    _download_error: ReadSignal<Option<String>>,
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
        <div class="relative">
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
                        <div class="border rounded-lg p-6 shadow-sm" style="background-color: var(--theme-background); border-color: var(--theme-sidebar-border)">
                            <div class="flex items-center justify-between">
                                <div class="flex-1">
                                    <h3 class="text-xl font-semibold mb-1" style="color: var(--theme-text-primary)">
                                        {translation_name.clone()}
                                    </h3>
                                    <p class="text-sm" style="color: var(--theme-text-secondary)">
                                        "Uitgegeven in " {translation_release_year.to_string()}
                                    </p>
                                </div>
                                <div class="ml-6 flex items-center">
                                    <svg class="animate-spin w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24">
                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                        <path class="opacity-75" fill="currentColor" d="m12 2a10 10 0 0 1 10 10h-2a8 8 0 0 0-8-8v-2z"></path>
                                    </svg>
                                    <div>
                                        <div class="text-sm font-medium" style="color: var(--theme-text-primary)">"Downloading..."</div>
                                        <div class="w-24 rounded-full h-1 mt-1" style="background-color: var(--theme-sidebar-border)">
                                            <div 
                                                class="h-1 rounded-full transition-all duration-300 ease-out" style="background-color: var(--theme-buttons-primary-background)"
                                                style:width={format!("{}%", download_progress.get() * 100.0)}
                                            ></div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else if is_downloaded {
                    view! {
                        <button 
                            class="w-full border rounded-lg p-6 shadow-sm hover:shadow-md transition-shadow text-left cursor-pointer"
                            style="background-color: var(--theme-background); border-color: var(--theme-sidebar-border)"
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
                            <div class="flex items-center justify-between">
                                <div class="flex-1">
                                    <h3 class="text-xl font-semibold mb-1" style="color: var(--theme-text-primary)">
                                        {translation_name.clone()}
                                        {
                                            let translation_short_name_clone2 = translation_short_name_ref.clone();
                                            move || {
                                            if is_switching.get() && selected_translation.get() == translation_short_name_clone2 {
                                                " (laden...)"
                                            } else {
                                                ""
                                            }
                                        }}
                                    </h3>
                                    <p class="text-sm" style="color: var(--theme-text-secondary)">
                                        "Uitgegeven in " {translation_release_year.to_string()}
                                    </p>
                                </div>
                                <div class="ml-6">
                                    {
                                        // Don't show delete button for Staten vertaling (sv) - it's the default
                                        if translation_short_name_ref != "sv" {
                                            view! {
                                                <button
                                                    class="px-3 py-2 rounded-md transition-colors font-medium text-sm translation-button-danger"
                                                    disabled=move || is_switching.get() || is_uninstalling
                                                    on:click={
                                                        let translation_short_name_clone4 = translation_short_name_ref.clone();
                                                        move |ev| {
                                                            ev.stop_propagation(); // Prevent triggering the parent button click
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
                                                            view! {
                                                                <div class="flex items-center gap-1">
                                                                    <svg class="animate-spin w-4 h-4" fill="none" viewBox="0 0 24 24">
                                                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                                                        <path class="opacity-75" fill="currentColor" d="m12 2a10 10 0 0 1 10 10h-2a8 8 0 0 0-8-8v-2z"></path>
                                                                    </svg>
                                                                    <span>"Verwijderen..."</span>
                                                                </div>
                                                            }
                                                        } else {
                                                            view! {
                                                                <div class="flex items-center gap-1">
                                                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                                                                    </svg>
                                                                    <span>"Verwijderen"</span>
                                                                </div>
                                                            }
                                                        }
                                                    }}
                                                </button>
                                            }.into_any()
                                        } else {
                                            view! { <></> }.into_any()
                                        }
                                    }
                                </div>
                            </div>
                        </button>
                    }.into_any()
                } else {
                    view! {
                        <button 
                            class="w-full border rounded-lg p-6 shadow-sm hover:shadow-md transition-shadow text-left cursor-pointer"
                            style="background-color: var(--theme-background); border-color: var(--theme-sidebar-border)"
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
                            <div class="flex items-center justify-between">
                                <div class="flex-1">
                                    <h3 class="text-xl font-semibold mb-1" style="color: var(--theme-text-primary)">
                                        {translation_name.clone()}
                                    </h3>
                                    <p class="text-sm" style="color: var(--theme-text-secondary)">
                                        "Uitgegeven in " {translation_release_year.to_string()}
                                    </p>
                                </div>
                                <div class="ml-6">
                                    <div class="px-3 py-1 rounded text-sm font-medium translation-button-success">
                                        "Download"
                                    </div>
                                </div>
                            </div>
                        </button>
                    }.into_any()
                }
            }}
            
            <div class="mt-4">
                <a
                    href=translation_wikipedia.clone()
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-sm hover:underline translation-link"
                >
                    "Meer informatie →"
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn HomeTranslationPicker(
    current_theme: ReadSignal<crate::themes::Theme>,
    set_current_theme: WriteSignal<crate::themes::Theme>,
) -> impl IntoView {
    let (selected_translation, set_selected_translation_signal) = signal(get_selected_translation().unwrap_or_else(|| "sv".to_string()));
    let (downloading_translation, set_downloading_translation) = signal::<Option<String>>(None);
    let (view_state, set_view_state) = signal(ViewState::LanguageSelection);
    
    // Debug: Watch downloading translation changes
    Effect::new(move |_| {
        let _current = downloading_translation.get();
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("Downloading translation changed to: {:?}", _current).into());
    });
    let (download_progress, set_download_progress) = signal::<f32>(0.0);
    let (download_status, set_download_status) = signal::<String>(String::new());
    let (download_error, set_download_error) = signal::<Option<String>>(None);
    let (is_switching, set_is_switching) = signal(false);
    let (uninstalling_translation, set_uninstalling_translation) = signal::<Option<String>>(None);
    
    // Debug: Watch uninstalling translation changes
    Effect::new(move |_| {
        let _current = uninstalling_translation.get();
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("Uninstalling translation changed to: {:?}", _current).into());
    });
    
    let (uninstall_error, _set_uninstall_error) = signal::<Option<String>>(None);
    let (ui_refresh_trigger, set_ui_refresh_trigger) = signal::<u32>(0);
    
    // Debug: Watch UI refresh trigger changes
    Effect::new(move |_| {
        let _current = ui_refresh_trigger.get();
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("UI refresh trigger changed to: {}", _current).into());
    });
    
    let navigate = use_navigate();
    let languages = get_available_languages();
    
    
    // Get the current URL parameters once at component initialization
    let location = leptos_router::hooks::use_location();
    let search_params = location.search.get();
    
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&format!("Search params: {}", search_params).into());
    
    let return_url = {
        use urlencoding::decode;
        // Parse return_url parameter
        if let Some(return_url_start) = search_params.find("return_url=") {
            let return_url_start = return_url_start + "return_url=".len();
            let return_url_end = search_params[return_url_start..]
                .find('&')
                .map(|pos| return_url_start + pos)
                .unwrap_or(search_params.len());
            
            let encoded_return_url = &search_params[return_url_start..return_url_end];
            
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&format!("Encoded return URL: {}", encoded_return_url).into());
            
            if let Ok(decoded_url) = decode(encoded_return_url) {
                let decoded_string = decoded_url.to_string();
                
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&format!("Decoded return URL: {}", decoded_string).into());
                
                Some(decoded_string)
            } else {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&"Failed to decode return URL".into());
                None
            }
        } else {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&"No return_url parameter found".into());
            None
        }
    };

    let navigate_to_first_chapter = move || {
        // Use the pre-parsed return URL if available
        if let Some(ref return_url) = return_url {
            navigate(return_url, NavigateOptions { scroll: false, ..Default::default() });
            return;
        }
        
        // Fallback: navigate to Genesis 1 if no return URL
        navigate("/Genesis/1", NavigateOptions { scroll: false, ..Default::default() });
    };
    
    view! {
        <div class="max-w-2xl mx-auto py-8">
            <div class="flex justify-between mb-4">
                {move || match view_state.get() {
                    ViewState::LanguageSelection => view! { <div></div> }.into_any(),
                    ViewState::TranslationSelection(_) => view! {
                        <button
                            class="p-3 rounded-full border transition-colors translation-button-secondary"
                            style="border-color: var(--theme-sidebar-border); background-color: var(--theme-background)"
                            on:click=move |_| {
                                set_view_state.set(ViewState::LanguageSelection);
                            }
                        >
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--theme-text-primary)">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                            </svg>
                        </button>
                    }.into_any()
                }}
                <ThemeSwitcher current_theme=current_theme set_current_theme=set_current_theme />
            </div>
            <div class="text-center mb-8">
                <h1 class="text-4xl font-bold mb-4" style="color: var(--theme-text-primary)">"Bijbel"</h1>
                <p class="text-lg mb-8" style="color: var(--theme-text-secondary)">
                    {move || match view_state.get() {
                        ViewState::LanguageSelection => "Kies een taal om te beginnen",
                        ViewState::TranslationSelection(_) => "Kies een vertaling om te beginnen met lezen",
                    }}
                </p>
            </div>
            
            <div class="space-y-4">
                {move || match view_state.get() {
                    ViewState::LanguageSelection => {
                        languages.clone().into_iter().map(|language| {
                            let language_name = language.display_name().to_string();
                            let language_clone = language.clone();
                            view! {
                                <div class="border rounded-lg p-6 shadow-sm hover:shadow-md transition-shadow cursor-pointer" style="background-color: var(--theme-background); border-color: var(--theme-sidebar-border)"
                                    on:click=move |_| {
                                        set_view_state.set(ViewState::TranslationSelection(language_clone.clone()));
                                    }
                                >
                                    <div class="flex items-center justify-between">
                                        <div class="flex-1">
                                            <h3 class="text-xl font-semibold mb-1" style="color: var(--theme-text-primary)">
                                                {language_name.clone()}
                                            </h3>
                                            <p class="text-sm" style="color: var(--theme-text-secondary)">
                                                {match language {
                                                    Language::Dutch => "Nederlandse vertalingen",
                                                    Language::English => "English translations",
                                                }}
                                            </p>
                                        </div>
                                        <div class="ml-6">
                                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--theme-text-muted)">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m9 5 7 7-7 7"></path>
                                            </svg>
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect_view().into_any()
                    }
                    ViewState::TranslationSelection(selected_language) => {
                        let selected_language_name = selected_language.display_name().to_string();
                        view! {
                            <div class="mb-6">
                                <h2 class="text-2xl font-semibold" style="color: var(--theme-text-primary)">
                                    {selected_language_name} " vertalingen"
                                </h2>
                            </div>
                            <div class="space-y-4">
                                {
                                    // Watch the refresh trigger to update the translation list when custom translations are added/removed
                                    let _ = ui_refresh_trigger.get();
                                    let translations = get_translations_by_language(&selected_language);
                                    translations.into_iter().map(|translation| {
                                    view! {
                                        <TranslationItem
                                            translation=translation
                                            downloading_translation=downloading_translation
                                            set_downloading_translation=set_downloading_translation
                                            download_progress=download_progress
                                            set_download_progress=set_download_progress
                                            _download_status=download_status
                                            set_download_status=set_download_status
                                            _download_error=download_error
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
                                }).collect_view()
                                }
                                
                                <CustomTranslationImport
                                    selected_language=RwSignal::new(selected_language.clone()).read_only()
                                    on_success=move || {
                                        set_ui_refresh_trigger.update(|n| *n += 1);
                                        let current_selected = get_selected_translation().unwrap_or_else(|| "sv".to_string());
                                        set_selected_translation_signal.set(current_selected);
                                    }
                                />
                            </div>
                        }.into_any()
                    }
                }}
            </div>
            
            <Show
                when=move || download_error.get().is_some()
                fallback=|| view! { <></> }
            >
                <div class="mt-6 p-4 border rounded-lg" style="background-color: var(--theme-buttons-danger-background); border-color: var(--theme-buttons-danger-background); color: var(--theme-buttons-danger-text)">
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
                <div class="mt-6 p-4 border rounded-lg" style="background-color: var(--theme-buttons-danger-background); border-color: var(--theme-buttons-danger-background); color: var(--theme-buttons-danger-text)">
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