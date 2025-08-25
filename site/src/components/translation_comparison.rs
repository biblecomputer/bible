/*!
 * Translation Comparison Component
 *
 * This component provides side-by-side comparison of Bible verses across
 * different translations. Users can select multiple translations and view
 * the current verse(s) displayed in parallel.
 *
 * Features:
 * - Multi-translation selection via checkboxes
 * - Automatic verse context from current URL
 * - Responsive design with theming support
 * - Loading states and error handling
 * - Keyboard shortcuts (Escape to close)
 */

use leptos::ev;
use leptos::prelude::*;
use leptos::web_sys::KeyboardEvent;
use wasm_bindgen_futures::spawn_local;

// Core types and utilities
use crate::core::{parse_verse_ranges_from_url, Verse, VerseRange};
use crate::instructions::types::Instruction;
use crate::storage::translations::get_translations;
use crate::storage::{get_downloaded_translations, load_downloaded_translation};
use crate::view_state::ViewStateSignal;

/// Internal state for tracking comparison data
#[derive(Debug, Clone)]
struct ComparisonData {
    /// Translation name for display
    translation_name: String,
    /// Verses from this translation
    verses: Vec<Verse>,
}

/**
 * Main Translation Comparison Component
 *
 * Renders a right-side panel that allows users to compare verses across
 * multiple Bible translations. The panel includes:
 * - Header with close button
 * - Translation selection checkboxes
 * - Comparison results display
 * - Loading states
 */
#[component]
pub fn TranslationComparison(
    /// The current book name being viewed
    current_book: ReadSignal<String>,
    /// The current chapter number being viewed
    current_chapter: ReadSignal<u32>,
    /// View state containing panel open/close state
    view_state: ViewStateSignal,
) -> impl IntoView {
    // === State Management ===

    // List of translation keys selected for comparison
    let (selected_translations, set_selected_translations) = signal::<Vec<String>>(Vec::new());

    // Processed comparison data with translation names and verses
    let (comparison_data, set_comparison_data) = signal::<Vec<ComparisonData>>(Vec::new());

    // Loading state for async operations
    let (loading, set_loading) = signal(false);

    // List of downloaded translations available for selection
    let (downloaded_translations, set_downloaded_translations) = signal::<Vec<String>>(Vec::new());

    // === Computed Values ===

    // Get the current verse ranges from the URL parameters
    let current_verse_ranges = Memo::new(move |_| parse_verse_ranges_from_url());

    // === Effects ===

    // Load available translations when panel opens
    // This effect runs whenever the panel is opened and populates
    // the list of available translations for comparison.
    Effect::new(move |_| {
        if view_state.with(|state| state.is_translation_comparison_open) {
            let translations = get_downloaded_translations();
            set_downloaded_translations.set(translations);
        }
    });

    // Load comparison data when translations are selected
    // This effect triggers whenever users select/deselect translations
    // and loads the corresponding verses for comparison.
    Effect::new(move |_| {
        let selected = selected_translations.get();

        if !selected.is_empty() && view_state.with(|state| state.is_translation_comparison_open) {
            let book = current_book.get();
            let chapter = current_chapter.get();
            let verse_ranges = current_verse_ranges.get();

            set_loading.set(true);

            spawn_local(async move {
                let mut comparison_results = Vec::new();

                // Load each selected translation
                for translation_key in selected {
                    if let Ok(bible) = load_downloaded_translation(&translation_key).await {
                        if let Ok(chapter_data) = bible.get_chapter(&book, chapter) {
                            // Filter verses based on current selection
                            let filtered_verses: Vec<Verse> = chapter_data
                                .verses
                                .iter()
                                .filter(|verse| {
                                    verse_ranges.iter().any(|range| {
                                        verse.verse >= range.start && verse.verse <= range.end
                                    })
                                })
                                .cloned()
                                .collect();

                            // Get user-friendly translation name
                            let translation_name = get_translations()
                                .iter()
                                .find(|t| t.short_name == translation_key)
                                .map(|t| t.name.clone())
                                .unwrap_or_else(|| translation_key.clone());

                            comparison_results.push(ComparisonData {
                                translation_name,
                                verses: filtered_verses,
                            });
                        }
                    }
                }

                set_comparison_data.set(comparison_results);
                set_loading.set(false);
            });
        } else {
            // Clear data when no translations selected
            set_comparison_data.set(Vec::new());
        }
    });

    // === Event Handlers ===

    // Close panel on Escape key press
    // Global keyboard listener that closes the comparison panel
    // when the user presses the Escape key.
    window_event_listener(ev::keydown, move |evt: KeyboardEvent| {
        if evt.key() == "Escape" && view_state.with(|state| state.is_translation_comparison_open) {
            evt.prevent_default();
            view_state.update(|state| {
                state.execute(&Instruction::CloseTranslationComparison);
            });
        }
    });

    // === Render Component ===

    view! {
        <Show when=move || view_state.with(|state| state.is_translation_comparison_open) fallback=|| view! { <></> }>
            {/* Main Panel Container */}
            <div class="fixed inset-y-0 right-0 w-96 bg-white shadow-lg z-30 flex flex-col border-l border-gray-200">

                {/* Panel Header */}
                <div class="flex items-center justify-between p-4 border-b border-gray-200">
                    <h2 class="text-lg font-semibold text-gray-800">Translation Comparison</h2>
                    <button
                        class="text-gray-500 hover:text-gray-700 transition-colors"
                        on:click=move |_| view_state.update(|state| state.is_translation_comparison_open = false)
                        aria-label="Close translation comparison panel"
                    >
                        {/* Close Icon */}
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                        </svg>
                    </button>
                </div>

                {/* Translation Selection Section */}
                <div class="p-4 border-b border-gray-200 bg-gray-50">
                    <h3 class="text-sm font-medium text-gray-700 mb-3">Select Translations to Compare</h3>
                    <div class="space-y-2 max-h-32 overflow-y-auto">
                        <For
                            each=move || downloaded_translations.get()
                            key=|translation| translation.clone()
                            children=move |translation: String| {
                                render_translation_checkbox(
                                    translation,
                                    selected_translations,
                                    set_selected_translations
                                )
                            }
                        />
                    </div>
                </div>

                {/* Comparison Results Section */}
                <div class="flex-1 overflow-y-auto p-4">
                    {render_comparison_results(loading, comparison_data, current_verse_ranges)}
                </div>

                {/* Panel Footer */}
                <div class="p-4 border-t border-gray-200 bg-gray-50">
                    <div class="text-xs text-gray-500 text-center">
                        <p>
                            Press
                            <kbd class="px-1 py-0.5 bg-gray-200 rounded text-xs">Esc</kbd>
                            or
                            <kbd class="px-1 py-0.5 bg-gray-200 rounded text-xs">C</kbd>
                            to close
                        </p>
                    </div>
                </div>
            </div>
        </Show>
    }
}

/**
 * Render a translation selection checkbox
 *
 * Creates a checkbox for selecting/deselecting a translation
 * for comparison. Includes the translation name and short code.
 */
fn render_translation_checkbox(
    translation: String,
    selected_translations: ReadSignal<Vec<String>>,
    set_selected_translations: WriteSignal<Vec<String>>,
) -> impl IntoView {
    let translation_clone = translation.clone();

    // Get display name for the translation
    let translation_display = get_translations()
        .iter()
        .find(|t| t.short_name == translation)
        .map(|t| format!("{} ({})", t.name, t.short_name))
        .unwrap_or_else(|| translation.clone());

    // Check if this translation is currently selected
    let is_selected = Memo::new(move |_| selected_translations.get().contains(&translation));

    view! {
        <label class="flex items-center space-x-2 cursor-pointer">
            <input
                type="checkbox"
                class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                prop:checked=move || is_selected.get()
                on:change=move |_| {
                    let mut current = selected_translations.get();
                    if current.contains(&translation_clone) {
                        // Remove if already selected
                        current.retain(|t| t != &translation_clone);
                    } else {
                        // Add if not selected
                        current.push(translation_clone.clone());
                    }
                    set_selected_translations.set(current);
                }
            />
            <span class="text-sm text-gray-700">{translation_display}</span>
        </label>
    }
}

/**
 * Render the comparison results section
 *
 * Shows either a loading state, empty state, or the actual
 * verse comparisons based on the current state.
 */
fn render_comparison_results(
    loading: ReadSignal<bool>,
    comparison_data: ReadSignal<Vec<ComparisonData>>,
    current_verse_ranges: Memo<Vec<VerseRange>>,
) -> impl IntoView {
    view! {
        <Show
            when=move || loading.get()
            fallback=move || {
                render_comparison_content(comparison_data, current_verse_ranges)
            }
        >
            {/* Loading State */}
            <div class="flex items-center justify-center py-8">
                <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
                <span class="ml-2 text-sm text-gray-600">Loading translations...</span>
            </div>
        </Show>
    }
}

/**
 * Render the actual comparison content
 *
 * Shows either the verse comparisons or an empty state message.
 */
fn render_comparison_content(
    comparison_data: ReadSignal<Vec<ComparisonData>>,
    current_verse_ranges: Memo<Vec<VerseRange>>,
) -> impl IntoView {
    view! {
        <Show
            when=move || !comparison_data.get().is_empty()
            fallback=move || {
                render_empty_state(current_verse_ranges)
            }
        >
            {/* Comparison Results */}
            <div class="space-y-6">
                <For
                    each=move || comparison_data.get()
                    key=|data| data.translation_name.clone()
                    children=move |data: ComparisonData| {
                        render_translation_verses(data)
                    }
                />
            </div>
        </Show>
    }
}

/**
 * Render empty state when no translations are selected
 */
fn render_empty_state(current_verse_ranges: Memo<Vec<VerseRange>>) -> impl IntoView {
    view! {
        <div class="text-center text-gray-500 mt-8">
            <p class="text-sm">Select translations above to see verse comparisons</p>
            <p class="text-xs mt-2 text-gray-400">
                "Showing verses: " {move || format_verse_ranges(&current_verse_ranges.get())}
            </p>
        </div>
    }
}

/**
 * Render verses for a single translation
 *
 * Creates a card showing the translation name and its verses.
 */
fn render_translation_verses(data: ComparisonData) -> impl IntoView {
    view! {
        <div class="border rounded-lg p-4 bg-gray-50">
            <h4 class="font-medium text-gray-800 mb-3 text-sm">
                {data.translation_name}
            </h4>
            <div class="space-y-2">
                <For
                    each=move || data.verses.clone()
                    key=|verse| verse.verse
                    children=move |verse: Verse| {
                        view! {
                            <div class="flex gap-2">
                                <span class="text-xs font-medium text-gray-500 mt-1 min-w-[20px]">
                                    {verse.verse}
                                </span>
                                <p class="text-sm text-gray-700 leading-relaxed">
                                    {verse.text}
                                </p>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

/**
 * Format verse ranges for display
 *
 * Converts a list of verse ranges into a human-readable string.
 * Examples: "5", "1-3", "1-3, 5, 10-12"
 */
fn format_verse_ranges(ranges: &[VerseRange]) -> String {
    ranges
        .iter()
        .map(|range| {
            if range.start == range.end {
                range.start.to_string()
            } else {
                format!("{}-{}", range.start, range.end)
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}
