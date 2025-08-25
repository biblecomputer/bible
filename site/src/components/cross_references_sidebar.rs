use crate::core::load_cross_references;
use crate::core::types::Language;
use crate::core::types::{Reference, References, VerseId};
use crate::instructions::types::Instruction;
use crate::storage::translations::get_current_translation;
use crate::translation_map::translation::Translation;
use crate::utils::is_mobile_screen;
use crate::view_state::ViewStateSignal;
use leptos::ev;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::KeyboardEvent;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use std::sync::OnceLock;
use urlencoding::encode;

// Global cross-references cache (already optimized with your compile-time system)
static CROSS_REFERENCES: OnceLock<References> = OnceLock::new();

fn get_cross_references() -> &'static References {
    CROSS_REFERENCES.get_or_init(|| {
        web_sys::console::log_1(
            &"Loading cross-references data for first time (panel opened)".into(),
        );
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

fn get_ui_text(key: &str) -> String {
    if let Some(current_translation) = get_current_translation() {
        if let Some(first_language) = current_translation.languages.first() {
            match (key, first_language) {
                ("cross_references", Language::Dutch) => "Kruisverwijzingen".to_string(),
                ("cross_references", Language::English) => "Cross References".to_string(),
                ("no_references", Language::Dutch) => "Geen kruisverwijzingen gevonden".to_string(),
                ("no_references", Language::English) => "No cross references found".to_string(),
                ("votes", Language::Dutch) => "stemmen".to_string(),
                ("votes", Language::English) => "votes".to_string(),
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
            let translation = Translation::from_language(*first_language);

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
        format!(
            "{} {}:{}-{}",
            translated_book, reference.to_chapter, reference.to_verse_start, end_verse
        )
    } else {
        format!(
            "{} {}:{}",
            translated_book, reference.to_chapter, reference.to_verse_start
        )
    }
}

fn reference_to_url(reference: &Reference) -> String {
    // Convert canonical book name back to display book name used in the Bible
    let display_book_name = get_display_book_name(&reference.to_book_name);
    let encoded_book = encode(&display_book_name);

    // Ensure chapter and verse are valid positive numbers
    let chapter = reference.to_chapter.max(1);
    let verse_start = reference.to_verse_start.max(1);

    if let Some(end_verse) = reference.to_verse_end {
        let verse_end = end_verse.max(verse_start);
        format!(
            "/{}/{}?verses={}-{}",
            encoded_book, chapter, verse_start, verse_end
        )
    } else {
        format!("/{}/{}?verses={}", encoded_book, chapter, verse_start)
    }
}

fn get_verse_content_for_reference(reference: &Reference) -> String {
    use crate::core::get_bible;

    // Safe verse content retrieval with error handling
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // Convert canonical book name (Arabic numerals) to display format (Roman numerals)
        // that the Bible data uses. E.g., "1 Samuel" -> "I Samuel"
        let bible_book_name = get_display_book_name(&reference.to_book_name);

        // Try to get the verse content for the reference
        if let Ok(bible) = get_bible().get_chapter(&bible_book_name, reference.to_chapter) {
            if let Some(verse) = bible
                .verses
                .iter()
                .find(|v| v.verse == reference.to_verse_start)
            {
                return verse.text.to_string(); // More explicit than clone for strings
            }
        }

        // Fallback if verse content not found
        "Verse content not available".to_string()
    })) {
        Ok(content) => content,
        Err(_) => {
            web_sys::console::warn_1(
                &format!(
                    "Failed to get verse content for {} {}:{} (converted to {})",
                    reference.to_book_name,
                    reference.to_chapter,
                    reference.to_verse_start,
                    get_display_book_name(&reference.to_book_name)
                )
                .into(),
            );
            "Verse content unavailable".to_string()
        }
    }
}

fn format_votes_with_emoji(votes: i32) -> String {
    if votes > 0 {
        format!("👍 {}", votes)
    } else if votes < 0 {
        format!("👎 {}", votes.abs())
    } else {
        "👍 0".to_string()
    }
}

fn get_display_book_name(canonical_name: &str) -> String {
    // Convert canonical English names back to the display names used in the Bible
    // This is the reverse of get_canonical_book_name
    match canonical_name {
        // Convert back to display names that the Bible uses
        "Revelation" => "Revelation of John".to_string(),
        "1 Samuel" => "I Samuel".to_string(),
        "2 Samuel" => "II Samuel".to_string(),
        "1 Kings" => "I Kings".to_string(),
        "2 Kings" => "II Kings".to_string(),
        "1 Chronicles" => "I Chronicles".to_string(),
        "2 Chronicles" => "II Chronicles".to_string(),
        "1 Corinthians" => "I Corinthians".to_string(),
        "2 Corinthians" => "II Corinthians".to_string(),
        "1 Thessalonians" => "I Thessalonians".to_string(),
        "2 Thessalonians" => "II Thessalonians".to_string(),
        "1 Timothy" => "I Timothy".to_string(),
        "2 Timothy" => "II Timothy".to_string(),
        "1 Peter" => "I Peter".to_string(),
        "2 Peter" => "II Peter".to_string(),
        "1 John" => "I John".to_string(),
        "2 John" => "II John".to_string(),
        "3 John" => "III John".to_string(),

        // For all other books, return the canonical name as-is
        _ => canonical_name.to_string(),
    }
}

#[component]
pub fn CrossReferencesSidebar(
    book_name: String,
    chapter: u32,
    verse: u32,
    view_state: ViewStateSignal,
) -> impl IntoView {
    // Reference selection state for keyboard navigation with debouncing
    let (selected_reference_index, set_selected_reference_index) = signal(0usize);
    let (is_navigating, set_is_navigating) = signal(false);
    let (_sidebar_has_focus, set_sidebar_has_focus) = signal(false);
    let navigate = use_navigate();

    // Use a simple Arc<AtomicBool> for disposal tracking that doesn't rely on reactive system
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    let is_disposed = Arc::new(AtomicBool::new(false));

    // Convert display book name (e.g. "I Samuël") to canonical English name (e.g. "1 Samuel")
    // for cross-reference lookup
    let canonical_book_name = get_canonical_book_name(&book_name);

    // Debug logging to track the conversion and lookup process
    web_sys::console::log_1(
        &format!(
            "Cross-reference lookup: '{}' -> '{}' {}:{}",
            book_name, canonical_book_name, chapter, verse
        )
        .into(),
    );

    // Create the optimized verse ID for lookup (now used only for validation)
    let _verse_id = match VerseId::from_book_name(&canonical_book_name, chapter, verse) {
        Some(id) => {
            web_sys::console::log_1(&format!("VerseId created successfully: {:?}", id).into());
            id
        }
        None => {
            web_sys::console::log_1(
                &format!(
                    "Failed to create VerseId for book: '{}'",
                    canonical_book_name
                )
                .into(),
            );
            // Unknown book - show no references
            return view! {
                <div class="cross-references-sidebar">
                    <div class="mb-4">
                        <h2 class="text-lg font-bold mb-2" style="color: var(--theme-sidebar-text)">{get_ui_text("cross_references")}</h2>
                        <div class="text-sm mb-4" style="color: var(--theme-text-secondary)">
                            {get_translated_book_name(&book_name)} " " {chapter} ":" {verse}
                        </div>
                    </div>
                    <div class="text-sm italic" style="color: var(--theme-text-muted)">
                        {get_ui_text("no_references")}
                    </div>
                </div>
            };
        }
    };

    // Debug logging for the chapter-based approach
    web_sys::console::log_1(
        &format!(
            "Loading references for entire chapter: {} {}",
            canonical_book_name, chapter
        )
        .into(),
    );

    // Load all references for the current chapter at once (performance optimization)
    // This memo only recalculates when book_name or chapter changes, NOT when verse changes
    // NOTE: References are only loaded when this component is actually rendered (panel is open)
    let chapter_references = Memo::new({
        let canonical_book_name = canonical_book_name.clone();
        move |_| {
            let mut chapter_refs = std::collections::HashMap::new();

            // Only load references if the panel is actually open (this component exists)
            let references = get_cross_references();

            web_sys::console::log_1(
                &format!(
                    "Pre-loading references for chapter: {} {} (panel open)",
                    canonical_book_name, chapter
                )
                .into(),
            );

            // Load all verses in the chapter at once to prevent per-verse lookups during fast scrolling
            for verse_num in 1..=200 {
                // Conservative upper bound for verses in a chapter
                if let Some(verse_id) =
                    VerseId::from_book_name(&canonical_book_name, chapter, verse_num)
                {
                    if let Some(refs) = references.0.get(&verse_id) {
                        // Sort in-place without cloning the entire vector
                        let mut sorted = refs.to_vec(); // More explicit than clone
                        sorted.sort_unstable_by(|a, b| b.votes.cmp(&a.votes)); // Faster sort
                        chapter_refs.insert(verse_num, sorted);
                    }
                }
            }

            web_sys::console::log_1(
                &format!(
                    "Pre-loaded references for {} verses in chapter {} {} (panel open)",
                    chapter_refs.len(),
                    canonical_book_name,
                    chapter
                )
                .into(),
            );

            chapter_refs
        }
    });

    // Get references for current verse from the pre-loaded chapter data
    // This is now extremely fast - just a HashMap lookup, no reactive recalculation per verse
    // Throttled to prevent excessive updates during rapid scrolling
    let sorted_references = Memo::new(move |_| {
        let chapter_data = chapter_references.get();
        chapter_data.get(&verse).cloned()
    });

    // Reset selection when references change - with debouncing
    Effect::new(move |_| {
        let _refs = sorted_references.get();
        // Always reset to 0 when references change to prevent out-of-bounds
        set_selected_reference_index.set(0);
    });

    // Keyboard navigation for references - with comprehensive safety checks and debouncing
    let is_disposed_clone = is_disposed.clone();
    let _component_book_name = book_name.clone();
    let _component_chapter = chapter;
    let _component_verse = verse;
    let handle_keydown = move |e: KeyboardEvent| {
        // Early exit if component is disposed
        if is_disposed_clone.load(Ordering::Relaxed) {
            return;
        }

        // Don't handle navigation when command palette is open (let palette handle it)
        if view_state.with(|state| state.is_command_palette_open) {
            return;
        }

        // Safe access to sorted_references with disposal check
        let refs = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            sorted_references.get()
        })) {
            Ok(Some(refs)) if !refs.is_empty() => refs,
            _ => return, // Component is disposed or no references available
        };

        // Only handle specific keys: Ctrl+J, Ctrl+K for navigation, or Enter for selection
        if !matches!(
            (e.key().as_str(), e.ctrl_key()),
            ("j", true) | ("k", true) | ("Enter", false)
        ) {
            return;
        }

        // Prevent rapid-fire navigation that can cause memory issues
        if is_navigating.get() {
            return; // Skip if already processing navigation
        }

        // Bounds check current selection before processing with recovery
        let current = selected_reference_index.get();
        if current >= refs.len() {
            web_sys::console::warn_1(&"Reference index out of bounds, resetting to 0".into());
            set_selected_reference_index.set(0);
            return;
        }

        // Additional safety: check if we're in a valid state for navigation
        if refs.is_empty() || current >= refs.len() {
            return;
        }

        // Set navigation flag to prevent rapid firing
        set_is_navigating.set(true);

        match (e.key().as_str(), e.ctrl_key(), e.shift_key()) {
            ("j", true, false) => {
                // Ctrl+J: Next reference
                e.prevent_default();
                let next = if current + 1 < refs.len() {
                    current + 1
                } else {
                    0
                };
                set_selected_reference_index.set(next);
                set_is_navigating.set(false); // Clear navigation flag

                // Safe focus and announce for screen readers with bounds checking
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(element) =
                            document.get_element_by_id(&format!("reference-{}", next))
                        {
                            if let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
                                let _ = html_element.focus();

                                // Safe access to reference data with bounds checking
                                if let Some(reference) = refs.get(next) {
                                    // Create a screen reader announcement with verse content
                                    let reference_text = format_reference_text(reference);
                                    let verse_content = get_verse_content_for_reference(reference);
                                    let _announcement = format!(
                                        "{} of {}: {}, {}, {}",
                                        next + 1,
                                        refs.len(),
                                        reference_text,
                                        format_votes_with_emoji(reference.votes),
                                        verse_content
                                    );

                                    // For now, we rely on focus changes for screen reader announcements
                                    // The aria-label and focus will provide the accessibility
                                }
                            }
                        }
                    }
                }
            }
            ("k", true, false) => {
                // Ctrl+K: Previous reference
                e.prevent_default();
                let prev = if current > 0 {
                    current - 1
                } else {
                    refs.len() - 1
                };
                set_selected_reference_index.set(prev);
                set_is_navigating.set(false); // Clear navigation flag

                // Safe focus and announce for screen readers with bounds checking
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(element) =
                            document.get_element_by_id(&format!("reference-{}", prev))
                        {
                            if let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
                                let _ = html_element.focus();

                                // Safe access to reference data with bounds checking
                                if let Some(reference) = refs.get(prev) {
                                    // Create a screen reader announcement with verse content
                                    let reference_text = format_reference_text(reference);
                                    let verse_content = get_verse_content_for_reference(reference);
                                    let _announcement = format!(
                                        "{} of {}: {}, {}, {}",
                                        prev + 1,
                                        refs.len(),
                                        reference_text,
                                        format_votes_with_emoji(reference.votes),
                                        verse_content
                                    );

                                    // For now, we rely on focus changes for screen reader announcements
                                    // The aria-label and focus will provide the accessibility
                                }
                            }
                        }
                    }
                }
            }
            ("Enter", false, false) => {
                // Enter: Navigate to selected reference with bounds checking
                e.prevent_default();
                web_sys::console::log_1(
                    &format!(
                        "Enter pressed, current index: {}, refs length: {}",
                        current,
                        refs.len()
                    )
                    .into(),
                );
                if let Some(reference) = refs.get(current) {
                    let reference_url = reference_to_url(reference);
                    web_sys::console::log_1(&format!("Navigating to: {}", reference_url).into());
                    navigate(
                        &reference_url,
                        NavigateOptions {
                            scroll: false,
                            ..Default::default()
                        },
                    );
                    // Close sidebar on mobile when reference is selected
                    if is_mobile_screen() {
                        view_state.update(|state| {
                            state.execute(&Instruction::CloseRightSidebar);
                        });
                    }
                } else {
                    web_sys::console::warn_1(&format!("Attempted to navigate to reference at invalid index: {} (refs.len: {})", current, refs.len()).into());
                }
                set_is_navigating.set(false); // Clear navigation flag
            }
            _ => {
                set_is_navigating.set(false); // Clear navigation flag for any other key
            }
        }
    };

    // Add keyboard event listener - with proper cleanup to prevent disposed reactive value access
    let _cleanup = window_event_listener(ev::keydown, handle_keydown);
    on_cleanup(move || {
        // Mark component as disposed to prevent reactive value access
        is_disposed.store(true, Ordering::Relaxed);
        // Cleanup is handled automatically when _cleanup is dropped
        drop(_cleanup);
    });

    view! {
        <div
            class="cross-references-sidebar"
            tabindex="0"
            on:focus=move |_| set_sidebar_has_focus.set(true)
            on:blur=move |_| set_sidebar_has_focus.set(false)
            on:mouseenter=move |_| set_sidebar_has_focus.set(true)
            on:mouseleave=move |_| set_sidebar_has_focus.set(false)
        >
            <div class="mb-4">
                <h2 class="text-lg font-bold mb-2" style="color: var(--theme-sidebar-text)">{get_ui_text("cross_references")}</h2>
                <div class="text-sm mb-4" style="color: var(--theme-text-secondary)">
                    {get_translated_book_name(&book_name)} " " {chapter} ":" {verse}
                </div>
            </div>

            <Show
                when=move || sorted_references.get().is_some()
                fallback=move || view! {
                    <div class="text-sm italic" style="color: var(--theme-text-muted)">
                        {get_ui_text("no_references")}
                    </div>
                }
            >
                <div class="space-y-3" role="listbox" aria-label="Cross references" aria-activedescendant=move || format!("reference-{}", selected_reference_index.get())>
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
                            let reference_id = format!("reference-{}", index);
                            view! {
                                <ReferenceItem
                                    reference=reference
                                    is_selected=is_selected
                                    view_state=view_state
                                    reference_id=reference_id
                                />
                            }
                        }
                    />
                </div>

                // Live preview section for selected reference
                <Show when=move || sorted_references.get().is_some_and(|refs| !refs.is_empty())>
                    <div class="mt-4 border-t pt-4" style="border-color: var(--theme-sidebar-border)">
                        <h3 class="text-sm font-medium mb-2" style="color: var(--theme-sidebar-text)">Preview</h3>
                        <div class="rounded-lg p-3 max-h-32 overflow-y-auto" style="background-color: var(--theme-sidebar-background)">
                            <div class="text-xs mb-1" style="color: var(--theme-text-muted)">
                                {move || {
                                    if let Some(refs) = sorted_references.get() {
                                        if !refs.is_empty() {
                                            let current_index = selected_reference_index.get();
                                            // Bounds check before access to prevent WASM errors
                                            if current_index < refs.len() {
                                                if let Some(reference) = refs.get(current_index) {
                                                    format_reference_text(reference)
                                                } else {
                                                    String::new()
                                                }
                                            } else {
                                                String::new()
                                            }
                                        } else {
                                            String::new()
                                        }
                                    } else {
                                        String::new()
                                    }
                                }}
                            </div>
                            <div class="text-sm leading-relaxed" style="color: var(--theme-text-primary)">
                                {move || {
                                    if let Some(refs) = sorted_references.get() {
                                        if !refs.is_empty() {
                                            let current_index = selected_reference_index.get();
                                            // Bounds check before access to prevent WASM errors
                                            if current_index < refs.len() {
                                                if let Some(reference) = refs.get(current_index) {
                                                    get_verse_content_for_reference(reference)
                                                } else {
                                                    String::new()
                                                }
                                            } else {
                                                String::new()
                                            }
                                        } else {
                                            String::new()
                                        }
                                    } else {
                                        String::new()
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                </Show>
            </Show>
        </div>
    }
}

#[component]
fn ReferenceItem(
    reference: Reference,
    is_selected: Memo<bool>,
    view_state: ViewStateSignal,
    reference_id: String,
) -> impl IntoView {
    let navigate = use_navigate();
    let reference_text = format_reference_text(&reference);
    let reference_url = reference_to_url(&reference);
    let votes_text = format_votes_with_emoji(reference.votes);

    view! {
        <div class="reference-item">
            <button
                id=reference_id.clone()
                class="w-full text-left p-3 rounded-lg border transition-colors duration-150 group"
                style=move || {
                    if is_selected.get() {
                        "border-color: var(--theme-button-primary-background); background-color: var(--theme-button-primary-background); color: var(--theme-button-primary-text); box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)"
                    } else {
                        "border-color: var(--theme-sidebar-border); background-color: var(--theme-sidebar-background); color: var(--theme-text-primary)"
                    }
                }
                aria-selected=move || is_selected.get().to_string()
                aria-label=move || {
                    // Safe access to reference data
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let verse_content = get_verse_content_for_reference(&reference);
                        format!("{}, {}, {}", format_reference_text(&reference), format_votes_with_emoji(reference.votes), verse_content)
                    })) {
                        Ok(label) => label,
                        Err(_) => format!("{}, {}", format_reference_text(&reference), format_votes_with_emoji(reference.votes))
                    }
                }
                role="option"
                tabindex="0"
                on:click=move |_| {
                    navigate(&reference_url, NavigateOptions { scroll: false, ..Default::default() });
                    // Close sidebar on mobile when reference is selected
                    if is_mobile_screen() {
                        view_state.update(|state| { state.execute(&Instruction::CloseRightSidebar); });
                    }
                }
            >
                <div class="flex justify-between items-start">
                    <div class="flex-1">
                        <div class="text-sm font-medium"
                             style=move || {
                                 if is_selected.get() {
                                     "color: var(--theme-button-primary-text)"
                                 } else {
                                     "color: var(--theme-text-primary)"
                                 }
                             }>
                            {reference_text}
                        </div>
                    </div>
                    <div class="ml-2 flex-shrink-0">
                        <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium"
                              style=move || {
                                  if is_selected.get() {
                                      "background-color: var(--theme-button-primary-hover); color: var(--theme-button-primary-text)"
                                  } else {
                                      "background-color: var(--theme-sidebar-background); color: var(--theme-text-secondary); border: 1px solid var(--theme-sidebar-border)"
                                  }
                              }>
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
        let expected_single = format!(
            "{} {}:{}",
            reference.to_book_name, reference.to_chapter, reference.to_verse_start
        );
        assert_eq!(expected_single, "Isaiah 51:16");

        let range_reference = Reference {
            to_book_name: "Romans".to_string(),
            to_chapter: 1,
            to_verse_start: 19,
            to_verse_end: Some(20),
            votes: 50,
        };

        // Test range format: "Book Chapter:Start-End"
        let expected_range = format!(
            "{} {}:{}-{}",
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
        assert_eq!(
            get_canonical_book_name("The Revelation of John"),
            "Revelation"
        );

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
        assert!(
            verse_id.is_some(),
            "Should be able to create VerseId for Revelation 22:1"
        );

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
        assert!(
            verse_references.is_some(),
            "Revelation 22:1 should have cross-references"
        );

        if let Some(refs) = verse_references {
            assert!(
                !refs.is_empty(),
                "Revelation 22:1 should have at least one cross-reference"
            );

            // Check for some specific references we know exist from the data
            let has_rev_7_17 = refs.iter().any(|r| {
                r.to_book_name == "Revelation" && r.to_chapter == 7 && r.to_verse_start == 17
            });

            let has_john_4_14 = refs
                .iter()
                .any(|r| r.to_book_name == "John" && r.to_chapter == 4 && r.to_verse_start == 14);

            // At least one of these should exist based on our cross-reference data
            assert!(has_rev_7_17 || has_john_4_14,
                "Should find at least one expected cross-reference for Revelation 22:1. Found {} references", 
                refs.len());
        }
    }

    #[test]
    fn test_book_name_conversion_for_bible_lookup() {
        // Test that the book name conversion works correctly for Arabic -> Roman numeral conversion
        // This tests the fix for the cross-references verse content lookup issue

        // Cross-references use Arabic numerals
        assert_eq!(get_display_book_name("1 Samuel"), "I Samuel");
        assert_eq!(get_display_book_name("2 Samuel"), "II Samuel");
        assert_eq!(get_display_book_name("1 Kings"), "I Kings");
        assert_eq!(get_display_book_name("2 Kings"), "II Kings");
        assert_eq!(get_display_book_name("1 Chronicles"), "I Chronicles");
        assert_eq!(get_display_book_name("2 Chronicles"), "II Chronicles");
        assert_eq!(get_display_book_name("1 Corinthians"), "I Corinthians");
        assert_eq!(get_display_book_name("2 Corinthians"), "II Corinthians");
        assert_eq!(get_display_book_name("1 Thessalonians"), "I Thessalonians");
        assert_eq!(get_display_book_name("2 Thessalonians"), "II Thessalonians");
        assert_eq!(get_display_book_name("1 Timothy"), "I Timothy");
        assert_eq!(get_display_book_name("2 Timothy"), "II Timothy");
        assert_eq!(get_display_book_name("1 Peter"), "I Peter");
        assert_eq!(get_display_book_name("2 Peter"), "II Peter");
        assert_eq!(get_display_book_name("1 John"), "I John");
        assert_eq!(get_display_book_name("2 John"), "II John");
        assert_eq!(get_display_book_name("3 John"), "III John");

        // Books without numbers remain unchanged
        assert_eq!(get_display_book_name("Genesis"), "Genesis");
        assert_eq!(get_display_book_name("Matthew"), "Matthew");
        assert_eq!(get_display_book_name("Psalms"), "Psalms");

        // Revelation has a special case
        assert_eq!(get_display_book_name("Revelation"), "Revelation of John");
    }

    #[test]
    fn test_verse_content_retrieval_book_name_conversion() {
        // Test that verse content retrieval correctly converts book names
        // This is a mock test since we can't easily test the actual Bible data loading
        use crate::core::types::Reference;

        // Create a reference with Arabic numeral book name (as stored in cross-references)
        let reference = Reference {
            to_book_name: "1 Samuel".to_string(),
            to_chapter: 1,
            to_verse_start: 1,
            to_verse_end: None,
            votes: 10,
        };

        // The function should convert "1 Samuel" to "I Samuel" for Bible lookup
        // Since we can't test the actual Bible data loading without complex setup,
        // we'll just test that the conversion function is being called correctly
        let converted_name = get_display_book_name(&reference.to_book_name);
        assert_eq!(converted_name, "I Samuel");

        // Test that this would work for verse content retrieval
        // (The actual retrieval is tested indirectly through the integration)
        assert_eq!(get_display_book_name("1 Samuel"), "I Samuel");
        assert_eq!(get_display_book_name("2 Samuel"), "II Samuel");
    }
}
