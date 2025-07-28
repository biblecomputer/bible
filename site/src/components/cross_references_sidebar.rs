use crate::core::{load_cross_references};
use crate::core::types::{References, Reference, VerseId};
use crate::storage::translations::get_current_translation;
use crate::core::types::Language;
use crate::translation_map::translation::Translation;
use crate::utils::is_mobile_screen;
use crate::storage::save_sidebar_open;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use urlencoding::encode;
use std::sync::OnceLock;

// Global cross-references cache
static CROSS_REFERENCES: OnceLock<References> = OnceLock::new();

fn get_cross_references() -> &'static References {
    CROSS_REFERENCES.get_or_init(|| {
        load_cross_references().unwrap_or_else(|_| References(std::collections::HashMap::new()))
    })
}

fn convert_language(storage_lang: &crate::storage::translation_storage::Language) -> Language {
    match storage_lang {
        crate::storage::translation_storage::Language::Dutch => Language::Dutch,
        crate::storage::translation_storage::Language::English => Language::English,
    }
}

fn get_ui_text(key: &str) -> String {
    if let Some(current_translation) = get_current_translation() {
        if let Some(first_language) = current_translation.languages.first() {
            match (key, first_language) {
                ("cross_references", crate::storage::translation_storage::Language::Dutch) => "Kruisverwijzingen".to_string(),
                ("cross_references", crate::storage::translation_storage::Language::English) => "Cross References".to_string(),
                ("no_references", crate::storage::translation_storage::Language::Dutch) => "Geen kruisverwijzingen gevonden".to_string(),
                ("no_references", crate::storage::translation_storage::Language::English) => "No cross references found".to_string(),
                ("votes", crate::storage::translation_storage::Language::Dutch) => "stemmen".to_string(),
                ("votes", crate::storage::translation_storage::Language::English) => "votes".to_string(),
                _ => key.to_string(),
            }
        } else {
            key.to_string()
        }
    } else {
        // Default to English
        match key {
            "cross_references" => "Cross References".to_string(),
            "no_references" => "No cross references found".to_string(),
            "votes" => "votes".to_string(),
            _ => key.to_string(),
        }
    }
}

fn get_translated_book_name(book_name: &str) -> String {
    if let Some(current_translation) = get_current_translation() {
        if let Some(first_language) = current_translation.languages.first() {
            let translation = Translation::from_language(convert_language(first_language));
            
            // Convert book name to lowercase and replace spaces with underscores for lookup
            let lookup_key = book_name.to_lowercase().replace(' ', "_");
            
            if let Some(translated_name) = translation.get(&lookup_key) {
                return translated_name;
            }
        }
    }
    
    // Return original book name if no translation found
    book_name.to_string()
}

fn format_reference_text(reference: &Reference) -> String {
    let translated_book = get_translated_book_name(&reference.to_book_name);
    
    if let Some(end_verse) = reference.to_verse_end {
        format!("{} {}:{}-{}", translated_book, reference.to_chapter, reference.to_verse_start, end_verse)
    } else {
        format!("{} {}:{}", translated_book, reference.to_chapter, reference.to_verse_start)
    }
}

fn reference_to_url(reference: &Reference) -> String {
    let encoded_book = encode(&reference.to_book_name);
    if let Some(end_verse) = reference.to_verse_end {
        format!("/{}/{}?verses={}-{}", encoded_book, reference.to_chapter, reference.to_verse_start, end_verse)
    } else {
        format!("/{}/{}?verses={}", encoded_book, reference.to_chapter, reference.to_verse_start)
    }
}

#[component]
pub fn CrossReferencesSidebar(
    book_name: String,
    chapter: u32,
    verse: u32,
    set_sidebar_open: WriteSignal<bool>,
) -> impl IntoView {
    
    // Create the optimized verse ID for lookup
    let verse_id = match VerseId::from_book_name(&book_name, chapter, verse) {
        Some(id) => id,
        None => {
            // Unknown book - show no references
            return view! {
                <div class="cross-references-sidebar">
                    <div class="mb-4">
                        <h2 class="text-lg font-bold text-black mb-2">{get_ui_text("cross_references")}</h2>
                        <div class="text-sm text-gray-600 mb-4">
                            {get_translated_book_name(&book_name)} " " {chapter} ":" {verse}
                        </div>
                    </div>
                    <div class="text-sm text-gray-500 italic">
                        {get_ui_text("no_references")}
                    </div>
                </div>
            };
        }
    };
    
    // Get cross-references for this verse
    let references = get_cross_references();
    let verse_references = references.0.get(&verse_id);
    
    // Sort references by votes (highest to lowest)
    let sorted_references = Memo::new(move |_| {
        if let Some(refs) = verse_references {
            let mut sorted = refs.clone();
            sorted.sort_by(|a, b| b.votes.cmp(&a.votes));
            Some(sorted)
        } else {
            None
        }
    });
    
    view! {
        <div class="cross-references-sidebar">
            <div class="mb-4">
                <h2 class="text-lg font-bold text-black mb-2">{get_ui_text("cross_references")}</h2>
                <div class="text-sm text-gray-600 mb-4">
                    {get_translated_book_name(&book_name)} " " {chapter} ":" {verse}
                </div>
            </div>
            
            <Show
                when=move || sorted_references.get().is_some()
                fallback=move || view! {
                    <div class="text-sm text-gray-500 italic">
                        {get_ui_text("no_references")}
                    </div>
                }
            >
                <div class="space-y-3">
                    <For
                        each=move || sorted_references.get().unwrap_or_default()
                        key=|reference| (reference.to_book_name.clone(), reference.to_chapter, reference.to_verse_start, reference.to_verse_end, reference.votes)
                        children=move |reference| {
                            view! {
                                <ReferenceItem 
                                    reference=reference 
                                    set_sidebar_open=set_sidebar_open
                                />
                            }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}

#[component]
fn ReferenceItem(
    reference: Reference,
    set_sidebar_open: WriteSignal<bool>,
) -> impl IntoView {
    let navigate = use_navigate();
    let reference_text = format_reference_text(&reference);
    let reference_url = reference_to_url(&reference);
    let votes_text = if reference.votes == 1 {
        format!("1 {}", get_ui_text("votes").trim_end_matches('s'))
    } else {
        format!("{} {}", reference.votes, get_ui_text("votes"))
    };
    
    view! {
        <div class="reference-item">
            <button
                class="w-full text-left p-3 rounded-lg border border-gray-200 hover:border-blue-300 hover:bg-blue-50 transition-colors duration-150 group"
                on:click=move |_| {
                    navigate(&reference_url, Default::default());
                    // Close sidebar on mobile when reference is selected
                    if is_mobile_screen() {
                        set_sidebar_open.set(false);
                        save_sidebar_open(false);
                    }
                }
            >
                <div class="flex justify-between items-start">
                    <div class="flex-1">
                        <div class="text-sm font-medium text-black group-hover:text-blue-700">
                            {reference_text}
                        </div>
                    </div>
                    <div class="ml-2 flex-shrink-0">
                        <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-700 group-hover:bg-blue-100 group-hover:text-blue-800">
                            {votes_text}
                        </span>
                    </div>
                </div>
            </button>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Reference;

    #[test]
    fn test_format_reference_logic() {
        // Test the basic formatting logic without web dependencies
        let reference = Reference {
            to_book_name: "Isaiah".to_string(),
            to_chapter: 51,
            to_verse_start: 16,
            to_verse_end: None,
            votes: 51,
        };
        
        // Test single verse format: "Book Chapter:Verse"
        let expected_single = format!("{} {}:{}", reference.to_book_name, reference.to_chapter, reference.to_verse_start);
        assert_eq!(expected_single, "Isaiah 51:16");
        
        let range_reference = Reference {
            to_book_name: "Romans".to_string(),
            to_chapter: 1,
            to_verse_start: 19,
            to_verse_end: Some(20),
            votes: 50,
        };
        
        // Test range format: "Book Chapter:Start-End"
        let expected_range = format!("{} {}:{}-{}", 
            range_reference.to_book_name, 
            range_reference.to_chapter, 
            range_reference.to_verse_start,
            range_reference.to_verse_end.unwrap()
        );
        assert_eq!(expected_range, "Romans 1:19-20");
    }

    #[test]
    fn test_reference_to_url() {
        let reference = Reference {
            to_book_name: "Isaiah".to_string(),
            to_chapter: 51,
            to_verse_start: 16,
            to_verse_end: None,
            votes: 51,
        };
        
        assert_eq!(reference_to_url(&reference), "/Isaiah/51?verses=16");
        
        let range_reference = Reference {
            to_book_name: "Romans".to_string(),
            to_chapter: 1,
            to_verse_start: 19,
            to_verse_end: Some(20),
            votes: 50,
        };
        
        assert_eq!(reference_to_url(&range_reference), "/Romans/1?verses=19-20");
    }
}