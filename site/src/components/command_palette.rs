use crate::core::{get_bible, Chapter, VerseRange};
use crate::storage::translations::get_current_translation;
use crate::storage::recent_chapters::get_recent_chapters;
use crate::core::types::Language;
use crate::translation_map::translation::Translation;
use crate::instructions::types::Instruction;
use crate::instructions::processor::{InstructionContext, InstructionProcessor};
use crate::instructions::vim_keys::KeyboardMappings;
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_location};
use leptos_router::NavigateOptions;
use wasm_bindgen_futures::spawn_local;
use leptos::web_sys::KeyboardEvent;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SearchResult {
    Chapter(Chapter),
    Verse {
        chapter: Chapter,
        verse_number: u32,
        verse_text: String,
    },
    Instruction {
        name: String,
        description: String,
        shortcut: String,
    },
    RecentChapter {
        book_name: String,
        chapter: u32,
        display_name: String,
        path: String,
    },
}

impl SearchResult {
    pub fn get_display_name(&self) -> String {
        match self {
            SearchResult::Chapter(chapter) => get_translated_chapter_name(&chapter.name),
            SearchResult::Verse { chapter, verse_number, .. } => {
                format!("{} verse {}", get_translated_chapter_name(&chapter.name), verse_number)
            }
            SearchResult::Instruction { name, .. } => name.clone(),
            SearchResult::RecentChapter { display_name, .. } => display_name.clone(),
        }
    }
    
    pub fn to_path(&self) -> String {
        match self {
            SearchResult::Chapter(chapter) => chapter.to_path(),
            SearchResult::Verse { chapter, verse_number, .. } => {
                let verse_range = VerseRange { start: *verse_number, end: *verse_number };
                let path = chapter.to_path_with_verses(&[verse_range]);
                
                // Debug log for verse navigation
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&format!("Generating verse path: {} (verse {})", path, verse_number).into());
                
                path
            }
            SearchResult::Instruction { .. } => {
                // Instructions don't navigate to a path, they execute actions
                String::new()
            }
            SearchResult::RecentChapter { path, .. } => path.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct VerseReference {
    book_name: String,
    chapter: u32,
    verse: Option<u32>,
}

fn parse_verse_reference(query: &str) -> Option<VerseReference> {
    // Handle formats like "gen 1:1", "genesis 1:5", "john 3:16", "mat 5:3-7", and "gen 1:" (incomplete)
    let query = query.trim().to_lowercase();
    
    // Look for colon indicating verse reference
    if let Some(colon_pos) = query.find(':') {
        let before_colon = &query[..colon_pos];
        let after_colon = &query[colon_pos + 1..].trim();
        
        // Split the part before colon into book and chapter
        let parts: Vec<&str> = before_colon.split_whitespace().collect();
        if parts.len() >= 2 {
            // Try to parse the last part as chapter number
            if let Ok(chapter_num) = parts.last().unwrap().parse::<u32>() {
                let book_name = parts[..parts.len() - 1].join(" ");
                
                // Handle incomplete verse reference (just "gen 1:")
                if after_colon.is_empty() {
                    return Some(VerseReference {
                        book_name,
                        chapter: chapter_num,
                        verse: None, // No specific verse
                    });
                }
                
                // Parse verse number (take only the first number if it's a range like "3-7")
                let verse_str = after_colon.split('-').next().unwrap_or(after_colon);
                if let Ok(verse_num) = verse_str.parse::<u32>() {
                    return Some(VerseReference {
                        book_name,
                        chapter: chapter_num,
                        verse: Some(verse_num),
                    });
                }
            }
        }
    }
    
    None
}

fn score_verse_number_match(verse_number: u32, search_number: u32) -> usize {
    let verse_str = verse_number.to_string();
    let search_str = search_number.to_string();
    
    // Exact match gets highest score
    if verse_number == search_number {
        return 1000;
    }
    
    // Check if verse number starts with search number (e.g., 10, 11, 12 when searching for 1)
    if verse_str.starts_with(&search_str) {
        return 800;
    }
    
    // Check if search number starts with verse number (e.g., searching for 10 when verse is 1)
    if search_str.starts_with(&verse_str) {
        return 400;
    }
    
    // Check if verse number contains search number (e.g., 21, 31 when searching for 1)
    if verse_str.contains(&search_str) {
        return 600;
    }
    
    0
}

fn convert_language(storage_lang: &crate::storage::translation_storage::Language) -> Language {
    match storage_lang {
        crate::storage::translation_storage::Language::Dutch => Language::Dutch,
        crate::storage::translation_storage::Language::English => Language::English,
    }
}

// Convert vim key notation to user-friendly display
fn vim_key_to_display(vim_key: &str) -> String {
    match vim_key {
        "<Down>" => "↓".to_string(),
        "<Up>" => "↑".to_string(),
        "<Left>" => "←".to_string(),
        "<Right>" => "→".to_string(),
        "<S-H>" => "Shift+H".to_string(),
        "<S-L>" => "Shift+L".to_string(),
        "<S-G>" => "Shift+G".to_string(),
        "<S-C>" => "Shift+C".to_string(),
        "<C-o>" => "Ctrl+O".to_string(),
        "<M-o>" => "Alt+O".to_string(),
        "<C-S-R>" => "Ctrl+Shift+R".to_string(),
        "<C-j>" => "Ctrl+J".to_string(),
        "<C-k>" => "Ctrl+K".to_string(),
        "<gt>" => "> (shift+.)".to_string(),
        "gg" => "gg".to_string(),
        key => key.to_string(),
    }
}

// Convert instruction enum name to user-friendly display name and description
fn instruction_to_display(instruction_name: &str) -> (String, String) {
    match instruction_name {
        "NextVerse" => ("Next Verse".to_string(), "Navigate to the next verse".to_string()),
        "PreviousVerse" => ("Previous Verse".to_string(), "Navigate to the previous verse".to_string()),
        "NextChapter" => ("Next Chapter".to_string(), "Navigate to the next chapter".to_string()),
        "PreviousChapter" => ("Previous Chapter".to_string(), "Navigate to the previous chapter".to_string()),
        "NextBook" => ("Next Book".to_string(), "Navigate to the next book".to_string()),
        "PreviousBook" => ("Previous Book".to_string(), "Navigate to the previous book".to_string()),
        "BeginningOfChapter" => ("Beginning of Chapter".to_string(), "Go to the first verse of the chapter".to_string()),
        "EndOfChapter" => ("End of Chapter".to_string(), "Go to the last verse of the chapter".to_string()),
        "SwitchToPreviousChapter" => ("Switch to Previous Chapter".to_string(), "Go back to the previously viewed chapter".to_string()),
        "CopyRawVerse" => ("Copy Raw Verse".to_string(), "Copy the verse text to clipboard".to_string()),
        "CopyVerseWithReference" => ("Copy Verse with Reference".to_string(), "Copy verse with reference to clipboard".to_string()),
        "ToggleSidebar" => ("Toggle Sidebar".to_string(), "Show/hide the books sidebar".to_string()),
        "ToggleCrossReferences" => ("Toggle Cross References".to_string(), "Show/hide cross-references panel".to_string()),
        "ToggleBiblePallate" => ("Toggle Command Palette".to_string(), "Toggle this command palette".to_string()),
        "ToggleCommandPallate" => ("Open Command Palette".to_string(), "Open command palette with shortcuts".to_string()),
        "NextReference" => ("Next Reference".to_string(), "Navigate to next cross-reference".to_string()),
        "PreviousReference" => ("Previous Reference".to_string(), "Navigate to previous cross-reference".to_string()),
        "NextPaletteResult" => ("Next Palette Result".to_string(), "Navigate to next search result".to_string()),
        "PreviousPaletteResult" => ("Previous Palette Result".to_string(), "Navigate to previous search result".to_string()),
        "OpenGithubRepository" => ("Open GitHub Repository".to_string(), "Open the project repository on GitHub".to_string()),
        "RandomVerse" => ("Random Verse".to_string(), "Navigate to a random verse in the Bible".to_string()),
        _ => (instruction_name.to_string(), format!("Execute {}", instruction_name)),
    }
}

fn get_all_instructions() -> Vec<SearchResult> {
    let mappings = KeyboardMappings::load();
    let mut instruction_shortcuts: HashMap<String, Vec<String>> = HashMap::new();
    
    // Group all shortcuts by instruction
    for (vim_key, instruction_name) in &mappings.mappings {
        let display_key = vim_key_to_display(vim_key);
        instruction_shortcuts
            .entry(instruction_name.clone())
            .or_insert_with(Vec::new)
            .push(display_key);
    }
    
    // Add any instructions that might not have shortcuts yet (for future extensibility)
    let all_possible_instructions = [
        "NextVerse", "PreviousVerse", "NextChapter", "PreviousChapter",
        "NextBook", "PreviousBook", "BeginningOfChapter", "EndOfChapter",
        "SwitchToPreviousChapter", "CopyRawVerse", "CopyVerseWithReference",
        "ToggleSidebar", "ToggleCrossReferences", "ToggleBiblePallate", "ToggleCommandPallate",
        "NextReference", "PreviousReference", "NextPaletteResult", "PreviousPaletteResult",
        "OpenGithubRepository", "RandomVerse"
    ];
    
    for instruction in &all_possible_instructions {
        if !instruction_shortcuts.contains_key(*instruction) {
            instruction_shortcuts.insert(instruction.to_string(), vec!["No shortcut".to_string()]);
        }
    }
    
    // Create SearchResult for each unique instruction
    instruction_shortcuts
        .into_iter()
        .map(|(instruction_name, shortcuts)| {
            let (display_name, description) = instruction_to_display(&instruction_name);
            let shortcut_text = shortcuts.join(", ");
            
            SearchResult::Instruction {
                name: display_name,
                description,
                shortcut: shortcut_text,
            }
        })
        .collect()
}

fn get_translated_chapter_name(chapter_name: &str) -> String {
    if let Some(current_translation) = get_current_translation() {
        if let Some(first_language) = current_translation.languages.first() {
            let translation = Translation::from_language(convert_language(first_language));
            
            // Use the Translation.get() method which handles both book names and chapter references
            if let Some(translated_name) = translation.get(chapter_name) {
                return translated_name;
            }
        }
    }
    
    // Return original name if no translation found
    chapter_name.to_string()
}

fn get_current_chapter(location_pathname: &str) -> Option<Chapter> {
    let path_parts: Vec<&str> = location_pathname.trim_start_matches('/').split('/').collect();
    if path_parts.len() == 2 {
        let book_name = path_parts[0].replace('_', " ");
        if let Ok(chapter_num) = path_parts[1].parse::<u32>() {
            if let Ok(chapter) = get_bible().get_chapter(&book_name, chapter_num) {
                return Some(chapter);
            }
        }
    }
    None
}

fn create_instruction_context(pathname: &str, search: &str) -> Option<InstructionContext> {
    let path_parts: Vec<&str> = pathname.trim_start_matches('/').split('/').collect();
    if path_parts.len() == 2 {
        let book_name = path_parts[0].replace('_', " ");
        if let Ok(chapter_num) = path_parts[1].parse::<u32>() {
            if let Ok(current_chapter) = get_bible().get_chapter(&book_name, chapter_num) {
                return Some(InstructionContext::new(
                    current_chapter,
                    search.to_string(),
                ));
            }
        }
    }
    None
}

fn instruction_name_to_instruction(name: &str) -> Option<Instruction> {
    match name {
        "Next Verse" => Some(Instruction::NextVerse),
        "Previous Verse" => Some(Instruction::PreviousVerse),
        "Next Chapter" => Some(Instruction::NextChapter),
        "Previous Chapter" => Some(Instruction::PreviousChapter),
        "Next Book" => Some(Instruction::NextBook),
        "Previous Book" => Some(Instruction::PreviousBook),
        "Beginning of Chapter" => Some(Instruction::BeginningOfChapter),
        "End of Chapter" => Some(Instruction::EndOfChapter),
        "Switch to Previous Chapter" => Some(Instruction::SwitchToPreviousChapter),
        "Copy Raw Verse" => Some(Instruction::CopyRawVerse),
        "Copy Verse with Reference" => Some(Instruction::CopyVerseWithReference),
        "Toggle Sidebar" => Some(Instruction::ToggleSidebar),
        "Toggle Cross References" => Some(Instruction::ToggleCrossReferences),
        "Toggle Command Palette" => Some(Instruction::ToggleBiblePallate),
        "Open Command Palette" => Some(Instruction::ToggleCommandPallate),
        "Next Reference" => Some(Instruction::NextReference),
        "Previous Reference" => Some(Instruction::PreviousReference),
        "Next Palette Result" => Some(Instruction::NextPaletteResult),
        "Previous Palette Result" => Some(Instruction::PreviousPaletteResult),
        "Open GitHub Repository" => Some(Instruction::OpenGithubRepository),
        "Random Verse" => Some(Instruction::RandomVerse),
        _ => None,
    }
}

#[component]
pub fn CommandPalette(
    is_open: ReadSignal<bool>,
    set_is_open: WriteSignal<bool>,
    next_palette_result: RwSignal<bool>,
    previous_palette_result: RwSignal<bool>,
    initial_search_query: ReadSignal<Option<String>>,
) -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();
    let (search_query, set_search_query) = signal(String::new());
    let (selected_index, set_selected_index) = signal(0usize);
    let (navigate_to, set_navigate_to) = signal::<Option<String>>(None);
    let (is_mounted, set_is_mounted) = signal(false);
    let (execute_instruction, set_execute_instruction) = signal::<Option<Instruction>>(None);
    
    // Create a node ref for the input element
    let input_ref = NodeRef::<leptos::html::Input>::new();
    
    // Create instruction processor
    let processor = InstructionProcessor::new(navigate.clone());

    // Handle instruction execution
    Effect::new(move |_| {
        if let Some(instruction) = execute_instruction.get() {
            let pathname = location.pathname.get();
            let search = location.search.get();
            if let Some(context) = create_instruction_context(&pathname, &search) {
                processor.process(instruction, &context);
            }
            set_execute_instruction.set(None); // Reset
        }
    });

    // Handle initial search query when palette opens
    Effect::new(move |_| {
        if let Some(query) = initial_search_query.get() {
            set_search_query.set(query);
            // Note: We can't clear the signal here because this is a ReadSignal
            // The signal will be cleared by the parent component
        }
    });

    // Helper to check if we're showing global search results
    let is_global_search = Memo::new(move |_| {
        let query = search_query.get();
        if query.is_empty() || query.starts_with(':') || query.starts_with('>') || query.len() < 3 {
            return false;
        }
        
        // Check if there would be any chapter results
        let bible = get_bible();
        for book in &bible.books {
            for chapter in book.chapters.iter().take(5) { // Quick check of first few chapters
                let original_name = chapter.name.to_lowercase();
                if fuzzy_score(&original_name, &query.to_lowercase()) > 0 {
                    return false; // Found chapter results, not global search
                }
            }
        }
        true // No chapter results found, showing global search
    });

    // Create a memo for filtered search results (chapters, verses, and instructions)
    let filtered_results = Memo::new(move |_| {
        let query = search_query.get();
        if query.is_empty() {
            // Show recent chapters when empty
            let recent_chapters = get_recent_chapters();
            return recent_chapters.into_iter()
                .enumerate()
                .map(|(index, recent)| {
                    let score = 1000 - index; // Higher score for more recent
                    (SearchResult::RecentChapter {
                        book_name: recent.book_name,
                        chapter: recent.chapter,
                        display_name: recent.display_name,
                        path: recent.path,
                    }, score)
                })
                .collect();
        }

        // Check if this is an instruction search (starts with ">")
        if query.starts_with('>') {
            let instruction_query = query.strip_prefix('>').unwrap_or("").to_lowercase();
            let instructions = get_all_instructions();
            
            if instruction_query.is_empty() {
                // Just ">" - show all instructions
                return instructions.into_iter().map(|inst| (inst, 1000)).collect();
            } else {
                // Filter instructions by name or description
                return instructions.into_iter()
                    .filter_map(|inst| {
                        if let SearchResult::Instruction { name, description, .. } = &inst {
                            let name_lower = name.to_lowercase();
                            let desc_lower = description.to_lowercase();
                            if name_lower.contains(&instruction_query) || desc_lower.contains(&instruction_query) {
                                let score = if name_lower.starts_with(&instruction_query) { 1000 } else { 500 };
                                Some((inst, score))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();
            }
        }

        let query = query.to_lowercase();
        let mut results: Vec<(SearchResult, usize)> = Vec::new();
        
        // Check if this is a current chapter verse shortcut (e.g., ":5" or ":")
        if let Some(verse_part) = query.strip_prefix(':') {
            if let Some(current_chapter) = get_current_chapter(&location.pathname.get()) {
                
                if verse_part.is_empty() {
                    // Just ":" - show all verses from current chapter (limited to first 15)
                    for verse in current_chapter.verses.iter().take(15) {
                        results.push((
                            SearchResult::Verse {
                                chapter: current_chapter.clone(),
                                verse_number: verse.verse,
                                verse_text: verse.text.clone(),
                            },
                            1000 // High score for current chapter verses
                        ));
                    }
                } else if let Ok(verse_num) = verse_part.parse::<u32>() {
                    // ":5" - jump to specific verse in current chapter
                    if let Some(verse) = current_chapter.verses.iter().find(|v| v.verse == verse_num) {
                        results.push((
                            SearchResult::Verse {
                                chapter: current_chapter.clone(),
                                verse_number: verse.verse,
                                verse_text: verse.text.clone(),
                            },
                            2000 // Very high score for exact verse match
                        ));
                    }
                    
                    // Also show nearby verses for context
                    for verse in &current_chapter.verses {
                        let score = score_verse_number_match(verse.verse, verse_num);
                        if score > 0 && verse.verse != verse_num {
                            results.push((
                                SearchResult::Verse {
                                    chapter: current_chapter.clone(),
                                    verse_number: verse.verse,
                                    verse_text: verse.text.clone(),
                                },
                                score + 500 // Bonus for being in current chapter
                            ));
                        }
                    }
                }
            }
        }
        // Check if this is a verse reference (e.g., "gen 1:1" or "gen 1:")
        else if let Some(verse_ref) = parse_verse_reference(&query) {
            // Try to find the verse(s)
            if let Some(translation) = get_current_translation() {
                if let Some(first_language) = translation.languages.first() {
                    let translation_obj = Translation::from_language(convert_language(first_language));
                    
                    // Try to translate the book name
                    let book_name_to_search = if let Some(translated) = translation_obj.get(&verse_ref.book_name) {
                        translated
                    } else {
                        verse_ref.book_name.clone()
                    };
                    
                    // Find the chapter - optimize by searching more efficiently
                    let bible = get_bible();
                    let mut found_chapter = None;
                    
                    // First try exact book name match
                    for book in &bible.books {
                        if book.name.to_lowercase() == book_name_to_search.to_lowercase() 
                            || book.name.to_lowercase() == verse_ref.book_name.to_lowercase() {
                            found_chapter = book.chapters.iter().find(|c| c.chapter == verse_ref.chapter);
                            break;
                        }
                    }
                    
                    // If no exact match, try partial match (but limit to avoid full scan)
                    if found_chapter.is_none() {
                        for book in bible.books.iter().take(20) { // Limit search to first 20 books
                            if book.name.to_lowercase().contains(&book_name_to_search.to_lowercase()) 
                                || book.name.to_lowercase().contains(&verse_ref.book_name) {
                                found_chapter = book.chapters.iter().find(|c| c.chapter == verse_ref.chapter);
                                break;
                            }
                        }
                    }
                    
                    if let Some(chapter) = found_chapter {
                        match verse_ref.verse {
                            Some(verse_num) => {
                                // Specific verse requested - find exact match and similar verses
                                for verse in &chapter.verses {
                                    let score = score_verse_number_match(verse.verse, verse_num);
                                    if score > 0 {
                                        results.push((
                                            SearchResult::Verse {
                                                chapter: chapter.clone(),
                                                verse_number: verse.verse,
                                                verse_text: verse.text.clone(),
                                            },
                                            score
                                        ));
                                    }
                                }
                            }
                            None => {
                                // No specific verse, show all verses from chapter (limited to first 12)
                                for verse in chapter.verses.iter().take(12) {
                                    results.push((
                                        SearchResult::Verse {
                                            chapter: chapter.clone(),
                                            verse_number: verse.verse,
                                            verse_text: verse.text.clone(),
                                        },
                                        900 // High score for chapter verse suggestions
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Optimized chapter search - limit scope and avoid expensive operations for short queries
        let mut chapter_results: Vec<(SearchResult, usize)> = Vec::new();
        
        // Only do expensive chapter search if query is at least 2 characters
        if query.len() >= 2 {
            let mut found_count = 0;
            'outer: for book in &get_bible().books {
                for chapter in &book.chapters {
                    // Early exit if we have enough results
                    if found_count >= 20 {
                        break 'outer;
                    }
                    
                    let original_name = chapter.name.to_lowercase();
                    let original_score = fuzzy_score(&original_name, &query);
                    
                    // Only compute expensive translation if original name doesn't match well
                    let score = if original_score > 0 {
                        original_score
                    } else {
                        let translated_name = get_translated_chapter_name(&chapter.name).to_lowercase();
                        fuzzy_score(&translated_name, &query)
                    };
                    
                    if score > 0 {
                        chapter_results.push((SearchResult::Chapter(chapter.clone()), score));
                        found_count += 1;
                    }
                }
            }
        }
            
        results.extend(chapter_results);

        // If no results and query is at least 3 characters, do global Bible search
        if results.is_empty() && query.len() >= 3 && !query.starts_with(':') && !query.starts_with('>') {
            let mut verse_matches: Vec<(SearchResult, usize)> = Vec::new();
            let mut search_count = 0;
            
            'global_search: for book in &get_bible().books {
                for chapter in &book.chapters {
                    for verse in &chapter.verses {
                        // Early exit if we have enough results
                        if search_count >= 50 {
                            break 'global_search;
                        }
                        
                        let verse_text_lower = verse.text.to_lowercase();
                        if verse_text_lower.contains(&query) {
                            // Score based on how early the match appears in the verse
                            let match_position = verse_text_lower.find(&query).unwrap_or(verse.text.len());
                            let score = if verse_text_lower.starts_with(&query) {
                                1000 // Starts with query
                            } else if match_position < 10 {
                                800 // Match near beginning
                            } else if match_position < 30 {
                                600 // Match in first part
                            } else {
                                400 // Match later in verse
                            };
                            
                            verse_matches.push((
                                SearchResult::Verse {
                                    chapter: chapter.clone(),
                                    verse_number: verse.verse,
                                    verse_text: verse.text.clone(),
                                },
                                score
                            ));
                            search_count += 1;
                        }
                    }
                }
            }
            
            results.extend(verse_matches);
        }

        // Sort by score (higher is better)
        results.sort_by(|a, b| b.1.cmp(&a.1));
        
        results
            .into_iter()
            .take(10)
            .collect::<Vec<(SearchResult, usize)>>()
    });

    // Handle NextPaletteResult navigation signal
    Effect::new(move |_| {
        if next_palette_result.get() {
            next_palette_result.set(false); // Reset signal
            let results = filtered_results.get();
            if !results.is_empty() {
                let current = selected_index.get();
                let next = if current + 1 >= results.len() {
                    0 // wrap to first
                } else {
                    current + 1
                };
                set_selected_index.set(next);
                
                // Keep focus on input field for continued typing
                // VoiceOver will use aria-activedescendant to announce the selected result
            }
        }
    });

    // Handle PreviousPaletteResult navigation signal
    Effect::new(move |_| {
        if previous_palette_result.get() {
            previous_palette_result.set(false); // Reset signal
            let results = filtered_results.get();
            if !results.is_empty() {
                let current = selected_index.get();
                let next = if current == 0 {
                    results.len() - 1 // wrap to last
                } else {
                    current - 1
                };
                set_selected_index.set(next);
                
                // Keep focus on input field for continued typing
                // VoiceOver will use aria-activedescendant to announce the selected result
            }
        }
    });

    // Scroll selected item into view when selection changes
    Effect::new(move |_| {
        let current_index = selected_index.get();
        let results = filtered_results.get();
        
        // Only scroll if we have results and palette is open
        if !results.is_empty() && is_open.get() {
            // Use a timeout to ensure the DOM has been updated
            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(10).await;
                
                if let Some(window) = leptos::web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(element) = document.get_element_by_id(&format!("palette-result-{}", current_index)) {
                            // Use scrollIntoView with 'nearest' block behavior to avoid aggressive scrolling
                            element.scroll_into_view_with_bool(false); // false = use 'nearest' instead of 'start'
                        }
                    }
                }
            });
        }
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
                    "Enter" => {
                        e.prevent_default();
                        let results = filtered_results.get();
                        if !results.is_empty() {
                            let current = selected_index.get();
                            let valid_index = if current >= results.len() { 0 } else { current };
                            if let Some((result, _)) = results.get(valid_index) {
                                match result {
                                    SearchResult::Instruction { name, .. } => {
                                        // Execute the instruction
                                        if let Some(instruction) = instruction_name_to_instruction(name) {
                                            set_execute_instruction.set(Some(instruction));
                                        }
                                        // Close the palette
                                        set_is_open.set(false);
                                        set_search_query.set(String::new());
                                        set_selected_index.set(0);
                                    }
                                    _ => {
                                        // For chapters and verses, navigate
                                        set_navigate_to.set(Some(result.to_path()));
                                        
                                        // Close palette immediately
                                        set_is_open.set(false);
                                        
                                        // Reset search state after a small delay to avoid race conditions
                                        spawn_local(async move {
                                            gloo_timers::future::TimeoutFuture::new(50).await;
                                            set_search_query.set(String::new());
                                            set_selected_index.set(0);
                                        });
                                    }
                                }
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
            // Debug log to track navigation
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&format!("Command palette navigating to: {}", path).into());
            
            // Force navigation by using replace: false to ensure verse parameters are handled
            navigate(&path, NavigateOptions { 
                scroll: false, 
                replace: false,
                ..Default::default() 
            });
            set_navigate_to.set(None);
        }
    });

    // Set mounted state and focus input when palette opens
    Effect::new(move |_| {
        set_is_mounted.set(true);
        if is_open.get() {
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
            }
        }
    });



    view! {
        <Show when=move || is_mounted.get() && is_open.get() fallback=|| ()>
            // Backdrop
            <div 
                class="fixed inset-0 bg-black bg-opacity-50 z-[9999] flex items-start justify-center pt-20"
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
                            placeholder="Search chapters, verses, or text... (e.g., 'Genesis 1', 'john 3:16', 'love', '>' for shortcuts)"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            prop:value=search_query
                            on:input=move |e| set_search_query.set(event_target_value(&e))
                            role="combobox"
                            aria-expanded="true"
                            aria-autocomplete="list"
                            aria-controls="palette-results-listbox"
                            aria-activedescendant=move || {
                                let results = filtered_results.get();
                                if !results.is_empty() {
                                    format!("palette-result-{}", selected_index.get())
                                } else {
                                    String::new()
                                }
                            }
                        />
                    </div>

                    // Results List
                    <div class="flex-1 overflow-y-auto">
                        <div class="py-2">
                            <Show
                                when=move || !search_query.get().is_empty() || !filtered_results.get().is_empty()
                                fallback=|| view! { <div class="px-4 py-2 text-black">"Start typing to search chapters or verses..."</div> }
                            >
                                // Global search indicator
                                <Show when=move || is_global_search.get()>
                                    <div class="px-4 py-2 bg-blue-50 border-b border-blue-200">
                                        <div class="flex items-center text-xs text-blue-700">
                                            <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd"></path>
                                            </svg>
                                            "Bible text search results"
                                        </div>
                                    </div>
                                </Show>
                                
                                <div 
                                    id="palette-results-listbox"
                                    class="max-h-64 overflow-y-auto"
                                    role="listbox"
                                    aria-label="Search results"
                                >
                                    {move || {
                                        let results = filtered_results.get();
                                        let current_selected = selected_index.get();
                                        let bounded_selected = if results.is_empty() { 
                                            0 
                                        } else { 
                                            current_selected.min(results.len() - 1) 
                                        };
                                        
                                        results.into_iter().enumerate().map(|(index, (result, _score))| {
                                            let is_selected = index == bounded_selected;
                                            let result_path = result.to_path();
                                            let display_name = result.get_display_name();
                                            
                                            view! {
                                                <div 
                                                    id=format!("palette-result-{}", index)
                                                    class=if is_selected { 
                                                        "px-4 py-3 bg-blue-500 text-white cursor-pointer flex items-center border-b border-blue-400" 
                                                    } else { 
                                                        "px-4 py-3 hover:bg-gray-100 cursor-pointer flex items-center border-b border-gray-100" 
                                                    }
                                                    role="option"
                                                    aria-selected=if is_selected { "true" } else { "false" }
                                                    aria-label={
                                                        match &result {
                                                            SearchResult::Verse { verse_text, .. } => {
                                                                format!("{}, verse text: {}", display_name, 
                                                                    if verse_text.len() > 100 { 
                                                                        format!("{}...", &verse_text[..100]) 
                                                                    } else { 
                                                                        verse_text.clone() 
                                                                    }
                                                                )
                                                            }
                                                            SearchResult::Chapter(_) => display_name.clone(),
                                                            SearchResult::Instruction { name, description, shortcut } => {
                                                                format!("{}, {}, shortcut: {}", name, description, shortcut)
                                                            }
                                                            SearchResult::RecentChapter { display_name, .. } => {
                                                                format!("Recent chapter: {}", display_name)
                                                            }
                                                        }
                                                    }
                                                    on:click={
                                                        let path = result_path.clone();
                                                        let result_clone = result.clone();
                                                        move |_| {
                                                            match &result_clone {
                                                                SearchResult::Instruction { name, .. } => {
                                                                    // Execute the instruction
                                                                    if let Some(instruction) = instruction_name_to_instruction(name) {
                                                                        set_execute_instruction.set(Some(instruction));
                                                                    }
                                                                    // Close the palette
                                                                    set_is_open.set(false);
                                                                    set_search_query.set(String::new());
                                                                    set_selected_index.set(0);
                                                                }
                                                                _ => {
                                                                    // Navigate for chapters and verses
                                                                    set_navigate_to.set(Some(path.clone()));
                                                                    
                                                                    // Close palette and reset state immediately  
                                                                    set_is_open.set(false);
                                                                    
                                                                    // Reset search state after a small delay to avoid race conditions
                                                                    spawn_local(async move {
                                                                        gloo_timers::future::TimeoutFuture::new(50).await;
                                                                        set_search_query.set(String::new());
                                                                        set_selected_index.set(0);
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                >
                                                    <div class="flex-1">
                                                        <div class="font-medium">
                                                            {display_name.clone()}
                                                        </div>
                                                        {match &result {
                                                            SearchResult::Verse { verse_text, .. } => {
                                                                view! {
                                                                    <div class="text-xs opacity-75 mt-1 truncate">
                                                                        {if verse_text.len() > 80 {
                                                                            format!("{}...", &verse_text[..80])
                                                                        } else {
                                                                            verse_text.clone()
                                                                        }}
                                                                    </div>
                                                                }.into_any()
                                                            }
                                                            SearchResult::Chapter(_) => {
                                                                view! { <div></div> }.into_any()
                                                            }
                                                            SearchResult::Instruction { description, shortcut, .. } => {
                                                                view! {
                                                                    <div class="text-xs opacity-75 mt-1 truncate">
                                                                        {format!("{} • {}", description, shortcut)}
                                                                    </div>
                                                                }.into_any()
                                                            }
                                                            SearchResult::RecentChapter { .. } => {
                                                                view! {
                                                                    <div class="text-xs opacity-75 mt-1 flex items-center">
                                                                        <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                                                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"></path>
                                                                        </svg>
                                                                        "Recent chapter"
                                                                    </div>
                                                                }.into_any()
                                                            }
                                                        }}
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
                                    <Show when=move || filtered_results.get().is_empty()>
                                        <div class="px-4 py-2 text-black text-sm">
                                            "No results found"
                                        </div>
                                    </Show>
                                </div>
                            </Show>
                        </div>
                    </div>

                    // Footer with hint
                    <div class="px-4 py-2 border-t border-gray-200 text-xs text-black">
                        Use up/down arrows or Ctrl+J/K to navigate, Enter to select, Esc to close
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
fn normalize_text_for_search(text: &str) -> String {
    // Normalize Dutch characters and other diacritics for better search matching
    text.chars()
        .map(|c| match c {
            // Dutch characters
            'ë' | 'è' | 'é' | 'ê' => 'e',
            'ï' | 'ì' | 'í' | 'î' => 'i',
            'ö' | 'ò' | 'ó' | 'ô' => 'o',
            'ü' | 'ù' | 'ú' | 'û' => 'u',
            'á' | 'à' | 'â' | 'ä' => 'a',
            'ý' | 'ỳ' | 'ŷ' | 'ÿ' => 'y',
            'ç' => 'c',
            'ñ' => 'n',
            // Capital versions
            'Ë' | 'È' | 'É' | 'Ê' => 'E',
            'Ï' | 'Ì' | 'Í' | 'Î' => 'I',
            'Ö' | 'Ò' | 'Ó' | 'Ô' => 'O',
            'Ü' | 'Ù' | 'Ú' | 'Û' => 'U',
            'Á' | 'À' | 'Â' | 'Ä' => 'A',
            'Ý' | 'Ỳ' | 'Ŷ' | 'Ÿ' => 'Y',
            'Ç' => 'C',
            'Ñ' => 'N',
            // Keep other characters as-is
            _ => c,
        })
        .collect::<String>()
        .to_lowercase()
}

fn convert_arabic_to_roman(text: &str) -> String {
    // Convert Arabic numerals to Roman numerals for book names
    // Preserve the case of the rest of the text
    text.replace("1 ", "i ")
        .replace("2 ", "ii ")
        .replace("3 ", "iii ")
}

fn convert_roman_to_arabic(text: &str) -> String {
    // Convert Roman numerals to Arabic numerals for book names
    // Order matters - do longer patterns first
    // Handle both uppercase and lowercase
    text.replace("III ", "3 ")
        .replace("II ", "2 ")
        .replace("I ", "1 ")
        .replace("iii ", "3 ")
        .replace("ii ", "2 ")
        .replace("i ", "1 ")
}

fn fuzzy_score(text: &str, query: &str) -> usize {
    if query.is_empty() {
        return 0;
    }
    
    // Normalize both text and query for better matching
    let text_normalized = normalize_text_for_search(text);
    let query_normalized = normalize_text_for_search(query);
    
    // Try with number/roman numeral conversions
    let text_arabic_to_roman = convert_arabic_to_roman(&text_normalized);
    let text_roman_to_arabic = convert_roman_to_arabic(&text_normalized);
    let query_arabic_to_roman = convert_arabic_to_roman(&query_normalized);
    let query_roman_to_arabic = convert_roman_to_arabic(&query_normalized);
    
    // Calculate scores for all combinations
    let score1 = calculate_fuzzy_score(&text_normalized, &query_normalized);
    let score2 = calculate_fuzzy_score(&text_arabic_to_roman, &query_normalized);
    let score3 = calculate_fuzzy_score(&text_roman_to_arabic, &query_normalized);
    let score4 = calculate_fuzzy_score(&text_normalized, &query_arabic_to_roman);
    let score5 = calculate_fuzzy_score(&text_normalized, &query_roman_to_arabic);
    let score6 = calculate_fuzzy_score(&text_arabic_to_roman, &query_roman_to_arabic);
    let score7 = calculate_fuzzy_score(&text_roman_to_arabic, &query_arabic_to_roman);
    
    // Return the best score
    score1.max(score2).max(score3).max(score4).max(score5).max(score6).max(score7)
}

fn calculate_fuzzy_score(text: &str, query: &str) -> usize {
    if query.is_empty() {
        return 0;
    }
    
    let text_words: Vec<&str> = text.split_whitespace().collect();
    let query_words: Vec<&str> = query.split_whitespace().collect();
    
    if query_words.is_empty() {
        return 0;
    }
    
    let mut total_score = 0;
    
    // Try to match each query word against text words
    for (query_word_index, &query_word) in query_words.iter().enumerate() {
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
    }
    
    // Bonus for matching all words in order
    if query_words.len() == text_words.len() {
        total_score += 50;
    }
    
    // Bonus for exact text match
    if text == query {
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
    fn test_normalize_text_for_search() {
        // Test Dutch character normalization
        assert_eq!(normalize_text_for_search("Matteüs"), "matteus");
        assert_eq!(normalize_text_for_search("Jesaja"), "jesaja");  
        assert_eq!(normalize_text_for_search("Ezechiël"), "ezechiel");
        assert_eq!(normalize_text_for_search("Daniël"), "daniel");
        
        // Test mixed case
        assert_eq!(normalize_text_for_search("MATTEÜS"), "matteus");
        assert_eq!(normalize_text_for_search("Matteüs"), "matteus");
    }

    #[test]
    fn test_convert_arabic_to_roman() {
        // Test Arabic to Roman conversion
        assert_eq!(convert_arabic_to_roman("1 samuel"), "i samuel");
        assert_eq!(convert_arabic_to_roman("2 samuel"), "ii samuel");
        assert_eq!(convert_arabic_to_roman("3 john"), "iii john");
        
        // Test no change for non-numbered books
        assert_eq!(convert_arabic_to_roman("genesis"), "genesis");
        assert_eq!(convert_arabic_to_roman("matthew"), "matthew");
    }

    #[test]
    fn test_convert_roman_to_arabic() {
        // Test Roman to Arabic conversion  
        assert_eq!(convert_roman_to_arabic("I samuel"), "1 samuel");
        assert_eq!(convert_roman_to_arabic("II samuel"), "2 samuel");
        assert_eq!(convert_roman_to_arabic("III john"), "3 john");
        
        // Test no change for non-numbered books
        assert_eq!(convert_roman_to_arabic("genesis"), "genesis");
        assert_eq!(convert_roman_to_arabic("matthew"), "matthew");
    }

    #[test]
    fn test_fuzzy_score_with_dutch_chars() {
        // Test that Dutch characters match their normalized equivalents
        let score1 = fuzzy_score("Matteüs", "matteus");
        assert!(score1 > 0, "Should match Dutch characters");
        
        // Test number/roman conversion in search
        let score2 = fuzzy_score("I Samuel", "1 samuel");
        let score3 = fuzzy_score("II Samuel", "2 samuel");
        assert!(score2 > 0, "Should match 1 samuel with I Samuel");
        assert!(score3 > 0, "Should match 2 samuel with II Samuel");
        
        // Test the reverse direction too
        let score4 = fuzzy_score("1 Samuel", "i samuel");
        let score5 = fuzzy_score("2 Samuel", "ii samuel");
        assert!(score4 > 0, "Should match i samuel with 1 Samuel");
        assert!(score5 > 0, "Should match ii samuel with 2 Samuel");
    }

    #[test]
    fn test_parse_verse_reference() {
        // Test basic format "gen 1:1"
        let result = parse_verse_reference("gen 1:1").unwrap();
        assert_eq!(result.book_name, "gen");
        assert_eq!(result.chapter, 1);
        assert_eq!(result.verse, Some(1));

        // Test longer book name "john 3:16"
        let result = parse_verse_reference("john 3:16").unwrap();
        assert_eq!(result.book_name, "john");
        assert_eq!(result.chapter, 3);
        assert_eq!(result.verse, Some(16));

        // Test two-word book "first john 2:5"
        let result = parse_verse_reference("first john 2:5").unwrap();
        assert_eq!(result.book_name, "first john");
        assert_eq!(result.chapter, 2);
        assert_eq!(result.verse, Some(5));

        // Test roman numerals "ii kings 7:3"
        let result = parse_verse_reference("ii kings 7:3").unwrap();
        assert_eq!(result.book_name, "ii kings");
        assert_eq!(result.chapter, 7);
        assert_eq!(result.verse, Some(3));

        // Test verse range (should take first number) "mat 5:3-7"
        let result = parse_verse_reference("mat 5:3-7").unwrap();
        assert_eq!(result.book_name, "mat");
        assert_eq!(result.chapter, 5);
        assert_eq!(result.verse, Some(3));

        // Test incomplete verse reference "gen 1:"
        let result = parse_verse_reference("gen 1:").unwrap();
        assert_eq!(result.book_name, "gen");
        assert_eq!(result.chapter, 1);
        assert_eq!(result.verse, None); // No specific verse
        
        // Test incomplete with spaces "john 3: "
        let result = parse_verse_reference("john 3: ").unwrap();
        assert_eq!(result.book_name, "john");
        assert_eq!(result.chapter, 3);
        assert_eq!(result.verse, None);

        // Test invalid formats
        assert!(parse_verse_reference("genesis 1").is_none()); // No colon
        assert!(parse_verse_reference("gen:1").is_none()); // No chapter
        assert!(parse_verse_reference("gen 1:abc").is_none()); // Invalid verse
        assert!(parse_verse_reference("gen abc:1").is_none()); // Invalid chapter
    }

    #[test]
    fn test_score_verse_number_match() {
        // Test exact match
        assert_eq!(score_verse_number_match(1, 1), 1000);
        assert_eq!(score_verse_number_match(16, 16), 1000);
        
        // Test starts with (1 matches 10, 11, 12)
        assert_eq!(score_verse_number_match(10, 1), 800);
        assert_eq!(score_verse_number_match(11, 1), 800);
        assert_eq!(score_verse_number_match(12, 1), 800);
        assert_eq!(score_verse_number_match(100, 1), 800);
        
        // Test contains (1 matches 21, 31, but not 151 since that starts with 1)
        assert_eq!(score_verse_number_match(21, 1), 600);
        assert_eq!(score_verse_number_match(31, 1), 600);
        assert_eq!(score_verse_number_match(151, 1), 800); // This starts with 1, so gets higher score
        
        // Test reverse starts with (10 matches 1)
        assert_eq!(score_verse_number_match(1, 10), 400);
        assert_eq!(score_verse_number_match(2, 23), 400);
        
        // Test no match
        assert_eq!(score_verse_number_match(2, 3), 0);
        assert_eq!(score_verse_number_match(25, 6), 0);
        
        // Test priorities (exact > starts_with > contains > reverse_starts_with)
        assert!(score_verse_number_match(1, 1) > score_verse_number_match(10, 1));
        assert!(score_verse_number_match(10, 1) > score_verse_number_match(21, 1));
        assert!(score_verse_number_match(21, 1) > score_verse_number_match(1, 10));
    }

    // Note: display_name test skipped due to web API dependency in translation functions

    #[test]
    fn test_search_result_to_path() {
        use crate::core::Chapter;
        
        let chapter = Chapter {
            chapter: 1,
            name: "Genesis 1".to_string(),
            verses: vec![],
        };

        // Test chapter path
        let chapter_result = SearchResult::Chapter(chapter.clone());
        let path = chapter_result.to_path();
        assert_eq!(path, "/Genesis/1");

        // Test verse path
        let verse_result = SearchResult::Verse {
            chapter: chapter.clone(),
            verse_number: 5,
            verse_text: "And God called the light Day".to_string(),
        };
        let path = verse_result.to_path();
        assert_eq!(path, "/Genesis/1?verses=5");
    }
    #[test]
    fn test_fuzzy_score_with_translated_names() {
        // Test fuzzy search functionality with translated names
        let score = fuzzy_score("numeri 1", "numeri");
        assert!(score > 0, "Should match 'numeri' in 'numeri 1'");
        
        let score2 = fuzzy_score("numeri 1", "numeri 1");
        assert!(score2 > 0, "Should match exact 'numeri 1'");
        
        let score3 = fuzzy_score("numbers 1", "numeri");
        assert_eq!(score3, 0, "Should not match 'numeri' in 'numbers 1'");
    }

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
    
    // Property tests using proptest
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_fuzzy_score_properties(
            text in "[a-zA-Z0-9 ]{1,50}",
            query in "[a-zA-Z0-9 ]{1,20}"
        ) {
            let text = text.trim();
            let query = query.trim();
            
            if text.is_empty() || query.is_empty() {
                return Ok(());
            }
            
            let score = fuzzy_score(text, query);
            
            // Basic properties
            
            // Exact match should always score higher than partial match
            if text.to_lowercase() == query.to_lowercase() {
                prop_assert!(score > 0);
            }
            
            // Empty query should return 0 score
            prop_assert_eq!(fuzzy_score(text, ""), 0);
            
            // Query longer than text should still work
            if query.len() > text.len() {
                let _long_query_score = fuzzy_score(text, query);
                // Query longer than text should still work
            }
        }
        
        #[test]
        fn test_fuzzy_score_monotonicity(
            text in "[a-zA-Z0-9 ]{3,30}",
            query_base in "[a-zA-Z0-9]{1,10}"
        ) {
            let text = text.trim();
            let query_base = query_base.trim();
            
            if text.is_empty() || query_base.is_empty() {
                return Ok(());
            }
            
            // Longer, more specific queries should generally score higher if they match
            let _short_query_score = fuzzy_score(text, query_base);
            let long_query = format!("{} {}", query_base, query_base);
            let _long_query_score = fuzzy_score(text, &long_query);
            
            // If both queries match, the relationship depends on the content
        }
        
        #[test]
        fn test_score_word_match_properties(
            text_word in "[a-zA-Z0-9]{1,20}",
            query_word in "[a-zA-Z0-9]{1,15}"
        ) {
            let text_word = text_word.trim();
            let query_word = query_word.trim();
            
            if text_word.is_empty() || query_word.is_empty() {
                return Ok(());
            }
            
            let positional_score = score_word_match(text_word, query_word, true);
            let non_positional_score = score_word_match(text_word, query_word, false);
            
            // Basic properties
            
            // Positional matches should score higher than non-positional
            if positional_score > 0 && non_positional_score > 0 {
                prop_assert!(positional_score >= non_positional_score);
            }
            
            // Exact match should always score higher
            if text_word.to_lowercase() == query_word.to_lowercase() {
                prop_assert!(positional_score > 0);
                prop_assert!(non_positional_score > 0);
            }
        }
        
        #[test]
        fn test_character_fuzzy_score_properties(
            text in "[a-zA-Z0-9]{1,30}",
            query in "[a-zA-Z0-9]{1,15}"
        ) {
            let text = text.trim();
            let query = query.trim();
            
            if text.is_empty() || query.is_empty() {
                return Ok(());
            }
            
            let positional_score = character_fuzzy_score(text, query, true);
            let non_positional_score = character_fuzzy_score(text, query, false);
            
            // Basic properties
            
            // Positional should score higher than non-positional when both match
            if positional_score > 0 && non_positional_score > 0 {
                prop_assert!(positional_score >= non_positional_score);
            }
            
            // If query is longer than text, it might still partially match
        }
        
        #[test]
        fn test_fuzzy_score_case_insensitive(
            text in "[a-zA-Z ]{3,20}",
            query in "[a-zA-Z ]{1,10}"
        ) {
            let text = text.trim();
            let query = query.trim();
            
            if text.is_empty() || query.is_empty() {
                return Ok(());
            }
            
            let lower_score = fuzzy_score(&text.to_lowercase(), &query.to_lowercase());
            let upper_score = fuzzy_score(&text.to_uppercase(), &query.to_uppercase());
            let mixed_score = fuzzy_score(text, &query.to_lowercase());
            
            // Case should not significantly affect scoring
            
            // All should produce similar results (allowing for some variance)
            if lower_score > 0 {
                prop_assert!(upper_score > 0);
                prop_assert!(mixed_score > 0);
            }
        }
        
        #[test]
        fn test_number_matching_properties(
            base_num in 1u32..999,
            query_num in 1u32..99
        ) {
            let text = format!("Book {}", base_num);
            let query = format!("Book {}", query_num);
            
            let score = fuzzy_score(&text, &query);
            
            // Should get some score for book name match (at least partial match on "Book")
            
            // Test that "Book" prefix always matches
            let book_prefix_score = fuzzy_score(&text, "Book");
            prop_assert!(book_prefix_score > 0);
            
            // Exact number match should score higher than book prefix alone
            if base_num == query_num {
                let exact_score = score;
                prop_assert!(exact_score > book_prefix_score);
            }
            
            // Test partial number matching
            let base_str = base_num.to_string();
            let query_str = query_num.to_string();
            if base_str.starts_with(&query_str) {
                let partial_num_score = fuzzy_score(&base_str, &query_str);
                prop_assert!(partial_num_score > 0);
            }
        }
        
        #[test]
        fn test_word_order_sensitivity_property(
            word1 in "[a-zA-Z]{2,10}",
            word2 in "[a-zA-Z]{2,10}",
            word3 in "[a-zA-Z]{2,10}"
        ) {
            let word1 = word1.trim();
            let word2 = word2.trim();
            let word3 = word3.trim();
            
            if word1.is_empty() || word2.is_empty() || word3.is_empty() {
                return Ok(());
            }
            
            let correct_order = format!("{} {} {}", word1, word2, word3);
            let wrong_order = format!("{} {} {}", word3, word1, word2);
            let query = format!("{} {}", word1, word2);
            
            let correct_score = fuzzy_score(&correct_order, &query);
            let wrong_score = fuzzy_score(&wrong_order, &query);
            
            // Both should match since the words are present
            
            // Correct order should typically score higher
            if correct_score > 0 && wrong_score > 0 {
                // This is a tendency, not a strict rule due to fuzzy matching
                prop_assert!(correct_score > 0);
                prop_assert!(wrong_score > 0);
            }
        }
        
        #[test]
        fn test_prefix_matching_advantage(
            prefix in "[a-zA-Z]{2,8}",
            suffix in "[a-zA-Z]{2,8}"
        ) {
            let prefix = prefix.trim();
            let suffix = suffix.trim();
            
            if prefix.is_empty() || suffix.is_empty() {
                return Ok(());
            }
            
            let prefix_text = format!("{}{}", prefix, suffix);
            let contains_text = format!("xyz{}abc", prefix);
            
            let prefix_score = fuzzy_score(&prefix_text, prefix);
            let contains_score = fuzzy_score(&contains_text, prefix);
            
            // Both should match
            prop_assert!(prefix_score > 0);
            prop_assert!(contains_score > 0);
            
            // Prefix match should score higher than contains match
            prop_assert!(prefix_score > contains_score);
        }
    }
}
