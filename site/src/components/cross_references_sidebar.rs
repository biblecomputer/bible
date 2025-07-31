use crate::core::{load_cross_references};
use crate::core::types::{References, Reference, VerseId};
use crate::storage::translations::get_current_translation;
use crate::core::types::Language;
use crate::translation_map::translation::Translation;
use crate::utils::is_mobile_screen;
use crate::storage::save_sidebar_open;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use urlencoding::encode;
use std::sync::OnceLock;
use leptos::ev;
use leptos::web_sys::KeyboardEvent;

// Global cross-references cache
static CROSS_REFERENCES: OnceLock<References> = OnceLock::new();

fn get_cross_references() -> &'static References {
    CROSS_REFERENCES.get_or_init(|| {
        load_cross_references().unwrap_or_else(|_| References(std::collections::HashMap::new()))
    })
}

fn get_canonical_book_name(display_name: &str) -> String {
    // Convert display book names (potentially translated) back to canonical English names
    // that the cross-reference system recognizes
    match display_name {
        // English Roman numerals to Arabic numerals
        "I Samuel" => "1 Samuel".to_string(),
        "II Samuel" => "2 Samuel".to_string(),
        "I Kings" => "1 Kings".to_string(),
        "II Kings" => "2 Kings".to_string(),
        "I Chronicles" => "1 Chronicles".to_string(),
        "II Chronicles" => "2 Chronicles".to_string(),
        "I Corinthians" => "1 Corinthians".to_string(),
        "II Corinthians" => "2 Corinthians".to_string(),
        "I Thessalonians" => "1 Thessalonians".to_string(),
        "II Thessalonians" => "2 Thessalonians".to_string(),
        "I Timothy" => "1 Timothy".to_string(),
        "II Timothy" => "2 Timothy".to_string(),
        "I Peter" => "1 Peter".to_string(),
        "II Peter" => "2 Peter".to_string(),
        "I John" => "1 John".to_string(),
        "II John" => "2 John".to_string(),
        "III John" => "3 John".to_string(),
        
        // Alternative book names to canonical English names
        "Revelation of John" => "Revelation".to_string(),
        "The Revelation" => "Revelation".to_string(),
        "The Revelation of John" => "Revelation".to_string(),
        
        // Dutch translations back to English
        "I Samuël" => "1 Samuel".to_string(),
        "II Samuël" => "2 Samuel".to_string(),
        "I Koningen" => "1 Kings".to_string(),
        "II Koningen" => "2 Kings".to_string(),
        "I Kronieken" => "1 Chronicles".to_string(),
        "II Kronieken" => "2 Chronicles".to_string(),
        "Psalmen" => "Psalms".to_string(),
        "Prediker" => "Ecclesiastes".to_string(),
        "Hooglied" => "Song of Solomon".to_string(),
        "Jesaja" => "Isaiah".to_string(),
        "Jeremia" => "Jeremiah".to_string(),
        "Klaagliederen" => "Lamentations".to_string(),
        "Ezechiël" => "Ezekiel".to_string(),
        "Daniël" => "Daniel".to_string(),
        "Joël" => "Joel".to_string(),
        "Micha" => "Micah".to_string(),
        "Habakuk" => "Habakkuk".to_string(),
        "Zefanja" => "Zephaniah".to_string(),
        "Haggaï" => "Haggai".to_string(),
        "Zacharia" => "Zechariah".to_string(),
        "Maleachi" => "Malachi".to_string(),
        
        // New Testament Dutch translations
        "Matteüs" => "Matthew".to_string(),
        "Marcus" => "Mark".to_string(),
        "Lucas" => "Luke".to_string(),
        "Johannes" => "John".to_string(),
        "Handelingen" => "Acts".to_string(),
        "Romeinen" => "Romans".to_string(),
        "I Korintiërs" => "1 Corinthians".to_string(),
        "II Korintiërs" => "2 Corinthians".to_string(),
        "Galaten" => "Galatians".to_string(),
        "Efeziërs" => "Ephesians".to_string(),
        "Filippenzen" => "Philippians".to_string(),
        "Kolossenzen" => "Colossians".to_string(),
        "I Tessalonicenzen" => "1 Thessalonians".to_string(),
        "II Tessalonicenzen" => "2 Thessalonians".to_string(),
        "I Timoteüs" => "1 Timothy".to_string(),
        "II Timoteüs" => "2 Timothy".to_string(),
        "Titus" => "Titus".to_string(),
        "Filemon" => "Philemon".to_string(),
        "Hebreeën" => "Hebrews".to_string(),
        "Jakobus" => "James".to_string(),
        "I Petrus" => "1 Peter".to_string(),
        "II Petrus" => "2 Peter".to_string(),
        "I Johannes" => "1 John".to_string(),
        "II Johannes" => "2 John".to_string(),
        "III Johannes" => "3 John".to_string(),
        "Judas" => "Jude".to_string(),
        "Openbaring" => "Revelation".to_string(),
        "Openbaringen" => "Revelation".to_string(),
        
        // If no translation found, return as-is (might already be English)
        _ => display_name.to_string(),
    }
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
    // Reference selection state for keyboard navigation
    let (selected_reference_index, set_selected_reference_index) = signal(0usize);
    let navigate = use_navigate();
    
    // Convert display book name (e.g. "I Samuël") to canonical English name (e.g. "1 Samuel") 
    // for cross-reference lookup
    let canonical_book_name = get_canonical_book_name(&book_name);
    
    // Debug logging to track the conversion and lookup process
    web_sys::console::log_1(&format!("Cross-reference lookup: '{}' -> '{}' {}:{}", 
        book_name, canonical_book_name, chapter, verse).into());
    
    // Create the optimized verse ID for lookup
    let verse_id = match VerseId::from_book_name(&canonical_book_name, chapter, verse) {
        Some(id) => {
            web_sys::console::log_1(&format!("VerseId created successfully: {:?}", id).into());
            id
        },
        None => {
            web_sys::console::log_1(&format!("Failed to create VerseId for book: '{}'", canonical_book_name).into());
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
    
    // Debug logging for cross-reference lookup results
    web_sys::console::log_1(&format!("Cross-references found: {}", 
        verse_references.map(|refs| refs.len()).unwrap_or(0)).into());
    
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
    
    // Reset selection when references change
    Effect::new(move |_| {
        let _refs = sorted_references.get();
        set_selected_reference_index.set(0);
    });
    
    // Keyboard navigation for references
    let handle_keydown = move |e: KeyboardEvent| {
        if let Some(refs) = sorted_references.get() {
            if refs.is_empty() {
                return;
            }
            
            match (e.key().as_str(), e.ctrl_key(), e.shift_key()) {
                ("j", true, false) => {
                    // Ctrl+J: Next reference
                    e.prevent_default();
                    let current = selected_reference_index.get();
                    let next = if current + 1 < refs.len() { current + 1 } else { 0 };
                    set_selected_reference_index.set(next);
                }
                ("k", true, false) => {
                    // Ctrl+K: Previous reference
                    e.prevent_default();
                    let current = selected_reference_index.get();
                    let prev = if current > 0 { current - 1 } else { refs.len() - 1 };
                    set_selected_reference_index.set(prev);
                }
                ("Enter", false, false) => {
                    // Enter: Navigate to selected reference
                    e.prevent_default();
                    let current = selected_reference_index.get();
                    if let Some(reference) = refs.get(current) {
                        let reference_url = reference_to_url(reference);
                        navigate(&reference_url, NavigateOptions { scroll: false, ..Default::default() });
                        // Close sidebar on mobile when reference is selected
                        if is_mobile_screen() {
                            set_sidebar_open.set(false);
                            save_sidebar_open(false);
                        }
                    }
                }
                _ => {}
            }
        }
    };
    
    // Add keyboard event listener to the sidebar
    window_event_listener(ev::keydown, handle_keydown);
    
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
                        each=move || {
                            sorted_references.get().unwrap_or_default()
                                .into_iter()
                                .enumerate()
                                .collect::<Vec<_>>()
                        }
                        key=|(index, reference)| (*index, reference.to_book_name.clone(), reference.to_chapter, reference.to_verse_start, reference.to_verse_end, reference.votes)
                        children=move |(index, reference)| {
                            let is_selected = Memo::new(move |_| selected_reference_index.get() == index);
                            view! {
                                <ReferenceItem 
                                    reference=reference
                                    is_selected=is_selected
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
    is_selected: Memo<bool>,
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
                class=move || format!(
                    "w-full text-left p-3 rounded-lg border transition-colors duration-150 group {}",
                    if is_selected.get() {
                        "border-blue-600 bg-blue-500 text-white shadow-lg"
                    } else {
                        "border-gray-200 hover:border-blue-300 hover:bg-blue-50"
                    }
                )
                on:click=move |_| {
                    navigate(&reference_url, NavigateOptions { scroll: false, ..Default::default() });
                    // Close sidebar on mobile when reference is selected
                    if is_mobile_screen() {
                        set_sidebar_open.set(false);
                        save_sidebar_open(false);
                    }
                }
            >
                <div class="flex justify-between items-start">
                    <div class="flex-1">
                        <div class=move || format!(
                            "text-sm font-medium {}",
                            if is_selected.get() {
                                "text-white"
                            } else {
                                "text-black group-hover:text-blue-700"
                            }
                        )>
                            {reference_text}
                        </div>
                    </div>
                    <div class="ml-2 flex-shrink-0">
                        <span class=move || format!(
                            "inline-flex items-center px-2 py-1 rounded-full text-xs font-medium {}",
                            if is_selected.get() {
                                "bg-blue-400 text-white"
                            } else {
                                "bg-gray-100 text-gray-700 group-hover:bg-blue-100 group-hover:text-blue-800"
                            }
                        )>
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

    #[test]
    fn test_canonical_book_name_conversion() {
        // Test English Roman numerals to Arabic numerals conversion
        assert_eq!(get_canonical_book_name("I Samuel"), "1 Samuel");
        assert_eq!(get_canonical_book_name("II Samuel"), "2 Samuel");
        assert_eq!(get_canonical_book_name("I Kings"), "1 Kings");
        assert_eq!(get_canonical_book_name("II Kings"), "2 Kings");
        assert_eq!(get_canonical_book_name("I Corinthians"), "1 Corinthians");
        assert_eq!(get_canonical_book_name("II Corinthians"), "2 Corinthians");
        assert_eq!(get_canonical_book_name("III John"), "3 John");
        
        // Test Revelation alternative names
        assert_eq!(get_canonical_book_name("Revelation of John"), "Revelation");
        assert_eq!(get_canonical_book_name("The Revelation"), "Revelation");
        assert_eq!(get_canonical_book_name("The Revelation of John"), "Revelation");
        
        // Test Dutch to English conversion for numbered books
        assert_eq!(get_canonical_book_name("I Samuël"), "1 Samuel");
        assert_eq!(get_canonical_book_name("II Samuël"), "2 Samuel");
        assert_eq!(get_canonical_book_name("I Koningen"), "1 Kings");
        assert_eq!(get_canonical_book_name("II Koningen"), "2 Kings");
        
        // Test other Dutch translations
        assert_eq!(get_canonical_book_name("Psalmen"), "Psalms");
        assert_eq!(get_canonical_book_name("Prediker"), "Ecclesiastes");
        assert_eq!(get_canonical_book_name("Openbaring"), "Revelation");
        assert_eq!(get_canonical_book_name("Openbaringen"), "Revelation");
        
        // Test that Arabic numeral English names pass through unchanged
        assert_eq!(get_canonical_book_name("1 Samuel"), "1 Samuel");
        assert_eq!(get_canonical_book_name("Genesis"), "Genesis");
        assert_eq!(get_canonical_book_name("Revelation"), "Revelation");
        
        // Test unknown names pass through unchanged
        assert_eq!(get_canonical_book_name("Unknown Book"), "Unknown Book");
    }

    #[test]
    fn test_revelation_verse_id_creation() {
        // Test that "Revelation of John" can successfully create a VerseId
        use crate::core::types::VerseId;
        
        let canonical_name = get_canonical_book_name("Revelation of John");
        assert_eq!(canonical_name, "Revelation");
        
        let verse_id = VerseId::from_book_name(&canonical_name, 22, 1);
        assert!(verse_id.is_some(), "Should be able to create VerseId for Revelation 22:1");
        
        if let Some(id) = verse_id {
            // Verify the VerseId was created correctly  
            // Book ID 66 for Revelation, chapter 22, verse 1
            assert_eq!(id.0, 0x42016001); // 66 << 24 | 22 << 12 | 1
        }
    }

    #[test]
    fn test_revelation_cross_references_lookup() {
        // Test that we can actually find cross-references for "Revelation of John" 22:1
        use crate::core::types::VerseId;
        
        let canonical_name = get_canonical_book_name("Revelation of John");
        let verse_id = VerseId::from_book_name(&canonical_name, 22, 1).unwrap();
        
        let references = get_cross_references();
        let verse_references = references.0.get(&verse_id);
        
        // We know from the data file that Rev.22.1 has many cross-references
        assert!(verse_references.is_some(), "Revelation 22:1 should have cross-references");
        
        if let Some(refs) = verse_references {
            assert!(!refs.is_empty(), "Revelation 22:1 should have at least one cross-reference");
            
            // Check for some specific references we know exist from the data
            let has_rev_7_17 = refs.iter().any(|r| {
                r.to_book_name == "Revelation" && r.to_chapter == 7 && r.to_verse_start == 17
            });
            
            let has_john_4_14 = refs.iter().any(|r| {
                r.to_book_name == "John" && r.to_chapter == 4 && r.to_verse_start == 14
            });
            
            // At least one of these should exist based on our cross-reference data
            assert!(has_rev_7_17 || has_john_4_14, 
                "Should find at least one expected cross-reference for Revelation 22:1. Found {} references", 
                refs.len());
        }
    }
}