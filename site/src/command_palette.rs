use crate::{Bible, Chapter};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos::web_sys::KeyboardEvent;

#[component]
pub fn CommandPalette(
    bible: Bible,
    is_open: ReadSignal<bool>,
    set_is_open: WriteSignal<bool>,
) -> impl IntoView {
    let navigate = use_navigate();
    let (search_query, set_search_query) = signal(String::new());
    let (selected_index, set_selected_index) = signal(0usize);
    let (navigate_to, set_navigate_to) = signal::<Option<String>>(None);
    
    // Create a node ref for the input element
    let input_ref = NodeRef::<leptos::html::Input>::new();

    // Create a memo for filtered chapters
    let filtered_chapters = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        if query.is_empty() {
            return Vec::new();
        }

        let mut results: Vec<(&Chapter, usize)> = bible
            .books
            .iter()
            .flat_map(|book| book.chapters.iter())
            .filter_map(|chapter| {
                let score = fuzzy_score(&chapter.name.to_lowercase(), &query);
                if score > 0 {
                    Some((chapter, score))
                } else {
                    None
                }
            })
            .collect();

        // Sort by score (higher is better)
        results.sort_by(|a, b| b.1.cmp(&a.1));
        
        results
            .into_iter()
            .take(10)
            .map(|(chapter, _)| chapter.clone())
            .collect::<Vec<Chapter>>()
    });

    // Set up global keyboard handling when palette is open
    let nav = navigate.clone();
    Effect::new(move |_| {
        if is_open.get() {
            let _nav = nav.clone();
            let handle_keydown = move |e: KeyboardEvent| {
                match e.key().as_str() {
                    "Escape" => {
                        set_is_open.set(false);
                        set_search_query.set(String::new());
                        set_selected_index.set(0);
                    }
                    "ArrowDown" => {
                        e.prevent_default();
                        let chapters = filtered_chapters.get();
                        if !chapters.is_empty() {
                            let current = selected_index.get();
                            // Always ensure we're within bounds
                            if current >= chapters.len() {
                                set_selected_index.set(0);
                            } else {
                                let next = if current + 1 >= chapters.len() {
                                    0 // wrap to first
                                } else {
                                    current + 1
                                };
                                set_selected_index.set(next);
                            }
                        }
                    }
                    "ArrowUp" => {
                        e.prevent_default();
                        let chapters = filtered_chapters.get();
                        if !chapters.is_empty() {
                            let current = selected_index.get();
                            // Always ensure we're within bounds
                            if current >= chapters.len() {
                                set_selected_index.set(chapters.len() - 1);
                            } else {
                                let next = if current == 0 {
                                    chapters.len() - 1 // wrap to last
                                } else {
                                    current - 1
                                };
                                set_selected_index.set(next);
                            }
                        }
                    }
                    "Enter" => {
                        e.prevent_default();
                        let chapters = filtered_chapters.get();
                        if !chapters.is_empty() {
                            let current = selected_index.get();
                            let valid_index = if current >= chapters.len() { 0 } else { current };
                            if let Some(chapter) = chapters.get(valid_index) {
                                set_navigate_to.set(Some(chapter.to_path()));
                                set_is_open.set(false);
                                set_search_query.set(String::new());
                                set_selected_index.set(0);
                            }
                        }
                    }
                    _ => {}
                }
            };
            
            let _cleanup = window_event_listener(leptos::ev::keydown, handle_keydown);
            // cleanup will happen when effect re-runs or component unmounts
        }
    });

    // Reset selected index when search changes or palette opens
    Effect::new(move |_| {
        search_query.track();
        set_selected_index.set(0);
    });
    
    // Reset selected index when palette opens
    Effect::new(move |_| {
        if is_open.get() {
            set_selected_index.set(0);
        }
    });

    // Handle navigation
    Effect::new(move |_| {
        if let Some(path) = navigate_to.get() {
            navigate(&path, Default::default());
            set_navigate_to.set(None);
        }
    });

    // Focus input when palette opens
    Effect::new(move |_| {
        if is_open.get() {
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
            }
        }
    });


    view! {
        <Show when=move || is_open.get() fallback=|| ()>
            // Backdrop
            <div 
                class="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-start justify-center pt-20"
                on:click=move |_| set_is_open.set(false)
            >
                // Command Palette Modal
                <div 
                    class="bg-white rounded-lg shadow-xl w-full max-w-lg mx-4 max-h-96 flex flex-col"
                    on:click=move |e| e.stop_propagation()
                >
                    // Search Input
                    <div class="p-4 border-b border-gray-200">
                        <input
                            node_ref=input_ref
                            type="text"
                            placeholder="Search chapters... (e.g., 'Genesis 1', 'John 3:16')"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            prop:value=search_query
                            on:input=move |e| set_search_query.set(event_target_value(&e))
                        />
                    </div>

                    // Results List
                    <div class="flex-1 overflow-y-auto">
                        <div class="py-2">
                            <Show
                                when=move || !search_query.get().is_empty()
                                fallback=|| view! { <div class="px-4 py-2 text-gray-500">"Start typing to search chapters..."</div> }
                            >
                                <div class="max-h-64 overflow-y-auto">
                                    {move || {
                                        let chapters = filtered_chapters.get();
                                        let current_selected = selected_index.get();
                                        let bounded_selected = if chapters.is_empty() { 
                                            0 
                                        } else { 
                                            current_selected.min(chapters.len() - 1) 
                                        };
                                        
                                        chapters.into_iter().enumerate().map(|(index, chapter)| {
                                            let is_selected = index == bounded_selected;
                                            view! {
                                                <div 
                                                    class=if is_selected { 
                                                        "px-4 py-3 bg-blue-500 text-white cursor-pointer flex items-center border-b border-blue-400" 
                                                    } else { 
                                                        "px-4 py-3 hover:bg-gray-100 cursor-pointer flex items-center border-b border-gray-100" 
                                                    }
                                                    on:click={
                                                        let chapter_path = chapter.to_path();
                                                        move |_| {
                                                            set_navigate_to.set(Some(chapter_path.clone()));
                                                            set_is_open.set(false);
                                                            set_search_query.set(String::new());
                                                            set_selected_index.set(0);
                                                        }
                                                    }
                                                >
                                                    <div class="flex-1">
                                                        <div class="font-medium">
                                                            {chapter.name.clone()}
                                                        </div>
                                                    </div>
                                                    {if is_selected {
                                                        view! {
                                                            <div class="text-xs opacity-75">
                                                                "↵"
                                                            </div>
                                                        }.into_any()
                                                    } else {
                                                        view! { <div></div> }.into_any()
                                                    }}
                                                </div>
                                            }
                                        }).collect_view()
                                    }}
                                    <Show when=move || filtered_chapters.get().is_empty()>
                                        <div class="px-4 py-2 text-gray-500 text-sm">
                                            "No chapters found"
                                        </div>
                                    </Show>
                                </div>
                            </Show>
                        </div>
                    </div>

                    // Footer with hint
                    <div class="px-4 py-2 border-t border-gray-200 text-xs text-gray-500">
                        Use up/down arrows to navigate, Enter to select, Esc to close
                    </div>
                </div>
            </div>
        </Show>
    }
}

/// Advanced fuzzy search that handles partial word matching and numbers
/// Examples:
/// - "ps 9" matches "psalmen 9" (partial word + number)
/// - "gen 3" matches "genesis 3" (partial word + number)  
/// - "john 3:16" matches "johannes 3:16" (partial word + full number)
fn fuzzy_score(text: &str, query: &str) -> usize {
    if query.is_empty() {
        return 0;
    }
    
    let text_lower = text.to_lowercase();
    let query_lower = query.to_lowercase();
    
    let text_words: Vec<&str> = text_lower.split_whitespace().collect();
    let query_words: Vec<&str> = query_lower.split_whitespace().collect();
    
    if query_words.is_empty() {
        return 0;
    }
    
    let mut total_score = 0;
    let mut query_word_index = 0;
    
    // Try to match each query word against text words
    for &query_word in &query_words {
        let mut best_word_score = 0;
        let mut found_match = false;
        
        // Check each text word for the best match
        for (text_word_index, &text_word) in text_words.iter().enumerate() {
            let word_score = score_word_match(text_word, query_word, text_word_index == query_word_index);
            if word_score > best_word_score {
                best_word_score = word_score;
                found_match = true;
            }
        }
        
        if !found_match {
            return 0; // Query word not found
        }
        
        total_score += best_word_score;
        query_word_index += 1;
    }
    
    // Bonus for matching all words in order
    if query_words.len() == text_words.len() {
        total_score += 50;
    }
    
    // Bonus for exact text match
    if text_lower == query_lower {
        total_score += 100;
    }
    
    total_score
}

/// Score how well a single query word matches a text word
fn score_word_match(text_word: &str, query_word: &str, is_positional_match: bool) -> usize {
    // Exact match gets highest score
    if text_word == query_word {
        return if is_positional_match { 100 } else { 80 };
    }
    
    // Check if text word starts with query word (partial match)
    if text_word.starts_with(query_word) {
        let match_ratio = (query_word.len() * 100) / text_word.len();
        return if is_positional_match { 
            50 + match_ratio / 2 
        } else { 
            30 + match_ratio / 2 
        };
    }
    
    // Check if query word is contained in text word
    if text_word.contains(query_word) {
        let match_ratio = (query_word.len() * 100) / text_word.len();
        return if is_positional_match { 
            20 + match_ratio / 4 
        } else { 
            10 + match_ratio / 4 
        };
    }
    
    // Check for number matching (both are numbers)
    if let (Ok(_), Ok(_)) = (text_word.parse::<u32>(), query_word.parse::<u32>()) {
        if text_word == query_word {
            return if is_positional_match { 90 } else { 70 };
        }
        // Partial number match (e.g., "9" matches "90")
        if text_word.starts_with(query_word) {
            return if is_positional_match { 60 } else { 40 };
        }
    }
    
    // Character-by-character fuzzy matching as fallback
    character_fuzzy_score(text_word, query_word, is_positional_match)
}

/// Character-level fuzzy matching for fallback cases
fn character_fuzzy_score(text: &str, query: &str, is_positional_match: bool) -> usize {
    let text_chars: Vec<char> = text.chars().collect();
    let query_chars: Vec<char> = query.chars().collect();
    
    let mut score = 0;
    let mut text_index = 0;
    let mut consecutive_matches = 0;
    
    for &query_char in &query_chars {
        let mut found = false;
        
        while text_index < text_chars.len() {
            if text_chars[text_index] == query_char {
                found = true;
                consecutive_matches += 1;
                score += 1 + consecutive_matches; // Bonus for consecutive matches
                text_index += 1;
                break;
            } else {
                consecutive_matches = 0;
                text_index += 1;
            }
        }
        
        if !found {
            return 0; // Query character not found
        }
    }
    
    // Apply positional bonus
    if is_positional_match {
        score = score * 3 / 2;
    }
    
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        assert!(fuzzy_score("genesis 1", "genesis 1") > fuzzy_score("genesis 1", "gen 1"));
        assert!(fuzzy_score("psalmen 9", "psalmen 9") > 0);
    }

    #[test]
    fn test_partial_word_matching() {
        // "ps 9" should match "psalmen 9"
        let score = fuzzy_score("psalmen 9", "ps 9");
        assert!(score > 0, "ps 9 should match psalmen 9, got score: {}", score);
        
        // "gen 3" should match "genesis 3"
        let score = fuzzy_score("genesis 3", "gen 3");
        assert!(score > 0, "gen 3 should match genesis 3, got score: {}", score);
        
        // "john 14" should match "johannes 14"
        let score = fuzzy_score("johannes 14", "john 14");
        assert!(score > 0, "john 14 should match johannes 14, got score: {}", score);
    }

    #[test]
    fn test_number_matching() {
        // "ps 90" should match "psalmen 90"
        let score = fuzzy_score("psalmen 90", "ps 90");
        assert!(score > 0, "ps 90 should match psalmen 90, got score: {}", score);
        
        // "9" should match "90" (partial number)
        let score = fuzzy_score("psalmen 90", "ps 9");
        assert!(score > 0, "ps 9 should partially match psalmen 90, got score: {}", score);
        
        // Exact number match should score higher than partial
        let exact_score = fuzzy_score("psalmen 9", "ps 9");
        let partial_score = fuzzy_score("psalmen 90", "ps 9");
        assert!(exact_score > partial_score, "Exact number match should score higher: {} vs {}", exact_score, partial_score);
    }

    #[test]
    fn test_score_ranking() {
        // More specific matches should score higher
        let results = vec![
            ("psalmen 9", fuzzy_score("psalmen 9", "ps 9")),
            ("psalmen 90", fuzzy_score("psalmen 90", "ps 9")),
            ("psalmen 19", fuzzy_score("psalmen 19", "ps 9")),
            ("psalm 9", fuzzy_score("psalm 9", "ps 9")),
        ];
        
        // psalmen 9 should score highest (exact number match)
        let psalmen_9_score = results.iter().find(|(name, _)| *name == "psalmen 9").unwrap().1;
        let psalmen_90_score = results.iter().find(|(name, _)| *name == "psalmen 90").unwrap().1;
        
        assert!(psalmen_9_score > psalmen_90_score, "Exact number match should score higher than partial: {} vs {}", psalmen_9_score, psalmen_90_score);
    }

    #[test]
    fn test_no_match_cases() {
        // Should return 0 for no match
        assert_eq!(fuzzy_score("genesis 1", "xyz"), 0);
        assert_eq!(fuzzy_score("psalmen 9", "abc"), 0);
        assert_eq!(fuzzy_score("", "test"), 0);
        assert_eq!(fuzzy_score("test", ""), 0);
    }

    #[test]
    fn test_case_insensitive() {
        // Should handle mixed case
        let score1 = fuzzy_score("Genesis 1", "gen 1");
        let score2 = fuzzy_score("genesis 1", "GEN 1");
        let score3 = fuzzy_score("GENESIS 1", "gen 1");
        
        assert!(score1 > 0 && score2 > 0 && score3 > 0, "Should handle mixed case");
    }

    #[test]
    fn test_word_order_sensitivity() {
        // Words in correct order should score higher
        let correct_order = fuzzy_score("genesis 3", "gen 3");
        let wrong_order = fuzzy_score("3 genesis", "gen 3");
        
        assert!(correct_order > wrong_order, "Correct word order should score higher: {} vs {}", correct_order, wrong_order);
    }

    #[test]
    fn test_starts_with_bonus() {
        // Word that starts with query should score higher than contains
        let starts_with_score = fuzzy_score("genesis 1", "gen");
        let contains_score = fuzzy_score("regeneration 1", "gen");
        
        assert!(starts_with_score > contains_score, "Starts-with should score higher than contains: {} vs {}", starts_with_score, contains_score);
    }

    #[test]
    fn test_comprehensive_example() {
        // Test the main use case: searching for "ps 9"
        let test_chapters = vec![
            "psalmen 9",
            "psalmen 90", 
            "psalmen 19",
            "psalm 9",
            "spreuken 9",
            "genesis 9",
        ];
        
        let mut scored_results: Vec<(&str, usize)> = test_chapters
            .iter()
            .map(|&chapter| (chapter, fuzzy_score(chapter, "ps 9")))
            .filter(|(_, score)| *score > 0)
            .collect();
        
        scored_results.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Should have results and prioritize exact number matches
        assert!(!scored_results.is_empty(), "Should find matches for 'ps 9'");
        
        // Both "psalm 9" and "psalmen 9" should be in top results
        let top_two: Vec<&str> = scored_results.iter().take(2).map(|(name, _)| *name).collect();
        assert!(top_two.contains(&"psalm 9") || top_two.contains(&"psalmen 9"), 
                "Top results should include psalm variants, got: {:?}", scored_results);
        
        // Should find multiple relevant results
        let psalm_matches: Vec<&str> = scored_results
            .iter()
            .map(|(name, _)| *name)
            .filter(|name| name.starts_with("psalm"))
            .collect();
        
        assert!(psalm_matches.len() >= 2, "Should find multiple psalm matches: {:?}", psalm_matches);
    }
}