use crate::core::types::Language;
use crate::core::Bible;
use crate::storage::{
    add_downloaded_translation, save_translation_to_cache, set_selected_translation,
    switch_bible_translation, BibleTranslation,
};
use gloo_storage::{LocalStorage, Storage};
use leptos::html::Input;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, FileReader, HtmlInputElement};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CustomTranslation {
    translation: BibleTranslation,
    bible_data: Bible,
}

const CUSTOM_TRANSLATIONS_KEY: &str = "custom_translations";

pub fn get_custom_translations() -> Vec<BibleTranslation> {
    LocalStorage::get::<Vec<BibleTranslation>>(CUSTOM_TRANSLATIONS_KEY).unwrap_or_default()
}

pub fn add_custom_translation(
    translation: &BibleTranslation,
) -> Result<(), gloo_storage::errors::StorageError> {
    let mut custom_translations = get_custom_translations();

    custom_translations.retain(|t| t.short_name != translation.short_name);
    custom_translations.push(translation.clone());

    LocalStorage::set(CUSTOM_TRANSLATIONS_KEY, &custom_translations)
}

pub fn _remove_custom_translation(
    short_name: &str,
) -> Result<(), gloo_storage::errors::StorageError> {
    let mut custom_translations = get_custom_translations();
    custom_translations.retain(|t| t.short_name != short_name);
    LocalStorage::set(CUSTOM_TRANSLATIONS_KEY, &custom_translations)
}

async fn save_custom_translation_to_cache(
    translation: &BibleTranslation,
    bible: &Bible,
) -> Result<(), Box<dyn std::error::Error>> {
    let cache_key = format!("translation_{}", translation.short_name);
    save_translation_to_cache(&cache_key, bible).await
}

#[component]
pub fn CustomTranslationImport(
    selected_language: ReadSignal<Language>,
    on_success: impl Fn() + Clone + Send + Sync + 'static,
) -> impl IntoView {
    let (show_import_modal, set_show_import_modal) = signal(false);
    let (import_error, set_import_error) = signal::<Option<String>>(None);
    let (is_importing, set_is_importing) = signal(false);

    let (translation_name, set_translation_name) = signal(String::new());
    let (release_year, set_release_year) = signal(String::new());
    let (_file_selected, set_file_selected) = signal(false);
    let (file_content, set_file_content) = signal::<Option<String>>(None);

    let file_input_ref = NodeRef::<Input>::new();

    let reset_form = move || {
        set_translation_name.set(String::new());
        set_release_year.set(String::new());
        set_file_selected.set(false);
        set_file_content.set(None);
        set_import_error.set(None);
        if let Some(input) = file_input_ref.get() {
            input.set_value("");
        }
    };

    let on_file_change = move |ev: Event| {
        let input = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            if let Some(files) = input.files() {
                if files.length() > 0 {
                    if let Some(file) = files.get(0) {
                        set_file_selected.set(true);
                        set_import_error.set(None);

                        let file_reader = FileReader::new().unwrap();
                        let file_reader_clone = file_reader.clone();

                        let onload = Closure::wrap(Box::new(move |_: Event| {
                            if let Some(result) = file_reader_clone.result().ok() {
                                if let Some(text) = result.as_string() {
                                    set_file_content.set(Some(text));
                                }
                            }
                        }) as Box<dyn FnMut(_)>);

                        file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                        onload.forget();

                        let _ = file_reader.read_as_text(&file);
                    }
                }
            }
        }
    };

    let handle_import = Callback::new(move |_| {
        if translation_name.get().trim().is_empty() {
            set_import_error.set(Some("Voer een naam in voor de vertaling".to_string()));
            return;
        }

        if release_year.get().trim().is_empty() {
            set_import_error.set(Some("Voer een uitgavejaar in".to_string()));
            return;
        }

        let year: u16 = match release_year.get().parse() {
            Ok(y) => y,
            Err(_) => {
                set_import_error.set(Some("Uitgavejaar moet een geldig getal zijn".to_string()));
                return;
            }
        };

        if let Some(text) = file_content.get() {
            set_is_importing.set(true);
            set_import_error.set(None);

            let name = translation_name.get();
            let lang = selected_language.get();
            let success_callback = on_success.clone();

            spawn_local(async move {
                match serde_json::from_str::<Bible>(&text) {
                    Ok(bible) => {
                        let short_name = format!(
                            "custom_{}",
                            js_sys::Math::random().to_string().replace("0.", "")[..8]
                                .to_lowercase()
                        );

                        let translation = BibleTranslation {
                            name: name,
                            short_name: short_name.clone(),
                            release_year: year,
                            iagon: String::new(),
                            languages: vec![lang],
                        };

                        match save_custom_translation_to_cache(&translation, &bible).await {
                            Ok(_) => {
                                if let Err(e) = add_custom_translation(&translation) {
                                    set_import_error.set(Some(format!("Fout bij opslaan: {}", e)));
                                    set_is_importing.set(false);
                                    return;
                                }

                                if let Err(e) = add_downloaded_translation(&short_name) {
                                    set_import_error
                                        .set(Some(format!("Fout bij registreren: {}", e)));
                                    set_is_importing.set(false);
                                    return;
                                }

                                let _ = set_selected_translation(&short_name);

                                if let Err(e) = switch_bible_translation(&short_name).await {
                                    leptos::logging::error!(
                                        "Failed to switch to imported translation: {}",
                                        e
                                    );
                                }

                                set_is_importing.set(false);
                                set_show_import_modal.set(false);
                                reset_form();
                                success_callback();
                            }
                            Err(e) => {
                                set_import_error
                                    .set(Some(format!("Fout bij opslaan naar cache: {}", e)));
                                set_is_importing.set(false);
                            }
                        }
                    }
                    Err(e) => {
                        set_import_error.set(Some(format!("Ongeldig JSON formaat: {}", e)));
                        set_is_importing.set(false);
                    }
                }
            });
        } else {
            set_import_error.set(Some("Selecteer een JSON bestand".to_string()));
        }
    });

    view! {
        <div>
            <button
                class="w-full border border-dashed rounded-lg p-6 shadow-sm hover:shadow-md transition-shadow"
                style="background-color: var(--theme-background); border-color: var(--theme-sidebar-border)"
                on:click=move |_| {
                    reset_form();
                    set_show_import_modal.set(true);
                }
            >
                <div class="flex items-center justify-between">
                    <div class="flex-1">
                        <h3 class="text-xl font-semibold mb-1" style="color: var(--theme-text-primary)">
                            "Importeer je eigen vertaling"
                        </h3>
                        <p class="text-sm" style="color: var(--theme-text-secondary)">
                            "Upload een JSON bestand met je Bijbelvertaling"
                        </p>
                    </div>
                    <div class="ml-6">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--theme-text-muted)">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
                        </svg>
                    </div>
                </div>
            </button>

            <Show
                when=move || show_import_modal.get()
                fallback=|| view! { <></> }
            >
                <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                    <div class="max-w-md w-full mx-4 rounded-lg p-6" style="background-color: var(--theme-background); border: 1px solid var(--theme-sidebar-border)">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-xl font-semibold" style="color: var(--theme-text-primary)">
                                "Vertaling importeren"
                            </h2>
                            <button
                                class="text-gray-400 hover:text-gray-600"
                                on:click=move |_| set_show_import_modal.set(false)
                            >
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                </svg>
                            </button>
                        </div>

                        <div class="space-y-4">
                            <div>
                                <label class="block text-sm font-medium mb-1" style="color: var(--theme-text-primary)">
                                    "Naam van de vertaling"
                                </label>
                                <input
                                    type="text"
                                    class="w-full px-3 py-2 border rounded-md"
                                    style="background-color: var(--theme-background); border-color: var(--theme-sidebar-border); color: var(--theme-text-primary)"
                                    placeholder="Bijv. Mijn Bijbelvertaling"
                                    prop:value=move || translation_name.get()
                                    on:input=move |ev| set_translation_name.set(event_target_value(&ev))
                                />
                            </div>

                            <div>
                                <label class="block text-sm font-medium mb-1" style="color: var(--theme-text-primary)">
                                    "Uitgavejaar"
                                </label>
                                <input
                                    type="number"
                                    class="w-full px-3 py-2 border rounded-md"
                                    style="background-color: var(--theme-background); border-color: var(--theme-sidebar-border); color: var(--theme-text-primary)"
                                    placeholder="2024"
                                    prop:value=move || release_year.get()
                                    on:input=move |ev| set_release_year.set(event_target_value(&ev))
                                />
                            </div>


                            <div>
                                <label class="block text-sm font-medium mb-1" style="color: var(--theme-text-primary)">
                                    "JSON bestand"
                                </label>
                                <input
                                    type="file"
                                    accept=".json"
                                    class="w-full px-3 py-2 border rounded-md"
                                    style="background-color: var(--theme-background); border-color: var(--theme-sidebar-border); color: var(--theme-text-primary)"
                                    node_ref=file_input_ref
                                    on:change=on_file_change
                                />
                                <p class="text-xs mt-1" style="color: var(--theme-text-muted)">
                                    "Upload een JSON bestand met de Bijbel structuur"
                                </p>
                            </div>

                            <Show
                                when=move || import_error.get().is_some()
                                fallback=|| view! { <></> }
                            >
                                <div class="p-3 rounded-md" style="background-color: var(--theme-buttons-danger-background); color: var(--theme-buttons-danger-text)">
                                    <p class="text-sm">{move || import_error.get().unwrap_or_default()}</p>
                                </div>
                            </Show>

                            <div class="flex gap-3">
                                <button
                                    class="flex-1 px-4 py-2 rounded-md border transition-colors"
                                    style="border-color: var(--theme-sidebar-border); color: var(--theme-text-primary)"
                                    on:click=move |_| set_show_import_modal.set(false)
                                    disabled=move || is_importing.get()
                                >
                                    "Annuleren"
                                </button>
                                <button
                                    class="flex-1 px-4 py-2 rounded-md transition-colors translation-button-primary"
                                    on:click=move |_| handle_import.run(())
                                    disabled=move || is_importing.get()
                                >
                                    {move || if is_importing.get() {
                                        "Importeren..."
                                    } else {
                                        "Importeren"
                                    }}
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
